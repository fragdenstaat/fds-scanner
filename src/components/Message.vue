<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Nachricht</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button :default-href="backHref" text="Zurück"></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <error-message v-if="error" :message="error" />
            <template v-else-if="message">
                <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                    <ion-refresher-content></ion-refresher-content>
                </ion-refresher>
                <h5 v-if="request">Nachricht in Anfrage „{{ request.title }}“ <small>[#{{ request.id }}]</small></h5>
                <h5 v-else>
                    <ion-skeleton-text :animated="true" style="width: 100%;"></ion-skeleton-text>
                    <ion-skeleton-text :animated="true" style="width: 40%;"></ion-skeleton-text>
                </h5>

                <h2 v-if="message.subject">{{ message.subject }}</h2>
                <h2 v-else><em>(kein Betreff)</em></h2>
                <p>
                    <template v-if="message.sender">von {{ message.sender }}</template>
                    vom {{ message.timestamp_label }}
                </p>

                <ion-card v-if="scanDoneAfterDeeplink && message.is_draft" color="secondary">
                    <ion-card-header>
                        <ion-card-title>Dokument wurde hochgeladen</ion-card-title>
                    </ion-card-header>
                    <ion-card-content>
                        <p>
                            Scannen Sie ein weiteres Dokument oder führen Sie das Anlegen der Postnachricht auf der
                            Webseite weiter.
                        </p>
                    </ion-card-content>
                </ion-card>
                <ion-card v-if="message.is_draft && request" color="warning">
                    <ion-card-header>
                        <ion-card-title>Entwurf</ion-card-title>
                    </ion-card-header>
                    <ion-card-content>
                        <p>Diese Nachricht ist noch ein Entwurf. Bitte vollenden Sie das Anlegen der Postnachricht.</p>

                        <ion-button :router-link="scanPath" router-direction="forward">
                            Scanne weiteres Dokument
                        </ion-button>
                        <hr />
                        <p>
                            Haben Sie alle Dokumente gescannt?
                        </p>
                        <ion-button @click="openPostUpload">
                            <ion-icon slot="start" :icon="openOutline" aria-label="Öffnen"></ion-icon>
                            Weiter auf der Webseite
                        </ion-button>
                    </ion-card-content>
                </ion-card>
                <div v-else class="ion-padding ion-text-center">
                    <ion-button :router-link="scanPath" router-direction="forward">
                        <template v-if="highlightAttachment">
                            Scanne weiteres Dokument
                        </template>
                        <template v-else>
                            Scanne Dokument
                        </template>
                    </ion-button>
                </div>

                <ion-list v-if="store.attachments.length > 0">
                    <ion-item v-for="attachment in store.attachments" :detail="false" :key="attachment.id" href="#"
                        :color="attachment.id === highlightAttachment ? 'success' : undefined"
                        @click="openAttachment(attachment.site_url)">
                        <ion-label>
                            <h2>{{ attachment.name }}</h2>
                            <p>
                                {{ attachment.filetype_label }} / {{ attachment.size_label }}
                            </p>
                        </ion-label>
                        <ion-icon v-if="!message.is_draft" slot="end" :icon="openOutline"
                            aria-label="Öffnen"></ion-icon>
                    </ion-item>
                </ion-list>
                <div v-if="loading" class="ion-text-center">
                    <ion-spinner></ion-spinner>
                </div>
                <div v-if="errorMessage" class="ion-text-center">
                    <p>{{ errorMessage }}</p>
                </div>
            </template>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import {
    IonBackButton,
    IonButton,
    IonButtons,
    IonCard,
    IonCardContent,
    IonCardHeader, IonCardTitle,
    IonContent, IonHeader,
    IonIcon,
    IonItem, IonLabel,
    IonList,
    IonPage,
    IonRefresher, IonRefresherContent,
    IonSkeletonText,
    IonSpinner,
    IonTitle, IonToolbar
} from '@ionic/vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import { openOutline } from 'ionicons/icons';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { account } from '../account.ts';
import { useFoiAttachmentsStore } from '../stores/foiattachments.ts';
import { FoiMessage, useFoiMessagesStore } from '../stores/foimessages.ts';
import { FoiRequest, useFoiRequestsStore } from '../stores/foirequests.ts';
import { useStoreLoader } from '../utils.ts';
import ErrorMessage from "./ErrorMessage.vue";


const foimessageStore = useFoiMessagesStore()
const foirequestStore = useFoiRequestsStore()
const store = useFoiAttachmentsStore()
const route = useRoute<"message">();

const messageId = parseInt(route.params.id);
const scanPath = `/message/${messageId}/scan/`;
const error = ref<string | null>(null)
let backHref = ref<string>("/")

let highlightAttachmentParam = route.query.highlight_attachment
let highlightAttachment: number | null = null
if (highlightAttachmentParam) {
    highlightAttachment = parseInt(highlightAttachmentParam as string)
}
const scanDoneAfterDeeplink = computed(() => {
    return account.mountedWithDeepUrl() && highlightAttachment !== null
})

const { loading, errorMessage, loadStoreObjects } = useStoreLoader(() => {
    return store.getAttachments(messageId);
});

let message: FoiMessage | null = null
let request = ref<FoiRequest | null>(null)
try {
    message = await foimessageStore.getMessage(messageId);
    backHref.value = `/request/${message.request_id}/`;
} catch (e) {
    error.value = (e as Error).toString()
}

onMounted(async () => {
    if (message === null) {
        return
    }
    try {
        let result = await Promise.all([
            foirequestStore.getRequest(message.request_id),
            store.getAttachments(messageId)
        ])
        request.value = result[0]
        loading.value = false;
    } catch (e) {
        error.value = (e as Error).toString()
    }
});
onUnmounted(() => {
    store.clearAttachments();
});

async function handleRefresh(event: CustomEvent) {
    try {
        await loadStoreObjects()
    } finally {
        event.target?.complete();
    }
}

function openAttachment(url: string) {
    if (message && !message.is_draft) {
        openUrl(url);
    }
}

function openPostUpload() {
    if (request.value) {
        const url = `https://fragdenstaat.de${request.value.url}postnachricht-erstellen/`
        openUrl(url);
    }
}

</script>

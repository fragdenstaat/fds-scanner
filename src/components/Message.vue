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
            <template v-else>
                <h2 v-if="message.subject">{{ message.subject }}</h2>
                <h2 v-else><em>(kein Betreff)</em></h2>

                <p>
                    <template v-if="message.sender">von {{ message.sender }}</template>
                    am {{ message.timestamp_label }}
                </p>
                <ion-badge v-if="message.is_draft">Entwurf</ion-badge>

                <div class="ion-padding ion-text-center">
                    <ion-button :router-link="scanPath" router-direction="forward">
                        Scanne Dokument
                    </ion-button>
                </div>
                <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                    <ion-refresher-content></ion-refresher-content>
                </ion-refresher>

                <ion-list>
                    <ion-item v-for="attachment in store.attachments"
                        :color="attachment.id === highlightAttachment ? 'success' : undefined">
                        <ion-label>
                            <h2>{{ attachment.name }}</h2>
                            <p>
                                {{ attachment.filetype_label }} / {{ attachment.size_label }}
                            </p>
                        </ion-label>
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
    IonBackButton, IonButton,
    IonButtons, IonContent, IonHeader,
    IonItem, IonLabel,
    IonList,
    IonPage,
    IonRefresher, IonRefresherContent,
    IonSpinner,
    IonTitle, IonToolbar,
} from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { useFoiAttachmentsStore } from '../stores/foiattachments.ts';
import { FoiMessage, useFoiMessagesStore } from '../stores/foimessages.ts';
import { useStoreLoader } from '../utils.ts';
import ErrorMessage from "./ErrorMessage.vue";


const foimessageStore = useFoiMessagesStore()
const store = useFoiAttachmentsStore()
const route = useRoute<"message">();

const messageId = parseInt(route.params.id);
const scanPath = `/message/${messageId}/scan`;
const error = ref<string | null>(null)
let backHref = ref<string>("/")

let highlightAttachmentParam = route.query.highlight_attachment
let highlightAttachment: number | null = null
if (highlightAttachmentParam) {
    highlightAttachment = parseInt(highlightAttachmentParam as string)
}

const { loading, errorMessage, loadStoreObjects } = useStoreLoader(() => {
    return store.getAttachments(messageId);
});

let message: FoiMessage
try {
    message = await foimessageStore.getMessage(messageId);
    backHref.value = `/request/${message.request_id}/`;
} catch (e) {
    error.value = (e as Error).toString()
}

onMounted(async () => {
    await store.getAttachments(messageId);
    loading.value = false;
});

async function handleRefresh(event: CustomEvent) {
    await loadStoreObjects()
    event.target?.complete();
}


</script>

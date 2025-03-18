<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Anhang</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button :default-href="backHref" text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-button slot="end">
                    <ion-icon slot="icon-only" :icon="shareOutline"></ion-icon>
                </ion-button>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <error-message v-if="error" :message="error" />
            <template v-else>
                <h6 v-if="request && message">Anhang an Nachricht vom {{ message.timestamp_label }} – „{{ request.title
                    }}“
                    <small>[#{{ request.id }}]</small>
                </h6>
                <h6 v-else>
                    <ion-skeleton-text :animated="true" style="width: 100%;"></ion-skeleton-text>
                    <ion-skeleton-text :animated="true" style="width: 60%;"></ion-skeleton-text>
                </h6>

                <h2>{{ attachment.name }}</h2>
                <p>
                    {{ attachment.filetype_label }} / {{ attachment.size_label }}
                </p>

                <div v-if="error" class="ion-text-center">
                    <p>{{ error }}</p>
                </div>

                <ion-button :href="attachment.file_url" target="_blank" rel="noopener noreferrer">
                    Datei öffnen
                </ion-button>

            </template>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import {
    IonBackButton,
    IonButton,
    IonButtons,
    IonContent, IonHeader,
    IonPage,
    IonSkeletonText,
    IonTitle, IonToolbar
} from '@ionic/vue';
import { shareOutline } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { FoiAttachment, useFoiAttachmentsStore } from '../stores/foiattachments.ts';
import { FoiMessage, useFoiMessagesStore } from '../stores/foimessages.ts';
import { FoiRequest, useFoiRequestsStore } from '../stores/foirequests.ts';
import ErrorMessage from "./ErrorMessage.vue";


const foiattachmentStore = useFoiAttachmentsStore()
const foimessageStore = useFoiMessagesStore()
const foirequestStore = useFoiRequestsStore()
const route = useRoute<"message">();

const attachmentId = parseInt(route.params.id);
const error = ref<string | null>(null)
let backHref = ref<string>("/")

let attachment: FoiAttachment
let message = ref<FoiMessage | null>(null)
let request = ref<FoiRequest | null>(null)
try {
    attachment = await foiattachmentStore.getAttachment(attachmentId);
    backHref.value = attachment.message_path;
} catch (e) {
    error.value = (e as Error).toString()
}

onMounted(async () => {
    message.value = await foimessageStore.getMessage(attachment.message_id);
    request.value = await foirequestStore.getRequest(message.value.request_id);
});


</script>

<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Nachricht</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button text="Zurück"></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            <h2>{{ message.subject }}</h2>
            <p>von {{ message.sender }}</p>

            <div class="ion-padding ion-text-center">
                <ion-button :router-link="scanPath" router-direction="forward">
                    Scanne Dokument
                </ion-button>
            </div>

            <ion-list>
                <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                    <ion-refresher-content></ion-refresher-content>
                </ion-refresher>

                <ion-item v-for="attachment in store.attachments">
                    <ion-label>
                        <h2>{{ attachment.name }}</h2>
                        <p>
                            {{ attachment.filetype }} / {{ attachment.size }} Bytes
                        </p>
                    </ion-label>
                </ion-item>
            </ion-list>
            <div v-if="loading" class="ion-text-center">
                <ion-spinner v-if="loading"></ion-spinner>
            </div>
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
    IonTitle, IonToolbar
} from '@ionic/vue';
import { ref } from 'vue';

import { onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useFoiAttachmentsStore } from '../stores/foiattachments.ts';
import { FoiMessage, useFoiMessagesStore } from '../stores/foimessages.ts';


const foimessageStore = useFoiMessagesStore()
const store = useFoiAttachmentsStore()

const route = useRoute<"message">();
const messageId = parseInt(route.params.id);
const scanPath = `/message/${messageId}/scan`;
const loading = ref(true)

const message: FoiMessage = foimessageStore.messageMap.get(messageId)!

onMounted(async () => {
    await store.getAttachments(messageId);
    loading.value = false;
});

async function handleRefresh(event: CustomEvent) {
    await store.getAttachments(messageId);
    event.target?.complete();
}


</script>
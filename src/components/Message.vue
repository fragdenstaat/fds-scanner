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
            <ion-spinner v-if="loading" class="ion-text-center"></ion-spinner>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import {
    IonBackButton,
    IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToolbar
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
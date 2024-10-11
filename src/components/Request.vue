<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-title>Anfrage</ion-title>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            <h2>{{ request.title }} <small>[#{{ request.id }}]</small></h2>
            <p>vom {{ request.created_at_label }}</p>

            <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                <ion-refresher-content></ion-refresher-content>
            </ion-refresher>

            <template v-if="store.messages.length === 0">
                <p>Keine Postnachrichten vorhanden</p>
            </template>

            <ion-list>
                <ion-item v-for="message in store.messages" :router-link="'/message/' + message.id"
                    router-direction="forward">
                    <ion-label>
                        <h2 v-if="message.subject">{{ message.subject }}</h2>
                        <h2 v-else><em>(kein Betreff)</em></h2>
                        <p>
                            {{ message.timestamp_label }}
                            von {{ message.sender }}
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
    IonBackButton,
    IonButtons,
    IonContent, IonHeader,
    IonItem,
    IonLabel,
    IonList,
    IonPage,
    IonRefresher, IonRefresherContent,
    IonSpinner,
    IonTitle, IonToolbar
} from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { FoiRequest, useFoiRequestsStore } from '../stores/foirequests.ts';

import { useFoiMessagesStore } from '../stores/foimessages.ts';


const foirequestStore = useFoiRequestsStore()
const store = useFoiMessagesStore()

const route = useRoute<"request">();
const requestId = parseInt(route.params.id);
const loading = ref(true)

const request: FoiRequest = foirequestStore.requestMap.get(requestId)!

onMounted(async () => {
    await store.getMessages(requestId);
    loading.value = false;
});

async function handleRefresh(event: CustomEvent) {
    await store.getMessages(requestId);
    event.target?.complete();
}


</script>
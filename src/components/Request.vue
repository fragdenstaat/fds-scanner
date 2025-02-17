<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button default-href="/" text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-title>Anfrage</ion-title>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            <error-message v-if="error" :message="error" />
            <template v-else>
                <h2>{{ request.title }} <small>[#{{ request.id }}]</small></h2>
                <p>vom {{ request.created_at_label }}</p>

                <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                    <ion-refresher-content></ion-refresher-content>
                </ion-refresher>


                <!-- <ion-button :router-link="createMessageUrl" router-direction="forward">
                Neue Postnachricht anlegen
            </ion-button> -->

                <template v-if="store.messages.length === 0">
                    <p>Keine Postnachrichten vorhanden</p>
                </template>

                <ion-list>
                    <ion-list-header v-if="store.messages.length > 0">
                        <ion-label>Bisherige Postnachrichten</ion-label>
                    </ion-list-header>
                    <ion-item v-for="message in store.messages" :router-link="'/message/' + message.id"
                        router-direction="forward">
                        <ion-label>
                            <h2 v-if="message.subject">{{ message.subject }}</h2>
                            <h2 v-else><em>(kein Betreff)</em></h2>
                            <p>
                                {{ message.timestamp_label }}
                                <template v-if="message.sender">von {{ message.sender }}</template>
                            </p>
                        </ion-label>
                        <ion-badge v-if="message.is_draft" slot="end">Entwurf</ion-badge>
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
    IonButtons,
    IonContent, IonHeader,
    IonItem,
    IonLabel,
    IonList,
    IonListHeader,
    IonPage,
    IonRefresher, IonRefresherContent,
    IonSpinner,
    IonTitle, IonToolbar
} from '@ionic/vue';
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { useFoiMessagesStore } from '../stores/foimessages.ts';
import { FoiRequest, useFoiRequestsStore } from '../stores/foirequests.ts';
import { useStoreLoader } from '../utils.ts';
import ErrorMessage from "./ErrorMessage.vue";


const foirequestStore = useFoiRequestsStore()
const store = useFoiMessagesStore()

const route = useRoute<"request">();
const requestId = parseInt(route.params.id);
const error = ref<string | null>(null)

// const createMessageUrl = `/request/${requestId}/create-message/`;
let request: FoiRequest
try {
    request = await foirequestStore.getRequest(requestId);
} catch (e) {
    console.error(e);
    error.value = (e as Error).toString()
}

const { loading, errorMessage, loadStoreObjects } = useStoreLoader(() => {
    return store.getMessages(requestId);
});

async function handleRefresh(event: CustomEvent) {
    await loadStoreObjects();
    event.target?.complete();
}
</script>
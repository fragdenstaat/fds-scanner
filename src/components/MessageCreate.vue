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
            <h2>Neue Postnachricht anlegen</h2>
            <p>
                Für Anfrage „{{ request.title }}“ <small>[#{{ request.id }}]</small>
            </p>

            <ion-segment v-model="letterSent">
                <ion-segment-button :value="0">
                    <ion-label>Post erhalten</ion-label>
                </ion-segment-button>
                <ion-segment-button :value="1">
                    <ion-label>Post gesendet</ion-label>
                </ion-segment-button>
            </ion-segment>

            <!-- Behörde wählen? -->

            <p>
                {dateLabel}
            </p>
            <ion-datetime locale="de-DE" presentation="date" v-model="receivedDate" :min="minDate"
                :max="today"></ion-datetime>

            <template v-if="letterSent === 0">
                <ion-checkbox>Es handelt sich um einen gelben Brief</ion-checkbox>
            </template>

        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import {
    IonBackButton,
    IonButtons,
    IonCheckbox,
    IonContent,
    IonDatetime,
    IonHeader,
    IonLabel,
    IonPage,
    IonSegment,
    IonSegmentButton,
    IonTitle, IonToolbar
} from '@ionic/vue';
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';
import { FoiRequest, useFoiRequestsStore } from '../stores/foirequests.ts';


const foirequestStore = useFoiRequestsStore()
const route = useRoute<"create-message">();
const requestId = parseInt(route.params.id);
const today = new Date().toISOString();

const request: FoiRequest = foirequestStore.requestMap.get(requestId)!
const minDate = request.created_at_date

const letterSent = ref(0)
const receivedDate = ref(today)
const dateLabel = computed(() => letterSent.value === 0 ? "Erhalten am" : "Gesendet am")


</script>
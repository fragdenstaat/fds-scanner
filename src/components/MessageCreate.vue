<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button :default-href="`/request/${requestId}/`" text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-title>Anfrage</ion-title>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            <error-message v-if="error" :message="error" />
            <template v-else>
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

                <ion-list>
                    <ion-item>
                        <ion-select v-model="publicBody" :label="publicBodyLabel" label-placement="stacked"
                            aria-label="Fruit" interface="modal" placeholder="Behörde wählen">
                            <ion-select-option v-for="pb in publicBodyOptions" :value="pb">{{ pb.name
                                }}</ion-select-option>
                        </ion-select>
                    </ion-item>
                    <ion-item>
                        <ion-label position="stacked">{{ dateLabel }}</ion-label>
                        <ion-datetime locale="de-DE" presentation="date" v-model="messageDate" :min="minDate"
                            :max="today"></ion-datetime>
                    </ion-item>
                </ion-list>

                <ion-button class="ion-padding" expand="block" @click="createMessage">Nachricht erstellen</ion-button>
            </template>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import {
    IonBackButton,
    IonButton,
    IonButtons,
    IonContent,
    IonDatetime,
    IonHeader,
    IonItem,
    IonLabel,
    IonList,
    IonPage,
    IonSegment,
    IonSegmentButton,
    IonSelect,
    IonSelectOption,
    IonTitle, IonToolbar,
    loadingController,
    useIonRouter
} from '@ionic/vue';
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';
import { useFoiMessagesStore } from '../stores/foimessages.ts';
import { FoiRequest, PublicBody, useFoiRequestsStore } from '../stores/foirequests.ts';
import ErrorMessage from './ErrorMessage.vue';


const foirequestStore = useFoiRequestsStore()
const foimessageStore = useFoiMessagesStore()
const route = useRoute<"create-message">();
const requestId = parseInt(route.params.id);
const ionRouter = useIonRouter();
const today = new Date().toISOString();

let request: FoiRequest;
try {
    request = await foirequestStore.getRequest(requestId);
} catch (e) {
    ionRouter.replace("/");
}

const minDate = computed(() => request!.created_at_date.toISOString())

const error = ref<string | null>(null)
const letterSent = ref(0)
const publicBody = ref<PublicBody>(request!.public_body)
const isResponse = computed(() => letterSent.value === 0)
const publicBodyLabel = computed(() => isResponse.value ? `Antwort von` : `Brief an`)
const publicBodyOptions = computed(() => [request!.public_body])

const messageDate = ref(today)
const dateLabel = computed(() => isResponse.value ? "Erhalten am" : "Gesendet am")

async function createMessage() {
    const loading = await loadingController.create({
        message: 'Nachricht wird erstellt...',
    });

    await loading.present();
    try {
        const newMessage = await foimessageStore.createMessage({
            request: request.resource_uri,
            timestamp: messageDate.value,
            kind: "post",
            is_response: isResponse.value,
            sender_public_body: isResponse ? publicBody.value.resource_uri : null,
            recipient_public_body: isResponse ? null : request.public_body.resource_uri,
        })
        ionRouter.navigate(`/message/${newMessage.id}/`, 'none', 'pop');
    } catch (e) {
        console.error(e)
        error.value = (e as Error).toString()
    } finally {
        await loading.dismiss();
    }
}

</script>
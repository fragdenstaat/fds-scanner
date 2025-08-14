<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Dokumentenscan</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button :default-href="messagePath" text="Zurück"></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <div v-if="!initializing" class="ion-text-center">
                <p v-if="errorMessage">{{ errorMessage }}</p>
                <ion-button @click="startScan">Scan starten</ion-button>
            </div>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import { IonBackButton, IonButton, IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToolbar, loadingController, onIonViewDidEnter, onIonViewDidLeave, onIonViewWillEnter, useIonRouter } from '@ionic/vue';
import { addPluginListener, invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { FoiAttachment } from '../stores/foiattachments';
import { FoiMessage, useFoiMessagesStore } from '../stores/foimessages';
import { useToastMessages } from '../utils';

interface PdfProgress {
    page: number
    total: number
}

interface UploadProgress {
    event: string
    payload: string
}

useToastMessages()

const foimessageStore = useFoiMessagesStore()
const route = useRoute<"message">();

const messageId = parseInt(route.params.id);
const messagePath = `/message/${messageId}/`;

const ionRouter = useIonRouter();
const errorMessage = ref<string>("");
const initializing = ref<boolean>(true);


let message: FoiMessage | null = null;
try {
    message = await foimessageStore.getMessage(messageId);
} catch (e) {
    errorMessage.value = (e as Error).toString()
}

function handlePluginEvent(event: PdfProgress) {
    console.log(event);
    if (event.page === 0) {
        loading!.message = "PDF wird erstellt..."
    } else {
        loading!.message = `PDF wird erstellt, Seite ${event.page} von ${event.total}...`
    }
}

let loading: HTMLIonLoadingElement;
let unlisten: null | (() => void) = null;

onIonViewDidEnter(async () => {
    if (message === null) {
        return
    }
    loading = await loadingController.create({
        message: '',
    });
    await addPluginListener(
        'documentcamera',
        'pdfprogress',
        handlePluginEvent
    )
    unlisten = await listen<string>("scan-progress", (state: UploadProgress) => {
        console.log("Scan progress state", state);
        if (state.payload === "upload_created") {
            loading.message = "Dokument wird hochgeladen..."
        } else if (state.payload === "upload_complete") {
            loading.message = "Erstelle Anhang..."
        } else if (state.payload === "attachment_created") {
            loading.message = "Anhang erstellt!"
        }
    })
    await startScan();
});

async function showError(message: string) {
    console.error(message);
    errorMessage.value = message;
    initializing.value = false;
    await loading.dismiss();
}


async function startScan() {
    if (message === null) {
        return
    }
    // Is there anything left to upload?
    loading.message = "Vorbereitung..."
    await loading.present();
    try {
        const lastUploadContinued = await invoke("upload_document")
        if (lastUploadContinued) {
            console.log("Last upload now ok")
            await showError("Letzter Upload wurde fortgesetzt.")
            return
        }
    } catch (e) {
        console.warn("Last upload errored")
        await showError(e!.toString())
        return
    }
    initializing.value = false;
    loading.message = "Starte Scan..."
    console.log("Starting scan")
    try {
        const scanOk = await invoke("scan_document", { message_resource_uri: message.resource_uri })
        if (!scanOk) {
            console.warn("Scan canceled")
            await showError("Scan canceled")
            return
        }
    } catch (e) {
        await showError(e!.toString())
        return
    }
    console.log("Uploading document")
    loading!.message = "Lade Dokument hoch..."
    let attachment: FoiAttachment | null = null;
    try {
        attachment = await invoke("upload_document")
        if (attachment === null) {
            await showError("Upload fehlgeschlagen!")
            return
        }
    } catch (e) {
        await showError(e!.toString())
        return
    }
    await loading.dismiss();
    ionRouter.navigate(`${messagePath}?highlight_attachment=${attachment.id}`, 'back', 'replace');
}

onIonViewWillEnter(() => {
    if (loading) {
        loading.dismiss();
    }
});

onIonViewDidLeave(() => {
    if (unlisten) {
        unlisten();
    }
    unlisten = null
    if (loading) {
        loading.dismiss();
    }
});



</script>
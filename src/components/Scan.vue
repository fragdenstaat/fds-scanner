<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Dokumentenscan</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button text="Zurück"></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <div class="ion-text-center">
                <p v-if="errorMessage">{{ errorMessage }}</p>
                <ion-button @click="startScan">Scan starten</ion-button>
            </div>
            <ion-toast v-if="errorMessage" :message="errorMessage" :duration="5000"></ion-toast>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import { IonBackButton, IonButton, IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToast, IonToolbar, loadingController, useIonRouter } from '@ionic/vue';
import { addPluginListener, invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';


const route = useRoute<"message">();
const messageId = parseInt(route.params.id);
const messagePath = `/message/${messageId}/`;

const ionRouter = useIonRouter();
const errorMessage = ref<string>("");

interface PdfProgress {
    page: number
    total: number
}

interface UploadProgress {
    event: string
    payload: string
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

onMounted(async () => {
    loading = await loadingController.create({
        message: '',
    });
    await addPluginListener(
        'documentcamera',
        'pdfprogress',
        handlePluginEvent
    )
    listen<string>("scan-progress", (state: UploadProgress) => {
        console.log("Scan progress state", state);
        if (state.payload === "upload_created") {
            loading.message = "Dokument wird hochgeladen..."
        } else if (state.payload === "upload_complete") {
            loading.message = "Erstelle Anhang..."
        } else if (state.payload === "attachment_created") {
            loading.message = "Anhang erstellt!"
        }
    });

    await startScan();
});

async function showError(message: string) {
    console.error(message);
    errorMessage.value = message;
    await loading.dismiss();
}


async function startScan() {
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
    loading.message = "Start Scan..."
    console.log("Starting scan")
    try {
        const scanOk = await invoke("scan_document", { message_id: messageId })
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
    try {
        const uploadOk = await invoke("upload_document")
        if (!uploadOk) {
            await showError("Kein Dokument gefunden!")
            return
        }
    } catch (e) {
        await showError(e!.toString())
        return
    }
    await loading.dismiss();
    ionRouter.navigate(messagePath, 'back', 'pop');
}

</script>
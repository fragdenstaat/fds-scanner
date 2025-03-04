<template>
    <ion-page>
        <ion-header translucent>
            <ion-toolbar>
                <ion-title>QR Code</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button default-href="/login/" text="Abbrechen"></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>
        <ion-content id="qrcode-content" class="ion-padding" fullscreen>
            <template v-if="denied">
                <h3>Die Kamera-Berechtigung wurde verweigert</h3>
                <ion-button @click="openAppSettings">Einstellungen anpassen</ion-button>
            </template>
            <template v-else>
                <template v-if="processing">
                </template>
                <template v-else-if="scanning">
                    <div class="ion-padding background ion-text-center">
                        <h3>Scannen Sie den QR Code</h3>
                    </div>
                </template>
                <template v-else>
                    <div class="ion-text-center">
                        <ion-button @click="startScan">Kamera starten</ion-button>
                    </div>
                </template>
            </template>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">
import { cancel, checkPermissions, Format, openAppSettings, requestPermissions, scan } from '@tauri-apps/plugin-barcode-scanner';

import { alertController, IonBackButton, IonButton, IonButtons, IonContent, IonHeader, IonPage, IonToolbar, loadingController, onIonViewDidEnter, useIonRouter } from '@ionic/vue';
import { onUnmounted, ref } from 'vue';
import { account } from '../account.ts';

let scanning = ref(true);
let bodyBackground = ref("transparent");
let processing = ref(false);
let denied = ref(false);
const ionRouter = useIonRouter();

async function setupScan() {
    const loading = await loadingController.create({
        message: 'Starte Kamera...',
    });
    await loading.present();
    const permissionState = await checkPermissions();
    if (permissionState === 'granted') {
        await loading.dismiss();
    } else {
        let result = await requestPermissions();
        await loading.dismiss();
        if (!result) {
            denied.value = true
            return
        }
    }
    await startScan();
};

async function startScan() {
    scanning.value = true;
    let scanResult
    try {
        scanResult = await scan({ windowed: true, formats: [Format.QRCode] });
    } catch (e) {
        console.warn("Scan failed", e);
        scanning.value = false;
        return
    }
    bodyBackground.value = "inherit";
    scanning.value = false;
    const loading = await loadingController.create({
        message: 'Login wird vorbereitet...',
    });
    await loading.present();
    if (!scanResult.content) {
        await loading.dismiss();
        return;
    }
    await cancel();
    processing.value = true;
    let result = await account.startLogin(scanResult.content);
    if (result === null) {
        console.log("Login result", result);
        let nextPath = account.getNextPath();
        ionRouter.navigate(nextPath, 'none', 'pop');
        await loading.dismiss();
    } else {
        console.error("Login failed", result);
        await loading.dismiss();
        const alert = await alertController.create({
            header: 'Login fehlgeschlagen',
            subHeader: 'Folgende Details wurden übermittelt',
            message: result,
            buttons: ['Schade'],
        });
        await alert.present();
    }
    processing.value = false;
}


onIonViewDidEnter(setupScan);
onUnmounted(async () => {
    // Reset body background
    bodyBackground.value = "inherit";
    if (scanning.value) {
        await cancel();
    }
});

</script>

<style scoped>
#qrcode-content {
    --background: transparent;
    background: transparent;
}

.background {
    background: var(--ion-background-color-step-50, #fff);
    width: 100%;
    border-radius: 10px;
}
</style>

<style>
body {
    background-color: v-bind(bodyBackground) !important;
}
</style>
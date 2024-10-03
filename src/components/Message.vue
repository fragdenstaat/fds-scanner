<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Details</ion-title>
                <ion-buttons slot="start">
                    <ion-back-button></ion-back-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            Message ID: {{ id }}
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import {
    IonBackButton,
    IonButton, IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToolbar
} from '@ionic/vue';
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'Request',
    components: {
        IonBackButton,
        IonButton, IonButtons,
        IonContent,
        IonHeader,
        IonPage,
        IonTitle,
        IonToolbar,
    },
    props: {
        id: String,
    },
});
</script>
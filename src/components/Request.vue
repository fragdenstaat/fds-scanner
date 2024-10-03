<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button></ion-back-button>
                </ion-buttons>
                <ion-title>Request</ion-title>
            </ion-toolbar>
        </ion-header>

        <ion-content class="ion-padding">
            Request ID: {{ id }}
            <ion-button router-link="/message/1" router-direction="forward">Message 1</ion-button>
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import {
    IonBackButton,
    IonButton, IonButtons,
    IonContent, IonHeader, IonPage,
    IonSkeletonText,
    IonTitle, IonToolbar,
} from '@ionic/vue';
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'Request',
    components: {
        IonButtons,
        IonBackButton,
        IonButton,
        IonContent,
        IonSkeletonText,
        IonHeader,
        IonPage,
        IonTitle,
        IonToolbar,
    },
    props: {
        id: String,
    },
    data() {
        return {
            loading: true,
        };
    },
    mounted() {
        setTimeout(() => {
            this.loading = false;
        }, 2000);
    },
});
</script>
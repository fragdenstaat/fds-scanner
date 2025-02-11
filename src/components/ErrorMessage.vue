<template>
    <ion-card color="danger">
        <ion-card-header>
            <ion-card-title>Fehler</ion-card-title>
            <ion-card-subtitle>Es ist ein Fehler aufgetreten</ion-card-subtitle>
        </ion-card-header>

        <ion-card-content> {{ props.message }} </ion-card-content>
    </ion-card>
</template>

<script setup lang="ts">
import { IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle } from '@ionic/vue';

const props = defineProps<{
    message: string
}>()
</script>

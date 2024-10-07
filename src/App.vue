<template>
  <ion-app>
    <ion-router-outlet v-if="setupComplete" />
    <ion-loading v-else message="Lade Anwendung..."></ion-loading>
  </ion-app>
</template>

<script setup lang="ts">
import { IonApp, IonLoading, IonRouterOutlet, useIonRouter } from '@ionic/vue';
import { onBeforeMount, ref } from 'vue';
import { account } from './account.ts';

let setupComplete = ref(false);
const ionRouter = useIonRouter();

onBeforeMount(async () => {
  console.log("Starting setup");
  let hasUser = await account.setupUser();
  if (hasUser) {
    ionRouter.navigate('/', 'none', 'replace');
  } else {
    ionRouter.navigate('/login', 'none', 'replace');
  }
  setupComplete.value = true;
});

</script>
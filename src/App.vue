<template>
  <suspense>
    <ion-app>
      <ion-router-outlet v-if="setupComplete" />
      <ion-loading v-else message="Lade Anwendung..."></ion-loading>
    </ion-app>
    <template #fallback>
      <ion-app>
        <ion-loading message="Lade Anwendung..."></ion-loading>
      </ion-app>
    </template>
  </suspense>
</template>

<script setup lang="ts">
import { IonApp, IonLoading, IonRouterOutlet, useIonRouter } from '@ionic/vue';
import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
import { onBeforeMount, ref } from 'vue';
import { account, getDeepPath, LOGIN_PATH } from './account.ts';


let setupComplete = ref(false);
const ionRouter = useIonRouter();

onBeforeMount(async () => {
  console.log("Starting setup");
  let [deepUrls, hasUser] = await Promise.all([getCurrent(), account.setupUser()])
  if (deepUrls !== null && deepUrls.length > 0) {
    account.setDeepUrl(deepUrls[0]);
  }
  if (hasUser) {
    let nextPath = account.getNextPath()
    console.log("Loggedin, navigating to", nextPath);
    ionRouter.navigate(nextPath, 'none', 'replace');
  } else {
    ionRouter.navigate(LOGIN_PATH, 'none', 'replace');
  }
  setupComplete.value = true;
});

onOpenUrl((urls) => {
  console.log('deep link:', urls);
  if (urls !== null && urls.length > 0) {
    const deepUrl = urls[0]
    if (account.isLoggedIn) {
      ionRouter.navigate(getDeepPath(deepUrl), 'none', 'replace');
    }
  }
});

</script>

<style>
html {
  height: 100dvh;
}

body {
  background-color: #f7f7f7;
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #0d0d0d;
  }
}
</style>
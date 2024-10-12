/* Core CSS required for Ionic components to work properly */
import '@ionic/vue/css/core.css';

/* Basic CSS for apps built with Ionic */
import '@ionic/vue/css/normalize.css';
import '@ionic/vue/css/structure.css';
import '@ionic/vue/css/typography.css';

/* Optional CSS utils that can be commented out */
import '@ionic/vue/css/display.css';
import '@ionic/vue/css/flex-utils.css';
import '@ionic/vue/css/float-elements.css';
import '@ionic/vue/css/padding.css';
import '@ionic/vue/css/text-alignment.css';
import '@ionic/vue/css/text-transformation.css';

import '@ionic/vue/css/palettes/dark.system.css';

import { IonicVue } from '@ionic/vue';
import { createPinia } from 'pinia';

import * as Sentry from "@sentry/vue";

import { createApp } from "vue";
import App from "./App.vue";

import router from './router.ts';

const app = createApp(App).use(IonicVue).use(router);

const pinia = createPinia()

app.use(pinia)

Sentry.init({
  app,
  dsn: import.meta.env.SENTRY_DSN,
  integrations: [
    Sentry.browserTracingIntegration({ router }),
    Sentry.replayIntegration(),
  ],

  // Set tracesSampleRate to 1.0 to capture 100%
  // of transactions for tracing.
  // We recommend adjusting this value in production
  tracesSampleRate: 1.0,

  // Set `tracePropagationTargets` to control for which URLs trace propagation should be enabled
  // tracePropagationTargets: ["localhost", /^https:\/\/yourserver\.io\/api/],

  // Capture Replay for 10% of all sessions,
  // plus for 100% of sessions with an error
  replaysSessionSampleRate: 1.0,
  replaysOnErrorSampleRate: 1.0,
});

router.isReady().then(() => {
  app.mount('#app');
});
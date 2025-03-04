import { createRouter, createWebHistory } from '@ionic/vue-router';

import { type RouteRecordInfo } from 'vue-router';

// Define an interface of routes
export interface RouteNamedMap {
    // each key is a name
    home: RouteRecordInfo<
        // here we have the same name
        'home',
        // this is the path, it will appear in autocompletion
        '/',
        // these are the raw params. In this case, there are no params allowed
        Record<never, never>,
        // these are the normalized params
        Record<never, never>
    >
    info: RouteRecordInfo<
        // here we have the same name
        'info',
        // this is the path, it will appear in autocompletion
        '/info/',
        // these are the raw params. In this case, there are no params allowed
        Record<never, never>,
        // these are the normalized params
        Record<never, never>
    >
    login: RouteRecordInfo<
        'login',
        '/login/',
        Record<never, never>,
        Record<never, never>
    >
    qrcode: RouteRecordInfo<
        'login-qrcode',
        '/login/qrcode/',
        Record<never, never>,
        Record<never, never>
    >
    account: RouteRecordInfo<
        'account',
        '/account',
        Record<never, never>,
        Record<never, never>
    >
    // repeat for each route..
    // Note you can name them whatever you want
    request: RouteRecordInfo<
        'request',
        '/request/:id/',
        { id: number | string }, // raw value
        { id: string } // normalized value
    >
    'create-message': RouteRecordInfo<
        'request',
        '/request/:id/create-message/',
        { id: number | string }, // raw value
        { id: string } // normalized value
    >
    message: RouteRecordInfo<
        'message',
        '/:message(message|draft)/:id/',
        { id: number | string, message: "message" | "draft" }, // raw value
        { id: string, message: "message" | "draft" } // normalized value
    >
    scan: RouteRecordInfo<
        'message-scan',
        '/:message(message|draft)/:id/scan/',
        { id: number | string, message: "message" | "draft" }, // raw value
        { id: string, message: "message" | "draft" } // normalized value
    >
    'not-found': RouteRecordInfo<
        'not-found',
        '/:pathMatch(.*)*',
        Record<never, never>,
        Record<never, never>
    >
}

// Last, you will need to augment the Vue Router types with this map of routes
declare module 'vue-router' {
    interface TypesConfig {
        RouteNamedMap: RouteNamedMap
    }
}

import Account from './components/Account.vue';
import Info from './components/Info.vue';
import Login from './components/Login.vue';
import Message from './components/Message.vue';
import MessageCreate from './components/MessageCreate.vue';
import QrCode from './components/QrCode.vue';
import Request from './components/Request.vue';
import RequestList from './components/RequestList.vue';
import Scan from './components/Scan.vue';

const routes = [
    { path: '/', name: 'home', component: RequestList },
    { path: '/info/', name: 'info', component: Info },
    { path: '/login/', name: 'login', component: Login },
    { path: '/login/qrcode/', name: 'login-qrcode', component: QrCode },
    { path: '/account/', name: 'account', component: Account },
    { path: '/request/:id/', name: 'request', component: Request },
    { path: '/request/:id/create-message/', name: 'create-message', component: MessageCreate },
    {
        path: '/:message(message|draft)/:id/', name: 'message', component: Message, query: {
            highlight_attachment: Number,
            required: false,
        }
    },
    { path: '/:message(message|draft)/:id/scan/', name: 'message-scan', component: Scan },
    { path: '/:pathMatch(.*)*', name: 'not-found', redirect: "/" },
];

const router = createRouter({
    history: createWebHistory("/"),
    routes,
});


export default router;
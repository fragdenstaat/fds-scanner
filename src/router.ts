import { createRouter, createWebHistory } from '@ionic/vue-router';

import type { RouteRecordInfo } from 'vue-router';

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
    login: RouteRecordInfo<
        // here we have the same name
        'login',
        // this is the path, it will appear in autocompletion
        '/login',
        // these are the raw params. In this case, there are no params allowed
        Record<never, never>,
        // these are the normalized params
        Record<never, never>
    >
    // repeat for each route..
    // Note you can name them whatever you want
    'request': RouteRecordInfo<
        'request',
        '/request/:id',
        { id: number | string }, // raw value
        { id: string } // normalized value
    >
    'message': RouteRecordInfo<
        'message',
        '/message/:id',
        { id: number | string }, // raw value
        { id: string } // normalized value
    >
}

// Last, you will need to augment the Vue Router types with this map of routes
declare module 'vue-router' {
    interface TypesConfig {
        RouteNamedMap: RouteNamedMap
    }
}

import Login from './components/Login.vue';
import Message from './components/Message.vue';
import Request from './components/Request.vue';
import RequestList from './components/RequestList.vue';

const routes = [
    { path: '/', name: 'home', component: RequestList },
    { path: '/login', name: 'login', component: Login },
    { path: '/request/:id', name: 'request', component: Request },
    { path: '/message/:id', name: 'message', component: Message },
];

const router = createRouter({
    history: createWebHistory("/"),
    routes,
});

// const isAuthenticated = false;

// router.beforeEach((to, _from, next) => {
//     if (to.name !== 'login' && !isAuthenticated) next({ name: 'login' })
//     else next()
// })

export default router;
import { invoke } from '@tauri-apps/api/core';
import router from './router.ts';

interface User {
    id: Number;
    first_name: string;
    full_name: string;
    email: string;
    is_staff: boolean | null;
}

type MaybeUser = User | null

class Account {
    #loggedIn: boolean = false;
    #user: User | null = null;
    #messages: string[] = [];

    constructor() {
        this.#loggedIn = false;
    }

    async setupUser() {
        try {
            let response = await invoke<MaybeUser>('get_user')
            if (response) {
                this.#user = response
                this.login()
                return true
            }
        } catch (error) {
            console.warn('Error getting user!', error)
        }
        return false
    }

    get isLoggedIn() {
        return this.#loggedIn;
    }

    get isStaff() {
        return !!this.#user?.is_staff;
    }

    get user() {
        return this.#user;
    }

    async startLogin(startUrl: string | null = null): Promise<string | null> {
        try {
            let response = await invoke('start_oauth', { start_url: startUrl })
            console.log('OAuth completed!', response)
            if (response) {
                await this.setupUser()
            }
            return null
        } catch (error) {
            console.error('Error!', error)
            return error!.toString()
        }
    }

    async startLogout(): Promise<string | null> {
        try {
            await invoke('logout')
            this.logout()
            return null
        } catch (error) {
            return error!.toString()
        }
    }

    login() {
        this.#loggedIn = true;
        this.addMessage(`Sie sind eingeloggt als ${this.#user?.email}!`)
    }

    logout() {
        this.#loggedIn = false;
        this.#user = null;
        this.addMessage(`Sie sind jetzt ausgeloggt!`)
    }

    addMessage(message: string) {
        this.#messages.push(message)
    }

    clearMessages() {
        let messages = this.#messages
        this.#messages = []
        return messages
    }
}

export const account = new Account()

router.beforeEach((to, _from, next) => {
    if (!to.path.startsWith('/login') && !account.isLoggedIn) next({ name: 'login' })
    else if (to.path.startsWith('/login') && account.isLoggedIn) next({ name: 'home' })
    else next()
})


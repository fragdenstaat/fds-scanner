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
    #deepUrl: string | null = null;

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

    setDeepUrl(url: string | undefined) {
        if (url !== undefined) {
            this.#deepUrl = url
        }
    }

    getNextPath(): string {
        if (this.#deepUrl !== null) {
            let url = this.#deepUrl
            return getDeepPath(url)
        }
        return "/"
    }

    get deepUrl() {
        return this.#deepUrl
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
        if (startUrl === null && this.#deepUrl !== null) {
            let deepUrl = new URL(this.#deepUrl)
            let deepStartUrl = deepUrl.searchParams.get("start_url")
            if (deepStartUrl !== null) {
                startUrl = deepStartUrl
                // Set start_url param as login start and remove from deep url
                deepUrl.searchParams.delete("start_url")
                this.#deepUrl = deepUrl.toString()
            }
        }
        try {
            console.log("Starting OAuth with start_url:", startUrl)
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

const BASE_PATH = "/app/scanner/deep"

export const getDeepPath = (deepUrl: string) => {
    let url = new URL(deepUrl);
    let path = url.pathname.replace(BASE_PATH, "");
    return path + url.search
}

router.beforeEach((to, _from, next) => {
    if (!to.path.startsWith('/login') && !account.isLoggedIn) next({ name: 'login' })
    else if (to.path.startsWith('/login') && account.isLoggedIn) next({ name: 'home' })
    else next()
})


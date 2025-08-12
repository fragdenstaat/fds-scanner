import { onIonViewDidLeave, onIonViewWillEnter } from '@ionic/vue';
import { invoke } from '@tauri-apps/api/core';
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';
import router from './router.ts';

interface User {
    id: Number;
    first_name: string;
    full_name: string;
    email: string;
    is_staff: boolean | null;
}

type MaybeUser = User | null


const DOMAIN = "fragdenstaat.de"
const APP_ORIGIN = "https://app.fragdenstaat.de"
const START_URL_ORIGIN = "https://fragdenstaat.de"
const BASE_PATH = "/app/scanner/deep"
export const LOGIN_PATH = "/login/"


class Account {
    #loggedIn: boolean = false;
    #user: User | null = null;
    #messages: string[] = [];
    #deepUrl: string | null = null;
    #mountedWithDeepUrl: boolean = false;

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

    setDeepUrl(url: string | undefined, onMount: boolean = false) {
        if (url !== undefined) {
            this.#deepUrl = url
            if (onMount) {
                this.#mountedWithDeepUrl = true
            }
        }
    }

    mountedWithDeepUrl(): boolean {
        return this.#mountedWithDeepUrl
    }

    getNextPath(): string {
        if (this.#deepUrl !== null) {
            let url = this.#deepUrl
            return getDeepPath(url)
        }
        return "/"
    }

    startLoginOnMount(): boolean {
        if (!this.isLoggedIn && this.getNextPath() !== "/") {
            return true
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

    sanitizeStartUrl(startUrl: string): string | null {
        const url = new URL(startUrl)
        if (url.origin === START_URL_ORIGIN) {
            return startUrl
        }
        return null
    }

    async startLogin(startUrl: string | null = null): Promise<string | null> {
        if (startUrl !== null) {
            let tempUrl = new URL(startUrl)
            if (tempUrl.origin === APP_ORIGIN && tempUrl.pathname.startsWith(BASE_PATH)) {
                // Use the start_url parameter instead if it exists 
                let tempStartUrl = tempUrl.searchParams.get("start_url")
                if (tempStartUrl !== null) {
                    // Set start_url param as login start and remove from deep url
                    startUrl = this.sanitizeStartUrl(tempStartUrl)
                    tempUrl.searchParams.delete("start_url")
                    this.#deepUrl = tempUrl.toString()
                } else {
                    // If the paramter is not present
                    // set url as deep url
                    this.#deepUrl = startUrl
                    startUrl = null
                }
            } else if (tempUrl.origin === START_URL_ORIGIN) {
                // If the URL is a deep link, set it as the deep URL
                this.#deepUrl = startUrl
                startUrl = null
            } else {
                // if the given URL is not an App URL, ignore it
                return "Invalid URL"
            }
        } else if (this.#deepUrl !== null) {
            let deepUrl = new URL(this.#deepUrl)
            let deepStartUrl = deepUrl.searchParams.get("start_url")
            if (deepStartUrl !== null) {
                startUrl = this.sanitizeStartUrl(deepStartUrl)
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
        this.#deepUrl = null;
        this.#mountedWithDeepUrl = false;
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

export const getDeepPath = (deepUrl: string) => {
    let url = new URL(deepUrl);
    let path = url.pathname.replace(BASE_PATH, "");
    return path + url.search
}

export const useLoggedOutDeepLinkNavigation = (startLoginFunc?: (url: string) => void) => {
    let unlistenFunc: (() => void) | null = null
    onIonViewWillEnter(() => {
        onOpenUrl((urls) => {
            console.log('deep link:', urls);
            if (urls !== null && urls.length > 0) {
                const deepUrl = urls[0]
                const url = new URL(deepUrl)
                if (url.protocol !== "https:" || !url.origin.endsWith(DOMAIN)) {
                    console.warn('Invalid deep link URL:', url)
                    return
                }
                if (!account.isLoggedIn) {
                    if (startLoginFunc) {
                        startLoginFunc(deepUrl)
                    } else {
                        account.setDeepUrl(deepUrl);
                        router.push(LOGIN_PATH);
                    }
                }
            }
        }).then(unlisten => {
            unlistenFunc = unlisten
        })
    })
    onIonViewDidLeave(() => {
        if (unlistenFunc !== null) {
            unlistenFunc()
            unlistenFunc = null
        }
    })
}


router.beforeEach((to, _from, next) => {
    if (!to.path.startsWith(LOGIN_PATH) && !to.path.startsWith('/info/') && !account.isLoggedIn) next({ name: 'login' })
    else if (to.path.startsWith(LOGIN_PATH) && account.isLoggedIn) next({ name: 'home' })
    else next()
})


import { toastController } from '@ionic/vue';
import { onMounted } from 'vue';
import { account } from './account.ts';


export function useToastMessages() {
    let toast: HTMLIonToastElement | null = null

    onMounted(async () => {
        let toastMessages = account.clearMessages()
        if (toastMessages.length === 0) {
            return
        }
        if (toast === null) {
            toast = await toastController.create({
                message: "",
                duration: 5000,
            });
        }
        toast.message = toastMessages.join('\n')
        await toast.present();
    });
    return { toast }
}

const localeOptions = {
    year: 'numeric' as const,
    month: 'numeric' as const,
    day: 'numeric' as const,
};

export function toLocaleDateString(date: Date): string {
    return date.toLocaleDateString('de-DE', localeOptions)
}
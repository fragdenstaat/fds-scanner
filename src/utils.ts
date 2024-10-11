import { onActivated, ref } from 'vue';
import { account } from './account.ts';

export function useToastMessages() {
    const toastMessages = ref<string[]>([]);
    onActivated(() => {
        toastMessages.value = account.clearMessages()
    });
    return { toastMessages }
}

const localeOptions = {
    year: 'numeric' as const,
    month: 'numeric' as const,
    day: 'numeric' as const,
};

export function toLocaleDateString(date: Date): string {
    return date.toLocaleDateString('de-DE', localeOptions)
}
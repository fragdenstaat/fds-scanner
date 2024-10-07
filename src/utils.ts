import { onActivated, ref } from 'vue';
import { account } from './account.ts';

export function useToastMessages() {
    const toastMessages = ref<string[]>([]);
    onActivated(() => {
        toastMessages.value = account.clearMessages()
    });
    return { toastMessages }
}
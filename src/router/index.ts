import { createRouter, createWebHistory } from 'vue-router';
import SplashScreen from '@/SplashScreen.vue';
import ChatView from '@/views/ChatView.vue';
import XESPanView from '@/views/XESPanView.vue';

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: '/',
			name: 'chat',
			component: ChatView,
		},
		{
			path: '/splashscreen',
			name: 'splashscreen',
			component: SplashScreen,
		},
		{
			path: '/xespan',
			name: 'xespan',
			component: XESPanView,
		},
	],
});

export default router;

<script setup lang="ts">
import WindowTitle from '@/components/WindowTitle.vue';
import { message, open } from '@tauri-apps/plugin-dialog';
import { ref } from 'vue';

const filePath = ref('');
async function openFile() {
	try {
		filePath.value = (await open()) ?? '';
	} catch (error) {
		await message(`发生了错误: ${error}`, {
			title: '错误!',
			kind: 'error',
		});
	}
}
</script>

<template>
	<window-title
		label="xespan"
		title="学而思网盘"
		:show-minimize="false"
		:hide-window="true"
	/>
	<div
		class="bg-gray-50 dark:bg-gray-900 w-full h-full fixed flex flex-col items-center space-y-4 p-4"
	>
		<div class="flex w-full justify-center items-center space-x-3">
			<input
				class="border border-emerald-500 rounded w-3/4 py-2 px-3 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent text-center font-mono"
				:value="filePath"
			/>
			<button
				class="border w-15 h-10 rounded border-emerald-500 text-emerald-500 hover:bg-emerald-500 hover:text-white font-sans"
				@click="openFile"
			>
				打开
			</button>
		</div>
		<div class="flex w-full justify-center items-center space-x-3">
			<input
				readonly
				class="border border-emerald-500 rounded w-3/4 py-2 px-3 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent text-center font-mono"
				:value="filePath"
			/>
			<button
				class="border w-15 h-10 rounded border-emerald-500 text-emerald-500 hover:bg-emerald-500 hover:text-white font-sans"
			>
				上传
			</button>
		</div>
	</div>
</template>

<style scoped></style>

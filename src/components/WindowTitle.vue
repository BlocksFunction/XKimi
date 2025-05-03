<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

defineProps({
	label: {
		type: String,
		required: true,
	},
	title: {
		type: String,
		required: true,
	},
	showMinimize: {
		type: Boolean,
		default: true,
	},
	showClose: {
		type: Boolean,
		default: true,
	},
	hideWindow: {
		type: Boolean,
		default: false,
	},
});
</script>

<template>
	<div
		class="h-12 bg-gray-50 dark:bg-gray-900 dark:text-white font-sans flex items-center justify-between px-5"
		data-tauri-drag-region
	>
		<div class="flex items-center">
			<p>
				{{ title }}
			</p>
		</div>
		<div class="flex items-center gap-4">
			<button
				class="border w-8 h-8 rounded border-emerald-500 text-emerald-500 hover:bg-emerald-500 hover:text-white font-sans"
				v-if="showMinimize"
				@click="
					(e) =>
						invoke('minimize_window', {
							label: label,
						})
				"
			>
				-
			</button>
			<button
				class="border w-8 h-8 rounded border-emerald-500 text-emerald-500 hover:bg-emerald-500 hover:text-white font-sans"
				@click="
					(e) =>
						hideWindow
							? invoke('hide_window', {
									label: label,
								})
							: invoke('close_window', {
									label: label,
								})
				"
				v-if="showClose"
			>
				X
			</button>
		</div>
	</div>
</template>

<style scoped></style>

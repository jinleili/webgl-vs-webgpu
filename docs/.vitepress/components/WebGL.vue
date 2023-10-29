<template>
    <div id="webgl-container">
        <div v-if="loading">
            正在加载 WASM 模块 ...
        </div>
    </div>
</template>

<script>
export default {
    name: "web-gl",
    props: {
        autoLoad: true,
    },
    data() {
        return {
            error: "",
            loading: true,
        };
    },
    methods: {
        async loadExample() {
            this.loading = true;
            try {
                const module = await import(/* @vite-ignore */`./wasm/webgl/webgl-vs-webgpu.js`.replace('_', '-'));
                module.default().then((instance) => {
                    this.loading = false;
                }, (e) => {
                    if (!`${e}`.includes("don't mind me. This isn't actually an error!")) {
                        this.showErr(e);
                    } else {
                        this.loading = false;
                    }
                });

            } catch (e) {
                this.showErr(e);
            }
        },

        showErr(err) {
            this.error = `An error occurred loading "${this.example}": ${err}`;
            console.error(err);
            this.loading = false;
        }
    },
    async mounted() {
        this.loadExample();
    }
};
</script>
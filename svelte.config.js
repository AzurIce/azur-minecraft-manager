import staticAdapter from "@sveltejs/adapter-static";
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: staticAdapter(),
  },
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],
};

export default config;

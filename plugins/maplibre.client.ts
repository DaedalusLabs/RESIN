export default defineNuxtPlugin(async () => {
   const maplibregl = await import("maplibre-gl");
   await import("maplibre-gl/dist/maplibre-gl.css");

   // Configure maplibre for 2D only
   const createMap = (
      container: HTMLElement,
      options: maplibregl.MapOptions,
   ) => {
      const defaultOptions: Partial<maplibregl.MapOptions> = {
         style: "/map-liberty.json",
         renderWorldCopies: false,
         maxPitch: 0,
         pitchWithRotate: false,
      };

      return new maplibregl.default.Map({
         ...defaultOptions,
         ...options,
         container,
      });
   };

   return {
      provide: {
         createMap,
         maplibregl: maplibregl.default,
      },
   };
});

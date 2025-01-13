import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";

export default defineNuxtPlugin(() => {
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

      return new maplibregl.Map({
         ...defaultOptions,
         ...options,
         container,
      });
   };

   return {
      provide: {
         createMap,
         maplibregl,
      },
   };
});

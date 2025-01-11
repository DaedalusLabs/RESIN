import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import MenuBar from "./MenuBar.vue";

describe("MenuBar", () => {
   it("renders properly", () => {
      const wrapper = mount(MenuBar);
      expect(wrapper.exists()).toBe(true);
   });
});

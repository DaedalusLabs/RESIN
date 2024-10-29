<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">
            Rent-to-own agreement
         </h3>
         <p class="text-sm text-gray-500">Please read carefully</p>
      </template>
      <template #content>
         <div
            class="mt-4 flex w-full flex-col items-center justify-center gap-4 text-gray-800"
         >
            <p>
               This Rent-to-Own Agreement ("Agreement") is made and entered into
               on this 11th day of June, 2024, by and between:
               <br />
               <br />
               Landlord/Seller:
               <br />
               Name: Daedalus Labs Ltd.
               <br />
               Address: 123 Innovation Drive, Hamilton, HM 12, Bermuda
               <br />
               Email: properties@daedalus.io
               <br />
               <br />
               Property:
               <br />
               Address: Mahonylaan 5, Paramaribo
               <br />
               <br />
               Terms and Conditions:
               <br />
            </p>
            <ul class="list-decimal">
               <li>
                  Term: The term of this Agreement shall commence on [Start
                  Date] and shall continue for a period of [Number of
                  Months/Years] unless terminated earlier as provided herein.
               </li>
               <li>
                  Rent: The Tenant/Buyer agrees to pay the Landlord/Seller a
                  monthly rent of [Rent Amount] SRD, payable in advance on the
                  [Day] of each month. Payments shall be made to the
                  Landlord/Seller at the above-stated address or at such other
                  place as the Landlord/Seller may designate in writing.
               </li>

               <li>
                  Option to Purchase: The Landlord/Seller grants the
                  Tenant/Buyer an exclusive option to purchase the Property
                  during the term of this Agreement. The purchase price of the
                  Property shall be [Purchase Price] SRD.
               </li>
               <li>
                  Option Consideration: The Tenant/Buyer shall pay the
                  Landlord/Seller a non-refundable option consideration of
                  [Option Consideration Amount] SRD, due upon signing this
                  Agreement. This amount shall be credited towards the purchase
                  price if the Tenant/Buyer exercises the option to purchase.
               </li>
               <li>
                  Rent Credits: A portion of the monthly rent payment,
                  specifically [Rent Credit Amount] SRD per month, shall be
                  credited towards the purchase price of the Property if the
                  Tenant/Buyer exercises the option to purchase. Maintenance and
               </li>
               <li>
                  Repairs: The Tenant/Buyer shall be responsible for all
                  maintenance and repairs to the Property during the term of
                  this Agreement, except for structural repairs which shall be
                  the responsibility of the Landlord/Seller. Taxes and
               </li>
               <li>
                  Insurance: The Landlord/Seller shall be responsible for
                  property taxes and insurance on the Property until the option
                  to purchase is exercised by the Tenant/Buyer. The Tenant/Buyer
                  is encouraged to obtain renter's insurance for personal
                  property.
               </li>
               <li>
                  Utilities: The Tenant/Buyer shall be responsible for all
                  utilities, including but not limited to water, electricity,
                  gas, and internet, from the commencement of this Agreement.
               </li>
               <li>
                  Default: In the event of a default by the Tenant/Buyer in the
                  payment of rent or any other obligations under this Agreement,
                  the Landlord/Seller shall have the right to terminate this
                  Agreement and retain all payments made hereunder as liquidated
                  damages. Exercise of
               </li>
               <li>
                  Option: The Tenant/Buyer may exercise the option to purchase
                  the Property by providing written notice to the
                  Landlord/Seller at any time during the term of this Agreement.
                  Upon exercise of the option, the parties shall enter into a
                  Purchase Agreement for the sale of the Property on terms
                  consistent with this Agreement. Governing
               </li>
               <li>
                  Law: This Agreement shall be governed by and construed in
                  accordance with the laws of the Republic of Suriname. Entire
               </li>
               <li>
                  Agreement: This Agreement constitutes the entire agreement
                  between the parties and supersedes all prior negotiations,
                  representations, or agreements, whether written or oral. IN
                  WITNESS WHEREOF, the parties hereto have executed this
                  Rent-to-Own Agreement as of the day and year first above
                  written. Please review this contract with a legal professional
                  to ensure it meets all legal requirements and adequately
                  protects your interests.
               </li>
            </ul>
         </div>
         <div class="mt-6 flex w-full justify-center gap-4">
            <FlowbiteBorderButton
               text="Back"
               :show-icon="false"
               @click="handleBack"
            >
               <template #icon-prepend>
                  <PhArrowLeft :size="24" weight="bold" />
               </template>
            </FlowbiteBorderButton>
            <FlowbiteButton text="Next" class="px-5 py-3" @click="handleNext" />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft } from "@phosphor-icons/vue";
const showDrawer = ref(true);
const emit = defineEmits(["close", "next", "back"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

watchEffect(() => {
   showDrawer.value = props.show;
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   emit("next");
};

const handleBack = () => {
   emit("back");
};
</script>

<style scoped>
p {
   @apply mb-4;
}
ul li:not(:last-child) {
   @apply mb-3 border-b border-gray-200 pb-3;
}
</style>

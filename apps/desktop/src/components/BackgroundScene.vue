<template>
  <TresMesh>
    <primitive
      :object="scene"
      :scale="0.5"
      ref="modelRef"
      :position="[0, -1, 0]"
    />
    >
  </TresMesh>
</template>

<script setup lang="ts">
const { scene } = await useGLTF("/models/checkmark.glb");
const modelRef = shallowRef<THREE.Mesh | null>(null);
const { onLoop } = useRenderLoop();
onLoop(({ delta, elapsed }) => {
  if (modelRef.value) {
    modelRef.value.rotation.z += delta * 0.5;
    // modelRef.value.rotation.x += delta * 0.5;
    // modelRef.value.rotation.y += delta * 0.5;
  }
});
onMounted(() => {
  console.log("mounted");
  if (modelRef.value) {
    modelRef.value.rotation.x = Math.PI / 2;
    modelRef.value.rotation.y = Math.PI;
  }
});
</script>

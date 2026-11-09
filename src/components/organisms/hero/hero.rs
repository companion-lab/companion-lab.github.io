use crate::components::atoms::Icon;
use dioxus::prelude::*;

use crate::components::molecules::DownloadButtons;

// Only referenced by the wasm-only module injection below (the static
// client build); unused in native checks.
#[cfg(target_arch = "wasm32")]
const DEMO_MODEL: Asset = asset!("/assets/model/demo.glb");

#[component]
pub fn Hero() -> Element {
    // Boot the 3D avatar renderer once the canvas is mounted (client only).
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        vrm::start_or_inject();
    });

    rsx! {
        section {
            class: "relative min-h-screen flex items-center pt-24 pb-12 overflow-hidden",
            div {
                class: "max-w-7xl mx-auto px-6 w-full grid grid-cols-1 lg:grid-cols-2 gap-12 items-center",
                // Left: Text Content
                div {
                    class: "flex flex-col items-start z-10 reveal active",
                    div {
                        class: "mb-6 flex w-full max-w-xl items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "cmp://home" }
                        span { class: "flex items-center gap-2 text-gray-400",
                            span { class: "hud-dot", "aria-hidden": "true" }
                            "avatar: local renderer"
                        }
                    }
                    h1 {
                        class: "text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black leading-[1.05] tracking-tight mb-6",
                        "A 3D VRM avatar "
                        br {}
                        span { class: "text-purple-500", "on your desktop" }
                        br {}
                        "for your AI agent"
                        span { class: "hud-cursor text-purple-400", "_" }
                    }
                    p {
                        class: "text-base sm:text-lg text-gray-400 mb-8 max-w-xl leading-relaxed",
                        "Puts a 3D VRM avatar on your desktop that acts as the visual body of your AI agent. It sits as a transparent overlay — no window chrome, no background — just your character floating on screen, expressions and motion in real time."
                    }
                    DownloadButtons { centered: false }
                    p {
                        class: "font-mono text-[11px] uppercase tracking-[0.15em] text-gray-400 mt-4",
                        "// desktop build via tauri v2 — models: pnpm models + moonshine stt"
                    }
                    p {
                        class: "font-mono text-[11px] uppercase tracking-[0.15em] text-gray-400 mt-2",
                        "// providers: openai-compatible · anthropic · gemini"
                    }
                }

                // Right: Hero Graphic
                div {
                    class: "relative w-full h-[480px] sm:h-[560px] flex items-center justify-center z-10 reveal active lg:justify-end",
                    div {
                        class: "hud-frame hud-grid relative w-full max-w-[540px] h-[440px] sm:h-[540px] border border-white/10 bg-[#08040f]/50 p-4 sm:p-6 flex flex-col",
                        div { class: "hud-frame-inner flex h-full flex-col",
                            div {
                                class: "flex items-center justify-between pb-3 font-mono text-[10px] uppercase tracking-[0.2em] text-gray-400",
                                span { class: "text-purple-300/80", "render.viewport // vrm-01" }
                                span { "live" }
                            }

                            // Live 3D avatar
                            div {
                                class: "relative w-full flex-1 min-h-0",
                                canvas {
                                    id: "hero-vrm-canvas",
                                    class: "absolute inset-0 h-full w-full",
                                }
                                div {
                                    id: "hero-vrm-loading",
                                    class: "absolute inset-0 flex items-center justify-center",
                                    span { class: "font-mono text-xs text-gray-400 tracking-[0.2em] uppercase animate-pulse", "loading avatar // please stand by" }
                                }
                                div {
                                    id: "hero-vrm-fallback",
                                    class: "absolute inset-0 hidden items-center justify-center",
                                    Icon { icon: "robot", class: "text-5xl text-white/80" }
                                }
                            }

                            div {
                                class: "flex items-center justify-between pt-3 font-mono text-[10px] uppercase tracking-[0.2em] text-gray-400",
                                span { "three.js // three-vrm" }
                                span { "realtime" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Three.js + @pixiv/three-vrm loader, only compiled into the wasm client.
// The libraries come from CDN; the module auto-starts on the current canvas
// and stays reachable as window.__startHeroVrm so remounts of the hero
// (route changes) can re-attach a fresh canvas.
//
// Performance-critical detail: everything 3D runs inside an OffscreenCanvas
// worker (WebGL, GLTF parse, VRM animation, render loop). Evaluating
// three.js and parsing the model are multi-hundred-ms main-thread tasks
// otherwise — exactly the long tasks that keep Total Blocking Time (and
// Time to Interactive) high while the page loads. Browsers without
// OffscreenCanvas/module-worker support fall back to the same scene on the
// main thread.
#[cfg(target_arch = "wasm32")]
mod vrm {
    use wasm_bindgen::prelude::*;

    // Generated by build.rs: one hashed asset URL per .vrma file found in
    // assets/vrma/ at compile time.
    include!(concat!(env!("OUT_DIR"), "/vrma_list.rs"));

    // Resolves the @pixiv packages' internal bare "three" import for the
    // main-thread fallback path (jsdelivr hosts both, same instance).
    const IMPORTMAP: &str = r#"{"imports":{"three":"https://cdn.jsdelivr.net/npm/three@0.169.0/build/three.module.min.js","three/addons/":"https://cdn.jsdelivr.net/npm/three@0.169.0/examples/jsm/","@pixiv/three-vrm":"https://cdn.jsdelivr.net/npm/@pixiv/three-vrm@3/lib/three-vrm.module.min.js","@pixiv/three-vrm-animation":"https://cdn.jsdelivr.net/npm/@pixiv/three-vrm-animation@3/lib/three-vrm-animation.module.min.js"}}"#;

    const MODULE_JS: &str = r#"
const CDN = 'https://cdn.jsdelivr.net/npm/three@0.169.0';
// Asset URLs are root-relative; resolve them to absolute URLs here on the
// main thread — the blob worker cannot resolve relative paths against its
// blob: base URL ("Failed to parse URL" in Request/fetch), so the worker
// and fallback sources below must receive fully-qualified URLs.
const MODEL_URL = new URL('__MODEL_URL__', location.href).href;
const VRMA_URLS = __VRMA_URLS__.map((u) => new URL(u, location.href).href);

// ── Worker source: the whole 3D scene on an OffscreenCanvas ─────────────
// Spawned from a Blob URL. Imports go through esm.sh, which rewrites the
// bare "three" specifiers inside @pixiv/three-vrm into absolute URLs —
// workers do not see the document's import map, so the jsdelivr packages
// (which bare-import 'three') would fail to resolve here. esm.sh pins every
// rewritten import to the same three@0.169.0 build, so all modules share
// one three instance. Backtick strings below are joined with the import
// lines into the final module source.
const WORKER_SRC = [
"import * as THREE from 'https://esm.sh/three@0.169.0';",
"import { GLTFLoader } from 'https://esm.sh/three@0.169.0/examples/jsm/loaders/GLTFLoader.js';",
"import { VRMLoaderPlugin, VRMUtils } from 'https://esm.sh/@pixiv/three-vrm@3?deps=three@0.169.0';",
"import { VRMAnimationLoaderPlugin, createVRMAnimationClip } from 'https://esm.sh/@pixiv/three-vrm-animation@3?deps=three@0.169.0';",
"const MODEL_URL = '" + MODEL_URL + "';",
"const VRMA_URLS = " + JSON.stringify(VRMA_URLS) + ";",
`let renderer, scene, camera, vrm, model, mixer, frameData;
let clips = []; // { url, clip }
let current = null;
let currentUrl = null;
const FADE = 0.8;
let clock = null;
const post = (type) => postMessage({ type });

// Drop the arms into a relaxed A-pose right after load so the avatar is
// never seen in the VRM's default T-pose while (or if) the idle clip loads;
// once the clip's action plays, the mixer overrides these rotations. Which
// Z-sign swings an arm down depends on how the model sits in its normalized
// rig (VRM0 and VRM1 face opposite ways), so probe one arm first.
const restPose = (vrm) => {
  if (!vrm || !vrm.humanoid) return;
  const h = vrm.humanoid;
  const bone = (name) => h.getNormalizedBoneNode(name);
  const leftArm = bone('leftUpperArm');
  const rightArm = bone('rightUpperArm');
  if (!leftArm || !rightArm) return;
  const restY = leftArm.getWorldPosition(new THREE.Vector3()).y;
  leftArm.rotation.set(0, 0, Math.PI / 2);
  h.update();
  const probeHand = bone('leftHand') || bone('leftLowerArm');
  const probeY = probeHand ? probeHand.getWorldPosition(new THREE.Vector3()).y : null;
  leftArm.rotation.set(0, 0, 0);
  const flip = probeY !== null && probeY > restY ? -1 : 1;
  const drop = flip * THREE.MathUtils.degToRad(65);
  leftArm.rotation.set(0, 0, drop);
  rightArm.rotation.set(0, 0, -drop);
  const leftFore = bone('leftLowerArm');
  const rightFore = bone('rightLowerArm');
  if (leftFore) leftFore.rotation.set(0, -0.25, 0);
  if (rightFore) rightFore.rotation.set(0, 0.25, 0);
  h.update();
};

// Playlist: preload every clip from the build-time-generated /assets/vrma/
// list and play them one by one in random order, crossfading between them.
// Failures are skipped; with no usable clips the avatar stays relaxed.
const playRandomClip = () => {
  if (!clips.length || !vrm || !mixer) return;
  let pick = clips[Math.floor(Math.random() * clips.length)];
  if (clips.length > 1) {
    while (pick.url === currentUrl) pick = clips[Math.floor(Math.random() * clips.length)];
  }
  currentUrl = pick.url;
  if (current) {
    const old = current;
    old.fadeOut(FADE);
    setTimeout(() => old.stop(), FADE * 1000 + 100);
  }
  // A brand-new action per play: clipAction would hand back the same cached
  // object when a clip replays, and play() on an exhausted action doesn't
  // rewind it — it re-finishes instantly and the avatar freezes.
  current = new THREE.AnimationAction(mixer, pick.clip, vrm.scene);
  current.setLoop(THREE.LoopOnce, 1);
  current.clampWhenFinished = true;
  current.play();
  current.fadeIn(FADE);
};

const loadVrmas = () => {
  if (!vrm) return;
  if (!VRMA_URLS.length) return;
  if (!mixer) {
    mixer = new THREE.AnimationMixer(vrm.scene);
    mixer.addEventListener('finished', () => {
      playRandomClip();
    });
  }
  let done = 0;
  const oneDone = () => {
    done += 1;
    if (done !== VRMA_URLS.length) return;
    if (!clips.length) return;
    playRandomClip();
  };
  VRMA_URLS.forEach((url) => {
    const animLoader = new GLTFLoader();
    animLoader.register((parser) => new VRMAnimationLoaderPlugin());
    animLoader.load(
      url,
      (gltfAnim) => {
        try {
          const anim = gltfAnim.userData.vrmAnimations && gltfAnim.userData.vrmAnimations[0];
          if (anim) clips.push({ url, clip: createVRMAnimationClip(anim, vrm) });
        } catch (e) {}
        oneDone();
      },
      undefined,
      () => oneDone()
    );
  });
};

// Frame the model so the whole performance stays inside the canvas: fit the
// T-pose height with extra margin for limbs that travel beyond it (hands
// overhead, hips travel), and budget width at 85% of height — dance clips
// spread the legs and swing the arms but never reach the full T-pose
// arm-span, which would needlessly shrink the avatar on portrait canvases.
const frame = () => {
  if (!frameData || !camera) return;
  const size = frameData.size, center = frameData.center;
  const halfFov = (camera.fov * Math.PI) / 360;
  const width = Math.min(Math.max(size.x, size.z), size.y * 0.85);
  const distH = (size.y / 2) / Math.tan(halfFov);
  const distW = (width / 2) / (Math.tan(halfFov) * camera.aspect);
  const dist = Math.max(distH, distW) * 1.25;
  camera.position.set(center.x, center.y + size.y * 0.05, center.z + dist);
  camera.lookAt(center.x, center.y, center.z);
};

const applySize = (w, h, dpr) => {
  if (!renderer || !camera) return;
  renderer.setPixelRatio(Math.min(dpr || 1, 2));
  renderer.setSize(w, h, false);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
  frame();
};

self.onmessage = (e) => {
  const msg = e.data || {};
  if (msg.type !== 'init') {
    if (msg.type === 'resize') applySize(msg.width, msg.height, msg.dpr);
    return;
  }
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ canvas: msg.canvas, alpha: true, antialias: true });
  } catch (err) {
    post('webgl-error');
    return;
  }
  renderer.setClearColor(0x000000, 0);

  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(30, 1, 0.05, 40);
  applySize(msg.width, msg.height, msg.dpr);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x3b3b58, 1.15));
  const dir = new THREE.DirectionalLight(0xffffff, 1.5);
  dir.position.set(1.5, 3, 2.5);
  scene.add(dir);
  const rim = new THREE.PointLight(0xa855f7, 8, 12);
  rim.position.set(-2, 1.5, -2.5);
  scene.add(rim);

  clock = new THREE.Clock();
  const tick = () => {
    const delta = clock.getDelta();
    if (mixer) mixer.update(delta);
    if (vrm) vrm.update(delta);
    renderer.render(scene, camera);
  };
  // requestAnimationFrame is not universally exposed in workers; the
  // timeout keeps the loop alive where it is missing.
  const schedule = () => {
    if (typeof requestAnimationFrame === 'function') requestAnimationFrame(schedule);
    else setTimeout(schedule, 16);
    tick();
  };
  schedule();

  new GLTFLoader()
    .register((parser) => new VRMLoaderPlugin(parser))
    .load(
      MODEL_URL,
      (gltf) => {
        vrm = gltf.userData.vrm || null;
        model = vrm ? vrm.scene : gltf.scene;
        model.traverse((o) => { o.frustumCulled = false; });
        if (vrm) {
          VRMUtils.rotateVRM0(vrm);
          vrm.scene.rotation.y = Math.PI;
        }
        scene.add(model);
        // Capture the T-pose bounds before posing: the height anchors
        // framing, and frame() adds margin plus a width budget.
        const box = new THREE.Box3().setFromObject(model);
        frameData = { size: box.getSize(new THREE.Vector3()), center: box.getCenter(new THREE.Vector3()) };
        restPose(vrm);
        frame();
        post('ready');
        loadVrmas();
      },
      undefined,
      () => post('error')
    );
};`
];

// ── Main-thread fallback: the same scene on the page thread ─────────────
// Used only when OffscreenCanvas/module workers are unavailable or the
// worker path failed. The document's import map (injected by the Rust side)
// resolves the bare "three" specifier inside the @pixiv packages, so these
// imports use jsdelivr URLs that map onto the same three instance.
const FALLBACK_SRC = [
"import * as THREE from '" + CDN + "/build/three.module.min.js';",
"import { GLTFLoader } from '" + CDN + "/examples/jsm/loaders/GLTFLoader.js';",
"import { VRMLoaderPlugin, VRMUtils } from 'https://cdn.jsdelivr.net/npm/@pixiv/three-vrm@3/lib/three-vrm.module.min.js';",
"import { VRMAnimationLoaderPlugin, createVRMAnimationClip } from 'https://cdn.jsdelivr.net/npm/@pixiv/three-vrm-animation@3/lib/three-vrm-animation.module.min.js';",
"const MODEL_URL = '" + MODEL_URL + "';",
"const VRMA_URLS = " + JSON.stringify(VRMA_URLS) + ";",
`function start(canvas) {
  // Deduping is handled by the bootstrap start(); this copy only runs when
  // the bootstrap already decided to fall back to the main thread.
  if (!canvas) return;
  const loading = document.getElementById('hero-vrm-loading');
  const fallback = document.getElementById('hero-vrm-fallback');
  if (loading) loading.style.display = 'flex';
  canvas.style.display = '';

  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ canvas, alpha: true, antialias: true });
  } catch (e) {
    canvas.style.display = 'none';
    if (loading) loading.style.display = 'none';
    if (fallback) fallback.style.display = 'flex';
    return;
  }
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.setClearColor(0x000000, 0);
  if (window.__stopHeroVrm) { try { window.__stopHeroVrm(); } catch (e) {} }

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(30, 1, 0.05, 40);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x3b3b58, 1.15));
  const dir = new THREE.DirectionalLight(0xffffff, 1.5);
  dir.position.set(1.5, 3, 2.5);
  scene.add(dir);
  const rim = new THREE.PointLight(0xa855f7, 8, 12);
  rim.position.set(-2, 1.5, -2.5);
  scene.add(rim);

  let vrm = null;
  let model = null;
  let mixer = null;
  let clips = [];
  let current = null;
  let currentUrl = null;
  const FADE = 0.8;

  const restPose = (vrm) => {
    if (!vrm || !vrm.humanoid) return;
    const h = vrm.humanoid;
    const bone = (name) => h.getNormalizedBoneNode(name);
    const leftArm = bone('leftUpperArm');
    const rightArm = bone('rightUpperArm');
    if (!leftArm || !rightArm) return;
    const restY = leftArm.getWorldPosition(new THREE.Vector3()).y;
    leftArm.rotation.set(0, 0, Math.PI / 2);
    h.update();
    const probeHand = bone('leftHand') || bone('leftLowerArm');
    const probeY = probeHand ? probeHand.getWorldPosition(new THREE.Vector3()).y : null;
    leftArm.rotation.set(0, 0, 0);
    const flip = probeY !== null && probeY > restY ? -1 : 1;
    const drop = flip * THREE.MathUtils.degToRad(65);
    leftArm.rotation.set(0, 0, drop);
    rightArm.rotation.set(0, 0, -drop);
    const leftFore = bone('leftLowerArm');
    const rightFore = bone('rightLowerArm');
    if (leftFore) leftFore.rotation.set(0, -0.25, 0);
    if (rightFore) rightFore.rotation.set(0, 0.25, 0);
    h.update();
  };

  const playRandomClip = () => {
    if (!clips.length || !vrm || !mixer) return;
    let pick = clips[Math.floor(Math.random() * clips.length)];
    if (clips.length > 1) {
      while (pick.url === currentUrl) pick = clips[Math.floor(Math.random() * clips.length)];
    }
    currentUrl = pick.url;
    if (current) {
      const old = current;
      old.fadeOut(FADE);
      setTimeout(() => old.stop(), FADE * 1000 + 100);
    }
    current = new THREE.AnimationAction(mixer, pick.clip, vrm.scene);
    current.setLoop(THREE.LoopOnce, 1);
    current.clampWhenFinished = true;
    current.play();
    current.fadeIn(FADE);
  };

  const loadVrmas = () => {
    if (!vrm) return;
    if (!VRMA_URLS.length) return;
    if (!mixer) {
      mixer = new THREE.AnimationMixer(vrm.scene);
      mixer.addEventListener('finished', () => {
        playRandomClip();
      });
    }
    let done = 0;
    const oneDone = () => {
      done += 1;
      if (done !== VRMA_URLS.length) return;
      if (!clips.length) return;
      playRandomClip();
    };
    VRMA_URLS.forEach((url) => {
      const animLoader = new GLTFLoader();
      animLoader.register((parser) => new VRMAnimationLoaderPlugin());
      animLoader.load(
        url,
        (gltfAnim) => {
          try {
            const anim = gltfAnim.userData.vrmAnimations && gltfAnim.userData.vrmAnimations[0];
            if (anim) clips.push({ url, clip: createVRMAnimationClip(anim, vrm) });
          } catch (e) {}
          oneDone();
        },
        undefined,
        () => oneDone()
      );
    });
  };

  let frameData = null;
  const frame = () => {
    if (!frameData) return;
    const size = frameData.size, center = frameData.center;
    const halfFov = (camera.fov * Math.PI) / 360;
    const width = Math.min(Math.max(size.x, size.z), size.y * 0.85);
    const distH = (size.y / 2) / Math.tan(halfFov);
    const distW = (width / 2) / (Math.tan(halfFov) * camera.aspect);
    const dist = Math.max(distH, distW) * 1.25;
    camera.position.set(center.x, center.y + size.y * 0.05, center.z + dist);
    camera.lookAt(center.x, center.y, center.z);
  };

  new GLTFLoader()
    .register((parser) => new VRMLoaderPlugin(parser))
    .load(
      MODEL_URL,
      (gltf) => {
        vrm = gltf.userData.vrm || null;
        model = vrm ? vrm.scene : gltf.scene;
        model.traverse((o) => { o.frustumCulled = false; });
        if (vrm) {
          VRMUtils.rotateVRM0(vrm);
          vrm.scene.rotation.y = Math.PI;
        }
        scene.add(model);
        const box = new THREE.Box3().setFromObject(model);
        frameData = { size: box.getSize(new THREE.Vector3()), center: box.getCenter(new THREE.Vector3()) };
        restPose(vrm);
        frame();
        if (loading) loading.style.display = 'none';
        loadVrmas();
      },
      undefined,
      () => {
        if (loading) loading.style.display = 'none';
        canvas.style.display = 'none';
        if (fallback) fallback.style.display = 'flex';
      }
    );

  const resize = () => {
    const w = canvas.clientWidth || 1;
    const h = canvas.clientHeight || 1;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    frame();
  };
  resize();
  if (window.ResizeObserver) new ResizeObserver(resize).observe(canvas);

  const clock = new THREE.Clock();
  renderer.setAnimationLoop(() => {
    const delta = clock.getDelta();
    if (mixer) mixer.update(delta);
    if (vrm) vrm.update(delta);
    renderer.render(scene, camera);
  });
  window.__stopHeroVrm = () => renderer.setAnimationLoop(null);
}

window.__startHeroVrmMain = start;
const auto = window.__heroVrmCanvas;
if (auto) window.__startHeroVrmMain(auto);`
];

// ── Bootstrap ────────────────────────────────────────────────────────────
const showFallback = () => {
  const loading = document.getElementById('hero-vrm-loading');
  const fallback = document.getElementById('hero-vrm-fallback');
  const canvas = document.getElementById('hero-vrm-canvas');
  if (loading) loading.style.display = 'none';
  if (canvas) canvas.style.display = 'none';
  if (fallback) fallback.style.display = 'flex';
};

const start = (canvas) => {
  if (!canvas || canvas.dataset.vrmActive === '1') return;
  canvas.dataset.vrmActive = '1';
  const loading = document.getElementById('hero-vrm-loading');
  if (loading) loading.style.display = 'flex';
  if (window.__stopHeroVrm) { try { window.__stopHeroVrm(); } catch (e) {} }

  const readSize = () => {
    const rect = canvas.getBoundingClientRect();
    return {
      w: Math.max(1, Math.round(rect.width)),
      h: Math.max(1, Math.round(rect.height)),
      dpr: window.devicePixelRatio || 1
    };
  };

  const canUseWorker =
    typeof Worker === 'function' &&
    typeof Blob === 'function' &&
    typeof URL !== 'undefined' &&
    typeof URL.createObjectURL === 'function' &&
    typeof canvas.transferControlToOffscreen === 'function';

  const startWorker = () => {
    let worker = null;
    try {
      worker = new Worker(
        URL.createObjectURL(new Blob([WORKER_SRC.join('\n')], { type: 'text/javascript' })),
        { type: 'module' }
      );
    } catch (e) {
      worker = null;
    }
    if (!worker) { startMainThread(); return; }
    let settled = false;
    worker.onmessage = (e) => {
      const type = e.data && e.data.type;
      if (type === 'ready') {
        settled = true;
        if (loading) loading.style.display = 'none';
      } else if (type === 'error' || type === 'webgl-error') {
        settled = true;
        showFallback();
      }
    };
    worker.onerror = () => {
      // Worker module/CDN failures land here (e.g. esm.sh unreachable).
      // Canvas control is already transferred at this point, so a
      // main-thread renderer cannot take over — surface the glyph instead
      // of leaving the spinner up.
      if (!settled) showFallback();
    };
    const s = readSize();
    let off = null;
    try {
      off = canvas.transferControlToOffscreen();
    } catch (e) {
      off = null;
    }
    if (off === null) {
      // Transfer failed (should not happen where the feature is detected):
      // control was never handed over, so the main-thread fallback can
      // still render this canvas.
      worker.terminate();
      worker = null;
      startMainThread();
      return;
    }
    worker.postMessage(
      { type: 'init', canvas: off, width: s.w, height: s.h, dpr: s.dpr },
      [off]
    );
    let ro = null;
    if (typeof ResizeObserver === 'function') {
      ro = new ResizeObserver(() => {
        const size = readSize();
        worker.postMessage({ type: 'resize', width: size.w, height: size.h, dpr: size.dpr });
      });
      ro.observe(canvas);
    }
    window.__stopHeroVrm = () => {
      if (ro) { ro.disconnect(); ro = null; }
      if (worker) { worker.terminate(); worker = null; }
    };
  };

  const startMainThread = () => {
    // Run the scene on the page thread. The fallback module reads
    // __heroVrmCanvas for its auto-start path and exposes
    // __startHeroVrmMain; it surfaces the fallback glyph itself if WebGL
    // is unavailable there too.
    window.__heroVrmCanvas = canvas;
    try {
      const url = URL.createObjectURL(new Blob([FALLBACK_SRC.join('\n')], { type: 'text/javascript' }));
      import(url).catch(() => showFallback());
    } catch (e) {
      showFallback();
    }
  };

  if (canUseWorker) {
    // Safari ships transferControlToOffscreen but no WebGL in workers —
    // detect that before transferring control (after a transfer the main
    // thread can never render this canvas again). A tiny probe worker
    // answers true/false; anything suspicious falls back to the main
    // thread, which every browser with WebGL can render.
    const PROBE_SRC = "postMessage((() => { try { return !!new OffscreenCanvas(1,1).getContext('webgl2'); } catch (e) { return false; } })());";
    let probe = null;
    try {
      probe = new Worker(URL.createObjectURL(new Blob([PROBE_SRC], { type: 'text/javascript' })));
    } catch (e) {
      probe = null;
    }
    if (probe) {
      const bail = () => {
        if (!probe) return;
        clearTimeout(probeTimer);
        try { probe.terminate(); } catch (e) {}
        probe = null;
        startMainThread();
      };
      const probeTimer = setTimeout(bail, 600);
      probe.onmessage = (e) => {
        clearTimeout(probeTimer);
        try { probe.terminate(); } catch (err) {}
        probe = null;
        if (e.data === true) startWorker();
        else startMainThread();
      };
      probe.onerror = bail;
      return;
    }
  }

  startMainThread();
};

window.__startHeroVrm = (canvas) => start(canvas);

const existing = document.getElementById('hero-vrm-canvas');
if (existing) window.__startHeroVrm(existing);
"#;

    pub fn start_or_inject() {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else { return };
        let Some(head) = doc.head() else { return };

        // The import map resolves the bare "three" specifier that the
        // @pixiv/three-vrm packages import internally — used by the
        // main-thread fallback module (the worker rewrites them via esm.sh
        // instead, since workers cannot see document import maps).
        if doc.get_element_by_id("hero-vrm-importmap").is_none() {
            if let Ok(el) = doc.create_element("script") {
                let _ = el.set_id("hero-vrm-importmap");
                let _ = el.set_attribute("type", "importmap");
                el.set_text_content(Some(IMPORTMAP));
                let _ = head.append_child(&el);
            }
        }

        // Module already installed? Point it at the current canvas (remount case).
        if let Ok(starter) = js_sys::Reflect::get(&window, &JsValue::from_str("__startHeroVrm")) {
            if starter.is_function() {
                if let Some(canvas) = doc.get_element_by_id("hero-vrm-canvas") {
                    let f = js_sys::Function::from(starter);
                    let _ = f.call1(&window, &canvas);
                }
                return;
            }
        }

        if doc.get_element_by_id("hero-vrm-module").is_some() {
            return; // still loading; it auto-starts on the current canvas
        }

        let model_url = super::DEMO_MODEL.to_string();
        // The clip list is generated by build.rs (one hashed asset!() URL per
        // .vrma file in assets/vrma/) and baked into the module as JSON.
        let vrma_json = format!(
            "[{}]",
            vrma_url_list()
                .iter()
                .map(|u| format!("\"{u}\""))
                .collect::<Vec<_>>()
                .join(",")
        );
        if let Ok(el) = doc.create_element("script") {
            let _ = el.set_id("hero-vrm-module");
            let _ = el.set_attribute("type", "module");
            el.set_text_content(Some(
                &MODULE_JS
                    .replace("__MODEL_URL__", &model_url)
                    .replace("__VRMA_URLS__", &vrma_json),
            ));
            let _ = head.append_child(&el);
        }
    }
}

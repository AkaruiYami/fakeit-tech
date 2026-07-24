(() => {
  // Scroll fade-in via IntersectionObserver
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add("visible");
          observer.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.1 }
  );

  document.querySelectorAll(".fade-in").forEach((el) => observer.observe(el));

  // Mobile nav toggle
  const toggle = document.querySelector(".nav-toggle");
  const links = document.querySelector(".nav-links");
  if (toggle && links) {
    toggle.addEventListener("click", () => {
      links.classList.toggle("open");
      toggle.classList.toggle("active");
    });
    links.querySelectorAll("a").forEach((a) => {
      a.addEventListener("click", () => {
        links.classList.remove("open");
        toggle.classList.remove("active");
      });
    });
  }

  // Pause videos not in viewport to save resources
  const videoObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        const video = entry.target;
        if (entry.isIntersecting) {
          video.play().catch(() => {});
        } else {
          video.pause();
        }
      });
    },
    { threshold: 0.2 }
  );

  document.querySelectorAll("video").forEach((v) => videoObserver.observe(v));

  // ── Preview Lightbox ──
  const overlay = document.createElement("div");
  overlay.className = "preview-overlay";
  overlay.innerHTML = `
    <div class="preview-overlay-inner">
      <button class="preview-close" aria-label="Close preview">&times;</button>
      <div class="preview-overlay-video"></div>
      <div class="preview-overlay-label"></div>
    </div>
  `;
  document.body.appendChild(overlay);

  const overlayVideo = overlay.querySelector(".preview-overlay-video");
  const overlayLabel = overlay.querySelector(".preview-overlay-label");
  const closeBtn = overlay.querySelector(".preview-close");

  function openLightbox(card) {
    const src = card.querySelector("video");
    if (!src) return;

    const video = document.createElement("video");
    video.autoplay = true;
    video.loop = true;
    video.muted = true;
    video.playsInline = true;

    src.querySelectorAll("source").forEach((s) => {
      const source = document.createElement("source");
      source.src = s.src;
      source.type = s.type;
      video.appendChild(source);
    });

    const name = card.querySelector(".preview-info h3");
    overlayLabel.textContent = name ? name.textContent : "";

    overlayVideo.innerHTML = "";
    overlayVideo.appendChild(video);
    overlay.classList.add("open");
    document.body.style.overflow = "hidden";
  }

  function closeLightbox() {
    overlay.classList.remove("open");
    document.body.style.overflow = "";
    setTimeout(() => { overlayVideo.innerHTML = ""; }, 300);
  }

  document.querySelectorAll(".preview-card").forEach((card) => {
    card.addEventListener("click", () => openLightbox(card));
  });

  closeBtn.addEventListener("click", closeLightbox);
  overlay.addEventListener("click", (e) => {
    if (e.target === overlay) closeLightbox();
  });
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") closeLightbox();
  });
})();

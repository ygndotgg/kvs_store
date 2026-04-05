"use client";

import { useEffect, useRef, useState } from "react";

const metrics = [
  {
    value: 3,
    suffix: "",
    prefix: "",
    label: "Core operations",
    sublabel: "Set, Get, Remove",
  },
  {
    value: 1,
    suffix: "",
    prefix: "",
    label: "Primary log write path",
    sublabel: "append-only persistence",
  },
  {
    value: 2,
    suffix: "",
    prefix: "",
    label: "Engine choices",
    sublabel: "custom KvStore and sled",
  },
];

function AnimatedNumber({ end, suffix = "", prefix = "" }: { end: number; suffix?: string; prefix?: string }) {
  const [count, setCount] = useState(0);
  const [isScrambling, setIsScrambling] = useState(true);
  const ref = useRef<HTMLDivElement>(null);
  const [hasAnimated, setHasAnimated] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting && !hasAnimated) {
        setHasAnimated(true);
        const duration = 2500;
        const startTime = performance.now();
        const animate = (currentTime: number) => {
          const elapsed = currentTime - startTime;
          const progress = Math.min(elapsed / duration, 1);
          const eased = 1 - Math.pow(1 - progress, 4);
          setCount(Math.floor(eased * end));
          setIsScrambling(progress < 0.8);
          if (progress < 1) requestAnimationFrame(animate);
        };
        requestAnimationFrame(animate);
      }
    }, { threshold: 0.5 });

    if (ref.current) observer.observe(ref.current);
    return () => observer.disconnect();
  }, [end, hasAnimated]);

  const displayValue = count.toLocaleString();

  return (
    <div ref={ref} className="inline-flex items-baseline">
      <span className="mr-1 text-muted-foreground">{prefix}</span>
      <span className="tabular-nums">
        {displayValue.split("").map((char, i) => (
          <span key={`${char}-${i}`} className={`inline-block transition-all duration-150 ${isScrambling && char !== "," ? "blur-[1px]" : ""}`}>
            {char}
          </span>
        ))}
      </span>
      <span className="text-muted-foreground">{suffix}</span>
    </div>
  );
}

function GridBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const timeRef = useRef(0);
  const frameRef = useRef(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const resize = () => {
      const rect = canvas.getBoundingClientRect();
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      canvas.width = rect.width * dpr;
      canvas.height = rect.height * dpr;
      ctx.scale(dpr, dpr);
    };

    resize();
    window.addEventListener("resize", resize);

    const render = () => {
      const rect = canvas.getBoundingClientRect();
      const width = rect.width;
      const height = rect.height;
      ctx.clearRect(0, 0, width, height);

      const gridSize = 60;
      const time = timeRef.current;

      for (let x = 0; x < width; x += gridSize) {
        for (let y = 0; y < height; y += gridSize) {
          const wave = Math.sin(x * 0.01 + y * 0.01 + time) * 0.5 + 0.5;
          const size = 1 + wave * 2;
          ctx.beginPath();
          ctx.arc(x, y, size, 0, Math.PI * 2);
          ctx.fillStyle = "rgba(255, 255, 255, 0.04)";
          ctx.fill();
        }
      }

      const pulseY = (time * 30) % height;
      ctx.strokeStyle = "rgba(255, 255, 255, 0.03)";
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(0, pulseY);
      ctx.lineTo(width, pulseY);
      ctx.stroke();

      timeRef.current += 0.02;
      frameRef.current = requestAnimationFrame(render);
    };

    render();

    return () => {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(frameRef.current);
    };
  }, []);

  return <canvas ref={canvasRef} className="absolute inset-0 pointer-events-none" style={{ width: "100%", height: "100%" }} />;
}

function DotGraph({
  color = "white",
  height = 32,
  freq1 = 0.35,
  freq2 = 0.12,
  freqT = 0.7,
  speed = 0.025,
  baseline = 0.3,
  amplitude = 0.5,
}: {
  color?: string;
  height?: number;
  freq1?: number;
  freq2?: number;
  freqT?: number;
  speed?: number;
  baseline?: number;
  amplitude?: number;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const frameRef = useRef(0);
  const timeRef = useRef(Math.random() * 100);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const width = canvas.offsetWidth || 300;
    const resolvedHeight = height;
    canvas.width = width * dpr;
    canvas.height = resolvedHeight * dpr;
    ctx.scale(dpr, dpr);

    const render = () => {
      ctx.clearRect(0, 0, width, resolvedHeight);
      const time = timeRef.current;
      const cols = Math.floor(width / 8);

      for (let i = 0; i < cols; i++) {
        const raw = baseline + amplitude * Math.sin(i * freq1 + time) * Math.cos(i * freq2 + time * freqT);
        const value = Math.max(0, Math.min(1, raw));
        const dotY = resolvedHeight - 4 - value * (resolvedHeight - 8);
        const x = i * 8 + 4;
        const alpha = 0.15 + value * 0.55;
        const radius = 1.5 + value * 1.2;

        ctx.beginPath();
        ctx.arc(x, dotY, radius, 0, Math.PI * 2);
        ctx.fillStyle =
          color === "green" ? `rgba(236, 168, 214, ${alpha})` : `rgba(255, 255, 255, ${alpha})`;
        ctx.fill();
      }

      timeRef.current += speed;
      frameRef.current = requestAnimationFrame(render);
    };

    render();
    return () => cancelAnimationFrame(frameRef.current);
  }, [amplitude, baseline, color, freq1, freq2, freqT, height, speed]);

  return <canvas ref={canvasRef} style={{ width: "100%", height: `${height}px`, display: "block" }} />;
}

export function MetricsSection() {
  const [isVisible, setIsVisible] = useState(false);
  const sectionRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) setIsVisible(true);
    }, { threshold: 0.1 });

    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="relative overflow-hidden py-32 lg:py-40">
      <GridBackground />

      <div className="relative z-10 mx-auto max-w-[1400px] px-6 lg:px-12">
        <div className="mb-20 grid gap-8 lg:mb-32 lg:grid-cols-12">
          <div className="lg:col-span-8 lg:col-start-1">
            <div className="mb-6 flex items-center gap-4">
              <span className="flex items-center gap-2 bg-[#eca8d6]/10 px-3 py-1 text-xs font-mono text-[#eca8d6]">
                <span className="h-2 w-2 animate-pulse rounded-full bg-[#eca8d6]" />
                IMPLEMENTED
              </span>
            </div>

            <h2
              className={`text-6xl font-display leading-[0.95] tracking-tight transition-all duration-1000 md:text-7xl lg:text-[140px] ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
            >
              Built around
              <br />
              <span className="text-muted-foreground">primitives.</span>
            </h2>
          </div>
        </div>

        <div className="grid gap-px bg-foreground/10 lg:grid-cols-3">
          {metrics.map((metric, index) => (
            <div
              key={metric.label}
              className={`bg-background p-8 transition-all duration-700 lg:p-10 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
              style={{ transitionDelay: `${index * 120}ms` }}
            >
              <div className="mb-12 text-6xl font-display lg:text-7xl">
                <AnimatedNumber end={metric.value} suffix={metric.suffix} prefix={metric.prefix} />
              </div>
              <h3 className="text-2xl font-display">{metric.label}</h3>
              <p className="mt-3 text-sm text-muted-foreground">{metric.sublabel}</p>
              <div className="mt-8">
                <DotGraph
                  color={index === 1 ? "green" : "white"}
                  height={32}
                  freq1={0.28 + index * 0.04}
                  freq2={0.1 + index * 0.03}
                  freqT={0.7}
                  speed={0.025 + index * 0.004}
                  baseline={0.28}
                  amplitude={0.54}
                />
              </div>
            </div>
          ))}
        </div>

        <div
          className={`mt-12 grid gap-4 transition-all delay-300 duration-1000 lg:grid-cols-3 ${
            isVisible ? "opacity-100" : "opacity-0"
          }`}
        >
          <div className="border border-foreground/10 bg-foreground/[0.02] p-6">
            <p className="text-sm font-mono text-muted-foreground">Protocol</p>
            <p className="mt-3 text-lg leading-relaxed text-foreground/80">
              Requests and responses are modeled as Rust enums and serialized as JSON over TCP.
            </p>
          </div>
          <div className="border border-foreground/10 bg-foreground/[0.02] p-6">
            <p className="text-sm font-mono text-muted-foreground">Concurrency</p>
            <p className="mt-3 text-lg leading-relaxed text-foreground/80">
              The design moved beyond thread-per-request to shared-queue and Rayon-backed pool strategies.
            </p>
          </div>
          <div className="border border-foreground/10 bg-foreground/[0.02] p-6">
            <p className="text-sm font-mono text-muted-foreground">Abstraction</p>
            <p className="mt-3 text-lg leading-relaxed text-foreground/80">
              `KvsEngine` keeps the service layer decoupled from storage implementation details.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}

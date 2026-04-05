"use client";

import { useEffect, useRef, useState } from "react";

const words = ["persist", "compact", "serve", "scale"];

function BlurWord({ word, trigger }: { word: string; trigger: number }) {
  const letters = word.split("");
  const stagger = 45;
  const duration = 500;
  const gradientHold = stagger * letters.length + duration + 200;

  const [letterStates, setLetterStates] = useState<{ opacity: number; blur: number }[]>(
    letters.map(() => ({ opacity: 0, blur: 20 })),
  );
  const [showGradient, setShowGradient] = useState(true);
  const framesRef = useRef<number[]>([]);
  const timersRef = useRef<ReturnType<typeof setTimeout>[]>([]);

  useEffect(() => {
    framesRef.current.forEach(cancelAnimationFrame);
    timersRef.current.forEach(clearTimeout);
    framesRef.current = [];
    timersRef.current = [];

    setLetterStates(letters.map(() => ({ opacity: 0, blur: 20 })));
    setShowGradient(true);

    letters.forEach((_, i) => {
      const timeout = setTimeout(() => {
        const start = performance.now();
        const tick = (now: number) => {
          const progress = Math.min((now - start) / duration, 1);
          const eased = 1 - Math.pow(1 - progress, 3);

          setLetterStates((previous) => {
            const next = [...previous];
            next[i] = { opacity: eased, blur: 20 * (1 - eased) };
            return next;
          });

          if (progress < 1) {
            const frame = requestAnimationFrame(tick);
            framesRef.current.push(frame);
          }
        };

        const frame = requestAnimationFrame(tick);
        framesRef.current.push(frame);
      }, i * stagger);

      timersRef.current.push(timeout);
    });

    const gradientTimer = setTimeout(() => setShowGradient(false), gradientHold);
    timersRef.current.push(gradientTimer);

    return () => {
      framesRef.current.forEach(cancelAnimationFrame);
      timersRef.current.forEach(clearTimeout);
    };
  }, [trigger]);

  const gradientColors = ["#eca8d6", "#d7c7ff", "#8ac5ff", "#f7d67e", "#eca8d6"];

  return (
    <>
      {letters.map((char, i) => {
        const colorIndex = (i / Math.max(letters.length - 1, 1)) * (gradientColors.length - 1);
        const lower = Math.floor(colorIndex);
        const upper = Math.min(lower + 1, gradientColors.length - 1);
        const t = colorIndex - lower;

        const hexToRgb = (hex: string) => {
          const r = parseInt(hex.slice(1, 3), 16);
          const g = parseInt(hex.slice(3, 5), 16);
          const b = parseInt(hex.slice(5, 7), 16);
          return [r, g, b];
        };

        const [r1, g1, b1] = hexToRgb(gradientColors[lower]);
        const [r2, g2, b2] = hexToRgb(gradientColors[upper]);
        const r = Math.round(r1 + (r2 - r1) * t);
        const g = Math.round(g1 + (g2 - g1) * t);
        const b = Math.round(b1 + (b2 - b1) * t);

        return (
          <span
            key={`${char}-${i}`}
            style={{
              display: "inline-block",
              opacity: letterStates[i]?.opacity ?? 0,
              filter: `blur(${letterStates[i]?.blur ?? 20}px)`,
              color: showGradient ? `rgb(${r},${g},${b})` : "white",
              transition: "color 0.4s ease",
            }}
          >
            {char}
          </span>
        );
      })}
    </>
  );
}

export function HeroSection() {
  const [isVisible, setIsVisible] = useState(false);
  const [wordIndex, setWordIndex] = useState(0);

  useEffect(() => {
    setIsVisible(true);
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setWordIndex((previous) => (previous + 1) % words.length);
    }, 2500);

    return () => clearInterval(interval);
  }, []);

  return (
    <section className="relative flex min-h-screen flex-col items-start justify-center overflow-hidden bg-black">
      <div className="absolute inset-0 z-0">
        <div className="absolute inset-0 bg-[radial-gradient(circle_at_20%_20%,rgba(236,168,214,0.18),transparent_32%),radial-gradient(circle_at_80%_25%,rgba(255,255,255,0.08),transparent_28%),radial-gradient(circle_at_60%_80%,rgba(118,178,255,0.14),transparent_30%),linear-gradient(180deg,rgba(8,10,14,0.7),rgba(3,4,6,0.96))]" />
        <div className="absolute inset-0 opacity-50 [background-image:linear-gradient(rgba(255,255,255,0.06)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.06)_1px,transparent_1px)] [background-size:88px_88px]" />
        <div className="absolute inset-0 bg-gradient-to-r from-black/70 via-black/40 to-black/20" />
      </div>

      <div className="absolute inset-0 z-[2] overflow-hidden opacity-20 pointer-events-none">
        {[...Array(8)].map((_, i) => (
          <div
            key={`h-${i}`}
            className="absolute h-px bg-white/10"
            style={{ top: `${12.5 * (i + 1)}%`, left: 0, right: 0 }}
          />
        ))}
        {[...Array(12)].map((_, i) => (
          <div
            key={`v-${i}`}
            className="absolute w-px bg-white/10"
            style={{ left: `${8.33 * (i + 1)}%`, top: 0, bottom: 0 }}
          />
        ))}
      </div>

      <div className="relative z-10 mx-auto w-full max-w-[1400px] px-6 py-32 lg:px-12 lg:py-40">
        <div className="lg:max-w-[58%]">
          <div
            className={`mb-8 transition-all duration-700 ${
              isVisible ? "translate-y-0 opacity-100" : "translate-y-4 opacity-0"
            }`}
          >
            <span className="inline-flex items-center gap-3 text-sm font-mono text-white/60">
              <span className="h-px w-8 bg-white/30" />
              Rust project case study · persistence + concurrency + networking
            </span>
          </div>

          <div className="mb-12">
            <h1
              className={`text-left font-display text-[clamp(2rem,6vw,7rem)] leading-[0.92] tracking-tight text-white transition-all duration-1000 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
            >
              <span className="block whitespace-nowrap">Concurrent key-value store,</span>
              <span className="block whitespace-nowrap">
                built to <span className="relative inline-block"><BlurWord word={words[wordIndex]} trigger={wordIndex} /></span>
              </span>
            </h1>

            <p
              className={`mt-8 max-w-2xl text-lg leading-relaxed text-white/68 transition-all duration-1000 delay-150 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-6 opacity-0"
              }`}
            >
              Aryavarth KVS evolved from a simple in-memory store into a durable Rust engine with an
              append-only log, compaction, JSON-over-TCP client/server communication, and concurrent
              request execution through thread-pool strategies.
            </p>

            <p
              className={`mt-4 max-w-2xl text-base leading-relaxed text-white/50 transition-all duration-1000 delay-200 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-6 opacity-0"
              }`}
            >
              The current milestone is a single-node service. Making it distributed is the next phase,
              and that will be completed soon by implementing the Raft runtime.
            </p>
          </div>
        </div>
      </div>

      <div
        className={`absolute right-0 bottom-12 left-0 px-6 lg:px-12 transition-all duration-700 delay-500 ${
          isVisible ? "opacity-100" : "opacity-0"
        }`}
      >
        <div className="mx-auto flex max-w-[1400px] flex-wrap items-start gap-8 lg:gap-20">
          {[
            { value: "O(1)", label: "index lookup via in-memory log pointers" },
            { value: "JSON/TCP", label: "wire protocol for client and server" },
            { value: "Raft", label: "next distributed runtime phase" },
          ].map((stat) => (
            <div key={stat.label} className="flex flex-col gap-2">
              <span className="text-3xl font-display text-white lg:text-4xl">{stat.value}</span>
              <span className="text-xs leading-tight text-white/50">{stat.label}</span>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

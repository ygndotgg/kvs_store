"use client";

import { useEffect, useRef, useState } from "react";

const phases = [
  { name: "Phase 1", title: "Persistent single-node engine", status: "completed" },
  { name: "Phase 2", title: "Network service + concurrent execution", status: "completed" },
  { name: "Phase 3", title: "Engine abstraction + sled comparison", status: "active" },
  { name: "Phase 4", title: "Distributed replication with Raft runtime", status: "next" },
];

export function InfrastructureSection() {
  const [isVisible, setIsVisible] = useState(false);
  const [activePhase, setActivePhase] = useState(0);
  const sectionRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) setIsVisible(true);
    }, { threshold: 0.1 });

    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setActivePhase((previous) => (previous + 1) % phases.length);
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  return (
    <section id="infra" ref={sectionRef} className="relative overflow-hidden py-32 lg:py-40">
      <div className="mx-auto max-w-[1400px] px-6 lg:px-12">
        <div className="mb-20">
          <span
            className={`mb-8 inline-flex items-center gap-4 text-sm font-mono text-muted-foreground transition-all duration-700 ${
              isVisible ? "opacity-100" : "opacity-0"
            }`}
          >
            <span className="h-px w-12 bg-foreground/20" />
            Roadmap
          </span>

          <div className="grid items-stretch gap-8 lg:grid-cols-[auto_1fr] lg:gap-16">
            <div
              className={`w-48 shrink-0 transition-all duration-1000 lg:w-72 xl:w-80 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
            >
              <div className="relative h-72 w-72 max-w-full rounded-full border border-foreground/15 bg-[radial-gradient(circle_at_center,rgba(236,168,214,0.18),transparent_26%),radial-gradient(circle_at_30%_30%,rgba(255,255,255,0.08),transparent_22%),linear-gradient(180deg,rgba(255,255,255,0.02),rgba(255,255,255,0.01))]">
                <div className="absolute inset-[16%] rounded-full border border-foreground/10" />
                <div className="absolute inset-[32%] rounded-full border border-[#eca8d6]/35" />
                <div className="absolute top-[18%] left-1/2 h-3 w-3 -translate-x-1/2 rounded-full bg-white" />
                <div className="absolute top-1/2 left-[22%] h-3 w-3 -translate-y-1/2 rounded-full bg-[#eca8d6]" />
                <div className="absolute top-1/2 right-[22%] h-3 w-3 -translate-y-1/2 rounded-full bg-[#eca8d6]" />
                <div className="absolute bottom-[18%] left-1/2 h-3 w-3 -translate-x-1/2 rounded-full bg-white/70" />
              </div>
            </div>

            <div className="flex flex-col justify-center">
              <h2
                className={`text-6xl font-display leading-[0.9] tracking-tight transition-all duration-1000 md:text-7xl lg:text-[128px] ${
                  isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
                }`}
              >
                Distributed is
                <br />
                <span className="text-muted-foreground">next.</span>
              </h2>

              <p
                className={`mt-8 max-w-lg text-xl leading-relaxed text-muted-foreground transition-all delay-100 duration-1000 ${
                  isVisible ? "opacity-100" : "opacity-0"
                }`}
              >
                This page now reflects the real state of the project: durable local storage, networked
                request handling, and concurrency are already implemented. Distributed consensus is the
                upcoming phase and will be completed soon by implementing the Raft runtime.
              </p>
            </div>
          </div>
        </div>

        <div className="grid gap-6 lg:grid-cols-3">
          <div
            className={`relative overflow-hidden border border-foreground/10 bg-foreground/[0.02] p-8 transition-all duration-700 lg:col-span-2 lg:p-12 ${
              isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
            }`}
          >
            <div className="absolute inset-0 opacity-70">
              <svg className="absolute inset-0 h-full w-full" style={{ pointerEvents: "none" }}>
                <defs>
                  <style>{`
                    @keyframes drawLine {
                      0% { stroke-dashoffset: 1000; opacity: 0; }
                      15% { opacity: 1; }
                      70% { opacity: 0.7; }
                      100% { stroke-dashoffset: 0; opacity: 0; }
                    }

                    .connecting-line {
                      stroke: #eca8d6;
                      stroke-width: 1.2;
                      fill: none;
                      stroke-dasharray: 1000;
                      animation: drawLine 3s ease-in-out infinite;
                    }
                  `}</style>
                </defs>
                {[...Array(11)].map((_, i) => {
                  const x1 = 12 + (i % 4) * 22;
                  const y1 = 20 + Math.floor(i / 4) * 24;
                  const x2 = 24 + ((i + 1) % 4) * 18;
                  const y2 = 24 + Math.floor((i + 1) / 4) * 22;

                  return (
                    <line
                      key={`line-${i}`}
                      x1={`${x1}%`}
                      y1={`${y1}%`}
                      x2={`${x2}%`}
                      y2={`${y2}%`}
                      className="connecting-line"
                      style={{ animationDelay: `${i * 0.15}s` }}
                    />
                  );
                })}
              </svg>

              {[...Array(12)].map((_, i) => (
                <div
                  key={i}
                  className="absolute h-1.5 w-1.5 rounded-full bg-[#eca8d6]"
                  style={{
                    left: `${14 + (i % 4) * 22}%`,
                    top: `${18 + Math.floor(i / 4) * 24}%`,
                    animation: `pulse 2s ease-in-out ${i * 0.1}s infinite`,
                  }}
                />
              ))}
            </div>

            <div className="relative z-10">
              <div className="mb-4 flex items-baseline gap-2">
                <span className="text-8xl font-display leading-none lg:text-[10rem]">4</span>
                <span className="text-2xl text-muted-foreground">phases</span>
              </div>
              <p className="max-w-md text-muted-foreground">
                The system is being built in layers: persistence, concurrency, pluggable engines, then
                consensus replication. The ordering matters because Raft needs a solid storage and request
                execution foundation underneath it.
              </p>
            </div>
          </div>

          <div className="flex flex-col gap-6">
            <div
              className={`border border-foreground/10 bg-foreground/[0.02] p-8 transition-all delay-100 duration-700 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
            >
              <span className="text-5xl font-display lg:text-6xl">TCP</span>
              <span className="mt-2 block text-sm text-muted-foreground">Current network boundary</span>
            </div>

            <div
              className={`border border-foreground/10 bg-foreground/[0.02] p-8 transition-all delay-200 duration-700 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
              }`}
            >
              <span className="text-5xl font-display lg:text-6xl">Raft</span>
              <span className="mt-2 block text-sm text-muted-foreground">Upcoming replication runtime</span>
            </div>
          </div>
        </div>

        <div
          className={`mt-12 grid gap-4 transition-all delay-300 duration-1000 lg:grid-cols-4 ${
            isVisible ? "opacity-100" : "opacity-0"
          }`}
        >
          {phases.map((phase, index) => (
            <div
              key={phase.name}
              className={`cursor-default border p-6 transition-all duration-300 ${
                activePhase === index ? "border-foreground/30 bg-foreground/[0.04]" : "border-foreground/10"
              }`}
            >
              <div className="mb-3 flex items-center gap-2">
                <span
                  className={`h-2 w-2 rounded-full transition-colors ${
                    activePhase === index ? "bg-[#eca8d6]" : "bg-foreground/20"
                  }`}
                />
                <span className="text-xs font-mono uppercase tracking-wider text-muted-foreground">
                  {phase.status}
                </span>
              </div>
              <span className="mb-1 block font-medium">{phase.name}</span>
              <span className="text-sm text-muted-foreground">{phase.title}</span>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

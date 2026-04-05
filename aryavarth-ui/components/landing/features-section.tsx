"use client";

import { useEffect, useRef, useState } from "react";

const features = [
  {
    number: "01",
    title: "Append-Only Persistence",
    description:
      "Every mutation is serialized and appended to disk so restart and crash recovery replay the log instead of losing the dataset.",
    stats: { value: "WAL", label: "durability-first write path" },
  },
  {
    number: "02",
    title: "LogPointer Index",
    description:
      "An in-memory `HashMap<String, LogPointer>` keeps key lookups direct while values remain in the append-only log on disk.",
    stats: { value: "O(1)", label: "key lookup path" },
  },
  {
    number: "03",
    title: "Compaction",
    description:
      "Dead log entries created by overwrites and removals are reclaimed by rewriting only live commands into a new file.",
    stats: { value: "1MB", label: "compaction threshold in current code" },
  },
  {
    number: "04",
    title: "Concurrent Service Layer",
    description:
      "The storage engine sits behind a trait and is served over TCP using a thread-pool based execution model instead of unbounded threads.",
    stats: { value: "3", label: "pool strategies explored" },
  },
];

export function FeaturesSection() {
  const [isVisible, setIsVisible] = useState(false);
  const sectionRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) setIsVisible(true);
    }, { threshold: 0.1 });

    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section id="features" ref={sectionRef} className="relative overflow-hidden py-24 lg:py-32">
      <div className="mx-auto max-w-[1400px] px-6 lg:px-12">
        <div className="relative mb-24 lg:mb-32">
          <div className="grid items-end gap-8 lg:grid-cols-12">
            <div className="lg:col-span-7">
              <span className="mb-6 inline-flex items-center gap-3 text-sm font-mono text-muted-foreground">
                <span className="h-px w-12 bg-foreground/30" />
                Current architecture
              </span>
              <h2
                className={`text-6xl font-display leading-[0.9] tracking-tight transition-all duration-1000 md:text-7xl lg:text-[128px] ${
                  isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
                }`}
              >
                Storage
                <br />
                <span className="text-muted-foreground">engine.</span>
              </h2>
            </div>
            <div className="lg:col-span-5 lg:pb-4">
              <p
                className={`text-xl leading-relaxed text-muted-foreground transition-all delay-200 duration-1000 ${
                  isVisible ? "translate-y-0 opacity-100" : "translate-y-4 opacity-0"
                }`}
              >
                The current implementation focuses on being a correct, durable, concurrent single-node
                store first. The design choices mirror real database concerns: write-ahead logging,
                indexing, compaction, protocol design, and execution strategy.
              </p>
            </div>
          </div>
        </div>

        <div className="grid gap-4 md:grid-cols-2 lg:gap-6 xl:grid-cols-4">
          {features.map((feature, index) => (
            <article
              key={feature.number}
              className={`relative overflow-hidden border border-foreground/10 bg-foreground/[0.02] p-8 transition-all duration-700 lg:p-10 ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-12 opacity-0"
              }`}
              style={{ transitionDelay: `${index * 120}ms` }}
            >
              <div className="absolute inset-0 bg-[radial-gradient(circle_at_top_right,rgba(236,168,214,0.12),transparent_38%)]" />
              <div className="relative">
                <span className="text-sm font-mono text-muted-foreground">{feature.number}</span>
                <h3 className="mt-4 text-3xl font-display leading-tight">{feature.title}</h3>
                <p className="mt-5 text-base leading-relaxed text-muted-foreground">{feature.description}</p>
                <div className="mt-10 border-t border-foreground/10 pt-6">
                  <span className="text-4xl font-display">{feature.stats.value}</span>
                  <span className="mt-2 block text-sm font-mono text-muted-foreground">
                    {feature.stats.label}
                  </span>
                </div>
              </div>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}

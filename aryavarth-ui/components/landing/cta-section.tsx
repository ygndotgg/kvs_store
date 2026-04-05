"use client";

import { useEffect, useRef, useState } from "react";
import { ArrowRight } from "lucide-react";
import { Button } from "@/components/ui/button";

export function CtaSection() {
  const [isVisible, setIsVisible] = useState(false);
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  const sectionRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) setIsVisible(true);
    }, { threshold: 0.2 });

    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  const handleMouseMove = (event: React.MouseEvent<HTMLDivElement>) => {
    const rect = event.currentTarget.getBoundingClientRect();
    setMousePosition({
      x: ((event.clientX - rect.left) / rect.width) * 100,
      y: ((event.clientY - rect.top) / rect.height) * 100,
    });
  };

  return (
    <section ref={sectionRef} className="relative overflow-hidden py-24 lg:py-32">
      <div className="mx-auto max-w-[1400px] px-6 lg:px-12">
        <div
          className={`relative border border-foreground transition-all duration-1000 ${
            isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
          }`}
          onMouseMove={handleMouseMove}
        >
          <div
            className="pointer-events-none absolute inset-0 opacity-10 transition-opacity duration-300"
            style={{
              background: `radial-gradient(600px circle at ${mousePosition.x}% ${mousePosition.y}%, rgba(0,0,0,0.15), transparent 40%)`,
            }}
          />

          <div className="relative z-10 px-8 py-16 lg:px-16 lg:py-24">
            <div className="flex flex-col items-center justify-between gap-12 lg:flex-row">
              <div className="flex-1">
                <h2 className="mb-8 text-6xl font-display leading-[0.95] tracking-tight md:text-7xl lg:text-[72px]">
                  Next stop:
                  <br />
                  distributed consensus.
                </h2>

                <p className="mb-12 max-w-xl text-xl leading-relaxed text-muted-foreground">
                  The current milestone proves persistence, indexing, compaction, networking, and
                  concurrent request handling. The next milestone is a distributed version backed by a
                  Raft runtime so replication and leader-based coordination become first-class.
                </p>

                <div className="flex flex-col items-start gap-4 sm:flex-row">
                  <Button
                    asChild
                    size="lg"
                    className="group h-14 rounded-full bg-foreground px-8 text-base text-background hover:bg-foreground/90"
                  >
                    <a href="#infra">
                      See roadmap
                      <ArrowRight className="ml-2 h-4 w-4 transition-transform group-hover:translate-x-1" />
                    </a>
                  </Button>
                  <Button
                    asChild
                    size="lg"
                    variant="outline"
                    className="h-14 rounded-full border-foreground/20 px-8 text-base hover:bg-foreground/5"
                  >
                    <a href="#developers">Review CLI</a>
                  </Button>
                </div>

                <p className="mt-8 font-mono text-sm text-muted-foreground">
                  Single-node today. Raft-backed distribution next.
                </p>
              </div>

              <div className="hidden h-[650px] w-[600px] items-end justify-center lg:flex">
                <div className="relative h-full w-full">
                  <div className="absolute inset-[18%] border border-foreground/15 bg-[radial-gradient(circle_at_top,rgba(236,168,214,0.18),transparent_28%)]" />
                  <div className="absolute top-[30%] left-[18%] h-32 w-32 rounded-full border border-foreground/20 bg-background/40" />
                  <div className="absolute top-[30%] right-[18%] h-32 w-32 rounded-full border border-foreground/20 bg-background/40" />
                  <div className="absolute top-[48%] left-1/2 h-44 w-px -translate-x-1/2 bg-gradient-to-b from-[#eca8d6] to-transparent" />
                  <div className="absolute top-[34%] left-[30%] h-px w-[40%] bg-gradient-to-r from-transparent via-[#eca8d6] to-transparent" />
                  <div className="absolute bottom-[18%] left-[24%] h-24 w-24 rounded-full border border-[#eca8d6]/30" />
                  <div className="absolute right-[24%] bottom-[18%] h-24 w-24 rounded-full border border-[#eca8d6]/30" />
                </div>
              </div>
            </div>
          </div>

          <div className="absolute top-0 right-0 h-32 w-32 border-b border-l border-foreground/10" />
          <div className="absolute bottom-0 left-0 h-32 w-32 border-t border-r border-foreground/10" />
        </div>
      </div>
    </section>
  );
}

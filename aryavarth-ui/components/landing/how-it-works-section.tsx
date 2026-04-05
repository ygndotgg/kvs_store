"use client";

import { useEffect, useRef, useState } from "react";

const steps = [
  {
    number: "01",
    title: "Write",
    subtitle: "commands to log",
    description:
      "Set and remove operations are serialized and appended, giving the store a durable command history to replay on restart.",
    code: `let serialized = serde_json::to_string(&cmd)?;
writeln!(inner.writer, "{}", serialized)?;
inner.writer.flush()?;`,
  },
  {
    number: "02",
    title: "Resolve",
    subtitle: "keys through index",
    description:
      "Reads avoid a full scan by using a LogPointer index to jump straight to the latest command bytes for a key.",
    code: `struct LogPointer {
  offset: u64,
  length: u64,
  file_id: u64,
}`,
  },
  {
    number: "03",
    title: "Serve",
    subtitle: "requests concurrently",
    description:
      "Clients send JSON requests over TCP, and the server executes them through a pluggable engine behind a bounded thread-pool strategy.",
    code: `pub enum Request {
  Set { key: String, value: String },
  Get { key: String },
  Remove { key: String },
}`,
  },
];

export function HowItWorksSection() {
  const [activeStep, setActiveStep] = useState(0);
  const [isVisible, setIsVisible] = useState(false);
  const sectionRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) setIsVisible(true);
    }, { threshold: 0.1 });

    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setActiveStep((previous) => (previous + 1) % steps.length);
    }, 6000);

    return () => clearInterval(interval);
  }, []);

  return (
    <section
      id="how-it-works"
      ref={sectionRef}
      className="relative overflow-hidden bg-[oklch(0.09_0.01_260)] py-24 text-white lg:py-32"
    >
      <div className="pointer-events-none absolute bottom-0 left-0 h-[400px] w-[400px] rounded-full bg-white/[0.02] blur-[100px]" />

      <div className="relative z-10 mx-auto max-w-[1400px] px-6 lg:px-12">
        <div className="relative mb-0 grid items-end gap-4 lg:mb-0 lg:grid-cols-2 lg:gap-12">
          <div className="overflow-hidden pb-0 lg:pb-32">
            <div
              className={`transition-all duration-1000 ${
                isVisible ? "translate-x-0 opacity-100" : "-translate-x-12 opacity-0"
              }`}
            >
              <span className="mb-8 inline-flex items-center gap-3 text-sm font-mono text-white/40">
                <span className="h-px w-12 bg-white/20" />
                Request lifecycle
              </span>
            </div>

            <h2
              className={`text-6xl font-display leading-[0.85] tracking-tight transition-all delay-100 duration-1000 md:text-7xl lg:text-[128px] ${
                isVisible ? "translate-y-0 opacity-100" : "translate-y-16 opacity-0"
              }`}
            >
              <span className="block">Persist.</span>
              <span className="block text-white/30">Index.</span>
              <span className="block text-white/10">Serve.</span>
            </h2>
          </div>

          <div
            className={`relative h-[320px] overflow-hidden transition-all delay-200 duration-1000 lg:h-[640px] ${
              isVisible ? "opacity-100" : "opacity-0"
            }`}
          >
            <div className="absolute inset-0 rounded-full border border-white/10 bg-[radial-gradient(circle_at_center,rgba(236,168,214,0.16),transparent_28%),radial-gradient(circle_at_center,rgba(255,255,255,0.10),transparent_45%)]" />
            <div className="absolute top-1/2 left-1/2 h-[72%] w-[72%] -translate-x-1/2 -translate-y-1/2 rounded-full border border-white/15" />
            <div className="absolute top-1/2 left-1/2 h-[48%] w-[48%] -translate-x-1/2 -translate-y-1/2 rounded-full border border-[#eca8d6]/40" />
            <div className="absolute inset-x-[18%] top-1/2 h-px bg-white/10" />
            <div className="absolute inset-y-[18%] left-1/2 w-px bg-white/10" />
            <div className="absolute left-[24%] top-[36%] h-3 w-3 rounded-full bg-[#eca8d6]" />
            <div className="absolute top-[50%] left-[50%] h-3 w-3 -translate-x-1/2 -translate-y-1/2 rounded-full bg-white" />
            <div className="absolute right-[24%] bottom-[26%] h-3 w-3 rounded-full bg-[#eca8d6]" />
          </div>
        </div>

        <div className="grid gap-4 lg:grid-cols-3">
          {steps.map((step, index) => (
            <button
              key={step.number}
              type="button"
              onClick={() => setActiveStep(index)}
              className={`relative border p-8 text-left transition-all duration-500 lg:p-12 ${
                activeStep === index ? "border-white/60 bg-black" : "border-white/25 bg-black hover:border-white/50"
              }`}
            >
              <div className="mb-8 flex items-center gap-4">
                <span
                  className={`text-4xl font-display transition-colors duration-300 ${
                    activeStep === index ? "text-[#eca8d6]" : "text-white/20"
                  }`}
                >
                  {step.number}
                </span>
                <div className="h-px flex-1 overflow-hidden bg-white/10">
                  {activeStep === index && <div className="h-full bg-[#eca8d6]/50 animate-progress" />}
                </div>
              </div>

              <h3 className="mb-2 text-3xl font-display lg:text-4xl">{step.title}</h3>
              <span className="mb-6 block text-xl font-display text-white/40">{step.subtitle}</span>
              <p className={`leading-relaxed text-white/60 transition-opacity duration-300 ${activeStep === index ? "opacity-100" : "opacity-60"}`}>
                {step.description}
              </p>

              <div
                className={`absolute right-0 bottom-0 left-0 h-1 origin-left bg-[#eca8d6] transition-transform duration-500 ${
                  activeStep === index ? "scale-x-100" : "scale-x-0"
                }`}
              />
            </button>
          ))}
        </div>

        <div className="mt-6 overflow-hidden border border-white/15 bg-black/70">
          <pre className="overflow-x-auto p-6 text-sm leading-7 text-white/80">
            <code>{steps[activeStep].code}</code>
          </pre>
        </div>
      </div>

      <style jsx>{`
        @keyframes progress {
          from {
            width: 0%;
          }
          to {
            width: 100%;
          }
        }

        .animate-progress {
          animation: progress 6s linear forwards;
        }
      `}</style>
    </section>
  );
}

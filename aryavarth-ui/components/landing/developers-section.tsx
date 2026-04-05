"use client";

import { useEffect, useRef, useState } from "react";

const features = [
  {
    title: "kvs-server --addr 127.0.0.1:4000 --engine kvs",
    description: "Starts the server and binds the selected engine.",
  },
  {
    title: "kvs-client set <KEY> <VALUE>",
    description: "Appends a Set request through the network boundary.",
  },
  {
    title: "kvs-client get <KEY>",
    description: "Reads through the service and prints the value or `Key not found`.",
  },
  {
    title: "kvs-client rm <KEY>",
    description: "Deletes a key and treats missing keys as an error.",
  },
];

export function DevelopersSection() {
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
    <section id="developers" ref={sectionRef} className="relative overflow-hidden py-24 lg:py-32">
      <div
        className={`pointer-events-none absolute right-0 bottom-0 h-[85%] w-[55%] transition-all delay-300 duration-1000 ${
          isVisible ? "opacity-100" : "opacity-0"
        }`}
      >
        <div className="absolute inset-0 bg-[radial-gradient(circle_at_70%_30%,rgba(236,168,214,0.16),transparent_22%),radial-gradient(circle_at_50%_65%,rgba(255,255,255,0.10),transparent_18%)]" />
        <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.06)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.06)_1px,transparent_1px)] [background-size:72px_72px]" />
        <div className="absolute inset-0 bg-gradient-to-r from-background via-background/60 to-transparent" />
      </div>

      <div className="relative z-10 mx-auto max-w-[1400px] px-6 lg:px-12">
        <div
          className={`mb-16 transition-all duration-700 ${
            isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
          }`}
        >
          <span className="mb-6 inline-flex items-center gap-3 text-sm font-mono text-muted-foreground">
            <span className="h-px w-8 bg-foreground/30" />
            CLI surface
          </span>
          <h2 className="text-6xl font-display leading-[0.9] tracking-tight md:text-7xl lg:text-[128px]">
            Run the store.
            <br />
            <span className="text-muted-foreground">Inspect the protocol.</span>
          </h2>
        </div>

        <div
          className={`max-w-3xl transition-all delay-100 duration-700 lg:max-w-[50%] ${
            isVisible ? "translate-y-0 opacity-100" : "translate-y-8 opacity-0"
          }`}
        >
          <p className="mb-12 max-w-md text-xl leading-relaxed text-muted-foreground">
            The project spec defines a client and server CLI over a custom protocol. The storage layer is
            abstracted behind `KvsEngine`, so the same server flow can run with the handwritten `KvStore`
            or with `sled`.
          </p>
          <div className="grid grid-cols-2 gap-6">
            {features.map((feature, index) => (
              <div
                key={feature.title}
                className={`transition-all duration-500 ${
                  isVisible ? "translate-y-0 opacity-100" : "translate-y-4 opacity-0"
                }`}
                style={{ transitionDelay: `${index * 50 + 200}ms` }}
              >
                <h3 className="mb-2 font-mono text-sm text-foreground">{feature.title}</h3>
                <p className="text-sm text-muted-foreground">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}

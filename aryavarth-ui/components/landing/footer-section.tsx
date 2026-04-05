"use client";

import { ArrowUpRight } from "lucide-react";

const footerLinks = {
  Project: [
    { name: "Architecture", href: "#features" },
    { name: "Execution Flow", href: "#how-it-works" },
    { name: "Roadmap", href: "#infra" },
  ],
  Runtime: [
    { name: "CLI Commands", href: "#developers" },
    { name: "Metrics", href: "#how-it-works" },
    { name: "Next Phase", href: "#infra" },
  ],
  Notes: [
    { name: "Persistent log store", href: "#features" },
    { name: "JSON over TCP", href: "#how-it-works" },
    { name: "Raft runtime soon", href: "#infra", badge: "Next" },
  ],
};

const socialLinks = [
  { name: "GitHub", href: "https://github.com/ygndotgg/kvs_store" },
  { name: "Project", href: "https://github.com/ygndotgg/kvs_store" },
];

export function FooterSection() {
  return (
    <footer className="relative bg-black">
      <div className="relative h-[340px] w-full overflow-hidden md:h-[420px]">
        <div className="absolute inset-0 bg-[radial-gradient(circle_at_20%_35%,rgba(236,168,214,0.22),transparent_18%),radial-gradient(circle_at_75%_20%,rgba(255,255,255,0.10),transparent_20%),linear-gradient(180deg,rgba(12,14,18,0.8),rgba(0,0,0,1))]" />
        <div className="absolute inset-0 opacity-50 [background-image:linear-gradient(rgba(255,255,255,0.08)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.08)_1px,transparent_1px)] [background-size:96px_96px]" />
        <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black" />
        <div className="absolute inset-0 bg-gradient-to-r from-black/40 via-transparent to-black/40" />
      </div>

      <div className="relative z-10 mx-auto max-w-[1400px] px-6 lg:px-12">
        <div className="py-16 lg:py-20">
          <div className="grid grid-cols-2 gap-12 md:grid-cols-5 lg:gap-8">
            <div className="col-span-2">
              <a href="#" className="mb-6 inline-flex items-center gap-2">
                <span className="text-2xl font-display text-white">ARYAVARTH</span>
                <span className="text-xs font-mono text-white/40">KVS</span>
              </a>

              <p className="mb-8 max-w-xs text-sm leading-relaxed text-white/50">
                A Rust key-value store project focused on correct persistence, concurrent request serving,
                pluggable engines, and a clear path toward Raft-based distribution.
              </p>

              <div className="flex gap-6">
                {socialLinks.map((link) => (
                  <a
                    key={link.name}
                    href={link.href}
                    target="_blank"
                    rel="noreferrer"
                    className="group flex items-center gap-1 text-sm text-white/40 transition-colors hover:text-white"
                  >
                    {link.name}
                    <ArrowUpRight className="h-3 w-3 -translate-x-1 opacity-0 transition-all group-hover:translate-x-0 group-hover:opacity-100" />
                  </a>
                ))}
              </div>
            </div>

            {Object.entries(footerLinks).map(([title, links]) => (
              <div key={title}>
                <h3 className="mb-6 text-sm font-medium text-white">{title}</h3>
                <ul className="space-y-4">
                  {links.map((link) => (
                    <li key={link.name}>
                      <a
                        href={link.href}
                        className="inline-flex items-center gap-2 text-sm text-white/40 transition-colors hover:text-white"
                      >
                        {link.name}
                        {"badge" in link && link.badge ? (
                          <span className="rounded-full bg-white px-2 py-0.5 text-xs text-black">
                            {link.badge}
                          </span>
                        ) : null}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>

        <div className="flex flex-col items-center justify-between gap-4 border-t border-white/10 py-8 md:flex-row">
          <p className="text-sm text-white/30">
            &copy; 2026 Aryavarth KVS. Project interface updated for the current system milestone.
          </p>

          <div className="flex items-center gap-4 text-sm text-white/30">
            <span className="flex items-center gap-2">
              <span className="h-2 w-2 rounded-full bg-[#eca8d6]" />
              Single-node runtime implemented
            </span>
          </div>
        </div>
      </div>
    </footer>
  );
}

"use client";

import { useEffect, useState } from "react";
import { Menu, X } from "lucide-react";
import { Button } from "@/components/ui/button";

const navLinks = [
  { name: "Architecture", href: "#features" },
  { name: "Execution", href: "#how-it-works" },
  { name: "Roadmap", href: "#infra" },
  { name: "CLI", href: "#developers" },
];

export function Navigation() {
  const [isScrolled, setIsScrolled] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20);
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <header
      className={`fixed z-50 transition-all duration-500 ${
        isScrolled ? "top-4 left-4 right-4" : "top-0 left-0 right-0"
      }`}
    >
      <nav
        className={`mx-auto transition-all duration-500 ${
          isScrolled || isMobileMenuOpen
            ? "max-w-[1200px] rounded-2xl border border-foreground/10 bg-background/80 shadow-lg backdrop-blur-xl"
            : "max-w-[1400px] bg-transparent"
        }`}
      >
        <div
          className={`flex items-center justify-between px-6 lg:px-8 transition-all duration-500 ${
            isScrolled ? "h-14" : "h-20"
          }`}
        >
          <a href="#" className="flex items-center gap-2 group">
            <span
              className={`font-display tracking-tight transition-all duration-500 ${
                isScrolled ? "text-xl text-foreground" : "text-2xl text-white"
              }`}
            >
              ARYAVARTH
            </span>
            <span
              className={`font-mono transition-all duration-500 ${
                isScrolled ? "mt-0.5 text-[10px] text-muted-foreground" : "mt-1 text-xs text-white/60"
              }`}
            >
              KVS
            </span>
          </a>

          <div className="hidden items-center gap-12 md:flex">
            {navLinks.map((link) => (
              <a
                key={link.name}
                href={link.href}
                className={`group relative text-sm transition-colors duration-300 ${
                  isScrolled ? "text-foreground/70 hover:text-foreground" : "text-white/70 hover:text-white"
                }`}
              >
                {link.name}
                <span
                  className={`absolute -bottom-1 left-0 h-px w-0 transition-all duration-300 group-hover:w-full ${
                    isScrolled ? "bg-foreground" : "bg-white"
                  }`}
                />
              </a>
            ))}
          </div>

          <div className="hidden items-center gap-4 md:flex">
            <a
              href="https://github.com/ygndotgg/kvs_store"
              target="_blank"
              rel="noreferrer"
              className={`transition-all duration-500 ${
                isScrolled ? "text-xs text-foreground/70 hover:text-foreground" : "text-sm text-white/70 hover:text-white"
              }`}
            >
              GitHub
            </a>
            <a
              href="#developers"
              className={`transition-all duration-500 ${
                isScrolled ? "text-xs text-foreground/70 hover:text-foreground" : "text-sm text-white/70 hover:text-white"
              }`}
            >
              View commands
            </a>
            <Button
              asChild
              size="sm"
              className={`rounded-full transition-all duration-500 ${
                isScrolled ? "h-8 bg-foreground px-4 text-xs text-background hover:bg-foreground/90" : "bg-white px-6 text-black hover:bg-white/90"
              }`}
            >
              <a href="#infra">See roadmap</a>
            </Button>
          </div>

          <button
            onClick={() => setIsMobileMenuOpen((value) => !value)}
            className={`p-2 transition-colors duration-500 md:hidden ${
              isScrolled || isMobileMenuOpen ? "text-foreground" : "text-white"
            }`}
            aria-label="Toggle menu"
          >
            {isMobileMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
          </button>
        </div>
      </nav>

      <div
        className={`fixed inset-0 z-40 bg-background transition-all duration-500 md:hidden ${
          isMobileMenuOpen ? "pointer-events-auto opacity-100" : "pointer-events-none opacity-0"
        }`}
      >
        <div className="flex h-full flex-col px-8 pt-28 pb-8">
          <div className="flex flex-1 flex-col justify-center gap-8">
            {navLinks.map((link, index) => (
              <a
                key={link.name}
                href={link.href}
                onClick={() => setIsMobileMenuOpen(false)}
                className={`text-5xl font-display text-foreground transition-all duration-500 hover:text-muted-foreground ${
                  isMobileMenuOpen ? "translate-y-0 opacity-100" : "translate-y-4 opacity-0"
                }`}
                style={{ transitionDelay: isMobileMenuOpen ? `${index * 75}ms` : "0ms" }}
              >
                {link.name}
              </a>
            ))}
          </div>

          <div
            className={`flex gap-4 border-t border-foreground/10 pt-8 transition-all duration-500 ${
              isMobileMenuOpen ? "translate-y-0 opacity-100" : "translate-y-4 opacity-0"
            }`}
            style={{ transitionDelay: isMobileMenuOpen ? "300ms" : "0ms" }}
          >
            <Button variant="outline" className="h-14 flex-1 rounded-full text-base" asChild>
              <a
                href="https://github.com/ygndotgg/kvs_store"
                target="_blank"
                rel="noreferrer"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                GitHub
              </a>
            </Button>
            <Button variant="outline" className="h-14 flex-1 rounded-full text-base" asChild>
              <a href="#developers" onClick={() => setIsMobileMenuOpen(false)}>
                View commands
              </a>
            </Button>
            <Button className="h-14 flex-1 rounded-full bg-foreground text-base text-background" asChild>
              <a href="#infra" onClick={() => setIsMobileMenuOpen(false)}>
                See roadmap
              </a>
            </Button>
          </div>
        </div>
      </div>
    </header>
  );
}

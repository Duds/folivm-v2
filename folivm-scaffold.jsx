import { useState, useMemo, useCallback, useRef, useEffect } from "react";

// ─── Design Tokens (Unified Light Mode) ─────────────────────────
const T = {
  bg: {
    app: "#EFECE5", titlebar: "#E8E5DD", sidebar: "#EFECE5", sidebarActive: "#E2DFD7",
    editor: "#FFFFFF", frontmatter: "#F7F5F0", rail: "#F5F3EE",
    tab: "#E8E5DD", tabActive: "#FFFFFF", statusbar: "#E8E5DD",
    hover: "rgba(0,0,0,0.04)", hoverStrong: "rgba(0,0,0,0.07)",
    accent: "#007BFF", accentSoft: "rgba(0,123,255,0.08)",
    blockquote: "#F0EDE6", canvas: "#E8E5DD",
    rulerBg: "#F5F3EE", rulerTick: "#C8C4BA", rulerLabel: "#A09A8E",
    rulerMargin: "rgba(0,123,255,0.06)", rulerMarginLine: "rgba(0,123,255,0.18)",
    dragTarget: "rgba(0,123,255,0.12)",
  },
  text: {
    primary: "#3A3632", secondary: "#7A756C", muted: "#A09A8E",
    inverse: "#FFFFFF", dark: "#1A1A1A", darkSec: "#555555", link: "#007BFF",
    yaml: { key: "#2E6BBF", value: "#A8553A", delim: "#7A756C" },
  },
  border: { subtle: "rgba(0,0,0,0.06)", medium: "rgba(0,0,0,0.08)", strong: "rgba(0,0,0,0.12)", active: "#007BFF", rail: "#E0DDD6" },
  r: { sm: "3px", md: "6px", lg: "8px" },
  f: {
    ui: "'SF Pro Text','Segoe UI',-apple-system,sans-serif",
    mono: "'SF Mono','Fira Code','JetBrains Mono',monospace",
    editor: "'Georgia','Iowan Old Style','Palatino Linotype',serif",
    heading: "'SF Pro Display','Segoe UI',-apple-system,sans-serif",
  },
};

// ─── Page Size Presets ───────────────────────────────────────────
const PAGE_SIZES = {
  A4:        { label: "A4",        w: 680, h: 960, marginT: 48, marginB: 48, marginL: 48, marginR: 48 },
  USLetter:  { label: "US Letter", w: 700, h: 906, marginT: 48, marginB: 48, marginL: 48, marginR: 48 },
  A5:        { label: "A5",        w: 500, h: 680, marginT: 36, marginB: 36, marginL: 36, marginR: 36 },
  Custom:    { label: "Custom",    w: 680, h: 880, marginT: 48, marginB: 48, marginL: 48, marginR: 48 },
};

// ─── Icons (compact) ─────────────────────────────────────────────
const I = {
  Folder: () => <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>,
  Search: () => <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>,
  Git: () => <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><line x1="6" y1="3" x2="6" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>,
  Ext: () => <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>,
  TOC: () => <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>,
  ChevR: ({s=12}) => <svg width={s} height={s} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><polyline points="9 18 15 12 9 6"/></svg>,
  ChevD: ({s=12}) => <svg width={s} height={s} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><polyline points="6 9 12 15 18 9"/></svg>,
  File: () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>,
  New: () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><line x1="12" y1="11" x2="12" y2="17"/><line x1="9" y1="14" x2="15" y2="14"/></svg>,
  Srch: () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>,
  Refresh: () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/></svg>,
  Settings: () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>,
  PanelL: () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>,
  PanelR: () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="15" y1="3" x2="15" y2="21"/></svg>,
  Close: () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>,
  Dots: () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="1.5" fill="currentColor"/><circle cx="19" cy="12" r="1.5" fill="currentColor"/><circle cx="5" cy="12" r="1.5" fill="currentColor"/></svg>,
  Grip: () => <svg width="10" height="14" viewBox="0 0 10 14" fill="currentColor" opacity="0.35"><circle cx="3" cy="2" r="1.2"/><circle cx="7" cy="2" r="1.2"/><circle cx="3" cy="7" r="1.2"/><circle cx="7" cy="7" r="1.2"/><circle cx="3" cy="12" r="1.2"/><circle cx="7" cy="12" r="1.2"/></svg>,
  ArrowUp: () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="18 15 12 9 6 15"/></svg>,
  ArrowDn: () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="6 9 12 15 18 9"/></svg>,
  Edit: () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>,
};

// ─── Sample Data ─────────────────────────────────────────────────
const sampleTree = {
  name: "PROJECT 1", type: "folder", expanded: true,
  children: [
    { name: "DISCOVERY", type: "folder", expanded: false, children: [{ name: "discovery.md", type: "file" }] },
    { name: "RESEARCH", type: "folder", expanded: false, children: [{ name: "research_draft.md", type: "file" }] },
    { name: "DELIVERABLES", type: "folder", expanded: true, children: [
      { name: "P1 DLOREM.MD", type: "file", active: true },
      { name: "P2 DLOREM.MD", type: "file" },
      { name: "P3 DLOREM.MD", type: "file" },
      { name: "P4 DLOREM.MD", type: "file" },
      { name: "DOC1.MD", type: "file" },
      { name: "DOC2.MD", type: "file" },
    ]},
  ],
};
const sampleTabs = [
  { name: "project1.md", active: true },
  { name: "discovery.md", active: false },
  { name: "research_draft.md", active: false },
];
const sampleFM = {
  title: "Project 1 Overview", author: "[AI Assistant & Human]",
  date: "2024-05-20", "ai-mode": "drafting (50k scale)", tags: "[project, overview, .md]",
};

// ─── Default Document Sections ───────────────────────────────────
const defaultSections = [
  { id: "s1", title: "Discovery", heading: "Discovery Phase", content: [
    { type: "p", text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed diam en nummy tempor incididunt ut labore et dolne rqua [discovery.md].' },
    { type: "p", text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do-eiusmod easmod incididunt ut labore et [discovery.md].' },
  ]},
  { id: "s2", title: "Research", heading: "Research Results", content: [
    { type: "p", text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do-eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut mnm am:' },
    { type: "list", items: ["List Bullet 1", "List Bullet 2", "List Number 1", "List Number 2"] },
    { type: "quote", text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor inter lincididunt ut labore, et dolore magna aliqua. Ut enim ad mininam.' },
  ]},
  { id: "s3", title: "Summary", heading: "Summary", content: [
    { type: "p", text: 'SIT AMET — dolor sit amet, consecteteur adipiscing elit, sed do elusmod tempor incididunt ut labore et dolore magna aliqua.' },
  ]},
];

// ─── Default Header/Footer Templates ─────────────────────────────
const defaultHeaderTemplate = { left: "{title}", centre: "", right: "{page}" };
const defaultFooterTemplate = { left: "{author} — {date}", centre: "", right: "Page {page} of {pages}" };

// ─── Shared Micro Components ─────────────────────────────────────
function Collapsible({ title, defaultOpen = false, badge, children }) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <div style={{ borderBottom: `1px solid ${T.border.rail}` }}>
      <div onClick={() => setOpen(!open)} style={{ display: "flex", alignItems: "center", gap: 6, padding: "10px 16px", cursor: "pointer", userSelect: "none" }}>
        {open ? <I.ChevD s={10}/> : <I.ChevR s={10}/>}
        <span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.darkSec, fontFamily: T.f.ui }}>{title}</span>
        {badge != null && <span style={{ fontSize: 10, color: T.text.muted, marginLeft: "auto", background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{badge}</span>}
      </div>
      {open && children}
    </div>
  );
}
function SideShell({ children, w = 260 }) {
  return <div style={{ width: w, background: T.bg.sidebar, borderRight: `1px solid ${T.border.subtle}`, display: "flex", flexDirection: "column", flexShrink: 0, overflow: "hidden" }}>{children}</div>;
}
function SideHead({ title, actions }) {
  return (
    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", padding: "10px 12px 6px", borderBottom: `1px solid ${T.border.subtle}` }}>
      <span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.08em", color: T.text.secondary, fontFamily: T.f.ui, textTransform: "uppercase" }}>{title}</span>
      {actions && <div style={{ display: "flex", gap: 6, color: T.text.muted }}>{actions}</div>}
    </div>
  );
}
function SideInput({ placeholder, icon }) {
  return (
    <div style={{ display: "flex", alignItems: "center", background: T.bg.sidebarActive, borderRadius: T.r.sm, padding: "6px 8px", gap: 6, border: `1px solid ${T.border.subtle}` }}>
      {icon && <span style={{ color: T.text.muted, display: "flex", flexShrink: 0 }}>{icon}</span>}
      <input placeholder={placeholder} style={{ background: "none", border: "none", outline: "none", color: T.text.primary, fontSize: 13, fontFamily: T.f.ui, width: "100%" }}/>
    </div>
  );
}
function SBtn({ children, primary, style: sx, onClick }) {
  return <button onClick={onClick} style={{ padding: "4px 10px", fontSize: 11, fontFamily: T.f.ui, fontWeight: 600, border: primary ? "none" : `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, cursor: "pointer", background: primary ? T.bg.accent : "transparent", color: primary ? T.text.inverse : T.text.secondary, ...sx }}>{children}</button>;
}
function NumberInput({ value, onChange, min, max, label, unit }) {
  return (
    <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
      {label && <span style={{ fontSize: 11, color: T.text.muted, fontFamily: T.f.ui, width: 48 }}>{label}</span>}
      <input type="number" value={value} min={min} max={max} onChange={(e) => onChange(+e.target.value)}
        style={{ width: 54, padding: "3px 6px", fontSize: 12, fontFamily: T.f.mono, border: `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, background: T.bg.editor, color: T.text.dark, outline: "none", textAlign: "right" }}/>
      {unit && <span style={{ fontSize: 10, color: T.text.muted, fontFamily: T.f.mono }}>{unit}</span>}
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════
// LEFT SIDEBAR PANELS
// ═══════════════════════════════════════════════════════════════════

function FileNode({ node, depth = 0 }) {
  const [exp, setExp] = useState(node.expanded ?? false);
  const isF = node.type === "folder";
  return (
    <div>
      <div onClick={() => isF && setExp(!exp)} style={{ display: "flex", alignItems: "center", gap: 4, padding: `3px 8px 3px ${12 + depth * 16}px`, cursor: "pointer", fontSize: 13, fontFamily: T.f.ui, color: node.active ? T.text.dark : T.text.primary, background: node.active ? T.bg.sidebarActive : "transparent", fontWeight: node.active ? 600 : 400, userSelect: "none" }}
        onMouseEnter={(e) => { if (!node.active) e.currentTarget.style.background = T.bg.hover; }} onMouseLeave={(e) => { if (!node.active) e.currentTarget.style.background = "transparent"; }}>
        {isF ? (exp ? <I.ChevD/> : <I.ChevR/>) : <span style={{ width: 12 }}/>}
        {!isF && <span style={{ color: T.text.muted }}><I.File/></span>}
        <span style={{ fontWeight: isF ? 600 : 400, letterSpacing: isF ? "0.03em" : 0 }}>{node.name}</span>
      </div>
      {isF && exp && node.children?.map((c, i) => <FileNode key={i} node={c} depth={depth + 1}/>)}
    </div>
  );
}

/* ── 4. Document Outline / TOC Panel ── */
function TOCPanel({ sections, onScrollTo }) {
  return (
    <SideShell>
      <SideHead title="Document Outline"/>
      <div style={{ flex: 1, overflowY: "auto", paddingTop: 4 }}>
        {/* Document title */}
        <div style={{ padding: "8px 12px", display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}
          onClick={() => onScrollTo?.("top")}
          onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
          <span style={{ fontSize: 10, fontFamily: T.f.mono, fontWeight: 700, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 3, padding: "1px 4px" }}>#</span>
          <span style={{ fontSize: 13, fontWeight: 700, color: T.text.dark, fontFamily: T.f.ui }}>PROJECT 1 OVERVIEW</span>
        </div>
        {/* Sections */}
        {sections.map((sec, idx) => (
          <div key={sec.id}
            style={{ padding: "6px 12px 6px 32px", display: "flex", alignItems: "center", gap: 8, cursor: "pointer", borderLeft: `2px solid ${idx === 0 ? T.bg.accent : "transparent"}`, transition: "all 0.12s ease" }}
            onClick={() => onScrollTo?.(sec.id)}
            onMouseEnter={(e) => { e.currentTarget.style.background = T.bg.hover; e.currentTarget.style.borderLeftColor = T.bg.accent; }}
            onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; if (idx !== 0) e.currentTarget.style.borderLeftColor = "transparent"; }}>
            <span style={{ fontSize: 10, fontFamily: T.f.mono, fontWeight: 700, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 3, padding: "1px 4px" }}>##</span>
            <span style={{ fontSize: 13, color: T.text.primary, fontFamily: T.f.ui }}>{sec.heading}</span>
            <span style={{ fontSize: 10, color: T.text.muted, marginLeft: "auto" }}>S{idx + 1}</span>
          </div>
        ))}
        {/* Stats */}
        <div style={{ padding: "16px 12px", borderTop: `1px solid ${T.border.subtle}`, marginTop: 8 }}>
          <div style={{ fontSize: 10, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui, marginBottom: 6 }}>DOCUMENT STATS</div>
          {[{ l: "Sections", v: sections.length }, { l: "Pages", v: 2 }, { l: "Words", v: "1,234" }, { l: "Characters", v: "5,678" }].map(({ l, v }) => (
            <div key={l} style={{ display: "flex", justifyContent: "space-between", padding: "2px 0", fontSize: 12, fontFamily: T.f.ui }}>
              <span style={{ color: T.text.muted }}>{l}</span>
              <span style={{ color: T.text.dark, fontWeight: 500, fontFamily: T.f.mono }}>{v}</span>
            </div>
          ))}
        </div>
      </div>
    </SideShell>
  );
}

/* Explorer Panel */
function ExplorerPanel() {
  return (
    <SideShell>
      <SideHead title="Explorer" actions={<><span style={{ cursor: "pointer" }}><I.New/></span><span style={{ cursor: "pointer" }}><I.Refresh/></span></>}/>
      <div style={{ flex: 1, overflowY: "auto", paddingTop: 4 }}><FileNode node={sampleTree}/></div>
      {["GIT-STYLE VERSIONING", "EXTENSIONS"].map((l) => (
        <div key={l} style={{ borderTop: `1px solid ${T.border.subtle}`, padding: "8px 12px", display: "flex", alignItems: "center", gap: 6, cursor: "pointer", color: T.text.secondary }}>
          <I.ChevR s={10}/><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", fontFamily: T.f.ui }}>{l}</span>
        </div>
      ))}
    </SideShell>
  );
}

/* Search / Find & Replace Panel */
function SearchPanel() {
  const [showReplace, setShowReplace] = useState(false);
  const [mc, setMc] = useState(false);
  const [ww, setWw] = useState(false);
  const [re, setRe] = useState(false);
  const results = [
    { file: "project1.md", line: 12, text: "consectetur adipiscing elit, sed diam en", match: "consectetur" },
    { file: "project1.md", line: 24, text: "consectetur adipiscing elit, sed do-eiusmod", match: "consectetur" },
    { file: "discovery.md", line: 3, text: "consectetur adipiscing elit, preliminary", match: "consectetur" },
    { file: "research_draft.md", line: 8, text: "consectetur findings from phase two", match: "consectetur" },
  ];
  const Chip = ({ label, active, onToggle }) => <button onClick={onToggle} style={{ width: 24, height: 22, display: "flex", alignItems: "center", justifyContent: "center", fontSize: 11, fontFamily: T.f.mono, fontWeight: 600, border: `1px solid ${active ? T.border.active : T.border.subtle}`, borderRadius: T.r.sm, cursor: "pointer", background: active ? T.bg.accentSoft : "transparent", color: active ? T.text.link : T.text.muted }}>{label}</button>;
  const grouped = {};
  results.forEach((r) => { if (!grouped[r.file]) grouped[r.file] = []; grouped[r.file].push(r); });
  return (
    <SideShell>
      <SideHead title="Find & Replace" actions={<span style={{ cursor: "pointer", fontSize: 11, color: T.text.link, fontFamily: T.f.ui }} onClick={() => setShowReplace(!showReplace)}>{showReplace ? "Hide" : "Replace"}</span>}/>
      <div style={{ padding: 12, display: "flex", flexDirection: "column", gap: 8 }}>
        <SideInput placeholder="Find in files..." icon={<I.Srch/>}/>
        <div style={{ display: "flex", gap: 4, alignItems: "center" }}>
          <Chip label="Aa" active={mc} onToggle={() => setMc(!mc)}/><Chip label="W" active={ww} onToggle={() => setWw(!ww)}/><Chip label=".*" active={re} onToggle={() => setRe(!re)}/>
          <span style={{ flex: 1 }}/><span style={{ fontSize: 11, color: T.text.muted, fontFamily: T.f.ui }}>4 results</span>
        </div>
        {showReplace && <div style={{ display: "flex", gap: 4, alignItems: "center" }}><div style={{ flex: 1 }}><SideInput placeholder="Replace with..."/></div><SBtn>1</SBtn><SBtn primary>All</SBtn></div>}
      </div>
      <div style={{ flex: 1, overflowY: "auto", borderTop: `1px solid ${T.border.subtle}` }}>
        {Object.entries(grouped).map(([file, rs]) => (
          <div key={file}>
            <div style={{ padding: "6px 12px", fontSize: 12, fontWeight: 600, color: T.text.primary, fontFamily: T.f.ui, background: T.bg.hover, display: "flex", justifyContent: "space-between" }}><span>{file}</span><span style={{ fontSize: 10, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{rs.length}</span></div>
            {rs.map((r, i) => (
              <div key={i} style={{ padding: "4px 12px 4px 24px", fontSize: 12, fontFamily: T.f.mono, color: T.text.secondary, cursor: "pointer", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}
                onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
                <span style={{ color: T.text.muted, marginRight: 8 }}>{r.line}</span>
                {r.text.split(r.match).map((part, pi, arr) => <span key={pi}>{part}{pi < arr.length - 1 && <span style={{ background: "rgba(255,200,0,0.35)", borderRadius: 2, padding: "0 1px", color: T.text.dark, fontWeight: 600 }}>{r.match}</span>}</span>)}
              </div>
            ))}
          </div>
        ))}
      </div>
    </SideShell>
  );
}

/* Git Versioning Panel */
function VersioningPanel() {
  const [msg, setMsg] = useState("");
  const staged = [{ file: "project1.md", s: "M", c: "#E5A50A" }, { file: "DOC1.MD", s: "A", c: "#28A745" }];
  const unstaged = [{ file: "discovery.md", s: "M", c: "#E5A50A" }, { file: "P2 DLOREM.MD", s: "M", c: "#E5A50A" }, { file: "research_draft.md", s: "D", c: "#DC3545" }];
  const history = [{ h: "a3f9c21", m: "Update discovery phase notes", a: "Human", t: "2m ago" }, { h: "7b2e1d0", m: "Add research results section", a: "AI Assistant", t: "15m ago" }, { h: "e91c4f8", m: "Initial project structure", a: "Human", t: "1h ago" }];
  const FC = ({ file, s, c }) => (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "3px 12px 3px 24px", fontSize: 13, fontFamily: T.f.ui, cursor: "pointer" }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
      <div style={{ display: "flex", alignItems: "center", gap: 6, overflow: "hidden" }}><span style={{ color: T.text.muted, flexShrink: 0 }}><I.File/></span><span style={{ color: T.text.primary, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{file}</span></div>
      <span style={{ fontSize: 11, fontWeight: 700, fontFamily: T.f.mono, color: c, flexShrink: 0, marginLeft: 8 }}>{s}</span>
    </div>
  );
  return (
    <SideShell>
      <SideHead title="Source Control" actions={<><span style={{ cursor: "pointer" }}><I.Refresh/></span><span style={{ cursor: "pointer" }}><I.Dots/></span></>}/>
      <div style={{ padding: 12, borderBottom: `1px solid ${T.border.subtle}` }}>
        <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 12, color: T.text.secondary, fontFamily: T.f.ui, marginBottom: 8 }}><I.Git/><span>main</span><span style={{ color: T.text.muted }}>•</span><span style={{ color: T.text.muted }}>3 pending</span></div>
        <textarea value={msg} onChange={(e) => setMsg(e.target.value)} placeholder="Commit message..." style={{ width: "100%", height: 56, resize: "none", padding: "6px 8px", background: T.bg.sidebarActive, border: `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, fontSize: 13, fontFamily: T.f.ui, color: T.text.primary, outline: "none", boxSizing: "border-box" }}/>
        <div style={{ display: "flex", gap: 6, marginTop: 8 }}><SBtn primary style={{ flex: 1 }}>Commit</SBtn><SBtn style={{ flex: 1 }}>Stash</SBtn></div>
      </div>
      <div style={{ flex: 1, overflowY: "auto" }}>
        <div style={{ padding: "8px 12px 4px", display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui }}>STAGED</span><span style={{ fontSize: 10, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{staged.length}</span></div>
        {staged.map((c, i) => <FC key={i} {...c}/>)}
        <div style={{ padding: "12px 12px 4px", display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui }}>CHANGES</span><span style={{ fontSize: 10, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{unstaged.length}</span></div>
        {unstaged.map((c, i) => <FC key={i} {...c}/>)}
        <div style={{ padding: "16px 12px 4px" }}><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui }}>RECENT COMMITS</span></div>
        {history.map((h, i) => (
          <div key={i} style={{ padding: "6px 12px 6px 24px", cursor: "pointer", borderBottom: i < history.length - 1 ? `1px solid ${T.border.subtle}` : "none" }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
            <div style={{ display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 13, color: T.text.primary, fontFamily: T.f.ui, fontWeight: 500, maxWidth: 160, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{h.m}</span><span style={{ fontSize: 10, fontFamily: T.f.mono, color: T.text.muted }}>{h.h}</span></div>
            <div style={{ fontSize: 11, color: T.text.muted, fontFamily: T.f.ui, marginTop: 2 }}>{h.a} • {h.t}</div>
          </div>
        ))}
      </div>
    </SideShell>
  );
}

/* Extensions Panel */
function ExtensionsPanel() {
  const installed = [
    { name: "AI Drafting Engine", ver: "2.1.0", auth: "DocForge", desc: "AI-powered content generation and suggestions", on: true, ico: "🤖" },
    { name: "YAML Frontmatter", ver: "1.4.2", auth: "DocForge", desc: "Parse and validate YAML metadata blocks", on: true, ico: "📋" },
    { name: "Markdown Lint", ver: "3.0.1", auth: "Community", desc: "Style and syntax checking for .md files", on: true, ico: "✅" },
    { name: "Word Count", ver: "1.0.0", auth: "Community", desc: "Real-time word and character stats", on: false, ico: "📊" },
  ];
  const available = [
    { name: "Citation Manager", ver: "1.2.0", auth: "Academic Tools", desc: "Chicago, APA, and MLA citation support", ico: "📚" },
    { name: "Export to DOCX", ver: "0.9.3", auth: "DocForge", desc: "Export markdown to formatted Word documents", ico: "📄" },
    { name: "Mermaid Diagrams", ver: "2.0.0", auth: "Community", desc: "Render Mermaid diagram syntax in preview", ico: "📐" },
  ];
  const Card = ({ ext, inst }) => {
    const [on, setOn] = useState(ext.on ?? false);
    return (
      <div style={{ padding: "10px 12px", borderBottom: `1px solid ${T.border.subtle}`, cursor: "pointer" }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
        <div style={{ display: "flex", alignItems: "flex-start", gap: 10 }}>
          <span style={{ fontSize: 22, lineHeight: 1, flexShrink: 0, marginTop: 1 }}>{ext.ico}</span>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 13, fontWeight: 600, color: T.text.dark, fontFamily: T.f.ui }}>{ext.name}</span><span style={{ fontSize: 10, color: T.text.muted, fontFamily: T.f.mono }}>{ext.ver}</span></div>
            <div style={{ fontSize: 11, color: T.text.muted, fontFamily: T.f.ui, marginTop: 1 }}>{ext.auth}</div>
            <div style={{ fontSize: 12, color: T.text.secondary, fontFamily: T.f.ui, marginTop: 4, lineHeight: 1.4 }}>{ext.desc}</div>
            <div style={{ marginTop: 6 }}>
              {inst ? (
                <div style={{ display: "flex", gap: 6, alignItems: "center" }}>
                  <div onClick={(e) => { e.stopPropagation(); setOn(!on); }} style={{ width: 32, height: 18, borderRadius: 9, cursor: "pointer", background: on ? T.bg.accent : "#C8C4BA", position: "relative", transition: "background 0.15s" }}>
                    <div style={{ width: 14, height: 14, borderRadius: "50%", background: "#fff", position: "absolute", top: 2, left: on ? 16 : 2, transition: "left 0.15s", boxShadow: "0 1px 2px rgba(0,0,0,0.15)" }}/>
                  </div>
                  <span style={{ fontSize: 11, color: on ? T.text.link : T.text.muted, fontFamily: T.f.ui }}>{on ? "Enabled" : "Disabled"}</span>
                </div>
              ) : <SBtn primary>Install</SBtn>}
            </div>
          </div>
        </div>
      </div>
    );
  };
  return (
    <SideShell>
      <SideHead title="Extensions"/>
      <div style={{ padding: 12 }}><SideInput placeholder="Search extensions..." icon={<I.Srch/>}/></div>
      <div style={{ flex: 1, overflowY: "auto" }}>
        <div style={{ padding: "4px 12px", display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui }}>INSTALLED</span><span style={{ fontSize: 10, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{installed.length}</span></div>
        {installed.map((e, i) => <Card key={i} ext={e} inst/>)}
        <div style={{ padding: "12px 12px 4px", display: "flex", justifyContent: "space-between" }}><span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.06em", color: T.text.secondary, fontFamily: T.f.ui }}>AVAILABLE</span><span style={{ fontSize: 10, color: T.text.muted, background: T.bg.sidebarActive, borderRadius: 8, padding: "1px 6px" }}>{available.length}</span></div>
        {available.map((e, i) => <Card key={i} ext={e} inst={false}/>)}
      </div>
    </SideShell>
  );
}

/* Sidebar Router */
function SidebarPanel({ activeView, sections, onScrollTo }) {
  if (activeView === "explorer") return <ExplorerPanel/>;
  if (activeView === "search") return <SearchPanel/>;
  if (activeView === "versioning") return <VersioningPanel/>;
  if (activeView === "extensions") return <ExtensionsPanel/>;
  if (activeView === "toc") return <TOCPanel sections={sections} onScrollTo={onScrollTo}/>;
  return null;
}

// ═══════════════════════════════════════════════════════════════════
// EDITOR CHROME (tabs, toolbar, ruler)
// ═══════════════════════════════════════════════════════════════════

function ActivityBar({ activeView, onViewChange }) {
  const items = [
    { id: "explorer", Ic: I.Folder }, { id: "search", Ic: I.Search },
    { id: "versioning", Ic: I.Git }, { id: "extensions", Ic: I.Ext },
    { id: "toc", Ic: I.TOC },
  ];
  return (
    <div style={{ width: 48, background: T.bg.sidebar, borderRight: `1px solid ${T.border.subtle}`, display: "flex", flexDirection: "column", alignItems: "center", paddingTop: 8, gap: 2, flexShrink: 0 }}>
      {items.map(({ id, Ic }) => (
        <button key={id} onClick={() => onViewChange(id)} title={id} style={{ width: 40, height: 40, display: "flex", alignItems: "center", justifyContent: "center", border: "none", cursor: "pointer", borderRadius: T.r.md, background: activeView === id ? T.bg.sidebarActive : "transparent", color: activeView === id ? T.text.primary : T.text.muted, transition: "all 0.15s" }}
          onMouseEnter={(e) => { if (activeView !== id) { e.currentTarget.style.background = T.bg.hover; e.currentTarget.style.color = T.text.secondary; }}}
          onMouseLeave={(e) => { if (activeView !== id) { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = T.text.muted; }}}>
          <Ic/>
        </button>
      ))}
    </div>
  );
}

function TabBar({ tabs }) {
  return (
    <div style={{ display: "flex", background: T.bg.tab, borderBottom: `1px solid ${T.border.medium}`, height: 36, alignItems: "flex-end", flexShrink: 0 }}>
      {tabs.map((t, i) => (
        <div key={i} style={{ display: "flex", alignItems: "center", gap: 6, padding: "0 14px", height: 34, background: t.active ? T.bg.tabActive : "transparent", borderTop: t.active ? `2px solid ${T.border.active}` : "2px solid transparent", borderRight: `1px solid ${T.border.subtle}`, fontSize: 13, fontFamily: T.f.ui, color: t.active ? T.text.dark : T.text.secondary, cursor: "pointer", userSelect: "none" }}>
          <span>{t.name}</span>{t.active && <span style={{ marginLeft: 4, opacity: 0.3, display: "flex", cursor: "pointer" }}><I.Close/></span>}
        </div>
      ))}
      <div style={{ flex: 1 }}/>
      <div style={{ display: "flex", alignItems: "center", gap: 2, paddingRight: 8, color: T.text.muted, cursor: "pointer" }}><I.Dots/></div>
    </div>
  );
}

function EditorToolbar() {
  return (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "6px 16px", borderBottom: `1px solid ${T.border.medium}`, background: T.bg.editor, flexShrink: 0 }}>
      <div style={{ display: "flex", alignItems: "center", gap: 8, color: T.text.muted }}>
        <button style={{ background: "none", border: "none", cursor: "pointer", color: "inherit", display: "flex" }}><I.New/></button>
        <button style={{ background: "none", border: "none", cursor: "pointer", color: "inherit", display: "flex" }}><I.Srch/></button>
      </div>
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <div style={{ display: "flex", alignItems: "center", gap: 4, fontSize: 12, color: T.text.secondary, fontFamily: T.f.ui }}><I.Git/><span>main</span><I.ChevD s={10}/></div>
        <div style={{ display: "flex", gap: 4, marginLeft: 8, color: T.text.muted }}>
          <button style={{ background: "none", border: "none", cursor: "pointer", color: "inherit", display: "flex" }}><I.Settings/></button>
          <button style={{ background: "none", border: "none", cursor: "pointer", color: "inherit", display: "flex" }}><I.Refresh/></button>
        </div>
      </div>
    </div>
  );
}

function Ruler({ marginL }) {
  const ticks = useMemo(() => {
    const items = [];
    for (let i = 0; i <= 80; i++) items.push({ pos: i, major: i % 10 === 0, mid: i % 5 === 0 && i % 10 !== 0 });
    return items;
  }, []);
  return (
    <div style={{ height: 24, background: T.bg.rulerBg, borderBottom: `1px solid ${T.border.medium}`, position: "relative", flexShrink: 0, overflow: "hidden", userSelect: "none" }}>
      <div style={{ position: "absolute", left: 0, top: 0, width: marginL, height: "100%", background: T.bg.rulerMargin, borderRight: `1px solid ${T.bg.rulerMarginLine}` }}/>
      <div style={{ position: "absolute", right: 0, top: 0, width: marginL, height: "100%", background: T.bg.rulerMargin, borderLeft: `1px solid ${T.bg.rulerMarginLine}` }}/>
      <div style={{ position: "absolute", left: marginL, right: marginL, top: 0, height: "100%" }}>
        {ticks.map(({ pos, major, mid }) => {
          const pct = (pos / 80) * 100;
          return (
            <div key={pos} style={{ position: "absolute", left: `${pct}%`, bottom: 0 }}>
              {major && <span style={{ fontSize: 8, fontFamily: T.f.mono, color: T.bg.rulerLabel, position: "absolute", top: 3, left: 0, transform: "translateX(-50%)" }}>{pos}</span>}
              <div style={{ width: 1, height: major ? 10 : mid ? 6 : 3, background: T.bg.rulerTick, position: "absolute", bottom: 0 }}/>
            </div>
          );
        })}
      </div>
      <div style={{ position: "absolute", left: marginL - 5, bottom: 0, borderLeft: "5px solid transparent", borderRight: "5px solid transparent", borderBottom: `6px solid ${T.text.secondary}`, cursor: "ew-resize" }} title="Left margin"/>
      <div style={{ position: "absolute", right: marginL - 5, bottom: 0, borderLeft: "5px solid transparent", borderRight: "5px solid transparent", borderBottom: `6px solid ${T.text.secondary}`, cursor: "ew-resize" }} title="Right margin"/>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════
// DOCUMENT CANVAS
// ═══════════════════════════════════════════════════════════════════

function FMBlock({ data }) {
  return (
    <div style={{ margin: "20px 48px", background: T.bg.frontmatter, borderRadius: T.r.lg, padding: "16px 20px", fontFamily: T.f.mono, fontSize: 13.5, lineHeight: 1.7, border: `1px solid ${T.border.medium}` }}>
      <div style={{ color: T.text.yaml.delim }}>---</div>
      {Object.entries(data).map(([k, v]) => <div key={k}><span style={{ color: T.text.yaml.key, fontWeight: 500 }}>{k}</span><span style={{ color: T.text.yaml.delim }}>: </span><span style={{ color: T.text.yaml.value }}>{v}</span></div>)}
      <div style={{ color: T.text.yaml.delim }}>---</div>
    </div>
  );
}

function resolveTemplate(tpl, vars) {
  return tpl.replace(/\{(\w+)\}/g, (_, k) => vars[k] ?? "");
}

function DocHeader({ template, vars }) {
  const l = resolveTemplate(template.left, vars);
  const c = resolveTemplate(template.centre, vars);
  const r = resolveTemplate(template.right, vars);
  return (
    <div style={{ padding: "12px 48px 8px", borderBottom: `1px solid ${T.border.medium}`, display: "flex", justifyContent: "space-between", alignItems: "baseline", flexShrink: 0, minHeight: 28 }}>
      <span style={{ fontSize: 11, fontFamily: T.f.ui, fontWeight: 600, color: T.text.muted, letterSpacing: "0.04em", flex: 1 }}>{l}</span>
      {c && <span style={{ fontSize: 11, fontFamily: T.f.ui, color: T.text.muted, flex: 1, textAlign: "center" }}>{c}</span>}
      <span style={{ fontSize: 10, fontFamily: T.f.mono, color: T.text.muted, flex: 1, textAlign: "right" }}>{r}</span>
    </div>
  );
}

function DocFooter({ template, vars }) {
  const l = resolveTemplate(template.left, vars);
  const c = resolveTemplate(template.centre, vars);
  const r = resolveTemplate(template.right, vars);
  return (
    <div style={{ padding: "8px 48px 12px", borderTop: `1px solid ${T.border.medium}`, display: "flex", justifyContent: "space-between", alignItems: "center", flexShrink: 0, marginTop: "auto", minHeight: 28 }}>
      <span style={{ fontSize: 10, fontFamily: T.f.ui, color: T.text.muted, flex: 1 }}>{l}</span>
      {c && <span style={{ fontSize: 10, fontFamily: T.f.ui, color: T.text.muted, flex: 1, textAlign: "center" }}>{c}</span>}
      <span style={{ fontSize: 10, fontFamily: T.f.mono, color: T.text.muted, flex: 1, textAlign: "right" }}>{r}</span>
    </div>
  );
}

/* 2. Draggable Section Break */
function SectionBreak({ label, id, onMoveUp, onMoveDown, isFirst, isLast }) {
  const [hover, setHover] = useState(false);
  return (
    <div
      style={{ display: "flex", alignItems: "center", gap: 8, padding: "20px 0", margin: "0 48px", userSelect: "none", position: "relative" }}
      onMouseEnter={() => setHover(true)} onMouseLeave={() => setHover(false)}
    >
      <div style={{ flex: 1, height: 1, background: T.border.rail, position: "relative" }}>
        <div style={{ position: "absolute", left: 0, top: -2, width: 5, height: 5, borderRadius: "50%", background: T.border.rail }}/>
      </div>
      {/* Drag handle + reorder controls */}
      {hover && (
        <div style={{ display: "flex", gap: 2, position: "absolute", left: -40, top: "50%", transform: "translateY(-50%)", alignItems: "center" }}>
          <span style={{ color: T.text.muted, cursor: "grab", display: "flex" }}><I.Grip/></span>
          <div style={{ display: "flex", flexDirection: "column", gap: 0 }}>
            {!isFirst && <button onClick={onMoveUp} style={{ background: "none", border: "none", cursor: "pointer", color: T.text.muted, padding: 0, display: "flex" }}><I.ArrowUp/></button>}
            {!isLast && <button onClick={onMoveDown} style={{ background: "none", border: "none", cursor: "pointer", color: T.text.muted, padding: 0, display: "flex" }}><I.ArrowDn/></button>}
          </div>
        </div>
      )}
      <span style={{ fontSize: 9, fontWeight: 700, letterSpacing: "0.1em", textTransform: "uppercase", fontFamily: T.f.ui, color: T.text.muted, background: T.bg.frontmatter, padding: "3px 10px", borderRadius: 10, border: `1px solid ${hover ? T.border.active : T.border.medium}`, transition: "border-color 0.15s", whiteSpace: "nowrap" }}>
        {label}
      </span>
      <div style={{ flex: 1, height: 1, background: T.border.rail, position: "relative" }}>
        <div style={{ position: "absolute", right: 0, top: -2, width: 5, height: 5, borderRadius: "50%", background: T.border.rail }}/>
      </div>
    </div>
  );
}

function MarginGuides({ ml }) {
  return <>
    <div style={{ position: "absolute", left: ml, top: 0, bottom: 0, width: 1, background: T.bg.rulerMarginLine, pointerEvents: "none", zIndex: 1 }}/>
    <div style={{ position: "absolute", right: ml, top: 0, bottom: 0, width: 1, background: T.bg.rulerMarginLine, pointerEvents: "none", zIndex: 1 }}/>
  </>;
}

function renderContent(content) {
  return content.map((block, i) => {
    if (block.type === "p") return <p key={i} style={{ marginBottom: 12 }}>{block.text}</p>;
    if (block.type === "list") return <ul key={i} style={{ paddingLeft: 24, marginBottom: 16 }}>{block.items.map((item, j) => <li key={j}>{item}</li>)}</ul>;
    if (block.type === "quote") return <blockquote key={i} style={{ margin: "16px 0", padding: "14px 20px", background: T.bg.blockquote, borderLeft: `3px solid ${T.border.rail}`, borderRadius: `0 ${T.r.sm} ${T.r.sm} 0`, fontStyle: "italic", color: T.text.darkSec }}>{block.text}</blockquote>;
    return null;
  });
}

/* Outliner */
function OutlinerBody({ sections }) {
  const typeIcons = { h1: "#", h2: "##", p: "¶", list: "•", quote: "❝" };
  const OutNode = ({ type, text, depth }) => (
    <div style={{ display: "flex", alignItems: "flex-start", gap: 6, padding: `4px 16px 4px ${16 + depth * 20}px`, fontSize: type.startsWith("h") ? 14 : 13, fontWeight: type.startsWith("h") ? 600 : 400, fontFamily: T.f.ui, color: type === "h1" ? T.text.dark : type === "h2" ? T.text.primary : T.text.secondary, cursor: "default" }}
      onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
      <span style={{ fontSize: 10, fontFamily: T.f.mono, fontWeight: 700, background: T.bg.sidebarActive, borderRadius: 3, padding: "1px 4px", flexShrink: 0, marginTop: 1, color: T.text.muted }}>{typeIcons[type] || "·"}</span>
      <span style={{ overflow: "hidden", textOverflow: "ellipsis", display: "-webkit-box", WebkitLineClamp: 2, WebkitBoxOrient: "vertical", lineHeight: 1.5 }}>{text}</span>
    </div>
  );
  return (
    <div style={{ flex: 1, overflowY: "auto", background: T.bg.editor }}>
      <FMBlock data={sampleFM}/>
      <OutNode type="h1" text="PROJECT 1 OVERVIEW" depth={0}/>
      {sections.map((sec) => (
        <div key={sec.id}>
          <OutNode type="h2" text={sec.heading} depth={1}/>
          {sec.content.map((b, i) => <OutNode key={i} type={b.type} text={b.text || b.items?.join(", ") || ""} depth={2}/>)}
        </div>
      ))}
    </div>
  );
}

/* WYSIWYG */
function WysiwygBody({ sections, onMoveSection, pageSize, headerTpl, footerTpl }) {
  const ml = pageSize.marginL;
  const totalPages = Math.max(1, Math.ceil(sections.length / 2));
  const tplVars = (pageNum) => ({ title: sampleFM.title, author: "AI Assistant & Human", date: sampleFM.date, page: String(pageNum), pages: String(totalPages) });

  return (
    <div style={{ flex: 1, overflowY: "auto", background: T.bg.canvas, padding: "24px 32px 40px", position: "relative" }}>
      {/* Render pages — 2 sections per page */}
      {Array.from({ length: totalPages }).map((_, pIdx) => {
        const pageSections = sections.slice(pIdx * 2, pIdx * 2 + 2);
        const vars = tplVars(pIdx + 1);
        return (
          <div key={pIdx}>
            {pIdx > 0 && <div style={{ height: 24 }}/>}
            <div style={{ background: "#FFFFFF", borderRadius: T.r.sm, boxShadow: "0 1px 4px rgba(0,0,0,0.08), 0 0 1px rgba(0,0,0,0.05)", width: "100%", maxWidth: pageSize.w, minHeight: pageSize.h, margin: "0 auto", position: "relative", display: "flex", flexDirection: "column" }}>
              <DocHeader template={headerTpl} vars={{ ...vars, section: pageSections[0]?.title || "" }}/>
              <MarginGuides ml={ml}/>
              {pIdx === 0 && <FMBlock data={sampleFM}/>}
              <div style={{ padding: `${pIdx === 0 ? 0 : 20}px ${ml}px 0`, fontFamily: T.f.editor, fontSize: 16, lineHeight: 1.75, color: T.text.dark, position: "relative", zIndex: 2, flex: 1 }}>
                {pIdx === 0 && <h1 style={{ fontFamily: T.f.heading, fontSize: 28, fontWeight: 700, margin: "8px 0 24px", letterSpacing: "-0.02em" }}>PROJECT 1 OVERVIEW</h1>}
                {pageSections.map((sec, sIdx) => {
                  const globalIdx = pIdx * 2 + sIdx;
                  return (
                    <div key={sec.id} id={sec.id}>
                      {(pIdx > 0 || sIdx > 0) && (
                        <SectionBreak
                          label={`Section — ${sec.title}`}
                          id={sec.id}
                          onMoveUp={() => onMoveSection(globalIdx, globalIdx - 1)}
                          onMoveDown={() => onMoveSection(globalIdx, globalIdx + 1)}
                          isFirst={globalIdx === 0}
                          isLast={globalIdx === sections.length - 1}
                        />
                      )}
                      <h2 style={{ fontFamily: T.f.heading, fontSize: 20, fontWeight: 600, margin: sIdx === 0 && pIdx === 0 ? "28px 0 12px" : "8px 0 12px" }}>{sec.heading}</h2>
                      {renderContent(sec.content)}
                    </div>
                  );
                })}
              </div>
              <DocFooter template={footerTpl} vars={vars}/>
            </div>
          </div>
        );
      })}
    </div>
  );
}

function EditorBody({ mode, sections, onMoveSection, pageSize, headerTpl, footerTpl }) {
  return mode === "draft"
    ? <OutlinerBody sections={sections}/>
    : <WysiwygBody sections={sections} onMoveSection={onMoveSection} pageSize={pageSize} headerTpl={headerTpl} footerTpl={footerTpl}/>;
}

// ═══════════════════════════════════════════════════════════════════
// RIGHT RAIL
// ═══════════════════════════════════════════════════════════════════

/* 1. Page Settings Panel */
function PageSettingsPanel({ pageSize, setPageSize, pageSizeKey, setPageSizeKey }) {
  return (
    <Collapsible title="PAGE SETTINGS" defaultOpen>
      <div style={{ padding: "0 16px 12px" }}>
        <div style={{ fontSize: 10, fontWeight: 600, letterSpacing: "0.06em", color: T.text.muted, fontFamily: T.f.ui, marginBottom: 4 }}>SIZE</div>
        <div style={{ display: "flex", gap: 4, marginBottom: 10, flexWrap: "wrap" }}>
          {Object.entries(PAGE_SIZES).map(([k, v]) => (
            <button key={k} onClick={() => { setPageSizeKey(k); setPageSize({ ...v }); }} style={{ padding: "4px 10px", fontSize: 11, fontFamily: T.f.ui, fontWeight: 600, border: pageSizeKey === k ? `2px solid ${T.bg.accent}` : `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, cursor: "pointer", background: pageSizeKey === k ? T.bg.accentSoft : "transparent", color: pageSizeKey === k ? T.text.link : T.text.secondary }}>
              {v.label}
            </button>
          ))}
        </div>
        <div style={{ fontSize: 10, fontWeight: 600, letterSpacing: "0.06em", color: T.text.muted, fontFamily: T.f.ui, marginBottom: 4, marginTop: 8 }}>DIMENSIONS</div>
        <div style={{ display: "flex", gap: 12, marginBottom: 10 }}>
          <NumberInput label="Width" value={pageSize.w} onChange={(v) => { setPageSize({ ...pageSize, w: v }); setPageSizeKey("Custom"); }} min={300} max={1200} unit="px"/>
          <NumberInput label="Height" value={pageSize.h} onChange={(v) => { setPageSize({ ...pageSize, h: v }); setPageSizeKey("Custom"); }} min={400} max={1600} unit="px"/>
        </div>
        <div style={{ fontSize: 10, fontWeight: 600, letterSpacing: "0.06em", color: T.text.muted, fontFamily: T.f.ui, marginBottom: 4, marginTop: 8 }}>MARGINS</div>
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "6px 12px" }}>
          <NumberInput label="Top" value={pageSize.marginT} onChange={(v) => { setPageSize({ ...pageSize, marginT: v }); setPageSizeKey("Custom"); }} min={0} max={200} unit="px"/>
          <NumberInput label="Bottom" value={pageSize.marginB} onChange={(v) => { setPageSize({ ...pageSize, marginB: v }); setPageSizeKey("Custom"); }} min={0} max={200} unit="px"/>
          <NumberInput label="Left" value={pageSize.marginL} onChange={(v) => { setPageSize({ ...pageSize, marginL: v }); setPageSizeKey("Custom"); }} min={0} max={200} unit="px"/>
          <NumberInput label="Right" value={pageSize.marginR} onChange={(v) => { setPageSize({ ...pageSize, marginR: v }); setPageSizeKey("Custom"); }} min={0} max={200} unit="px"/>
        </div>
      </div>
    </Collapsible>
  );
}

/* 3. Header/Footer Template Editor */
function TemplateEditor({ label, template, onChange }) {
  const [editing, setEditing] = useState(false);
  const fields = [
    { key: "left", label: "Left" },
    { key: "centre", label: "Centre" },
    { key: "right", label: "Right" },
  ];
  const vars = ["{title}", "{author}", "{date}", "{page}", "{pages}", "{section}"];

  return (
    <div style={{ padding: "0 16px 12px" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 6 }}>
        <span style={{ fontSize: 10, fontWeight: 600, letterSpacing: "0.06em", color: T.text.muted, fontFamily: T.f.ui }}>{label}</span>
        <button onClick={() => setEditing(!editing)} style={{ background: "none", border: "none", cursor: "pointer", color: T.text.link, display: "flex", alignItems: "center", gap: 3, fontSize: 10, fontFamily: T.f.ui }}>
          <I.Edit/> {editing ? "Done" : "Edit"}
        </button>
      </div>
      {/* Preview */}
      <div style={{ display: "flex", justifyContent: "space-between", padding: "6px 10px", background: T.bg.editor, border: `1px solid ${T.border.medium}`, borderRadius: T.r.sm, marginBottom: editing ? 8 : 0, fontSize: 10, fontFamily: T.f.ui, color: T.text.muted }}>
        <span>{template.left || "—"}</span>
        <span>{template.centre || "—"}</span>
        <span>{template.right || "—"}</span>
      </div>
      {/* Edit fields */}
      {editing && (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          {fields.map(({ key, label: fl }) => (
            <div key={key} style={{ display: "flex", alignItems: "center", gap: 6 }}>
              <span style={{ fontSize: 10, color: T.text.muted, fontFamily: T.f.ui, width: 40 }}>{fl}</span>
              <input value={template[key]} onChange={(e) => onChange({ ...template, [key]: e.target.value })}
                style={{ flex: 1, padding: "3px 6px", fontSize: 11, fontFamily: T.f.mono, border: `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, background: T.bg.editor, color: T.text.dark, outline: "none" }}/>
            </div>
          ))}
          <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginTop: 2 }}>
            {vars.map((v) => (
              <span key={v} style={{ fontSize: 9, fontFamily: T.f.mono, color: T.text.link, background: T.bg.accentSoft, borderRadius: 3, padding: "1px 5px", cursor: "default" }}>{v}</span>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

/* Variables Panel (unchanged Figma-style) */
function VariablesPanel() {
  const [activeCol, setActiveCol] = useState("Theme Colors");
  const [activeMode, setActiveMode] = useState("Light");
  const collections = [
    { name: "Theme Colors", modes: ["Light", "Dark"], groups: [
      { name: "Primitives", exp: true, vars: [
        { n: "blue-500", t: "color", v: { Light: "#007BFF", Dark: "#4DA3FF" } },
        { n: "grey-900", t: "color", v: { Light: "#1A1A1A", Dark: "#F5F5F5" } },
        { n: "grey-100", t: "color", v: { Light: "#F5F3EE", Dark: "#2D2D2D" } },
      ]},
      { name: "Semantic", exp: true, vars: [
        { n: "text/primary", t: "color", v: { Light: "#1A1A1A", Dark: "#F5F5F5" }, alias: "grey-900" },
        { n: "text/link", t: "color", v: { Light: "#007BFF", Dark: "#4DA3FF" }, alias: "blue-500" },
        { n: "bg/surface", t: "color", v: { Light: "#FFFFFF", Dark: "#1E1E1E" } },
      ]},
    ]},
    { name: "Spacing", modes: ["Desktop", "Mobile"], groups: [
      { name: "Layout", exp: true, vars: [
        { n: "margin/page", t: "number", v: { Desktop: "48", Mobile: "16" } },
        { n: "gap/section", t: "number", v: { Desktop: "32", Mobile: "20" } },
        { n: "radius/sm", t: "number", v: { Desktop: "3", Mobile: "3" } },
      ]},
    ]},
    { name: "Typography", modes: ["Default"], groups: [
      { name: "Fonts", exp: true, vars: [
        { n: "font/heading", t: "string", v: { Default: "SF Pro Display" } },
        { n: "font/body", t: "string", v: { Default: "Georgia" } },
      ]},
    ]},
    { name: "Flags", modes: ["Default"], groups: [
      { name: "Feature Flags", exp: true, vars: [
        { n: "show/frontmatter", t: "boolean", v: { Default: "true" } },
        { n: "show/ai-suggestions", t: "boolean", v: { Default: "true" } },
      ]},
    ]},
  ];
  const col = collections.find((c) => c.name === activeCol) || collections[0];
  const vtIcons = {
    color: (c) => <span style={{ width: 14, height: 14, borderRadius: 3, background: c, display: "inline-block", border: `1px solid ${T.border.medium}`, flexShrink: 0 }}/>,
    number: () => <span style={{ width: 16, height: 16, borderRadius: 3, background: "#E8E0F0", display: "inline-flex", alignItems: "center", justifyContent: "center", fontSize: 9, fontWeight: 800, fontFamily: T.f.mono, color: "#7C5DAC", flexShrink: 0 }}>#</span>,
    string: () => <span style={{ width: 16, height: 16, borderRadius: 3, background: "#D9F0E0", display: "inline-flex", alignItems: "center", justifyContent: "center", fontSize: 9, fontWeight: 800, fontFamily: T.f.mono, color: "#3A7D4E", flexShrink: 0 }}>S</span>,
    boolean: () => <span style={{ width: 16, height: 16, borderRadius: 3, background: "#FDE8D0", display: "inline-flex", alignItems: "center", justifyContent: "center", fontSize: 9, fontWeight: 800, fontFamily: T.f.mono, color: "#B06C20", flexShrink: 0 }}>B</span>,
  };
  const VG = ({ group }) => {
    const [open, setOpen] = useState(group.exp);
    return (
      <div>
        <div onClick={() => setOpen(!open)} style={{ display: "flex", alignItems: "center", gap: 6, padding: "6px 16px", cursor: "pointer" }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
          {open ? <I.ChevD s={10}/> : <I.ChevR s={10}/>}
          <span style={{ fontSize: 11, fontWeight: 700, letterSpacing: "0.04em", color: T.text.secondary, fontFamily: T.f.ui }}>{group.name}</span>
          <span style={{ fontSize: 10, color: T.text.muted, marginLeft: "auto" }}>{group.vars.length}</span>
        </div>
        {open && group.vars.map((v, i) => (
          <div key={i} style={{ display: "flex", alignItems: "center", gap: 8, padding: "4px 16px 4px 34px", fontSize: 12, fontFamily: T.f.ui, cursor: "pointer", minHeight: 28 }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
            {v.t === "color" ? vtIcons.color(v.v[activeMode]) : vtIcons[v.t]?.()}
            <span style={{ flex: 1, color: T.text.primary, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{v.n}</span>
            {v.alias && <span style={{ fontSize: 9, fontFamily: T.f.mono, color: T.text.link, background: T.bg.accentSoft, borderRadius: 3, padding: "1px 4px", flexShrink: 0 }}>→ {v.alias}</span>}
            <span style={{ fontSize: 11, fontFamily: T.f.mono, color: T.text.muted, flexShrink: 0 }}>{v.t === "boolean" ? <span style={{ color: v.v[activeMode] === "true" ? "#3A7D4E" : "#B06C20", fontWeight: 600 }}>{v.v[activeMode]}</span> : v.v[activeMode]}</span>
          </div>
        ))}
      </div>
    );
  };
  return (
    <div>
      <div style={{ padding: "10px 16px 8px", borderBottom: `1px solid ${T.border.rail}` }}>
        <div style={{ fontSize: 10, fontWeight: 600, letterSpacing: "0.06em", color: T.text.muted, fontFamily: T.f.ui, marginBottom: 4 }}>COLLECTION</div>
        <select value={activeCol} onChange={(e) => { setActiveCol(e.target.value); const n = collections.find((c) => c.name === e.target.value); if (n) setActiveMode(n.modes[0]); }}
          style={{ width: "100%", padding: "5px 8px", fontSize: 12, fontFamily: T.f.ui, border: `1px solid ${T.border.subtle}`, borderRadius: T.r.sm, background: T.bg.editor, color: T.text.dark, outline: "none", cursor: "pointer" }}>
          {collections.map((c) => <option key={c.name} value={c.name}>{c.name}</option>)}
        </select>
      </div>
      {col.modes.length > 1 && (
        <div style={{ display: "flex", borderBottom: `1px solid ${T.border.rail}`, padding: "0 16px" }}>
          {col.modes.map((m) => <button key={m} onClick={() => setActiveMode(m)} style={{ padding: "6px 12px", border: "none", cursor: "pointer", fontSize: 11, fontWeight: 600, fontFamily: T.f.ui, background: "transparent", color: activeMode === m ? T.text.dark : T.text.muted, borderBottom: activeMode === m ? `2px solid ${T.bg.accent}` : "2px solid transparent" }}>{m}</button>)}
          <span style={{ flex: 1 }}/><button style={{ background: "none", border: "none", fontSize: 14, cursor: "pointer", color: T.text.muted, padding: "4px" }}>+</button>
        </div>
      )}
      <div style={{ paddingTop: 4 }}>{col.groups.map((g, i) => <VG key={i} group={g}/>)}</div>
      <div style={{ padding: "8px 16px" }}><button style={{ width: "100%", padding: "6px 0", border: `1px dashed ${T.border.rail}`, borderRadius: T.r.sm, background: "transparent", cursor: "pointer", fontSize: 12, color: T.text.muted, fontFamily: T.f.ui }}>+ Add variable</button></div>
    </div>
  );
}

/* Styles Panel (unchanged WYSIWYG previews) */
function StylesPanel() {
  const heads = [{ tag: "H1", font: "SF Pro Display", sz: 28, w: 700, sp: "-0.02em", txt: "Heading One" }, { tag: "H2", font: "SF Pro Display", sz: 20, w: 600, sp: "0", txt: "Heading Two" }, { tag: "H3", font: "SF Pro Display", sz: 16, w: 600, sp: "0.01em", txt: "Heading Three" }];
  const bodies = [{ n: "Body", f: "Georgia", sz: 16, w: 400, s: "normal", txt: "The quick brown fox jumps over the lazy dog." }, { n: "Caption", f: "SF Pro Text", sz: 12, w: 400, s: "normal", txt: "Figure 1: Caption text" }, { n: "Strong", f: "Georgia", sz: 16, w: 700, s: "normal", txt: "Bold emphasis text" }, { n: "Emphasised", f: "Georgia", sz: 16, w: 400, s: "italic", txt: "Italic emphasis text" }, { n: "Code", f: "SF Mono", sz: 13, w: 400, s: "normal", txt: "const x = 42;", mono: true }];
  const elements = [{ n: "Blockquote", txt: "A quoted passage…", bc: T.border.rail, bg: T.bg.blockquote, it: true }, { n: "Table Heading", txt: "Column Header", bg: "#C8DFC8", b: true }, { n: "Link", txt: "[discovery.md]", color: T.text.link, u: true }];
  const colors = [{ n: "Accent Blue", v: "#007BFF", u: "Links, active" }, { n: "Warning Orange", v: "#FFC107", u: "Alerts" }, { n: "Success Green", v: "#284745", u: "Confirmations" }, { n: "Text Primary", v: "#1A1A1A", u: "Body text" }, { n: "Surface", v: "#F5F3EE", u: "Backgrounds" }];
  const StyleCard = ({ children, label, meta }) => (
    <div style={{ padding: "6px 10px", borderRadius: T.r.sm, border: `1px solid ${T.border.medium}`, cursor: "pointer" }} onMouseEnter={(e) => (e.currentTarget.style.borderColor = T.border.active)} onMouseLeave={(e) => (e.currentTarget.style.borderColor = T.border.medium)}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 3 }}><span style={{ fontSize: 10, fontWeight: 700, fontFamily: T.f.mono, color: T.text.muted }}>{label}</span>{meta && <span style={{ fontSize: 10, fontFamily: T.f.mono, color: T.text.muted }}>{meta}</span>}</div>
      {children}
    </div>
  );
  return (
    <div>
      <Collapsible title="HEADINGS" defaultOpen>
        <div style={{ padding: "0 16px 12px", display: "flex", flexDirection: "column", gap: 8 }}>
          {heads.map((h) => <StyleCard key={h.tag} label={h.tag} meta={`${h.sz}px / ${h.w}`}><div style={{ fontFamily: h.font, fontSize: Math.min(h.sz, 22), fontWeight: h.w, letterSpacing: h.sp, color: T.text.dark, lineHeight: 1.3 }}>{h.txt}</div></StyleCard>)}
        </div>
      </Collapsible>
      <Collapsible title="BODY & TEXT" defaultOpen>
        <div style={{ padding: "0 16px 12px", display: "flex", flexDirection: "column", gap: 6 }}>
          {bodies.map((s) => <StyleCard key={s.n} label={s.n} meta={`${s.f} ${s.sz}px`}><div style={{ fontFamily: s.mono ? T.f.mono : s.f, fontSize: Math.min(s.sz, 15), fontWeight: s.w, fontStyle: s.s, color: T.text.dark, lineHeight: 1.5, background: s.mono ? T.bg.sidebarActive : "transparent", borderRadius: s.mono ? 3 : 0, padding: s.mono ? "2px 6px" : 0 }}>{s.txt}</div></StyleCard>)}
        </div>
      </Collapsible>
      <Collapsible title="ELEMENTS" defaultOpen>
        <div style={{ padding: "0 16px 12px", display: "flex", flexDirection: "column", gap: 6 }}>
          {elements.map((el) => <StyleCard key={el.n} label={el.n}><div style={{ fontSize: 13, fontFamily: T.f.editor, lineHeight: 1.5, color: el.color || T.text.dark, fontWeight: el.b ? 600 : 400, fontStyle: el.it ? "italic" : "normal", textDecoration: el.u ? "underline" : "none", textUnderlineOffset: el.u ? 3 : 0, background: el.bg || "transparent", borderLeft: el.bc ? `3px solid ${el.bc}` : "none", padding: el.bg || el.bc ? "4px 8px" : 0, borderRadius: el.bg ? 3 : 0 }}>{el.txt}</div></StyleCard>)}
        </div>
      </Collapsible>
      <Collapsible title="COLOR STYLES" defaultOpen>
        <div style={{ padding: "0 16px 12px" }}>
          {colors.map((cs) => (
            <div key={cs.n} style={{ display: "flex", alignItems: "center", gap: 8, padding: "5px 0", fontSize: 12, fontFamily: T.f.ui, cursor: "pointer" }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hover)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>
              <span style={{ width: 18, height: 18, borderRadius: 4, background: cs.v, display: "inline-block", border: `1px solid ${T.border.medium}`, flexShrink: 0 }}/>
              <div style={{ flex: 1 }}><div style={{ fontWeight: 500, color: T.text.dark }}>{cs.n}</div><div style={{ fontSize: 10, color: T.text.muted }}>{cs.u}</div></div>
              <span style={{ fontSize: 10, fontFamily: T.f.mono, color: T.text.muted }}>{cs.v}</span>
            </div>
          ))}
        </div>
      </Collapsible>
    </div>
  );
}

/* Right Rail */
function RightRail({ activeTab, onTabChange, pageSize, setPageSize, pageSizeKey, setPageSizeKey, headerTpl, setHeaderTpl, footerTpl, setFooterTpl }) {
  return (
    <div style={{ width: 290, background: T.bg.rail, borderLeft: `1px solid ${T.border.rail}`, display: "flex", flexDirection: "column", flexShrink: 0, overflow: "hidden" }}>
      <div style={{ display: "flex", borderBottom: `1px solid ${T.border.rail}`, flexShrink: 0 }}>
        {["Draft", "Design"].map((tab) => (
          <button key={tab} onClick={() => onTabChange(tab.toLowerCase())} style={{ flex: 1, padding: "10px 0", border: "none", cursor: "pointer", fontSize: 13, fontWeight: 600, fontFamily: T.f.ui, background: activeTab === tab.toLowerCase() ? T.bg.rail : "#EAE7E0", color: activeTab === tab.toLowerCase() ? T.text.dark : T.text.darkSec, borderBottom: activeTab === tab.toLowerCase() ? `2px solid ${T.bg.accent}` : "2px solid transparent" }}>{tab}</button>
        ))}
      </div>
      <div style={{ flex: 1, overflowY: "auto" }}>
        {activeTab === "design" && <StylesPanel/>}
        <PageSettingsPanel pageSize={pageSize} setPageSize={setPageSize} pageSizeKey={pageSizeKey} setPageSizeKey={setPageSizeKey}/>
        <Collapsible title="HEADER & FOOTER" defaultOpen>
          <TemplateEditor label="HEADER" template={headerTpl} onChange={setHeaderTpl}/>
          <TemplateEditor label="FOOTER" template={footerTpl} onChange={setFooterTpl}/>
        </Collapsible>
        <Collapsible title="VARIABLES" defaultOpen={activeTab === "draft"}>
          <VariablesPanel/>
        </Collapsible>
      </div>
    </div>
  );
}

/* Status Bar */
function StatusBar() {
  return (
    <div style={{ height: 24, background: T.bg.statusbar, display: "flex", alignItems: "center", justifyContent: "space-between", padding: "0 12px", fontSize: 11, color: T.text.secondary, fontFamily: T.f.ui, flexShrink: 0, borderTop: `1px solid ${T.border.subtle}` }}>
      <div style={{ display: "flex", gap: 14 }}><span>Ln 1, Col 1</span><span>Tab Size: 4</span><span>UTF-8</span></div>
      <div style={{ display: "flex", gap: 14, alignItems: "center" }}>
        <span>WORDS: 1,234</span><span>CHARS: 5,678</span>
        <span style={{ display: "flex", alignItems: "center", gap: 4 }}><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5"><line x1="6" y1="3" x2="6" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>Git: main</span>
        <span>Last Sync: 2m ago</span><span>AI Status: Synced</span>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════
// MAIN APPLICATION
// ═══════════════════════════════════════════════════════════════════
export default function DocForgeApp() {
  const [activeView, setActiveView] = useState("explorer");
  const [showLeft, setShowLeft] = useState(true);
  const [showRight, setShowRight] = useState(true);
  const [railTab, setRailTab] = useState("design");

  // Document state
  const [sections, setSections] = useState(defaultSections);
  const [pageSizeKey, setPageSizeKey] = useState("A4");
  const [pageSize, setPageSize] = useState({ ...PAGE_SIZES.A4 });
  const [headerTpl, setHeaderTpl] = useState({ ...defaultHeaderTemplate });
  const [footerTpl, setFooterTpl] = useState({ ...defaultFooterTemplate });

  // 2. Section reorder handler
  const moveSection = useCallback((from, to) => {
    if (to < 0 || to >= sections.length) return;
    setSections((prev) => {
      const next = [...prev];
      const [item] = next.splice(from, 1);
      next.splice(to, 0, item);
      return next;
    });
  }, [sections.length]);

  // 4. Scroll-to handler for TOC
  const scrollTo = useCallback((id) => {
    if (id === "top") {
      document.querySelector("[data-editor-scroll]")?.scrollTo({ top: 0, behavior: "smooth" });
    } else {
      document.getElementById(id)?.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }, []);

  return (
    <div style={{ width: "100%", height: "100vh", display: "flex", flexDirection: "column", overflow: "hidden", fontFamily: T.f.ui, background: T.bg.app }}>
      {/* Title Bar */}
      <div style={{ height: 38, background: T.bg.titlebar, display: "flex", alignItems: "center", borderBottom: `1px solid ${T.border.subtle}`, WebkitAppRegion: "drag", flexShrink: 0 }}>
        {/* Traffic lights */}
        <div style={{ display: "flex", gap: 8, alignItems: "center", paddingLeft: 12 }}>
          {["#FF5F57", "#FEBC2E", "#28C840"].map((c, i) => <div key={i} style={{ width: 12, height: 12, borderRadius: "50%", background: c, border: `1px solid ${c}88` }}/>)}
        </div>
        {/* Menu — left aligned */}
        <div style={{ display: "flex", gap: 2, fontSize: 13, color: T.text.primary, fontFamily: T.f.ui, WebkitAppRegion: "no-drag", marginLeft: 16 }}>
          {["File", "Edit", "Selection", "View", "Help"].map((m) => (
            <span key={m} style={{ padding: "4px 10px", cursor: "pointer", borderRadius: T.r.sm }} onMouseEnter={(e) => (e.currentTarget.style.background = T.bg.hoverStrong)} onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}>{m}</span>
          ))}
        </div>
        <div style={{ flex: 1 }}/>
        {/* Drawer toggles — right aligned */}
        <div style={{ display: "flex", gap: 4, paddingRight: 12, WebkitAppRegion: "no-drag" }}>
          <button onClick={() => setShowLeft(!showLeft)} style={{ background: showLeft ? T.bg.hoverStrong : "transparent", border: "none", cursor: "pointer", color: T.text.secondary, borderRadius: T.r.sm, padding: 4, display: "flex" }} title="Toggle left panel"><I.PanelL/></button>
          <button onClick={() => setShowRight(!showRight)} style={{ background: showRight ? T.bg.hoverStrong : "transparent", border: "none", cursor: "pointer", color: T.text.secondary, borderRadius: T.r.sm, padding: 4, display: "flex" }} title="Toggle right panel"><I.PanelR/></button>
        </div>
      </div>

      {/* Main Body */}
      <div style={{ flex: 1, display: "flex", overflow: "hidden" }}>
        <ActivityBar activeView={activeView} onViewChange={setActiveView}/>
        {showLeft && <SidebarPanel activeView={activeView} sections={sections} onScrollTo={scrollTo}/>}
        <div style={{ flex: 1, display: "flex", flexDirection: "column", overflow: "hidden" }}>
          <TabBar tabs={sampleTabs}/>
          <EditorToolbar/>
          <Ruler marginL={pageSize.marginL}/>
          <EditorBody mode={railTab} sections={sections} onMoveSection={moveSection} pageSize={pageSize} headerTpl={headerTpl} footerTpl={footerTpl}/>
        </div>
        {showRight && <RightRail activeTab={railTab} onTabChange={setRailTab} pageSize={pageSize} setPageSize={setPageSize} pageSizeKey={pageSizeKey} setPageSizeKey={setPageSizeKey} headerTpl={headerTpl} setHeaderTpl={setHeaderTpl} footerTpl={footerTpl} setFooterTpl={setFooterTpl}/>}
      </div>

      <StatusBar/>
    </div>
  );
}

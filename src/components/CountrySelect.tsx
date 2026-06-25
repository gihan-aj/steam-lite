import { useEffect, useRef, useState } from "react";
import { COUNTRIES, Country, findCountry } from "../countries";

export function CountrySelect({value, onChange} : {value: string; onChange: (code: string) => void}){
    const [open, setOpen] = useState(false);
    const [search, setSearch] = useState("");
    const inputRef = useRef<HTMLInputElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);

    const selected = findCountry(value);

    const filtered = search.length === 0
        ? COUNTRIES
        : COUNTRIES.filter(c => 
            c.name.toLowerCase().includes(search.toLowerCase()) ||
            c.name.toLowerCase().includes(search.toLowerCase())
        );

    // Close on outside click
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (!containerRef.current?.contains(e.target as Node)) {
                setOpen(false);
                setSearch("");
            }
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, []);

    // Focus search when drop down opens
    useEffect(() => {
        if(open) inputRef.current?.focus();
    }, [open]);

    const handleSelect = (country: Country) => {
        onChange(country.code);
        setOpen(false);
        setSearch('');
    };

    const sharedInputStyle = {
      background: "#1c1e27",
      border: "1px solid #2a2d3a",
      borderRadius: 6,
      padding: "8px 12px",
      color: "#e0e2e8",
      fontSize: 13,
      outline: "none",
      width: "100%",
      cursor: "pointer",
    } as const;

     return (
       <div ref={containerRef} style={{ position: "relative" }}>
         {/* Trigger button — shows selected country */}
         <button
           onClick={() => setOpen((o) => !o)}
           style={{
             ...sharedInputStyle,
             display: "flex",
             alignItems: "center",
             justifyContent: "space-between",
             gap: 8,
             border: `1px solid ${open ? "#3d6ef8" : "#2a2d3a"}`,
             cursor: "pointer",
             textAlign: "left",
           }}
         >
           <span>
             {selected
               ? `${selected.name} (${selected.code.toUpperCase()})`
               : "Select your region…"}
           </span>
           {/* Chevron — rotates when open */}
           <svg
             width="12"
             height="12"
             viewBox="0 0 12 12"
             style={{
               flexShrink: 0,
               transform: open ? "rotate(180deg)" : "rotate(0)",
               transition: "transform 0.15s",
               color: "#5a5f72",
             }}
           >
             <path
               d="M2 4l4 4 4-4"
               stroke="currentColor"
               strokeWidth="1.5"
               fill="none"
               strokeLinecap="round"
               strokeLinejoin="round"
             />
           </svg>
         </button>

         {/* Dropdown */}
         {open && (
           <div
             style={{
               position: "absolute",
               top: "calc(100% + 4px)",
               left: 0,
               right: 0,
               background: "#1c1e27",
               border: "1px solid #3d6ef8",
               borderRadius: 8,
               zIndex: 100,
               overflow: "hidden",
               boxShadow: "0 8px 24px rgba(0,0,0,0.4)",
             }}
           >
             {/* Search input */}
             <div
               style={{
                 padding: "8px 8px 6px",
                 borderBottom: "1px solid #1a1d28",
               }}
             >
               <input
                 ref={inputRef}
                 type="text"
                 placeholder="Search country or code…"
                 value={search}
                 onChange={(e) => setSearch(e.target.value)}
                 style={{
                   ...sharedInputStyle,
                   cursor: "text",
                   padding: "6px 10px",
                   fontSize: 12,
                 }}
               />
             </div>

             {/* Country list — fixed height with scroll */}
             <div style={{ maxHeight: 220, overflowY: "auto" }}>
               {filtered.length === 0 ? (
                 <div
                   style={{
                     padding: "12px 16px",
                     fontSize: 12,
                     color: "#5a5f72",
                   }}
                 >
                   No countries match "{search}"
                 </div>
               ) : (
                 filtered.map((country) => {
                   const isSelected = country.code === value;
                   return (
                     <button
                       key={country.code}
                       onClick={() => handleSelect(country)}
                       style={{
                         display: "flex",
                         alignItems: "center",
                         justifyContent: "space-between",
                         width: "100%",
                         padding: "8px 14px",
                         background: isSelected ? "#1e2540" : "transparent",
                         border: "none",
                         cursor: "pointer",
                         textAlign: "left",
                         gap: 8,
                       }}
                       onMouseEnter={(e) =>
                         !isSelected &&
                         ((e.currentTarget as HTMLElement).style.background =
                           "#16181f")
                       }
                       onMouseLeave={(e) =>
                         !isSelected &&
                         ((e.currentTarget as HTMLElement).style.background =
                           "transparent")
                       }
                     >
                       <span
                         style={{
                           fontSize: 13,
                           color: isSelected ? "#e0e2e8" : "#9096a8",
                         }}
                       >
                         {country.name}
                       </span>
                       <span
                         style={{
                           fontSize: 11,
                           fontWeight: 600,
                           color: isSelected ? "#3d6ef8" : "#3a3f58",
                           flexShrink: 0,
                         }}
                       >
                         {country.code.toUpperCase()}
                       </span>
                     </button>
                   );
                 })
               )}
             </div>
           </div>
         )}
       </div>
     );
}
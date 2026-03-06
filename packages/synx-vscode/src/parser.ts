/**
 * SYNX IDE Parser — lightweight parser that returns AST-like node tree
 * with position info for diagnostics, navigation, and IntelliSense.
 * Zero external deps. Used exclusively inside the VS Code extension.
 */

// ─── Types ───────────────────────────────────────────────────────────────────

export type SynxVal = string | number | boolean | null | SynxVal[] | { [k: string]: SynxVal };

export interface SynxNode {
  key: string;
  rawValue: string;
  value: SynxVal;
  line: number;
  column: number;
  indent: number;
  markers: string[];
  markerArgs: string[];
  constraints: string;
  typeHint: string;
  children: SynxNode[];
  parent: SynxNode | null;
  isListItem: boolean;
}

export interface ParsedDoc {
  mode: 'static' | 'active';
  modeLine: number;
  nodes: SynxNode[];
  /** Flat map: dotPath → node (e.g. "server.ssl.cert") */
  keyMap: Map<string, SynxNode>;
  /** All nodes in document order */
  allNodes: SynxNode[];
  lines: string[];
}

// ─── Regex ───────────────────────────────────────────────────────────────────

const LINE_RE = /^([^\s\[:\-#/(][^\s\[:(]*)(?:\((\w+)\))?(?:\[([^\]]*)\])?(?::([\w:]+))?\s*(.*)$/;

// ─── Cast ────────────────────────────────────────────────────────────────────

function cast(val: string, hint?: string): SynxVal {
  if (hint === 'string') return val;
  if (hint === 'int') return parseInt(val, 10) || 0;
  if (hint === 'float') return parseFloat(val) || 0;
  if (hint === 'bool') return val === 'true';
  if (val === 'true') return true;
  if (val === 'false') return false;
  if (val === 'null') return null;
  if (/^-?\d+$/.test(val)) return parseInt(val, 10);
  if (/^-?\d+\.\d+$/.test(val)) return parseFloat(val);
  return val;
}

function stripComment(val: string): string {
  let r = val;
  const s1 = r.indexOf(' //');
  if (s1 !== -1) r = r.substring(0, s1);
  const s2 = r.indexOf(' #');
  if (s2 !== -1) r = r.substring(0, s2);
  return r.trimEnd();
}

// ─── Parse ───────────────────────────────────────────────────────────────────

export function parseSynx(text: string): ParsedDoc {
  const lines = text.split(/\r?\n/);
  const doc: ParsedDoc = {
    mode: 'static',
    modeLine: -1,
    nodes: [],
    keyMap: new Map(),
    allNodes: [],
    lines,
  };

  const stack: Array<{ indent: number; nodes: SynxNode[]; path: string }> = [
    { indent: -1, nodes: doc.nodes, path: '' },
  ];

  let block: { node: SynxNode; indent: number } | null = null;
  let listTarget: { indent: number; arr: SynxNode } | null = null;

  for (let i = 0; i < lines.length; i++) {
    const raw = lines[i];
    const trimmed = raw.trim();

    // Mode
    if (trimmed === '!active') {
      doc.mode = 'active';
      doc.modeLine = i;
      continue;
    }
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) continue;

    const indent = raw.search(/\S/);

    // Block continuation
    if (block && indent > block.indent) {
      const existing = block.node.value as string;
      block.node.value = (existing ? existing + '\n' : '') + trimmed;
      block.node.rawValue = block.node.value as string;
      continue;
    } else {
      block = null;
    }

    // List item
    if (trimmed.startsWith('- ')) {
      const itemVal = stripComment(trimmed.substring(2).trim());
      if (listTarget && indent > listTarget.indent) {
        // Check if it's a list-of-objects item (has sub-content)
        const m = itemVal.match(/^([^\s\[:#/][^\s\[:(]*)\s+(.*)$/);
        if (m) {
          const childNode: SynxNode = {
            key: m[1], rawValue: m[2], value: cast(m[2]),
            line: i, column: indent + 2, indent,
            markers: [], markerArgs: [], constraints: '', typeHint: '',
            children: [], parent: listTarget.arr, isListItem: true,
          };
          listTarget.arr.children.push(childNode);
          doc.allNodes.push(childNode);
        } else {
          const childNode: SynxNode = {
            key: `[${listTarget.arr.children.length}]`,
            rawValue: itemVal, value: cast(itemVal),
            line: i, column: indent + 2, indent,
            markers: [], markerArgs: [], constraints: '', typeHint: '',
            children: [], parent: listTarget.arr, isListItem: true,
          };
          listTarget.arr.children.push(childNode);
          doc.allNodes.push(childNode);
        }
        continue;
      }
    }

    if (listTarget && indent <= listTarget.indent) {
      listTarget = null;
    }

    const m = trimmed.match(LINE_RE);
    if (!m) continue;

    const [, key, typeHint, constraintStr, markerChain, rawVal] = m;
    let cleanVal = rawVal ? stripComment(rawVal.trim()) : '';
    const markers = markerChain ? markerChain.split(':').filter(Boolean) : [];
    let markerArgs: string[] = [];

    if (markers.includes('random') && cleanVal) {
      const nums = cleanVal.split(/\s+/).filter(s => /^\d+(\.\d+)?$/.test(s));
      if (nums.length && nums.length === cleanVal.split(/\s+/).length) {
        markerArgs = nums;
        cleanVal = '';
      }
    }

    // Pop stack
    while (stack.length > 1 && stack[stack.length - 1].indent >= indent) stack.pop();
    const parent = stack[stack.length - 1];

    const dotPath = parent.path ? `${parent.path}.${key}` : key;
    const column = raw.indexOf(key);

    const node: SynxNode = {
      key,
      rawValue: cleanVal,
      value: cast(cleanVal, typeHint),
      line: i,
      column,
      indent,
      markers,
      markerArgs,
      constraints: constraintStr || '',
      typeHint: typeHint || '',
      children: [],
      parent: null,
      isListItem: false,
    };

    parent.nodes.push(node);
    doc.keyMap.set(dotPath, node);
    doc.allNodes.push(node);

    const isList = markers.some(mk => ['random', 'unique', 'geo', 'join'].includes(mk));

    if (cleanVal === '|') {
      node.value = '';
      node.rawValue = '';
      block = { node, indent };
    } else if (isList && !cleanVal) {
      node.value = [];
      listTarget = { indent, arr: node };
    } else if (!cleanVal) {
      // peek next non-empty line
      let pk = i + 1;
      while (pk < lines.length && !lines[pk].trim()) pk++;
      if (pk < lines.length && lines[pk].trim().startsWith('- ')) {
        node.value = [];
        listTarget = { indent, arr: node };
      } else {
        node.value = {};
        stack.push({ indent, nodes: node.children, path: dotPath });
      }
    }
  }

  return doc;
}

// ─── Resolve (for preview / convert) ─────────────────────────────────────────

export function resolveToObject(doc: ParsedDoc): Record<string, SynxVal> {
  const root: Record<string, SynxVal> = {};
  buildObject(doc.nodes, root);
  if (doc.mode === 'active') {
    resolveActive(root, root);
  }
  return root;
}

function buildObject(nodes: SynxNode[], target: Record<string, SynxVal>) {
  for (const n of nodes) {
    if (n.isListItem) continue;
    if (Array.isArray(n.value)) {
      target[n.key] = n.children.map(c => c.children.length > 0 ? buildChild(c) : c.value);
    } else if (typeof n.value === 'object' && n.value !== null) {
      const sub: Record<string, SynxVal> = {};
      buildObject(n.children, sub);
      target[n.key] = sub;
    } else {
      target[n.key] = n.value;
    }
  }
}

function buildChild(node: SynxNode): SynxVal {
  if (node.children.length === 0) return node.value;
  const obj: Record<string, SynxVal> = {};
  obj[node.key] = node.value;
  for (const c of node.children) {
    obj[c.key] = c.children.length > 0 ? buildChild(c) : c.value;
  }
  return obj;
}

function resolveActive(obj: Record<string, any>, root: Record<string, any>) {
  for (const key of Object.keys(obj)) {
    const val = obj[key];
    if (val && typeof val === 'object' && !Array.isArray(val)) {
      resolveActive(val, root);
    }
  }
}

// ─── Calc evaluator (safe, no eval) ──────────────────────────────────────────

export function safeCalc(expr: string, vars: Record<string, number>): number | null {
  let resolved = expr;
  for (const [k, v] of Object.entries(vars)) {
    resolved = resolved.replace(new RegExp(`\\b${k.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\b`, 'g'), String(v));
  }
  // Validate: only digits, operators, spaces, dots, parentheses
  if (!/^[\d\s+\-*/%().]+$/.test(resolved)) return null;
  try {
    // Recursive descent evaluator
    return evalExpr(resolved.replace(/\s+/g, ''));
  } catch {
    return null;
  }
}

function evalExpr(s: string): number {
  let pos = 0;
  function parseExpr(): number {
    let result = parseTerm();
    while (pos < s.length && (s[pos] === '+' || s[pos] === '-')) {
      const op = s[pos++];
      const right = parseTerm();
      result = op === '+' ? result + right : result - right;
    }
    return result;
  }
  function parseTerm(): number {
    let result = parseFactor();
    while (pos < s.length && (s[pos] === '*' || s[pos] === '/' || s[pos] === '%')) {
      const op = s[pos++];
      const right = parseFactor();
      result = op === '*' ? result * right : op === '/' ? result / right : result % right;
    }
    return result;
  }
  function parseFactor(): number {
    if (s[pos] === '(') {
      pos++;
      const result = parseExpr();
      pos++; // skip ')'
      return result;
    }
    if (s[pos] === '-') {
      pos++;
      return -parseFactor();
    }
    const start = pos;
    while (pos < s.length && (s[pos] >= '0' && s[pos] <= '9' || s[pos] === '.')) pos++;
    return parseFloat(s.substring(start, pos));
  }
  return parseExpr();
}

// ─── Serialize to SYNX ──────────────────────────────────────────────────────

export function serializeToSynx(obj: Record<string, any>, indent = 0): string {
  let out = '';
  const spaces = ' '.repeat(indent);

  for (const [key, val] of Object.entries(obj)) {
    if (Array.isArray(val)) {
      out += `${spaces}${key}\n`;
      for (const v of val) {
        if (v && typeof v === 'object' && !Array.isArray(v)) {
          const entries = Object.entries(v);
          if (entries.length > 0) {
            out += `${spaces}  - ${entries[0][0]} ${entries[0][1]}\n`;
            for (let i = 1; i < entries.length; i++) {
              out += `${spaces}    ${entries[i][0]} ${entries[i][1]}\n`;
            }
          }
        } else {
          out += `${spaces}  - ${v}\n`;
        }
      }
    } else if (typeof val === 'object' && val !== null) {
      out += `${spaces}${key}\n`;
      out += serializeToSynx(val, indent + 2);
    } else if (typeof val === 'string' && val.includes('\n')) {
      out += `${spaces}${key} |\n`;
      for (const line of val.split('\n')) {
        out += `${spaces}  ${line}\n`;
      }
    } else {
      out += `${spaces}${key} ${val}\n`;
    }
  }

  return out;
}

// ─── JSON → SYNX converter ──────────────────────────────────────────────────

export function jsonToSynx(json: string): string {
  const obj = JSON.parse(json);
  return serializeToSynx(obj);
}

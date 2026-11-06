'use strict';
const LOCAL = ['localhost', '127.0.0.1', '[::1]'].includes(location.hostname);
const SOURCE = LOCAL ? '/snapshot.json' : 'https://raw.githubusercontent.com/iand675/cinch/main/bootstrap/progress.json';
const $ = id => document.getElementById(id);
const STATES = ['verified', 'blocked', 'running', 'pending', 'recheck'];
const LABELS = {verified:'Verified', blocked:'Blocked', running:'In progress', pending:'Pending', recheck:'Needs rerun'};
const SYMBOLS = {verified:'✓', blocked:'!', running:'◷', pending:'□', recheck:'↻'};
const expanded = new Set(), collapsed = new Set();
let current = null, busy = false, live = false, lastChecked = null, filter = 'all';
function element(tag, text, className) {
  const el = document.createElement(tag);
  if (text !== undefined) el.textContent = text;
  if (className) el.className = className;
  return el;
}
function badge(state, text) { return element('span', SYMBOLS[state] + ' ' + (text || LABELS[state]), 'badge '+state); }
function externalLink(text, href) {
  const a = element('a', text);
  if (typeof href === 'string' && /^https:\/\/(github\.com|raw\.githubusercontent\.com)\//.test(href)) a.href = href;
  return a;
}
function validate(data) {
  if (![1, 2].includes(data?.schema) || !Number.isFinite(Date.parse(data.updatedAt))) throw new Error('Invalid status');
  for (const key of ['status', 'summary', 'activity']) if (typeof data[key] !== 'string') throw new Error('Invalid text');
  for (const key of ['checks', 'milestones']) if (!Array.isArray(data[key])) throw new Error('Invalid list');
  if (!/^[a-f0-9]{40}$/.test(data.commit)) throw new Error('Invalid commit');
  const c = data.checklist;
  if (!c || c.acceptanceTotal !== 5 || c.acceptance.length !== 5 || !Array.isArray(c.gaps) || !Array.isArray(c.suites)) throw new Error('Checklist unavailable');
  const rows = [...c.acceptance, ...c.gaps, ...c.suites.flatMap(s => s.items)];
  if (rows.length !== c.total || new Set(rows.map(r => r.id)).size !== rows.length || rows.some(r => !STATES.includes(r.state))) throw new Error('Invalid checklist inventory');
  for (const state of STATES) if (rows.filter(r => r.state === state).length !== c.counts[state]) throw new Error('Invalid checklist counts');
  if (c.acceptanceVerified !== c.acceptance.filter(r => r.state === 'verified').length) throw new Error('Invalid acceptance count');
  return data;
}
function keepDetails(root) {
  root.querySelectorAll('details[data-key]').forEach(d => { if (collapsed.has(d.dataset.key)) d.open = false; else if (expanded.has(d.dataset.key)) d.open = true; });
}
function previousResult(row) {
  const o = row.observed;
  if (!o || o.status === 'pending') return null;
  const box = element('div', undefined, 'observation');
  const label = ({passed:'Passed',failed:'Failed',running:'Running',ignored:'Ignored'})[o.status] || o.status;
  const old = row.state === 'recheck' || row.state === 'pending';
  box.append(element('p', (old ? 'Earlier result: ' : 'Recorded result: ') + label + (o.source ? ' · compiler '+o.source.slice(0,8) : ''), 'observed-status '+(o.status === 'failed' ? 'failure' : '')));
  if (o.finishedAt) box.append(element('p', 'Finished '+new Date(o.finishedAt).toLocaleString(), 'footnote'));
  if (o.cases?.length) {
    const cases = element('ul', undefined, 'case-list');
    o.cases.forEach(c => {
      const li = element('li'); li.append(element('span', c.name, 'case-name'), element('span', c.status, 'case-outcome '+(c.status === 'failed' ? 'failure' : ''))); cases.append(li);
    }); box.append(cases);
  }
  if (o.logSha256) box.append(element('p', 'Retained log SHA256 '+o.logSha256, 'hash'));
  return box;
}
function rowElement(row, compact) {
  const d = element('details', undefined, 'check-item '+row.state+(compact ? ' compact' : ''));
  d.dataset.key = row.id;
  const summary = element('summary');
  const title = element('span', row.title, 'item-title');
  summary.append(badge(row.state), title, element('span', '+', 'disclosure')); d.append(summary);
  const body = element('div', undefined, 'item-body');
  body.append(element('p', row.detail), element('p', 'Done when: '+row.doneWhen, 'done-when'));
  if (row.category) body.append(element('p', 'Role: '+row.category, 'footnote'));
  if (row.currentTests != null) body.append(element('p', row.currentTests+' passing tests in the current complete target.', 'current-result'));
  if (row.evidence) body.append(externalLink('Source / evidence ↗', row.evidence));
  const previous = previousResult(row); if (previous) body.append(previous);
  d.append(body); return d;
}
function matches(row) {
  const query = $('search').value.trim().toLowerCase();
  const stateMatch = filter === 'all' || (filter === 'open' ? row.state !== 'verified' : row.state === filter);
  return stateMatch && (!query || [row.title, row.detail, row.doneWhen, row.category, ...(row.observed?.cases || []).map(c=>c.name)].join(' ').toLowerCase().includes(query));
}
function renderChecklist() {
  const c = current.checklist;
  let shown = 0;
  for (const [id,rows] of [['acceptance',c.acceptance], ['gaps',c.gaps]]) {
    const selected = rows.filter(matches); shown += selected.length;
    $(id).replaceChildren(...selected.map(r=>rowElement(r,false)));
    if (!selected.length) $(id).append(element('p','No items match this filter.','empty'));
    keepDetails($(id));
  }
  const suites = c.suites.map(s => {
    const selected = s.items.filter(matches); shown += selected.length;
    if (!selected.length) return null;
    const d = element('details', undefined, 'suite'); d.dataset.key = 'suite:'+s.id;
    const summary = element('summary');
    const heading = element('div'); heading.append(element('h3',s.title), element('p',s.total+' '+s.unit+' in the full inventory','suite-unit'));
    const number = element('div',undefined,'suite-number'); number.append(element('strong',s.counts.verified+' / '+s.total),element('span','verified for current inputs'));
    summary.append(heading, number, element('span','+','disclosure')); d.append(summary);
    const body = element('div',undefined,'suite-body'); body.append(element('p',s.detail,'suite-description'));
    const pills = element('div',undefined,'suite-counts'); STATES.forEach(state=>{if(s.counts[state])pills.append(badge(state,s.counts[state]+' '+LABELS[state].toLowerCase()));}); body.append(pills);
    if (s.observedCounts) {
      const n = s.observedCounts;
      body.append(element('p','Earlier full run · compiler '+(s.observedSource || '').slice(0,8)+': '+(n.passed||0)+' passed, '+(n.failed||0)+' failed, '+(n.running||0)+' running, '+(n.pending||0)+' pending. These observations do not count as a current pass.','previous-run'));
    }
    if (selected.length !== s.total) body.append(element('p',selected.length+' matching '+s.unit+' shown.','footnote'));
    body.append(...selected.map(r=>rowElement(r,true))); d.append(body);
    if ($('search').value.trim() || !['all','open'].includes(filter)) d.open = true;
    return d;
  }).filter(Boolean);
  $('suites').replaceChildren(...suites);
  if (!suites.length) $('suites').append(element('p','No test entries match this filter.','empty'));
  keepDetails($('suites'));
  $('filter-result').textContent = shown+' of '+c.total+' checklist entries shown · '+c.suites.length+' test inventories · entries have different scopes and are not a pass-rate percentage.';
  $('filters').replaceChildren(...['all','open',...STATES].map(state => {
    const count = state === 'all' ? c.total : state === 'open' ? c.total-c.counts.verified : c.counts[state];
    const button = element('button',(state === 'all' ? 'All' : state === 'open' ? 'Outstanding' : LABELS[state])+' '+count,'filter '+state);
    button.type='button'; button.dataset.filter=state; button.setAttribute('aria-pressed',String(filter===state)); return button;
  }));
}
function render(data) {
  current = validate(data);
  const c = data.checklist;
  $('signed-off').textContent = c.acceptanceVerified;
  $('acceptance-track').replaceChildren(...c.acceptance.map((r,i)=>{
    const a = element('a', (i+1)+'. '+r.title.replace(/^\d+\. /,''), 'gate '+r.state); a.href='#requirements'; a.title=LABELS[r.state]; return a;
  }));
  const older = c.acceptance.filter(r=>r.observed?.status==='passed' && r.state!=='verified').length;
  $('chain-context').textContent = older ? 'An earlier compiler proved '+older+' of the five requirements, including the four-stage bootstrap chain. The latest compiler needs a fresh matching chain and complete passing suites before final sign-off.' : 'Final sign-off requires all five acceptance requirements on the same compiler inputs. Open each requirement below for its exact completion condition.';
  for (const key of ['status','summary','activity']) $(key).textContent = data[key];
  $('cinch-revision').textContent = c.target.compiler?.slice(0,12) || 'unknown';
  $('native-revision').textContent = c.target.native?.slice(0,12) || 'unknown';
  $('method-note').textContent = c.explanation;
  $('active').replaceChildren(...c.active.map(a=>{
    const card=element('article',undefined,'active-card');
    const terminal=/^(passed|failed|completed)/.test(a.status || '');
    const label=a.earlier?'Earlier compiler · running':terminal&&a.failed?'Failures recorded':terminal?'Execution finished':a.status?.startsWith('waiting')?'Queued':'Validation running';
    card.append(element('span',label,'eyebrow'),element('h3',a.title),element('strong',a.passed+' / '+a.total),element('p','complete groups passed. '+(a.failed ? a.failed+' failed. ' : '')+(a.auditPending?'Independent audit and publication pending.':'Current phase: '+(a.phase || 'queued')))); return card;
  }));
  if (!c.active.length) $('active').append(element('p','No active validation reported.'));
  renderChecklist();
  $('checks').replaceChildren(...data.checks.map(c=>{
    const state = c.state === 'passed' ? 'verified' : c.state === 'failed' || c.state === 'blocked' ? 'blocked' : c.state === 'running' ? 'running' : 'pending';
    const row=element('article',undefined,'evidence-check '+state);row.append(badge(state),element('h3',c.title),element('p',c.detail));return row;
  }));
  $('milestones').replaceChildren(...data.milestones.map(m=>{const li=element('li');li.append(element('h3',m.title),element('p',m.detail));return li;}));
  keepDetails(document);
  $('commit').textContent=data.commit.slice(0,7);$('commit').href='https://github.com/iand675/cinch/commit/'+data.commit;
  updateFreshness();
}
function updateFreshness() {
  if (!current) return;
  const date=new Date(current.updatedAt), hours=Math.max(0,(Date.now()-date)/3600000);
  $('updated').dateTime=date.toISOString();$('updated').textContent=new Intl.DateTimeFormat(undefined,{dateStyle:'medium',timeStyle:'short'}).format(date);
  $('connection').classList.toggle('warning',!live||hours>6);
  $('connection').textContent=!live?'Live refresh unavailable · saved checkpoint':hours>6?'No update published in over 6 hours':(LOCAL?'Local evidence connected':'GitHub evidence connected')+' · refreshes every 30s';
  $('checked').textContent=lastChecked?'Last successful check: '+lastChecked.toLocaleTimeString()+'. Evidence time changes only when the checkpoint changes.':'Showing the bundled checkpoint while connecting.';
}
async function refresh() {
  if(busy)return;busy=true;$('refresh').disabled=true;
  try {
    const response=await fetch(SOURCE+'?t='+Date.now(),{cache:'no-store',signal:AbortSignal.timeout(12000)});
    if(!response.ok)throw new Error('Status fetch failed');
    const data=validate(await response.json());
    if(JSON.stringify(data)!==JSON.stringify(current))render(data);
    live=true;lastChecked=new Date();
    try{localStorage.setItem('cinch-bootstrap-checklist-v2',JSON.stringify(data));}catch{}
  }catch{live=false;}
  finally{busy=false;$('refresh').disabled=false;updateFreshness();}
}
document.addEventListener('toggle',event=>{
  const d=event.target;if(!d.matches?.('details[data-key]')||!d.isConnected)return;
  if(d.open){expanded.add(d.dataset.key);collapsed.delete(d.dataset.key);}else{expanded.delete(d.dataset.key);collapsed.add(d.dataset.key);}
},true);
$('search').addEventListener('input',renderChecklist);
$('filters').addEventListener('click',event=>{const button=event.target.closest('button[data-filter]');if(button){filter=button.dataset.filter;renderChecklist();}});
$('expand-all').addEventListener('click',()=>document.querySelectorAll('#acceptance details,#gaps details,#suites details.suite').forEach(d=>{d.open=true;expanded.add(d.dataset.key);collapsed.delete(d.dataset.key);}));
$('collapse-all').addEventListener('click',()=>document.querySelectorAll('main details').forEach(d=>{d.open=false;expanded.delete(d.dataset.key);collapsed.add(d.dataset.key);}));
try{render(window.CINCH_SNAPSHOT);}catch{$('status').textContent='Checklist unavailable';$('summary').textContent='The saved status is incomplete. Refresh to retrieve the full checklist.';}
try{const cached=validate(JSON.parse(localStorage.getItem('cinch-bootstrap-checklist-v2')));if(!current||Date.parse(cached.updatedAt)>Date.parse(current.updatedAt))render(cached);}catch{}
$('refresh').addEventListener('click',refresh);
document.addEventListener('visibilitychange',()=>{if(!document.hidden)refresh();});
setInterval(()=>{if(!document.hidden)refresh();},30000);
refresh();

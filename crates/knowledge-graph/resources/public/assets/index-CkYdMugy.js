import{r as e,t}from"./rolldown-runtime-QTnfLwEv.js";import{a as n,i as r,n as i,r as a,t as o}from"./vendor-json-view-XsbV5yWD.js";import{a as s,c,d as l,f as u,i as d,l as f,n as p,o as m,r as h,s as g,t as _,u as v}from"./vendor-xyflow-CZnfePCo.js";import{n as y,r as b,t as x}from"./vendor-markdown-Di8s8vbi.js";import{i as S,n as C,r as ee,t as te}from"./vendor-panels-CXf4xYpQ.js";(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();var ne=t((e=>{function t(e,t){var n=e.length;e.push(t);a:for(;0<n;){var r=n-1>>>1,a=e[r];if(0<i(a,t))e[r]=t,e[n]=a,n=r;else break a}}function n(e){return e.length===0?null:e[0]}function r(e){if(e.length===0)return null;var t=e[0],n=e.pop();if(n!==t){e[0]=n;a:for(var r=0,a=e.length,o=a>>>1;r<o;){var s=2*(r+1)-1,c=e[s],l=s+1,u=e[l];if(0>i(c,n))l<a&&0>i(u,c)?(e[r]=u,e[l]=n,r=l):(e[r]=c,e[s]=n,r=s);else if(l<a&&0>i(u,n))e[r]=u,e[l]=n,r=l;else break a}}return t}function i(e,t){var n=e.sortIndex-t.sortIndex;return n===0?e.id-t.id:n}if(e.unstable_now=void 0,typeof performance==`object`&&typeof performance.now==`function`){var a=performance;e.unstable_now=function(){return a.now()}}else{var o=Date,s=o.now();e.unstable_now=function(){return o.now()-s}}var c=[],l=[],u=1,d=null,f=3,p=!1,m=!1,h=!1,g=!1,_=typeof setTimeout==`function`?setTimeout:null,v=typeof clearTimeout==`function`?clearTimeout:null,y=typeof setImmediate<`u`?setImmediate:null;function b(e){for(var i=n(l);i!==null;){if(i.callback===null)r(l);else if(i.startTime<=e)r(l),i.sortIndex=i.expirationTime,t(c,i);else break;i=n(l)}}function x(e){if(h=!1,b(e),!m)if(n(c)!==null)m=!0,S||(S=!0,T());else{var t=n(l);t!==null&&ie(x,t.startTime-e)}}var S=!1,C=-1,ee=5,te=-1;function ne(){return g?!0:!(e.unstable_now()-te<ee)}function w(){if(g=!1,S){var t=e.unstable_now();te=t;var i=!0;try{a:{m=!1,h&&(h=!1,v(C),C=-1),p=!0;var a=f;try{b:{for(b(t),d=n(c);d!==null&&!(d.expirationTime>t&&ne());){var o=d.callback;if(typeof o==`function`){d.callback=null,f=d.priorityLevel;var s=o(d.expirationTime<=t);if(t=e.unstable_now(),typeof s==`function`){d.callback=s,b(t),i=!0;break b}d===n(c)&&r(c),b(t)}else r(c);d=n(c)}if(d!==null)i=!0;else{var u=n(l);u!==null&&ie(x,u.startTime-t),i=!1}}break a}finally{d=null,f=a,p=!1}i=void 0}}finally{i?T():S=!1}}}var T;if(typeof y==`function`)T=function(){y(w)};else if(typeof MessageChannel<`u`){var re=new MessageChannel,E=re.port2;re.port1.onmessage=w,T=function(){E.postMessage(null)}}else T=function(){_(w,0)};function ie(t,n){C=_(function(){t(e.unstable_now())},n)}e.unstable_IdlePriority=5,e.unstable_ImmediatePriority=1,e.unstable_LowPriority=4,e.unstable_NormalPriority=3,e.unstable_Profiling=null,e.unstable_UserBlockingPriority=2,e.unstable_cancelCallback=function(e){e.callback=null},e.unstable_forceFrameRate=function(e){0>e||125<e?console.error(`forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported`):ee=0<e?Math.floor(1e3/e):5},e.unstable_getCurrentPriorityLevel=function(){return f},e.unstable_next=function(e){switch(f){case 1:case 2:case 3:var t=3;break;default:t=f}var n=f;f=t;try{return e()}finally{f=n}},e.unstable_requestPaint=function(){g=!0},e.unstable_runWithPriority=function(e,t){switch(e){case 1:case 2:case 3:case 4:case 5:break;default:e=3}var n=f;f=e;try{return t()}finally{f=n}},e.unstable_scheduleCallback=function(r,i,a){var o=e.unstable_now();switch(typeof a==`object`&&a?(a=a.delay,a=typeof a==`number`&&0<a?o+a:o):a=o,r){case 1:var s=-1;break;case 2:s=250;break;case 5:s=1073741823;break;case 4:s=1e4;break;default:s=5e3}return s=a+s,r={id:u++,callback:i,priorityLevel:r,startTime:a,expirationTime:s,sortIndex:-1},a>o?(r.sortIndex=a,t(l,r),n(c)===null&&r===n(l)&&(h?(v(C),C=-1):h=!0,ie(x,a-o))):(r.sortIndex=s,t(c,r),m||p||(m=!0,S||(S=!0,T()))),r},e.unstable_shouldYield=ne,e.unstable_wrapCallback=function(e){var t=f;return function(){var n=f;f=t;try{return e.apply(this,arguments)}finally{f=n}}}})),w=t(((e,t)=>{t.exports=ne()})),T=t((e=>{var t=w(),r=n(),i=u();function a(e){var t=`https://react.dev/errors/`+e;if(1<arguments.length){t+=`?args[]=`+encodeURIComponent(arguments[1]);for(var n=2;n<arguments.length;n++)t+=`&args[]=`+encodeURIComponent(arguments[n])}return`Minified React error #`+e+`; visit `+t+` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`}function o(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function s(e){var t=e,n=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,t.flags&4098&&(n=t.return),e=t.return;while(e)}return t.tag===3?n:null}function c(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function l(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function d(e){if(s(e)!==e)throw Error(a(188))}function f(e){var t=e.alternate;if(!t){if(t=s(e),t===null)throw Error(a(188));return t===e?e:null}for(var n=e,r=t;;){var i=n.return;if(i===null)break;var o=i.alternate;if(o===null){if(r=i.return,r!==null){n=r;continue}break}if(i.child===o.child){for(o=i.child;o;){if(o===n)return d(i),e;if(o===r)return d(i),t;o=o.sibling}throw Error(a(188))}if(n.return!==r.return)n=i,r=o;else{for(var c=!1,l=i.child;l;){if(l===n){c=!0,n=i,r=o;break}if(l===r){c=!0,r=i,n=o;break}l=l.sibling}if(!c){for(l=o.child;l;){if(l===n){c=!0,n=o,r=i;break}if(l===r){c=!0,r=o,n=i;break}l=l.sibling}if(!c)throw Error(a(189))}}if(n.alternate!==r)throw Error(a(190))}if(n.tag!==3)throw Error(a(188));return n.stateNode.current===n?e:t}function p(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=p(e),t!==null)return t;e=e.sibling}return null}var m=Object.assign,h=Symbol.for(`react.element`),g=Symbol.for(`react.transitional.element`),_=Symbol.for(`react.portal`),v=Symbol.for(`react.fragment`),y=Symbol.for(`react.strict_mode`),b=Symbol.for(`react.profiler`),x=Symbol.for(`react.consumer`),S=Symbol.for(`react.context`),C=Symbol.for(`react.forward_ref`),ee=Symbol.for(`react.suspense`),te=Symbol.for(`react.suspense_list`),ne=Symbol.for(`react.memo`),T=Symbol.for(`react.lazy`),re=Symbol.for(`react.activity`),E=Symbol.for(`react.memo_cache_sentinel`),ie=Symbol.iterator;function ae(e){return typeof e!=`object`||!e?null:(e=ie&&e[ie]||e[`@@iterator`],typeof e==`function`?e:null)}var oe=Symbol.for(`react.client.reference`);function D(e){if(e==null)return null;if(typeof e==`function`)return e.$$typeof===oe?null:e.displayName||e.name||null;if(typeof e==`string`)return e;switch(e){case v:return`Fragment`;case b:return`Profiler`;case y:return`StrictMode`;case ee:return`Suspense`;case te:return`SuspenseList`;case re:return`Activity`}if(typeof e==`object`)switch(e.$$typeof){case _:return`Portal`;case S:return e.displayName||`Context`;case x:return(e._context.displayName||`Context`)+`.Consumer`;case C:var t=e.render;return e=e.displayName,e||=(e=t.displayName||t.name||``,e===``?`ForwardRef`:`ForwardRef(`+e+`)`),e;case ne:return t=e.displayName||null,t===null?D(e.type)||`Memo`:t;case T:t=e._payload,e=e._init;try{return D(e(t))}catch{}}return null}var se=Array.isArray,O=r.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,k=i.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,ce={pending:!1,data:null,method:null,action:null},le=[],ue=-1;function A(e){return{current:e}}function j(e){0>ue||(e.current=le[ue],le[ue]=null,ue--)}function M(e,t){ue++,le[ue]=e.current,e.current=t}var de=A(null),fe=A(null),pe=A(null),me=A(null);function he(e,t){switch(M(pe,t),M(fe,e),M(de,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?Vd(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=Vd(t),e=Hd(t,e);else switch(e){case`svg`:e=1;break;case`math`:e=2;break;default:e=0}}j(de),M(de,e)}function ge(){j(de),j(fe),j(pe)}function _e(e){e.memoizedState!==null&&M(me,e);var t=de.current,n=Hd(t,e.type);t!==n&&(M(fe,e),M(de,n))}function ve(e){fe.current===e&&(j(de),j(fe)),me.current===e&&(j(me),Qf._currentValue=ce)}var ye,be;function xe(e){if(ye===void 0)try{throw Error()}catch(e){var t=e.stack.trim().match(/\n( *(at )?)/);ye=t&&t[1]||``,be=-1<e.stack.indexOf(`
    at`)?` (<anonymous>)`:-1<e.stack.indexOf(`@`)?`@unknown:0:0`:``}return`
`+ye+e+be}var Se=!1;function Ce(e,t){if(!e||Se)return``;Se=!0;var n=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var r={DetermineComponentFrameRoot:function(){try{if(t){var n=function(){throw Error()};if(Object.defineProperty(n.prototype,"props",{set:function(){throw Error()}}),typeof Reflect==`object`&&Reflect.construct){try{Reflect.construct(n,[])}catch(e){var r=e}Reflect.construct(e,[],n)}else{try{n.call()}catch(e){r=e}e.call(n.prototype)}}else{try{throw Error()}catch(e){r=e}(n=e())&&typeof n.catch==`function`&&n.catch(function(){})}}catch(e){if(e&&r&&typeof e.stack==`string`)return[e.stack,r.stack]}return[null,null]}};r.DetermineComponentFrameRoot.displayName=`DetermineComponentFrameRoot`;var i=Object.getOwnPropertyDescriptor(r.DetermineComponentFrameRoot,`name`);i&&i.configurable&&Object.defineProperty(r.DetermineComponentFrameRoot,"name",{value:`DetermineComponentFrameRoot`});var a=r.DetermineComponentFrameRoot(),o=a[0],s=a[1];if(o&&s){var c=o.split(`
`),l=s.split(`
`);for(i=r=0;r<c.length&&!c[r].includes(`DetermineComponentFrameRoot`);)r++;for(;i<l.length&&!l[i].includes(`DetermineComponentFrameRoot`);)i++;if(r===c.length||i===l.length)for(r=c.length-1,i=l.length-1;1<=r&&0<=i&&c[r]!==l[i];)i--;for(;1<=r&&0<=i;r--,i--)if(c[r]!==l[i]){if(r!==1||i!==1)do if(r--,i--,0>i||c[r]!==l[i]){var u=`
`+c[r].replace(` at new `,` at `);return e.displayName&&u.includes(`<anonymous>`)&&(u=u.replace(`<anonymous>`,e.displayName)),u}while(1<=r&&0<=i);break}}}finally{Se=!1,Error.prepareStackTrace=n}return(n=e?e.displayName||e.name:``)?xe(n):``}function we(e,t){switch(e.tag){case 26:case 27:case 5:return xe(e.type);case 16:return xe(`Lazy`);case 13:return e.child!==t&&t!==null?xe(`Suspense Fallback`):xe(`Suspense`);case 19:return xe(`SuspenseList`);case 0:case 15:return Ce(e.type,!1);case 11:return Ce(e.type.render,!1);case 1:return Ce(e.type,!0);case 31:return xe(`Activity`);default:return``}}function Te(e){try{var t=``,n=null;do t+=we(e,n),n=e,e=e.return;while(e);return t}catch(e){return`
Error generating stack: `+e.message+`
`+e.stack}}var Ee=Object.prototype.hasOwnProperty,De=t.unstable_scheduleCallback,Oe=t.unstable_cancelCallback,ke=t.unstable_shouldYield,Ae=t.unstable_requestPaint,je=t.unstable_now,Me=t.unstable_getCurrentPriorityLevel,Ne=t.unstable_ImmediatePriority,Pe=t.unstable_UserBlockingPriority,Fe=t.unstable_NormalPriority,Ie=t.unstable_LowPriority,Le=t.unstable_IdlePriority,Re=t.log,ze=t.unstable_setDisableYieldValue,Be=null,Ve=null;function He(e){if(typeof Re==`function`&&ze(e),Ve&&typeof Ve.setStrictMode==`function`)try{Ve.setStrictMode(Be,e)}catch{}}var Ue=Math.clz32?Math.clz32:Ke,We=Math.log,Ge=Math.LN2;function Ke(e){return e>>>=0,e===0?32:31-(We(e)/Ge|0)|0}var qe=256,Je=262144,Ye=4194304;function Xe(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function Ze(e,t,n){var r=e.pendingLanes;if(r===0)return 0;var i=0,a=e.suspendedLanes,o=e.pingedLanes;e=e.warmLanes;var s=r&134217727;return s===0?(s=r&~a,s===0?o===0?n||(n=r&~e,n!==0&&(i=Xe(n))):i=Xe(o):i=Xe(s)):(r=s&~a,r===0?(o&=s,o===0?n||(n=s&~e,n!==0&&(i=Xe(n))):i=Xe(o)):i=Xe(r)),i===0?0:t!==0&&t!==i&&(t&a)===0&&(a=i&-i,n=t&-t,a>=n||a===32&&n&4194048)?t:i}function Qe(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function $e(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function et(){var e=Ye;return Ye<<=1,!(Ye&62914560)&&(Ye=4194304),e}function tt(e){for(var t=[],n=0;31>n;n++)t.push(e);return t}function nt(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function rt(e,t,n,r,i,a){var o=e.pendingLanes;e.pendingLanes=n,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=n,e.entangledLanes&=n,e.errorRecoveryDisabledLanes&=n,e.shellSuspendCounter=0;var s=e.entanglements,c=e.expirationTimes,l=e.hiddenUpdates;for(n=o&~n;0<n;){var u=31-Ue(n),d=1<<u;s[u]=0,c[u]=-1;var f=l[u];if(f!==null)for(l[u]=null,u=0;u<f.length;u++){var p=f[u];p!==null&&(p.lane&=-536870913)}n&=~d}r!==0&&it(e,r,0),a!==0&&i===0&&e.tag!==0&&(e.suspendedLanes|=a&~(o&~t))}function it(e,t,n){e.pendingLanes|=t,e.suspendedLanes&=~t;var r=31-Ue(t);e.entangledLanes|=t,e.entanglements[r]=e.entanglements[r]|1073741824|n&261930}function at(e,t){var n=e.entangledLanes|=t;for(e=e.entanglements;n;){var r=31-Ue(n),i=1<<r;i&t|e[r]&t&&(e[r]|=t),n&=~i}}function ot(e,t){var n=t&-t;return n=n&42?1:st(n),(n&(e.suspendedLanes|t))===0?n:0}function st(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function ct(e){return e&=-e,2<e?8<e?e&134217727?32:268435456:8:2}function lt(){var e=k.p;return e===0?(e=window.event,e===void 0?32:mp(e.type)):e}function ut(e,t){var n=k.p;try{return k.p=e,t()}finally{k.p=n}}var dt=Math.random().toString(36).slice(2),ft=`__reactFiber$`+dt,N=`__reactProps$`+dt,pt=`__reactContainer$`+dt,mt=`__reactEvents$`+dt,ht=`__reactListeners$`+dt,gt=`__reactHandles$`+dt,_t=`__reactResources$`+dt,vt=`__reactMarker$`+dt;function yt(e){delete e[ft],delete e[N],delete e[mt],delete e[ht],delete e[gt]}function bt(e){var t=e[ft];if(t)return t;for(var n=e.parentNode;n;){if(t=n[pt]||n[ft]){if(n=t.alternate,t.child!==null||n!==null&&n.child!==null)for(e=df(e);e!==null;){if(n=e[ft])return n;e=df(e)}return t}e=n,n=e.parentNode}return null}function xt(e){if(e=e[ft]||e[pt]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function St(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(a(33))}function Ct(e){var t=e[_t];return t||=e[_t]={hoistableStyles:new Map,hoistableScripts:new Map},t}function wt(e){e[vt]=!0}var Tt=new Set,Et={};function Dt(e,t){Ot(e,t),Ot(e+`Capture`,t)}function Ot(e,t){for(Et[e]=t,e=0;e<t.length;e++)Tt.add(t[e])}var kt=RegExp(`^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`),At={},jt={};function Mt(e){return Ee.call(jt,e)?!0:Ee.call(At,e)?!1:kt.test(e)?jt[e]=!0:(At[e]=!0,!1)}function Nt(e,t,n){if(Mt(t))if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:e.removeAttribute(t);return;case`boolean`:var r=t.toLowerCase().slice(0,5);if(r!==`data-`&&r!==`aria-`){e.removeAttribute(t);return}}e.setAttribute(t,``+n)}}function Pt(e,t,n){if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(t);return}e.setAttribute(t,``+n)}}function Ft(e,t,n,r){if(r===null)e.removeAttribute(n);else{switch(typeof r){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(n);return}e.setAttributeNS(t,n,``+r)}}function It(e){switch(typeof e){case`bigint`:case`boolean`:case`number`:case`string`:case`undefined`:return e;case`object`:return e;default:return``}}function Lt(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()===`input`&&(t===`checkbox`||t===`radio`)}function Rt(e,t,n){var r=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&r!==void 0&&typeof r.get==`function`&&typeof r.set==`function`){var i=r.get,a=r.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return i.call(this)},set:function(e){n=``+e,a.call(this,e)}}),Object.defineProperty(e,t,{enumerable:r.enumerable}),{getValue:function(){return n},setValue:function(e){n=``+e},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function zt(e){if(!e._valueTracker){var t=Lt(e)?`checked`:`value`;e._valueTracker=Rt(e,t,``+e[t])}}function Bt(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var n=t.getValue(),r=``;return e&&(r=Lt(e)?e.checked?`true`:`false`:e.value),e=r,e===n?!1:(t.setValue(e),!0)}function Vt(e){if(e||=typeof document<`u`?document:void 0,e===void 0)return null;try{return e.activeElement||e.body}catch{return e.body}}var Ht=/[\n"\\]/g;function Ut(e){return e.replace(Ht,function(e){return`\\`+e.charCodeAt(0).toString(16)+` `})}function Wt(e,t,n,r,i,a,o,s){e.name=``,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`?e.type=o:e.removeAttribute(`type`),t==null?o!==`submit`&&o!==`reset`||e.removeAttribute(`value`):o===`number`?(t===0&&e.value===``||e.value!=t)&&(e.value=``+It(t)):e.value!==``+It(t)&&(e.value=``+It(t)),t==null?n==null?r!=null&&e.removeAttribute(`value`):Kt(e,o,It(n)):Kt(e,o,It(t)),i==null&&a!=null&&(e.defaultChecked=!!a),i!=null&&(e.checked=i&&typeof i!=`function`&&typeof i!=`symbol`),s!=null&&typeof s!=`function`&&typeof s!=`symbol`&&typeof s!=`boolean`?e.name=``+It(s):e.removeAttribute(`name`)}function Gt(e,t,n,r,i,a,o,s){if(a!=null&&typeof a!=`function`&&typeof a!=`symbol`&&typeof a!=`boolean`&&(e.type=a),t!=null||n!=null){if(!(a!==`submit`&&a!==`reset`||t!=null)){zt(e);return}n=n==null?``:``+It(n),t=t==null?n:``+It(t),s||t===e.value||(e.value=t),e.defaultValue=t}r??=i,r=typeof r!=`function`&&typeof r!=`symbol`&&!!r,e.checked=s?e.checked:!!r,e.defaultChecked=!!r,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`&&(e.name=o),zt(e)}function Kt(e,t,n){t===`number`&&Vt(e.ownerDocument)===e||e.defaultValue===``+n||(e.defaultValue=``+n)}function qt(e,t,n,r){if(e=e.options,t){t={};for(var i=0;i<n.length;i++)t[`$`+n[i]]=!0;for(n=0;n<e.length;n++)i=t.hasOwnProperty(`$`+e[n].value),e[n].selected!==i&&(e[n].selected=i),i&&r&&(e[n].defaultSelected=!0)}else{for(n=``+It(n),t=null,i=0;i<e.length;i++){if(e[i].value===n){e[i].selected=!0,r&&(e[i].defaultSelected=!0);return}t!==null||e[i].disabled||(t=e[i])}t!==null&&(t.selected=!0)}}function Jt(e,t,n){if(t!=null&&(t=``+It(t),t!==e.value&&(e.value=t),n==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=n==null?``:``+It(n)}function Yt(e,t,n,r){if(t==null){if(r!=null){if(n!=null)throw Error(a(92));if(se(r)){if(1<r.length)throw Error(a(93));r=r[0]}n=r}n??=``,t=n}n=It(t),e.defaultValue=n,r=e.textContent,r===n&&r!==``&&r!==null&&(e.value=r),zt(e)}function Xt(e,t){if(t){var n=e.firstChild;if(n&&n===e.lastChild&&n.nodeType===3){n.nodeValue=t;return}}e.textContent=t}var Zt=new Set(`animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(` `));function Qt(e,t,n){var r=t.indexOf(`--`)===0;n==null||typeof n==`boolean`||n===``?r?e.setProperty(t,``):t===`float`?e.cssFloat=``:e[t]=``:r?e.setProperty(t,n):typeof n!=`number`||n===0||Zt.has(t)?t===`float`?e.cssFloat=n:e[t]=(``+n).trim():e[t]=n+`px`}function $t(e,t,n){if(t!=null&&typeof t!=`object`)throw Error(a(62));if(e=e.style,n!=null){for(var r in n)!n.hasOwnProperty(r)||t!=null&&t.hasOwnProperty(r)||(r.indexOf(`--`)===0?e.setProperty(r,``):r===`float`?e.cssFloat=``:e[r]=``);for(var i in t)r=t[i],t.hasOwnProperty(i)&&n[i]!==r&&Qt(e,i,r)}else for(var o in t)t.hasOwnProperty(o)&&Qt(e,o,t[o])}function en(e){if(e.indexOf(`-`)===-1)return!1;switch(e){case`annotation-xml`:case`color-profile`:case`font-face`:case`font-face-src`:case`font-face-uri`:case`font-face-format`:case`font-face-name`:case`missing-glyph`:return!1;default:return!0}}var tn=new Map([[`acceptCharset`,`accept-charset`],[`htmlFor`,`for`],[`httpEquiv`,`http-equiv`],[`crossOrigin`,`crossorigin`],[`accentHeight`,`accent-height`],[`alignmentBaseline`,`alignment-baseline`],[`arabicForm`,`arabic-form`],[`baselineShift`,`baseline-shift`],[`capHeight`,`cap-height`],[`clipPath`,`clip-path`],[`clipRule`,`clip-rule`],[`colorInterpolation`,`color-interpolation`],[`colorInterpolationFilters`,`color-interpolation-filters`],[`colorProfile`,`color-profile`],[`colorRendering`,`color-rendering`],[`dominantBaseline`,`dominant-baseline`],[`enableBackground`,`enable-background`],[`fillOpacity`,`fill-opacity`],[`fillRule`,`fill-rule`],[`floodColor`,`flood-color`],[`floodOpacity`,`flood-opacity`],[`fontFamily`,`font-family`],[`fontSize`,`font-size`],[`fontSizeAdjust`,`font-size-adjust`],[`fontStretch`,`font-stretch`],[`fontStyle`,`font-style`],[`fontVariant`,`font-variant`],[`fontWeight`,`font-weight`],[`glyphName`,`glyph-name`],[`glyphOrientationHorizontal`,`glyph-orientation-horizontal`],[`glyphOrientationVertical`,`glyph-orientation-vertical`],[`horizAdvX`,`horiz-adv-x`],[`horizOriginX`,`horiz-origin-x`],[`imageRendering`,`image-rendering`],[`letterSpacing`,`letter-spacing`],[`lightingColor`,`lighting-color`],[`markerEnd`,`marker-end`],[`markerMid`,`marker-mid`],[`markerStart`,`marker-start`],[`overlinePosition`,`overline-position`],[`overlineThickness`,`overline-thickness`],[`paintOrder`,`paint-order`],[`panose-1`,`panose-1`],[`pointerEvents`,`pointer-events`],[`renderingIntent`,`rendering-intent`],[`shapeRendering`,`shape-rendering`],[`stopColor`,`stop-color`],[`stopOpacity`,`stop-opacity`],[`strikethroughPosition`,`strikethrough-position`],[`strikethroughThickness`,`strikethrough-thickness`],[`strokeDasharray`,`stroke-dasharray`],[`strokeDashoffset`,`stroke-dashoffset`],[`strokeLinecap`,`stroke-linecap`],[`strokeLinejoin`,`stroke-linejoin`],[`strokeMiterlimit`,`stroke-miterlimit`],[`strokeOpacity`,`stroke-opacity`],[`strokeWidth`,`stroke-width`],[`textAnchor`,`text-anchor`],[`textDecoration`,`text-decoration`],[`textRendering`,`text-rendering`],[`transformOrigin`,`transform-origin`],[`underlinePosition`,`underline-position`],[`underlineThickness`,`underline-thickness`],[`unicodeBidi`,`unicode-bidi`],[`unicodeRange`,`unicode-range`],[`unitsPerEm`,`units-per-em`],[`vAlphabetic`,`v-alphabetic`],[`vHanging`,`v-hanging`],[`vIdeographic`,`v-ideographic`],[`vMathematical`,`v-mathematical`],[`vectorEffect`,`vector-effect`],[`vertAdvY`,`vert-adv-y`],[`vertOriginX`,`vert-origin-x`],[`vertOriginY`,`vert-origin-y`],[`wordSpacing`,`word-spacing`],[`writingMode`,`writing-mode`],[`xmlnsXlink`,`xmlns:xlink`],[`xHeight`,`x-height`]]),nn=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function rn(e){return nn.test(``+e)?`javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`:e}function an(){}var on=null;function sn(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var cn=null,ln=null;function un(e){var t=xt(e);if(t&&(e=t.stateNode)){var n=e[N]||null;a:switch(e=t.stateNode,t.type){case`input`:if(Wt(e,n.value,n.defaultValue,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name),t=n.name,n.type===`radio`&&t!=null){for(n=e;n.parentNode;)n=n.parentNode;for(n=n.querySelectorAll(`input[name="`+Ut(``+t)+`"][type="radio"]`),t=0;t<n.length;t++){var r=n[t];if(r!==e&&r.form===e.form){var i=r[N]||null;if(!i)throw Error(a(90));Wt(r,i.value,i.defaultValue,i.defaultValue,i.checked,i.defaultChecked,i.type,i.name)}}for(t=0;t<n.length;t++)r=n[t],r.form===e.form&&Bt(r)}break a;case`textarea`:Jt(e,n.value,n.defaultValue);break a;case`select`:t=n.value,t!=null&&qt(e,!!n.multiple,t,!1)}}}var dn=!1;function fn(e,t,n){if(dn)return e(t,n);dn=!0;try{return e(t)}finally{if(dn=!1,(cn!==null||ln!==null)&&(bu(),cn&&(t=cn,e=ln,ln=cn=null,un(t),e)))for(t=0;t<e.length;t++)un(e[t])}}function pn(e,t){var n=e.stateNode;if(n===null)return null;var r=n[N]||null;if(r===null)return null;n=r[t];a:switch(t){case`onClick`:case`onClickCapture`:case`onDoubleClick`:case`onDoubleClickCapture`:case`onMouseDown`:case`onMouseDownCapture`:case`onMouseMove`:case`onMouseMoveCapture`:case`onMouseUp`:case`onMouseUpCapture`:case`onMouseEnter`:(r=!r.disabled)||(e=e.type,r=!(e===`button`||e===`input`||e===`select`||e===`textarea`)),e=!r;break a;default:e=!1}if(e)return null;if(n&&typeof n!=`function`)throw Error(a(231,t,typeof n));return n}var mn=!(typeof window>`u`||window.document===void 0||window.document.createElement===void 0),hn=!1;if(mn)try{var gn={};Object.defineProperty(gn,"passive",{get:function(){hn=!0}}),window.addEventListener(`test`,gn,gn),window.removeEventListener(`test`,gn,gn)}catch{hn=!1}var _n=null,vn=null,yn=null;function bn(){if(yn)return yn;var e,t=vn,n=t.length,r,i=`value`in _n?_n.value:_n.textContent,a=i.length;for(e=0;e<n&&t[e]===i[e];e++);var o=n-e;for(r=1;r<=o&&t[n-r]===i[a-r];r++);return yn=i.slice(e,1<r?1-r:void 0)}function xn(e){var t=e.keyCode;return`charCode`in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function Sn(){return!0}function Cn(){return!1}function wn(e){function t(t,n,r,i,a){for(var o in this._reactName=t,this._targetInst=r,this.type=n,this.nativeEvent=i,this.target=a,this.currentTarget=null,e)e.hasOwnProperty(o)&&(t=e[o],this[o]=t?t(i):i[o]);return this.isDefaultPrevented=(i.defaultPrevented==null?!1===i.returnValue:i.defaultPrevented)?Sn:Cn,this.isPropagationStopped=Cn,this}return m(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var e=this.nativeEvent;e&&(e.preventDefault?e.preventDefault():typeof e.returnValue!=`unknown`&&(e.returnValue=!1),this.isDefaultPrevented=Sn)},stopPropagation:function(){var e=this.nativeEvent;e&&(e.stopPropagation?e.stopPropagation():typeof e.cancelBubble!=`unknown`&&(e.cancelBubble=!0),this.isPropagationStopped=Sn)},persist:function(){},isPersistent:Sn}),t}var Tn={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},En=wn(Tn),Dn=m({},Tn,{view:0,detail:0}),On=wn(Dn),kn,An,jn,Mn=m({},Dn,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Un,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return`movementX`in e?e.movementX:(e!==jn&&(jn&&e.type===`mousemove`?(kn=e.screenX-jn.screenX,An=e.screenY-jn.screenY):An=kn=0,jn=e),kn)},movementY:function(e){return`movementY`in e?e.movementY:An}}),Nn=wn(Mn),Pn=wn(m({},Mn,{dataTransfer:0})),Fn=wn(m({},Dn,{relatedTarget:0})),In=wn(m({},Tn,{animationName:0,elapsedTime:0,pseudoElement:0})),Ln=wn(m({},Tn,{clipboardData:function(e){return`clipboardData`in e?e.clipboardData:window.clipboardData}})),Rn=wn(m({},Tn,{data:0})),zn={Esc:`Escape`,Spacebar:` `,Left:`ArrowLeft`,Up:`ArrowUp`,Right:`ArrowRight`,Down:`ArrowDown`,Del:`Delete`,Win:`OS`,Menu:`ContextMenu`,Apps:`ContextMenu`,Scroll:`ScrollLock`,MozPrintableKey:`Unidentified`},Bn={8:`Backspace`,9:`Tab`,12:`Clear`,13:`Enter`,16:`Shift`,17:`Control`,18:`Alt`,19:`Pause`,20:`CapsLock`,27:`Escape`,32:` `,33:`PageUp`,34:`PageDown`,35:`End`,36:`Home`,37:`ArrowLeft`,38:`ArrowUp`,39:`ArrowRight`,40:`ArrowDown`,45:`Insert`,46:`Delete`,112:`F1`,113:`F2`,114:`F3`,115:`F4`,116:`F5`,117:`F6`,118:`F7`,119:`F8`,120:`F9`,121:`F10`,122:`F11`,123:`F12`,144:`NumLock`,145:`ScrollLock`,224:`Meta`},Vn={Alt:`altKey`,Control:`ctrlKey`,Meta:`metaKey`,Shift:`shiftKey`};function Hn(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=Vn[e])?!!t[e]:!1}function Un(){return Hn}var Wn=wn(m({},Dn,{key:function(e){if(e.key){var t=zn[e.key]||e.key;if(t!==`Unidentified`)return t}return e.type===`keypress`?(e=xn(e),e===13?`Enter`:String.fromCharCode(e)):e.type===`keydown`||e.type===`keyup`?Bn[e.keyCode]||`Unidentified`:``},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Un,charCode:function(e){return e.type===`keypress`?xn(e):0},keyCode:function(e){return e.type===`keydown`||e.type===`keyup`?e.keyCode:0},which:function(e){return e.type===`keypress`?xn(e):e.type===`keydown`||e.type===`keyup`?e.keyCode:0}})),Gn=wn(m({},Mn,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0})),Kn=wn(m({},Dn,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Un})),qn=wn(m({},Tn,{propertyName:0,elapsedTime:0,pseudoElement:0})),Jn=wn(m({},Mn,{deltaX:function(e){return`deltaX`in e?e.deltaX:`wheelDeltaX`in e?-e.wheelDeltaX:0},deltaY:function(e){return`deltaY`in e?e.deltaY:`wheelDeltaY`in e?-e.wheelDeltaY:`wheelDelta`in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0})),Yn=wn(m({},Tn,{newState:0,oldState:0})),Xn=[9,13,27,32],Zn=mn&&`CompositionEvent`in window,Qn=null;mn&&`documentMode`in document&&(Qn=document.documentMode);var $n=mn&&`TextEvent`in window&&!Qn,er=mn&&(!Zn||Qn&&8<Qn&&11>=Qn),tr=` `,nr=!1;function rr(e,t){switch(e){case`keyup`:return Xn.indexOf(t.keyCode)!==-1;case`keydown`:return t.keyCode!==229;case`keypress`:case`mousedown`:case`focusout`:return!0;default:return!1}}function ir(e){return e=e.detail,typeof e==`object`&&`data`in e?e.data:null}var ar=!1;function or(e,t){switch(e){case`compositionend`:return ir(t);case`keypress`:return t.which===32?(nr=!0,tr):null;case`textInput`:return e=t.data,e===tr&&nr?null:e;default:return null}}function sr(e,t){if(ar)return e===`compositionend`||!Zn&&rr(e,t)?(e=bn(),yn=vn=_n=null,ar=!1,e):null;switch(e){case`paste`:return null;case`keypress`:if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case`compositionend`:return er&&t.locale!==`ko`?null:t.data;default:return null}}var cr={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function lr(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t===`input`?!!cr[e.type]:t===`textarea`}function ur(e,t,n,r){cn?ln?ln.push(r):ln=[r]:cn=r,t=Ed(t,`onChange`),0<t.length&&(n=new En(`onChange`,`change`,null,n,r),e.push({event:n,listeners:t}))}var dr=null,fr=null;function pr(e){yd(e,0)}function mr(e){if(Bt(St(e)))return e}function hr(e,t){if(e===`change`)return t}var gr=!1;if(mn){var _r;if(mn){var vr=`oninput`in document;if(!vr){var yr=document.createElement(`div`);yr.setAttribute(`oninput`,`return;`),vr=typeof yr.oninput==`function`}_r=vr}else _r=!1;gr=_r&&(!document.documentMode||9<document.documentMode)}function P(){dr&&(dr.detachEvent(`onpropertychange`,br),fr=dr=null)}function br(e){if(e.propertyName===`value`&&mr(fr)){var t=[];ur(t,fr,e,sn(e)),fn(pr,t)}}function xr(e,t,n){e===`focusin`?(P(),dr=t,fr=n,dr.attachEvent(`onpropertychange`,br)):e===`focusout`&&P()}function Sr(e){if(e===`selectionchange`||e===`keyup`||e===`keydown`)return mr(fr)}function Cr(e,t){if(e===`click`)return mr(t)}function wr(e,t){if(e===`input`||e===`change`)return mr(t)}function Tr(e,t){return e===t&&(e!==0||1/e==1/t)||e!==e&&t!==t}var Er=typeof Object.is==`function`?Object.is:Tr;function Dr(e,t){if(Er(e,t))return!0;if(typeof e!=`object`||!e||typeof t!=`object`||!t)return!1;var n=Object.keys(e),r=Object.keys(t);if(n.length!==r.length)return!1;for(r=0;r<n.length;r++){var i=n[r];if(!Ee.call(t,i)||!Er(e[i],t[i]))return!1}return!0}function Or(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function kr(e,t){var n=Or(e);e=0;for(var r;n;){if(n.nodeType===3){if(r=e+n.textContent.length,e<=t&&r>=t)return{node:n,offset:t-e};e=r}a:{for(;n;){if(n.nextSibling){n=n.nextSibling;break a}n=n.parentNode}n=void 0}n=Or(n)}}function Ar(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Ar(e,t.parentNode):`contains`in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function jr(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=Vt(e.document);t instanceof e.HTMLIFrameElement;){try{var n=typeof t.contentWindow.location.href==`string`}catch{n=!1}if(n)e=t.contentWindow;else break;t=Vt(e.document)}return t}function Mr(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t===`input`&&(e.type===`text`||e.type===`search`||e.type===`tel`||e.type===`url`||e.type===`password`)||t===`textarea`||e.contentEditable===`true`)}var Nr=mn&&`documentMode`in document&&11>=document.documentMode,Pr=null,Fr=null,Ir=null,Lr=!1;function Rr(e,t,n){var r=n.window===n?n.document:n.nodeType===9?n:n.ownerDocument;Lr||Pr==null||Pr!==Vt(r)||(r=Pr,`selectionStart`in r&&Mr(r)?r={start:r.selectionStart,end:r.selectionEnd}:(r=(r.ownerDocument&&r.ownerDocument.defaultView||window).getSelection(),r={anchorNode:r.anchorNode,anchorOffset:r.anchorOffset,focusNode:r.focusNode,focusOffset:r.focusOffset}),Ir&&Dr(Ir,r)||(Ir=r,r=Ed(Fr,`onSelect`),0<r.length&&(t=new En(`onSelect`,`select`,null,t,n),e.push({event:t,listeners:r}),t.target=Pr)))}function zr(e,t){var n={};return n[e.toLowerCase()]=t.toLowerCase(),n[`Webkit`+e]=`webkit`+t,n[`Moz`+e]=`moz`+t,n}var Br={animationend:zr(`Animation`,`AnimationEnd`),animationiteration:zr(`Animation`,`AnimationIteration`),animationstart:zr(`Animation`,`AnimationStart`),transitionrun:zr(`Transition`,`TransitionRun`),transitionstart:zr(`Transition`,`TransitionStart`),transitioncancel:zr(`Transition`,`TransitionCancel`),transitionend:zr(`Transition`,`TransitionEnd`)},Vr={},Hr={};mn&&(Hr=document.createElement(`div`).style,`AnimationEvent`in window||(delete Br.animationend.animation,delete Br.animationiteration.animation,delete Br.animationstart.animation),`TransitionEvent`in window||delete Br.transitionend.transition);function Ur(e){if(Vr[e])return Vr[e];if(!Br[e])return e;var t=Br[e],n;for(n in t)if(t.hasOwnProperty(n)&&n in Hr)return Vr[e]=t[n];return e}var Wr=Ur(`animationend`),Gr=Ur(`animationiteration`),Kr=Ur(`animationstart`),qr=Ur(`transitionrun`),Jr=Ur(`transitionstart`),Yr=Ur(`transitioncancel`),Xr=Ur(`transitionend`),Zr=new Map,Qr=`abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(` `);Qr.push(`scrollEnd`);function $r(e,t){Zr.set(e,t),Dt(t,[e])}var ei=typeof reportError==`function`?reportError:function(e){if(typeof window==`object`&&typeof window.ErrorEvent==`function`){var t=new window.ErrorEvent(`error`,{bubbles:!0,cancelable:!0,message:typeof e==`object`&&e&&typeof e.message==`string`?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process==`object`&&typeof process.emit==`function`){process.emit(`uncaughtException`,e);return}console.error(e)},ti=[],ni=0,ri=0;function ii(){for(var e=ni,t=ri=ni=0;t<e;){var n=ti[t];ti[t++]=null;var r=ti[t];ti[t++]=null;var i=ti[t];ti[t++]=null;var a=ti[t];if(ti[t++]=null,r!==null&&i!==null){var o=r.pending;o===null?i.next=i:(i.next=o.next,o.next=i),r.pending=i}a!==0&&ci(n,i,a)}}function ai(e,t,n,r){ti[ni++]=e,ti[ni++]=t,ti[ni++]=n,ti[ni++]=r,ri|=r,e.lanes|=r,e=e.alternate,e!==null&&(e.lanes|=r)}function oi(e,t,n,r){return ai(e,t,n,r),li(e)}function si(e,t){return ai(e,null,null,t),li(e)}function ci(e,t,n){e.lanes|=n;var r=e.alternate;r!==null&&(r.lanes|=n);for(var i=!1,a=e.return;a!==null;)a.childLanes|=n,r=a.alternate,r!==null&&(r.childLanes|=n),a.tag===22&&(e=a.stateNode,e===null||e._visibility&1||(i=!0)),e=a,a=a.return;return e.tag===3?(a=e.stateNode,i&&t!==null&&(i=31-Ue(n),e=a.hiddenUpdates,r=e[i],r===null?e[i]=[t]:r.push(t),t.lane=n|536870912),a):null}function li(e){if(50<du)throw du=0,fu=null,Error(a(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var ui={};function di(e,t,n,r){this.tag=e,this.key=n,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=r,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function fi(e,t,n,r){return new di(e,t,n,r)}function pi(e){return e=e.prototype,!(!e||!e.isReactComponent)}function mi(e,t){var n=e.alternate;return n===null?(n=fi(e.tag,t,e.key,e.mode),n.elementType=e.elementType,n.type=e.type,n.stateNode=e.stateNode,n.alternate=e,e.alternate=n):(n.pendingProps=t,n.type=e.type,n.flags=0,n.subtreeFlags=0,n.deletions=null),n.flags=e.flags&65011712,n.childLanes=e.childLanes,n.lanes=e.lanes,n.child=e.child,n.memoizedProps=e.memoizedProps,n.memoizedState=e.memoizedState,n.updateQueue=e.updateQueue,t=e.dependencies,n.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},n.sibling=e.sibling,n.index=e.index,n.ref=e.ref,n.refCleanup=e.refCleanup,n}function hi(e,t){e.flags&=65011714;var n=e.alternate;return n===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=n.childLanes,e.lanes=n.lanes,e.child=n.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=n.memoizedProps,e.memoizedState=n.memoizedState,e.updateQueue=n.updateQueue,e.type=n.type,t=n.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function gi(e,t,n,r,i,o){var s=0;if(r=e,typeof e==`function`)pi(e)&&(s=1);else if(typeof e==`string`)s=Uf(e,n,de.current)?26:e===`html`||e===`head`||e===`body`?27:5;else a:switch(e){case re:return e=fi(31,n,t,i),e.elementType=re,e.lanes=o,e;case v:return _i(n.children,i,o,t);case y:s=8,i|=24;break;case b:return e=fi(12,n,t,i|2),e.elementType=b,e.lanes=o,e;case ee:return e=fi(13,n,t,i),e.elementType=ee,e.lanes=o,e;case te:return e=fi(19,n,t,i),e.elementType=te,e.lanes=o,e;default:if(typeof e==`object`&&e)switch(e.$$typeof){case S:s=10;break a;case x:s=9;break a;case C:s=11;break a;case ne:s=14;break a;case T:s=16,r=null;break a}s=29,n=Error(a(130,e===null?`null`:typeof e,``)),r=null}return t=fi(s,n,t,i),t.elementType=e,t.type=r,t.lanes=o,t}function _i(e,t,n,r){return e=fi(7,e,r,t),e.lanes=n,e}function vi(e,t,n){return e=fi(6,e,null,t),e.lanes=n,e}function yi(e){var t=fi(18,null,null,0);return t.stateNode=e,t}function bi(e,t,n){return t=fi(4,e.children===null?[]:e.children,e.key,t),t.lanes=n,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var xi=new WeakMap;function Si(e,t){if(typeof e==`object`&&e){var n=xi.get(e);return n===void 0?(t={value:e,source:t,stack:Te(t)},xi.set(e,t),t):n}return{value:e,source:t,stack:Te(t)}}var Ci=[],wi=0,Ti=null,Ei=0,Di=[],Oi=0,ki=null,F=1,Ai=``;function ji(e,t){Ci[wi++]=Ei,Ci[wi++]=Ti,Ti=e,Ei=t}function Mi(e,t,n){Di[Oi++]=F,Di[Oi++]=Ai,Di[Oi++]=ki,ki=e;var r=F;e=Ai;var i=32-Ue(r)-1;r&=~(1<<i),n+=1;var a=32-Ue(t)+i;if(30<a){var o=i-i%5;a=(r&(1<<o)-1).toString(32),r>>=o,i-=o,F=1<<32-Ue(t)+i|n<<i|r,Ai=a+e}else F=1<<a|n<<i|r,Ai=e}function Ni(e){e.return!==null&&(ji(e,1),Mi(e,1,0))}function Pi(e){for(;e===Ti;)Ti=Ci[--wi],Ci[wi]=null,Ei=Ci[--wi],Ci[wi]=null;for(;e===ki;)ki=Di[--Oi],Di[Oi]=null,Ai=Di[--Oi],Di[Oi]=null,F=Di[--Oi],Di[Oi]=null}function Fi(e,t){Di[Oi++]=F,Di[Oi++]=Ai,Di[Oi++]=ki,F=t.id,Ai=t.overflow,ki=e}var I=null,L=null,R=!1,Ii=null,Li=!1,Ri=Error(a(519));function zi(e){throw Gi(Si(Error(a(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?`text`:`HTML`,``)),e)),Ri}function Bi(e){var t=e.stateNode,n=e.type,r=e.memoizedProps;switch(t[ft]=e,t[N]=r,n){case`dialog`:Q(`cancel`,t),Q(`close`,t);break;case`iframe`:case`object`:case`embed`:Q(`load`,t);break;case`video`:case`audio`:for(n=0;n<_d.length;n++)Q(_d[n],t);break;case`source`:Q(`error`,t);break;case`img`:case`image`:case`link`:Q(`error`,t),Q(`load`,t);break;case`details`:Q(`toggle`,t);break;case`input`:Q(`invalid`,t),Gt(t,r.value,r.defaultValue,r.checked,r.defaultChecked,r.type,r.name,!0);break;case`select`:Q(`invalid`,t);break;case`textarea`:Q(`invalid`,t),Yt(t,r.value,r.defaultValue,r.children)}n=r.children,typeof n!=`string`&&typeof n!=`number`&&typeof n!=`bigint`||t.textContent===``+n||!0===r.suppressHydrationWarning||Md(t.textContent,n)?(r.popover!=null&&(Q(`beforetoggle`,t),Q(`toggle`,t)),r.onScroll!=null&&Q(`scroll`,t),r.onScrollEnd!=null&&Q(`scrollend`,t),r.onClick!=null&&(t.onclick=an),t=!0):t=!1,t||zi(e,!0)}function Vi(e){for(I=e.return;I;)switch(I.tag){case 5:case 31:case 13:Li=!1;return;case 27:case 3:Li=!0;return;default:I=I.return}}function Hi(e){if(e!==I)return!1;if(!R)return Vi(e),R=!0,!1;var t=e.tag,n;if((n=t!==3&&t!==27)&&((n=t===5)&&(n=e.type,n=!(n!==`form`&&n!==`button`)||Ud(e.type,e.memoizedProps)),n=!n),n&&L&&zi(e),Vi(e),t===13){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));L=uf(e)}else if(t===31){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));L=uf(e)}else t===27?(t=L,Zd(e.type)?(e=lf,lf=null,L=e):L=t):L=I?cf(e.stateNode.nextSibling):null;return!0}function Ui(){L=I=null,R=!1}function Wi(){var e=Ii;return e!==null&&(Zl===null?Zl=e:Zl.push.apply(Zl,e),Ii=null),e}function Gi(e){Ii===null?Ii=[e]:Ii.push(e)}var Ki=A(null),qi=null,Ji=null;function Yi(e,t,n){M(Ki,t._currentValue),t._currentValue=n}function Xi(e){e._currentValue=Ki.current,j(Ki)}function Zi(e,t,n){for(;e!==null;){var r=e.alternate;if((e.childLanes&t)===t?r!==null&&(r.childLanes&t)!==t&&(r.childLanes|=t):(e.childLanes|=t,r!==null&&(r.childLanes|=t)),e===n)break;e=e.return}}function Qi(e,t,n,r){var i=e.child;for(i!==null&&(i.return=e);i!==null;){var o=i.dependencies;if(o!==null){var s=i.child;o=o.firstContext;a:for(;o!==null;){var c=o;o=i;for(var l=0;l<t.length;l++)if(c.context===t[l]){o.lanes|=n,c=o.alternate,c!==null&&(c.lanes|=n),Zi(o.return,n,e),r||(s=null);break a}o=c.next}}else if(i.tag===18){if(s=i.return,s===null)throw Error(a(341));s.lanes|=n,o=s.alternate,o!==null&&(o.lanes|=n),Zi(s,n,e),s=null}else s=i.child;if(s!==null)s.return=i;else for(s=i;s!==null;){if(s===e){s=null;break}if(i=s.sibling,i!==null){i.return=s.return,s=i;break}s=s.return}i=s}}function $i(e,t,n,r){e=null;for(var i=t,o=!1;i!==null;){if(!o){if(i.flags&524288)o=!0;else if(i.flags&262144)break}if(i.tag===10){var s=i.alternate;if(s===null)throw Error(a(387));if(s=s.memoizedProps,s!==null){var c=i.type;Er(i.pendingProps.value,s.value)||(e===null?e=[c]:e.push(c))}}else if(i===me.current){if(s=i.alternate,s===null)throw Error(a(387));s.memoizedState.memoizedState!==i.memoizedState.memoizedState&&(e===null?e=[Qf]:e.push(Qf))}i=i.return}e!==null&&Qi(t,e,n,r),t.flags|=262144}function ea(e){for(e=e.firstContext;e!==null;){if(!Er(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function ta(e){qi=e,Ji=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function na(e){return ia(qi,e)}function ra(e,t){return qi===null&&ta(e),ia(e,t)}function ia(e,t){var n=t._currentValue;if(t={context:t,memoizedValue:n,next:null},Ji===null){if(e===null)throw Error(a(308));Ji=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else Ji=Ji.next=t;return n}var aa=typeof AbortController<`u`?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(t,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(e){return e()})}},oa=t.unstable_scheduleCallback,sa=t.unstable_NormalPriority,ca={$$typeof:S,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function la(){return{controller:new aa,data:new Map,refCount:0}}function ua(e){e.refCount--,e.refCount===0&&oa(sa,function(){e.controller.abort()})}var da=null,fa=0,pa=0,ma=null;function ha(e,t){if(da===null){var n=da=[];fa=0,pa=dd(),ma={status:`pending`,value:void 0,then:function(e){n.push(e)}}}return fa++,t.then(ga,ga),t}function ga(){if(--fa===0&&da!==null){ma!==null&&(ma.status=`fulfilled`);var e=da;da=null,pa=0,ma=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function _a(e,t){var n=[],r={status:`pending`,value:null,reason:null,then:function(e){n.push(e)}};return e.then(function(){r.status=`fulfilled`,r.value=t;for(var e=0;e<n.length;e++)(0,n[e])(t)},function(e){for(r.status=`rejected`,r.reason=e,e=0;e<n.length;e++)(0,n[e])(void 0)}),r}var va=O.S;O.S=function(e,t){eu=je(),typeof t==`object`&&t&&typeof t.then==`function`&&ha(e,t),va!==null&&va(e,t)};var ya=A(null);function ba(){var e=ya.current;return e===null?q.pooledCache:e}function xa(e,t){t===null?M(ya,ya.current):M(ya,t.pool)}function Sa(){var e=ba();return e===null?null:{parent:ca._currentValue,pool:e}}var Ca=Error(a(460)),wa=Error(a(474)),Ta=Error(a(542)),Ea={then:function(){}};function Da(e){return e=e.status,e===`fulfilled`||e===`rejected`}function Oa(e,t,n){switch(n=e[n],n===void 0?e.push(t):n!==t&&(t.then(an,an),t=n),t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Ma(e),e;default:if(typeof t.status==`string`)t.then(an,an);else{if(e=q,e!==null&&100<e.shellSuspendCounter)throw Error(a(482));e=t,e.status=`pending`,e.then(function(e){if(t.status===`pending`){var n=t;n.status=`fulfilled`,n.value=e}},function(e){if(t.status===`pending`){var n=t;n.status=`rejected`,n.reason=e}})}switch(t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Ma(e),e}throw Aa=t,Ca}}function ka(e){try{var t=e._init;return t(e._payload)}catch(e){throw typeof e==`object`&&e&&typeof e.then==`function`?(Aa=e,Ca):e}}var Aa=null;function ja(){if(Aa===null)throw Error(a(459));var e=Aa;return Aa=null,e}function Ma(e){if(e===Ca||e===Ta)throw Error(a(483))}var Na=null,Pa=0;function Fa(e){var t=Pa;return Pa+=1,Na===null&&(Na=[]),Oa(Na,e,t)}function Ia(e,t){t=t.props.ref,e.ref=t===void 0?null:t}function La(e,t){throw t.$$typeof===h?Error(a(525)):(e=Object.prototype.toString.call(t),Error(a(31,e===`[object Object]`?`object with keys {`+Object.keys(t).join(`, `)+`}`:e)))}function Ra(e){function t(t,n){if(e){var r=t.deletions;r===null?(t.deletions=[n],t.flags|=16):r.push(n)}}function n(n,r){if(!e)return null;for(;r!==null;)t(n,r),r=r.sibling;return null}function r(e){for(var t=new Map;e!==null;)e.key===null?t.set(e.index,e):t.set(e.key,e),e=e.sibling;return t}function i(e,t){return e=mi(e,t),e.index=0,e.sibling=null,e}function o(t,n,r){return t.index=r,e?(r=t.alternate,r===null?(t.flags|=67108866,n):(r=r.index,r<n?(t.flags|=67108866,n):r)):(t.flags|=1048576,n)}function s(t){return e&&t.alternate===null&&(t.flags|=67108866),t}function c(e,t,n,r){return t===null||t.tag!==6?(t=vi(n,e.mode,r),t.return=e,t):(t=i(t,n),t.return=e,t)}function l(e,t,n,r){var a=n.type;return a===v?d(e,t,n.props.children,r,n.key):t!==null&&(t.elementType===a||typeof a==`object`&&a&&a.$$typeof===T&&ka(a)===t.type)?(t=i(t,n.props),Ia(t,n),t.return=e,t):(t=gi(n.type,n.key,n.props,null,e.mode,r),Ia(t,n),t.return=e,t)}function u(e,t,n,r){return t===null||t.tag!==4||t.stateNode.containerInfo!==n.containerInfo||t.stateNode.implementation!==n.implementation?(t=bi(n,e.mode,r),t.return=e,t):(t=i(t,n.children||[]),t.return=e,t)}function d(e,t,n,r,a){return t===null||t.tag!==7?(t=_i(n,e.mode,r,a),t.return=e,t):(t=i(t,n),t.return=e,t)}function f(e,t,n){if(typeof t==`string`&&t!==``||typeof t==`number`||typeof t==`bigint`)return t=vi(``+t,e.mode,n),t.return=e,t;if(typeof t==`object`&&t){switch(t.$$typeof){case g:return n=gi(t.type,t.key,t.props,null,e.mode,n),Ia(n,t),n.return=e,n;case _:return t=bi(t,e.mode,n),t.return=e,t;case T:return t=ka(t),f(e,t,n)}if(se(t)||ae(t))return t=_i(t,e.mode,n,null),t.return=e,t;if(typeof t.then==`function`)return f(e,Fa(t),n);if(t.$$typeof===S)return f(e,ra(e,t),n);La(e,t)}return null}function p(e,t,n,r){var i=t===null?null:t.key;if(typeof n==`string`&&n!==``||typeof n==`number`||typeof n==`bigint`)return i===null?c(e,t,``+n,r):null;if(typeof n==`object`&&n){switch(n.$$typeof){case g:return n.key===i?l(e,t,n,r):null;case _:return n.key===i?u(e,t,n,r):null;case T:return n=ka(n),p(e,t,n,r)}if(se(n)||ae(n))return i===null?d(e,t,n,r,null):null;if(typeof n.then==`function`)return p(e,t,Fa(n),r);if(n.$$typeof===S)return p(e,t,ra(e,n),r);La(e,n)}return null}function m(e,t,n,r,i){if(typeof r==`string`&&r!==``||typeof r==`number`||typeof r==`bigint`)return e=e.get(n)||null,c(t,e,``+r,i);if(typeof r==`object`&&r){switch(r.$$typeof){case g:return e=e.get(r.key===null?n:r.key)||null,l(t,e,r,i);case _:return e=e.get(r.key===null?n:r.key)||null,u(t,e,r,i);case T:return r=ka(r),m(e,t,n,r,i)}if(se(r)||ae(r))return e=e.get(n)||null,d(t,e,r,i,null);if(typeof r.then==`function`)return m(e,t,n,Fa(r),i);if(r.$$typeof===S)return m(e,t,n,ra(t,r),i);La(t,r)}return null}function h(i,a,s,c){for(var l=null,u=null,d=a,h=a=0,g=null;d!==null&&h<s.length;h++){d.index>h?(g=d,d=null):g=d.sibling;var _=p(i,d,s[h],c);if(_===null){d===null&&(d=g);break}e&&d&&_.alternate===null&&t(i,d),a=o(_,a,h),u===null?l=_:u.sibling=_,u=_,d=g}if(h===s.length)return n(i,d),R&&ji(i,h),l;if(d===null){for(;h<s.length;h++)d=f(i,s[h],c),d!==null&&(a=o(d,a,h),u===null?l=d:u.sibling=d,u=d);return R&&ji(i,h),l}for(d=r(d);h<s.length;h++)g=m(d,i,h,s[h],c),g!==null&&(e&&g.alternate!==null&&d.delete(g.key===null?h:g.key),a=o(g,a,h),u===null?l=g:u.sibling=g,u=g);return e&&d.forEach(function(e){return t(i,e)}),R&&ji(i,h),l}function y(i,s,c,l){if(c==null)throw Error(a(151));for(var u=null,d=null,h=s,g=s=0,_=null,v=c.next();h!==null&&!v.done;g++,v=c.next()){h.index>g?(_=h,h=null):_=h.sibling;var y=p(i,h,v.value,l);if(y===null){h===null&&(h=_);break}e&&h&&y.alternate===null&&t(i,h),s=o(y,s,g),d===null?u=y:d.sibling=y,d=y,h=_}if(v.done)return n(i,h),R&&ji(i,g),u;if(h===null){for(;!v.done;g++,v=c.next())v=f(i,v.value,l),v!==null&&(s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return R&&ji(i,g),u}for(h=r(h);!v.done;g++,v=c.next())v=m(h,i,g,v.value,l),v!==null&&(e&&v.alternate!==null&&h.delete(v.key===null?g:v.key),s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return e&&h.forEach(function(e){return t(i,e)}),R&&ji(i,g),u}function b(e,r,o,c){if(typeof o==`object`&&o&&o.type===v&&o.key===null&&(o=o.props.children),typeof o==`object`&&o){switch(o.$$typeof){case g:a:{for(var l=o.key;r!==null;){if(r.key===l){if(l=o.type,l===v){if(r.tag===7){n(e,r.sibling),c=i(r,o.props.children),c.return=e,e=c;break a}}else if(r.elementType===l||typeof l==`object`&&l&&l.$$typeof===T&&ka(l)===r.type){n(e,r.sibling),c=i(r,o.props),Ia(c,o),c.return=e,e=c;break a}n(e,r);break}else t(e,r);r=r.sibling}o.type===v?(c=_i(o.props.children,e.mode,c,o.key),c.return=e,e=c):(c=gi(o.type,o.key,o.props,null,e.mode,c),Ia(c,o),c.return=e,e=c)}return s(e);case _:a:{for(l=o.key;r!==null;){if(r.key===l)if(r.tag===4&&r.stateNode.containerInfo===o.containerInfo&&r.stateNode.implementation===o.implementation){n(e,r.sibling),c=i(r,o.children||[]),c.return=e,e=c;break a}else{n(e,r);break}else t(e,r);r=r.sibling}c=bi(o,e.mode,c),c.return=e,e=c}return s(e);case T:return o=ka(o),b(e,r,o,c)}if(se(o))return h(e,r,o,c);if(ae(o)){if(l=ae(o),typeof l!=`function`)throw Error(a(150));return o=l.call(o),y(e,r,o,c)}if(typeof o.then==`function`)return b(e,r,Fa(o),c);if(o.$$typeof===S)return b(e,r,ra(e,o),c);La(e,o)}return typeof o==`string`&&o!==``||typeof o==`number`||typeof o==`bigint`?(o=``+o,r!==null&&r.tag===6?(n(e,r.sibling),c=i(r,o),c.return=e,e=c):(n(e,r),c=vi(o,e.mode,c),c.return=e,e=c),s(e)):n(e,r)}return function(e,t,n,r){try{Pa=0;var i=b(e,t,n,r);return Na=null,i}catch(t){if(t===Ca||t===Ta)throw t;var a=fi(29,t,null,e.mode);return a.lanes=r,a.return=e,a}}}var za=Ra(!0),Ba=Ra(!1),Va=!1;function Ha(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function Ua(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Wa(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function Ga(e,t,n){var r=e.updateQueue;if(r===null)return null;if(r=r.shared,K&2){var i=r.pending;return i===null?t.next=t:(t.next=i.next,i.next=t),r.pending=t,t=li(e),ci(e,null,n),t}return ai(e,r,t,n),li(e)}function Ka(e,t,n){if(t=t.updateQueue,t!==null&&(t=t.shared,n&4194048)){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,at(e,n)}}function qa(e,t){var n=e.updateQueue,r=e.alternate;if(r!==null&&(r=r.updateQueue,n===r)){var i=null,a=null;if(n=n.firstBaseUpdate,n!==null){do{var o={lane:n.lane,tag:n.tag,payload:n.payload,callback:null,next:null};a===null?i=a=o:a=a.next=o,n=n.next}while(n!==null);a===null?i=a=t:a=a.next=t}else i=a=t;n={baseState:r.baseState,firstBaseUpdate:i,lastBaseUpdate:a,shared:r.shared,callbacks:r.callbacks},e.updateQueue=n;return}e=n.lastBaseUpdate,e===null?n.firstBaseUpdate=t:e.next=t,n.lastBaseUpdate=t}var Ja=!1;function Ya(){if(Ja){var e=ma;if(e!==null)throw e}}function Xa(e,t,n,r){Ja=!1;var i=e.updateQueue;Va=!1;var a=i.firstBaseUpdate,o=i.lastBaseUpdate,s=i.shared.pending;if(s!==null){i.shared.pending=null;var c=s,l=c.next;c.next=null,o===null?a=l:o.next=l,o=c;var u=e.alternate;u!==null&&(u=u.updateQueue,s=u.lastBaseUpdate,s!==o&&(s===null?u.firstBaseUpdate=l:s.next=l,u.lastBaseUpdate=c))}if(a!==null){var d=i.baseState;o=0,u=l=c=null,s=a;do{var f=s.lane&-536870913,p=f!==s.lane;if(p?(Y&f)===f:(r&f)===f){f!==0&&f===pa&&(Ja=!0),u!==null&&(u=u.next={lane:0,tag:s.tag,payload:s.payload,callback:null,next:null});a:{var h=e,g=s;f=t;var _=n;switch(g.tag){case 1:if(h=g.payload,typeof h==`function`){d=h.call(_,d,f);break a}d=h;break a;case 3:h.flags=h.flags&-65537|128;case 0:if(h=g.payload,f=typeof h==`function`?h.call(_,d,f):h,f==null)break a;d=m({},d,f);break a;case 2:Va=!0}}f=s.callback,f!==null&&(e.flags|=64,p&&(e.flags|=8192),p=i.callbacks,p===null?i.callbacks=[f]:p.push(f))}else p={lane:f,tag:s.tag,payload:s.payload,callback:s.callback,next:null},u===null?(l=u=p,c=d):u=u.next=p,o|=f;if(s=s.next,s===null){if(s=i.shared.pending,s===null)break;p=s,s=p.next,p.next=null,i.lastBaseUpdate=p,i.shared.pending=null}}while(1);u===null&&(c=d),i.baseState=c,i.firstBaseUpdate=l,i.lastBaseUpdate=u,a===null&&(i.shared.lanes=0),Gl|=o,e.lanes=o,e.memoizedState=d}}function Za(e,t){if(typeof e!=`function`)throw Error(a(191,e));e.call(t)}function Qa(e,t){var n=e.callbacks;if(n!==null)for(e.callbacks=null,e=0;e<n.length;e++)Za(n[e],t)}var $a=A(null),eo=A(0);function to(e,t){e=Ul,M(eo,e),M($a,t),Ul=e|t.baseLanes}function z(){M(eo,Ul),M($a,$a.current)}function no(){Ul=eo.current,j($a),j(eo)}var ro=A(null),B=null;function io(e){var t=e.alternate;M(lo,lo.current&1),M(ro,e),B===null&&(t===null||$a.current!==null||t.memoizedState!==null)&&(B=e)}function ao(e){M(lo,lo.current),M(ro,e),B===null&&(B=e)}function oo(e){e.tag===22?(M(lo,lo.current),M(ro,e),B===null&&(B=e)):so(e)}function so(){M(lo,lo.current),M(ro,ro.current)}function co(e){j(ro),B===e&&(B=null),j(lo)}var lo=A(0);function uo(e){for(var t=e;t!==null;){if(t.tag===13){var n=t.memoizedState;if(n!==null&&(n=n.dehydrated,n===null||af(n)||of(n)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder===`forwards`||t.memoizedProps.revealOrder===`backwards`||t.memoizedProps.revealOrder===`unstable_legacy-backwards`||t.memoizedProps.revealOrder===`together`)){if(t.flags&128)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var V=0,H=null,U=null,fo=null,po=!1,mo=!1,ho=!1,go=0,_o=0,vo=null,yo=0;function bo(){throw Error(a(321))}function xo(e,t){if(t===null)return!1;for(var n=0;n<t.length&&n<e.length;n++)if(!Er(e[n],t[n]))return!1;return!0}function So(e,t,n,r,i,a){return V=a,H=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,O.H=e===null||e.memoizedState===null?Rs:zs,ho=!1,a=n(r,i),ho=!1,mo&&(a=wo(t,n,r,i)),Co(e),a}function Co(e){O.H=Ls;var t=U!==null&&U.next!==null;if(V=0,fo=U=H=null,po=!1,_o=0,vo=null,t)throw Error(a(300));e===null||nc||(e=e.dependencies,e!==null&&ea(e)&&(nc=!0))}function wo(e,t,n,r){H=e;var i=0;do{if(mo&&(vo=null),_o=0,mo=!1,25<=i)throw Error(a(301));if(i+=1,fo=U=null,e.updateQueue!=null){var o=e.updateQueue;o.lastEffect=null,o.events=null,o.stores=null,o.memoCache!=null&&(o.memoCache.index=0)}O.H=Bs,o=t(n,r)}while(mo);return o}function To(){var e=O.H,t=e.useState()[0];return t=typeof t.then==`function`?Mo(t):t,e=e.useState()[0],(U===null?null:U.memoizedState)!==e&&(H.flags|=1024),t}function Eo(){var e=go!==0;return go=0,e}function Do(e,t,n){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~n}function Oo(e){if(po){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}po=!1}V=0,fo=U=H=null,mo=!1,_o=go=0,vo=null}function ko(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return fo===null?H.memoizedState=fo=e:fo=fo.next=e,fo}function Ao(){if(U===null){var e=H.alternate;e=e===null?null:e.memoizedState}else e=U.next;var t=fo===null?H.memoizedState:fo.next;if(t!==null)fo=t,U=e;else{if(e===null)throw H.alternate===null?Error(a(467)):Error(a(310));U=e,e={memoizedState:U.memoizedState,baseState:U.baseState,baseQueue:U.baseQueue,queue:U.queue,next:null},fo===null?H.memoizedState=fo=e:fo=fo.next=e}return fo}function jo(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Mo(e){var t=_o;return _o+=1,vo===null&&(vo=[]),e=Oa(vo,e,t),t=H,(fo===null?t.memoizedState:fo.next)===null&&(t=t.alternate,O.H=t===null||t.memoizedState===null?Rs:zs),e}function W(e){if(typeof e==`object`&&e){if(typeof e.then==`function`)return Mo(e);if(e.$$typeof===S)return na(e)}throw Error(a(438,String(e)))}function No(e){var t=null,n=H.updateQueue;if(n!==null&&(t=n.memoCache),t==null){var r=H.alternate;r!==null&&(r=r.updateQueue,r!==null&&(r=r.memoCache,r!=null&&(t={data:r.data.map(function(e){return e.slice()}),index:0})))}if(t??={data:[],index:0},n===null&&(n=jo(),H.updateQueue=n),n.memoCache=t,n=t.data[t.index],n===void 0)for(n=t.data[t.index]=Array(e),r=0;r<e;r++)n[r]=E;return t.index++,n}function Po(e,t){return typeof t==`function`?t(e):t}function Fo(e){return Io(Ao(),U,e)}function Io(e,t,n){var r=e.queue;if(r===null)throw Error(a(311));r.lastRenderedReducer=n;var i=e.baseQueue,o=r.pending;if(o!==null){if(i!==null){var s=i.next;i.next=o.next,o.next=s}t.baseQueue=i=o,r.pending=null}if(o=e.baseState,i===null)e.memoizedState=o;else{t=i.next;var c=s=null,l=null,u=t,d=!1;do{var f=u.lane&-536870913;if(f===u.lane?(V&f)===f:(Y&f)===f){var p=u.revertLane;if(p===0)l!==null&&(l=l.next={lane:0,revertLane:0,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null}),f===pa&&(d=!0);else if((V&p)===p){u=u.next,p===pa&&(d=!0);continue}else f={lane:0,revertLane:u.revertLane,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=f,s=o):l=l.next=f,H.lanes|=p,Gl|=p;f=u.action,ho&&n(o,f),o=u.hasEagerState?u.eagerState:n(o,f)}else p={lane:f,revertLane:u.revertLane,gesture:u.gesture,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=p,s=o):l=l.next=p,H.lanes|=f,Gl|=f;u=u.next}while(u!==null&&u!==t);if(l===null?s=o:l.next=c,!Er(o,e.memoizedState)&&(nc=!0,d&&(n=ma,n!==null)))throw n;e.memoizedState=o,e.baseState=s,e.baseQueue=l,r.lastRenderedState=o}return i===null&&(r.lanes=0),[e.memoizedState,r.dispatch]}function Lo(e){var t=Ao(),n=t.queue;if(n===null)throw Error(a(311));n.lastRenderedReducer=e;var r=n.dispatch,i=n.pending,o=t.memoizedState;if(i!==null){n.pending=null;var s=i=i.next;do o=e(o,s.action),s=s.next;while(s!==i);Er(o,t.memoizedState)||(nc=!0),t.memoizedState=o,t.baseQueue===null&&(t.baseState=o),n.lastRenderedState=o}return[o,r]}function Ro(e,t,n){var r=H,i=Ao(),o=R;if(o){if(n===void 0)throw Error(a(407));n=n()}else n=t();var s=!Er((U||i).memoizedState,n);if(s&&(i.memoizedState=n,nc=!0),i=i.queue,ls(Vo.bind(null,r,i,e),[e]),i.getSnapshot!==t||s||fo!==null&&fo.memoizedState.tag&1){if(r.flags|=2048,is(9,{destroy:void 0},Bo.bind(null,r,i,n,t),null),q===null)throw Error(a(349));o||V&127||zo(r,t,n)}return n}function zo(e,t,n){e.flags|=16384,e={getSnapshot:t,value:n},t=H.updateQueue,t===null?(t=jo(),H.updateQueue=t,t.stores=[e]):(n=t.stores,n===null?t.stores=[e]:n.push(e))}function Bo(e,t,n,r){t.value=n,t.getSnapshot=r,Ho(t)&&Uo(e)}function Vo(e,t,n){return n(function(){Ho(t)&&Uo(e)})}function Ho(e){var t=e.getSnapshot;e=e.value;try{var n=t();return!Er(e,n)}catch{return!0}}function Uo(e){var t=si(e,2);t!==null&&hu(t,e,2)}function Wo(e){var t=ko();if(typeof e==`function`){var n=e;if(e=n(),ho){He(!0);try{n()}finally{He(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Po,lastRenderedState:e},t}function Go(e,t,n,r){return e.baseState=n,Io(e,U,typeof r==`function`?r:Po)}function Ko(e,t,n,r,i){if(Ps(e))throw Error(a(485));if(e=t.action,e!==null){var o={payload:i,action:e,next:null,isTransition:!0,status:`pending`,value:null,reason:null,listeners:[],then:function(e){o.listeners.push(e)}};O.T===null?o.isTransition=!1:n(!0),r(o),n=t.pending,n===null?(o.next=t.pending=o,qo(t,o)):(o.next=n.next,t.pending=n.next=o)}}function qo(e,t){var n=t.action,r=t.payload,i=e.state;if(t.isTransition){var a=O.T,o={};O.T=o;try{var s=n(i,r),c=O.S;c!==null&&c(o,s),Jo(e,t,s)}catch(n){Xo(e,t,n)}finally{a!==null&&o.types!==null&&(a.types=o.types),O.T=a}}else try{a=n(i,r),Jo(e,t,a)}catch(n){Xo(e,t,n)}}function Jo(e,t,n){typeof n==`object`&&n&&typeof n.then==`function`?n.then(function(n){Yo(e,t,n)},function(n){return Xo(e,t,n)}):Yo(e,t,n)}function Yo(e,t,n){t.status=`fulfilled`,t.value=n,Zo(t),e.state=n,t=e.pending,t!==null&&(n=t.next,n===t?e.pending=null:(n=n.next,t.next=n,qo(e,n)))}function Xo(e,t,n){var r=e.pending;if(e.pending=null,r!==null){r=r.next;do t.status=`rejected`,t.reason=n,Zo(t),t=t.next;while(t!==r)}e.action=null}function Zo(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function Qo(e,t){return t}function $o(e,t){if(R){var n=q.formState;if(n!==null){a:{var r=H;if(R){if(L){b:{for(var i=L,a=Li;i.nodeType!==8;){if(!a){i=null;break b}if(i=cf(i.nextSibling),i===null){i=null;break b}}a=i.data,i=a===`F!`||a===`F`?i:null}if(i){L=cf(i.nextSibling),r=i.data===`F!`;break a}}zi(r)}r=!1}r&&(t=n[0])}}return n=ko(),n.memoizedState=n.baseState=t,r={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Qo,lastRenderedState:t},n.queue=r,n=js.bind(null,H,r),r.dispatch=n,r=Wo(!1),a=Ns.bind(null,H,!1,r.queue),r=ko(),i={state:t,dispatch:null,action:e,pending:null},r.queue=i,n=Ko.bind(null,H,i,a,n),i.dispatch=n,r.memoizedState=e,[t,n,!1]}function es(e){return ts(Ao(),U,e)}function ts(e,t,n){if(t=Io(e,t,Qo)[0],e=Fo(Po)[0],typeof t==`object`&&t&&typeof t.then==`function`)try{var r=Mo(t)}catch(e){throw e===Ca?Ta:e}else r=t;t=Ao();var i=t.queue,a=i.dispatch;return n!==t.memoizedState&&(H.flags|=2048,is(9,{destroy:void 0},ns.bind(null,i,n),null)),[r,a,e]}function ns(e,t){e.action=t}function rs(e){var t=Ao(),n=U;if(n!==null)return ts(t,n,e);Ao(),t=t.memoizedState,n=Ao();var r=n.queue.dispatch;return n.memoizedState=e,[t,r,!1]}function is(e,t,n,r){return e={tag:e,create:n,deps:r,inst:t,next:null},t=H.updateQueue,t===null&&(t=jo(),H.updateQueue=t),n=t.lastEffect,n===null?t.lastEffect=e.next=e:(r=n.next,n.next=e,e.next=r,t.lastEffect=e),e}function as(){return Ao().memoizedState}function os(e,t,n,r){var i=ko();H.flags|=e,i.memoizedState=is(1|t,{destroy:void 0},n,r===void 0?null:r)}function ss(e,t,n,r){var i=Ao();r=r===void 0?null:r;var a=i.memoizedState.inst;U!==null&&r!==null&&xo(r,U.memoizedState.deps)?i.memoizedState=is(t,a,n,r):(H.flags|=e,i.memoizedState=is(1|t,a,n,r))}function cs(e,t){os(8390656,8,e,t)}function ls(e,t){ss(2048,8,e,t)}function us(e){H.flags|=4;var t=H.updateQueue;if(t===null)t=jo(),H.updateQueue=t,t.events=[e];else{var n=t.events;n===null?t.events=[e]:n.push(e)}}function ds(e){var t=Ao().memoizedState;return us({ref:t,nextImpl:e}),function(){if(K&2)throw Error(a(440));return t.impl.apply(void 0,arguments)}}function fs(e,t){return ss(4,2,e,t)}function ps(e,t){return ss(4,4,e,t)}function ms(e,t){if(typeof t==`function`){e=e();var n=t(e);return function(){typeof n==`function`?n():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function hs(e,t,n){n=n==null?null:n.concat([e]),ss(4,4,ms.bind(null,t,e),n)}function gs(){}function _s(e,t){var n=Ao();t=t===void 0?null:t;var r=n.memoizedState;return t!==null&&xo(t,r[1])?r[0]:(n.memoizedState=[e,t],e)}function vs(e,t){var n=Ao();t=t===void 0?null:t;var r=n.memoizedState;if(t!==null&&xo(t,r[1]))return r[0];if(r=e(),ho){He(!0);try{e()}finally{He(!1)}}return n.memoizedState=[r,t],r}function ys(e,t,n){return n===void 0||V&1073741824&&!(Y&261930)?e.memoizedState=t:(e.memoizedState=n,e=mu(),H.lanes|=e,Gl|=e,n)}function bs(e,t,n,r){return Er(n,t)?n:$a.current===null?!(V&42)||V&1073741824&&!(Y&261930)?(nc=!0,e.memoizedState=n):(e=mu(),H.lanes|=e,Gl|=e,t):(e=ys(e,n,r),Er(e,t)||(nc=!0),e)}function xs(e,t,n,r,i){var a=k.p;k.p=a!==0&&8>a?a:8;var o=O.T,s={};O.T=s,Ns(e,!1,t,n);try{var c=i(),l=O.S;l!==null&&l(s,c),typeof c==`object`&&c&&typeof c.then==`function`?Ms(e,t,_a(c,r),pu(e)):Ms(e,t,r,pu(e))}catch(n){Ms(e,t,{then:function(){},status:`rejected`,reason:n},pu())}finally{k.p=a,o!==null&&s.types!==null&&(o.types=s.types),O.T=o}}function Ss(){}function Cs(e,t,n,r){if(e.tag!==5)throw Error(a(476));var i=ws(e).queue;xs(e,i,t,ce,n===null?Ss:function(){return Ts(e),n(r)})}function ws(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:ce,baseState:ce,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Po,lastRenderedState:ce},next:null};var n={};return t.next={memoizedState:n,baseState:n,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Po,lastRenderedState:n},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function Ts(e){var t=ws(e);t.next===null&&(t=e.alternate.memoizedState),Ms(e,t.next.queue,{},pu())}function Es(){return na(Qf)}function Ds(){return Ao().memoizedState}function Os(){return Ao().memoizedState}function ks(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var n=pu();e=Wa(n);var r=Ga(t,e,n);r!==null&&(hu(r,t,n),Ka(r,t,n)),t={cache:la()},e.payload=t;return}t=t.return}}function As(e,t,n){var r=pu();n={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},Ps(e)?Fs(t,n):(n=oi(e,t,n,r),n!==null&&(hu(n,e,r),Is(n,t,r)))}function js(e,t,n){Ms(e,t,n,pu())}function Ms(e,t,n,r){var i={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null};if(Ps(e))Fs(t,i);else{var a=e.alternate;if(e.lanes===0&&(a===null||a.lanes===0)&&(a=t.lastRenderedReducer,a!==null))try{var o=t.lastRenderedState,s=a(o,n);if(i.hasEagerState=!0,i.eagerState=s,Er(s,o))return ai(e,t,i,0),q===null&&ii(),!1}catch{}if(n=oi(e,t,i,r),n!==null)return hu(n,e,r),Is(n,t,r),!0}return!1}function Ns(e,t,n,r){if(r={lane:2,revertLane:dd(),gesture:null,action:r,hasEagerState:!1,eagerState:null,next:null},Ps(e)){if(t)throw Error(a(479))}else t=oi(e,n,r,2),t!==null&&hu(t,e,2)}function Ps(e){var t=e.alternate;return e===H||t!==null&&t===H}function Fs(e,t){mo=po=!0;var n=e.pending;n===null?t.next=t:(t.next=n.next,n.next=t),e.pending=t}function Is(e,t,n){if(n&4194048){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,at(e,n)}}var Ls={readContext:na,use:W,useCallback:bo,useContext:bo,useEffect:bo,useImperativeHandle:bo,useLayoutEffect:bo,useInsertionEffect:bo,useMemo:bo,useReducer:bo,useRef:bo,useState:bo,useDebugValue:bo,useDeferredValue:bo,useTransition:bo,useSyncExternalStore:bo,useId:bo,useHostTransitionStatus:bo,useFormState:bo,useActionState:bo,useOptimistic:bo,useMemoCache:bo,useCacheRefresh:bo};Ls.useEffectEvent=bo;var Rs={readContext:na,use:W,useCallback:function(e,t){return ko().memoizedState=[e,t===void 0?null:t],e},useContext:na,useEffect:cs,useImperativeHandle:function(e,t,n){n=n==null?null:n.concat([e]),os(4194308,4,ms.bind(null,t,e),n)},useLayoutEffect:function(e,t){return os(4194308,4,e,t)},useInsertionEffect:function(e,t){os(4,2,e,t)},useMemo:function(e,t){var n=ko();t=t===void 0?null:t;var r=e();if(ho){He(!0);try{e()}finally{He(!1)}}return n.memoizedState=[r,t],r},useReducer:function(e,t,n){var r=ko();if(n!==void 0){var i=n(t);if(ho){He(!0);try{n(t)}finally{He(!1)}}}else i=t;return r.memoizedState=r.baseState=i,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:i},r.queue=e,e=e.dispatch=As.bind(null,H,e),[r.memoizedState,e]},useRef:function(e){var t=ko();return e={current:e},t.memoizedState=e},useState:function(e){e=Wo(e);var t=e.queue,n=js.bind(null,H,t);return t.dispatch=n,[e.memoizedState,n]},useDebugValue:gs,useDeferredValue:function(e,t){return ys(ko(),e,t)},useTransition:function(){var e=Wo(!1);return e=xs.bind(null,H,e.queue,!0,!1),ko().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,n){var r=H,i=ko();if(R){if(n===void 0)throw Error(a(407));n=n()}else{if(n=t(),q===null)throw Error(a(349));Y&127||zo(r,t,n)}i.memoizedState=n;var o={value:n,getSnapshot:t};return i.queue=o,cs(Vo.bind(null,r,o,e),[e]),r.flags|=2048,is(9,{destroy:void 0},Bo.bind(null,r,o,n,t),null),n},useId:function(){var e=ko(),t=q.identifierPrefix;if(R){var n=Ai,r=F;n=(r&~(1<<32-Ue(r)-1)).toString(32)+n,t=`_`+t+`R_`+n,n=go++,0<n&&(t+=`H`+n.toString(32)),t+=`_`}else n=yo++,t=`_`+t+`r_`+n.toString(32)+`_`;return e.memoizedState=t},useHostTransitionStatus:Es,useFormState:$o,useActionState:$o,useOptimistic:function(e){var t=ko();t.memoizedState=t.baseState=e;var n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=n,t=Ns.bind(null,H,!0,n),n.dispatch=t,[e,t]},useMemoCache:No,useCacheRefresh:function(){return ko().memoizedState=ks.bind(null,H)},useEffectEvent:function(e){var t=ko(),n={impl:e};return t.memoizedState=n,function(){if(K&2)throw Error(a(440));return n.impl.apply(void 0,arguments)}}},zs={readContext:na,use:W,useCallback:_s,useContext:na,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:vs,useReducer:Fo,useRef:as,useState:function(){return Fo(Po)},useDebugValue:gs,useDeferredValue:function(e,t){return bs(Ao(),U.memoizedState,e,t)},useTransition:function(){var e=Fo(Po)[0],t=Ao().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:Ro,useId:Ds,useHostTransitionStatus:Es,useFormState:es,useActionState:es,useOptimistic:function(e,t){return Go(Ao(),U,e,t)},useMemoCache:No,useCacheRefresh:Os};zs.useEffectEvent=ds;var Bs={readContext:na,use:W,useCallback:_s,useContext:na,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:vs,useReducer:Lo,useRef:as,useState:function(){return Lo(Po)},useDebugValue:gs,useDeferredValue:function(e,t){var n=Ao();return U===null?ys(n,e,t):bs(n,U.memoizedState,e,t)},useTransition:function(){var e=Lo(Po)[0],t=Ao().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:Ro,useId:Ds,useHostTransitionStatus:Es,useFormState:rs,useActionState:rs,useOptimistic:function(e,t){var n=Ao();return U===null?(n.baseState=e,[e,n.queue.dispatch]):Go(n,U,e,t)},useMemoCache:No,useCacheRefresh:Os};Bs.useEffectEvent=ds;function Vs(e,t,n,r){t=e.memoizedState,n=n(r,t),n=n==null?t:m({},t,n),e.memoizedState=n,e.lanes===0&&(e.updateQueue.baseState=n)}var Hs={enqueueSetState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Wa(r);i.payload=t,n!=null&&(i.callback=n),t=Ga(e,i,r),t!==null&&(hu(t,e,r),Ka(t,e,r))},enqueueReplaceState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Wa(r);i.tag=1,i.payload=t,n!=null&&(i.callback=n),t=Ga(e,i,r),t!==null&&(hu(t,e,r),Ka(t,e,r))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var n=pu(),r=Wa(n);r.tag=2,t!=null&&(r.callback=t),t=Ga(e,r,n),t!==null&&(hu(t,e,n),Ka(t,e,n))}};function Us(e,t,n,r,i,a,o){return e=e.stateNode,typeof e.shouldComponentUpdate==`function`?e.shouldComponentUpdate(r,a,o):t.prototype&&t.prototype.isPureReactComponent?!Dr(n,r)||!Dr(i,a):!0}function Ws(e,t,n,r){e=t.state,typeof t.componentWillReceiveProps==`function`&&t.componentWillReceiveProps(n,r),typeof t.UNSAFE_componentWillReceiveProps==`function`&&t.UNSAFE_componentWillReceiveProps(n,r),t.state!==e&&Hs.enqueueReplaceState(t,t.state,null)}function Gs(e,t){var n=t;if(`ref`in t)for(var r in n={},t)r!==`ref`&&(n[r]=t[r]);if(e=e.defaultProps)for(var i in n===t&&(n=m({},n)),e)n[i]===void 0&&(n[i]=e[i]);return n}function Ks(e){ei(e)}function qs(e){console.error(e)}function Js(e){ei(e)}function Ys(e,t){try{var n=e.onUncaughtError;n(t.value,{componentStack:t.stack})}catch(e){setTimeout(function(){throw e})}}function Xs(e,t,n){try{var r=e.onCaughtError;r(n.value,{componentStack:n.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(e){setTimeout(function(){throw e})}}function Zs(e,t,n){return n=Wa(n),n.tag=3,n.payload={element:null},n.callback=function(){Ys(e,t)},n}function Qs(e){return e=Wa(e),e.tag=3,e}function $s(e,t,n,r){var i=n.type.getDerivedStateFromError;if(typeof i==`function`){var a=r.value;e.payload=function(){return i(a)},e.callback=function(){Xs(t,n,r)}}var o=n.stateNode;o!==null&&typeof o.componentDidCatch==`function`&&(e.callback=function(){Xs(t,n,r),typeof i!=`function`&&(ru===null?ru=new Set([this]):ru.add(this));var e=r.stack;this.componentDidCatch(r.value,{componentStack:e===null?``:e})})}function ec(e,t,n,r,i){if(n.flags|=32768,typeof r==`object`&&r&&typeof r.then==`function`){if(t=n.alternate,t!==null&&$i(t,n,i,!0),n=ro.current,n!==null){switch(n.tag){case 31:case 13:return B===null?Du():n.alternate===null&&Wl===0&&(Wl=3),n.flags&=-257,n.flags|=65536,n.lanes=i,r===Ea?n.flags|=16384:(t=n.updateQueue,t===null?n.updateQueue=new Set([r]):t.add(r),Gu(e,r,i)),!1;case 22:return n.flags|=65536,r===Ea?n.flags|=16384:(t=n.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([r])},n.updateQueue=t):(n=t.retryQueue,n===null?t.retryQueue=new Set([r]):n.add(r)),Gu(e,r,i)),!1}throw Error(a(435,n.tag))}return Gu(e,r,i),Du(),!1}if(R)return t=ro.current,t===null?(r!==Ri&&(t=Error(a(423),{cause:r}),Gi(Si(t,n))),e=e.current.alternate,e.flags|=65536,i&=-i,e.lanes|=i,r=Si(r,n),i=Zs(e.stateNode,r,i),qa(e,i),Wl!==4&&(Wl=2)):(!(t.flags&65536)&&(t.flags|=256),t.flags|=65536,t.lanes=i,r!==Ri&&(e=Error(a(422),{cause:r}),Gi(Si(e,n)))),!1;var o=Error(a(520),{cause:r});if(o=Si(o,n),Xl===null?Xl=[o]:Xl.push(o),Wl!==4&&(Wl=2),t===null)return!0;r=Si(r,n),n=t;do{switch(n.tag){case 3:return n.flags|=65536,e=i&-i,n.lanes|=e,e=Zs(n.stateNode,r,e),qa(n,e),!1;case 1:if(t=n.type,o=n.stateNode,!(n.flags&128)&&(typeof t.getDerivedStateFromError==`function`||o!==null&&typeof o.componentDidCatch==`function`&&(ru===null||!ru.has(o))))return n.flags|=65536,i&=-i,n.lanes|=i,i=Qs(i),$s(i,e,n,r),qa(n,i),!1}n=n.return}while(n!==null);return!1}var tc=Error(a(461)),nc=!1;function G(e,t,n,r){t.child=e===null?Ba(t,null,n,r):za(t,e.child,n,r)}function rc(e,t,n,r,i){n=n.render;var a=t.ref;if(`ref`in r){var o={};for(var s in r)s!==`ref`&&(o[s]=r[s])}else o=r;return ta(t),r=So(e,t,n,o,a,i),s=Eo(),e!==null&&!nc?(Do(e,t,i),Dc(e,t,i)):(R&&s&&Ni(t),t.flags|=1,G(e,t,r,i),t.child)}function ic(e,t,n,r,i){if(e===null){var a=n.type;return typeof a==`function`&&!pi(a)&&a.defaultProps===void 0&&n.compare===null?(t.tag=15,t.type=a,ac(e,t,a,r,i)):(e=gi(n.type,null,r,t,t.mode,i),e.ref=t.ref,e.return=t,t.child=e)}if(a=e.child,!Oc(e,i)){var o=a.memoizedProps;if(n=n.compare,n=n===null?Dr:n,n(o,r)&&e.ref===t.ref)return Dc(e,t,i)}return t.flags|=1,e=mi(a,r),e.ref=t.ref,e.return=t,t.child=e}function ac(e,t,n,r,i){if(e!==null){var a=e.memoizedProps;if(Dr(a,r)&&e.ref===t.ref)if(nc=!1,t.pendingProps=r=a,Oc(e,i))e.flags&131072&&(nc=!0);else return t.lanes=e.lanes,Dc(e,t,i)}return pc(e,t,n,r,i)}function oc(e,t,n,r){var i=r.children,a=e===null?null:e.memoizedState;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),r.mode===`hidden`){if(t.flags&128){if(a=a===null?n:a.baseLanes|n,e!==null){for(r=t.child=e.child,i=0;r!==null;)i=i|r.lanes|r.childLanes,r=r.sibling;r=i&~a}else r=0,t.child=null;return cc(e,t,a,n,r)}if(n&536870912)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&xa(t,a===null?null:a.cachePool),a===null?z():to(t,a),oo(t);else return r=t.lanes=536870912,cc(e,t,a===null?n:a.baseLanes|n,n,r)}else a===null?(e!==null&&xa(t,null),z(),so(t)):(xa(t,a.cachePool),to(t,a),so(t),t.memoizedState=null);return G(e,t,i,n),t.child}function sc(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function cc(e,t,n,r,i){var a=ba();return a=a===null?null:{parent:ca._currentValue,pool:a},t.memoizedState={baseLanes:n,cachePool:a},e!==null&&xa(t,null),z(),oo(t),e!==null&&$i(e,t,r,!0),t.childLanes=i,null}function lc(e,t){return t=Sc({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function uc(e,t,n){return za(t,e.child,null,n),e=lc(t,t.pendingProps),e.flags|=2,co(t),t.memoizedState=null,e}function dc(e,t,n){var r=t.pendingProps,i=(t.flags&128)!=0;if(t.flags&=-129,e===null){if(R){if(r.mode===`hidden`)return e=lc(t,r),t.lanes=536870912,sc(null,e);if(ao(t),(e=L)?(e=rf(e,Li),e=e!==null&&e.data===`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:ki===null?null:{id:F,overflow:Ai},retryLane:536870912,hydrationErrors:null},n=yi(e),n.return=t,t.child=n,I=t,L=null)):e=null,e===null)throw zi(t);return t.lanes=536870912,null}return lc(t,r)}var o=e.memoizedState;if(o!==null){var s=o.dehydrated;if(ao(t),i)if(t.flags&256)t.flags&=-257,t=uc(e,t,n);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(a(558));else if(nc||$i(e,t,n,!1),i=(n&e.childLanes)!==0,nc||i){if(r=q,r!==null&&(s=ot(r,n),s!==0&&s!==o.retryLane))throw o.retryLane=s,si(e,s),hu(r,e,s),tc;Du(),t=uc(e,t,n)}else e=o.treeContext,L=cf(s.nextSibling),I=t,R=!0,Ii=null,Li=!1,e!==null&&Fi(t,e),t=lc(t,r),t.flags|=4096;return t}return e=mi(e.child,{mode:r.mode,children:r.children}),e.ref=t.ref,t.child=e,e.return=t,e}function fc(e,t){var n=t.ref;if(n===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof n!=`function`&&typeof n!=`object`)throw Error(a(284));(e===null||e.ref!==n)&&(t.flags|=4194816)}}function pc(e,t,n,r,i){return ta(t),n=So(e,t,n,r,void 0,i),r=Eo(),e!==null&&!nc?(Do(e,t,i),Dc(e,t,i)):(R&&r&&Ni(t),t.flags|=1,G(e,t,n,i),t.child)}function mc(e,t,n,r,i,a){return ta(t),t.updateQueue=null,n=wo(t,r,n,i),Co(e),r=Eo(),e!==null&&!nc?(Do(e,t,a),Dc(e,t,a)):(R&&r&&Ni(t),t.flags|=1,G(e,t,n,a),t.child)}function hc(e,t,n,r,i){if(ta(t),t.stateNode===null){var a=ui,o=n.contextType;typeof o==`object`&&o&&(a=na(o)),a=new n(r,a),t.memoizedState=a.state!==null&&a.state!==void 0?a.state:null,a.updater=Hs,t.stateNode=a,a._reactInternals=t,a=t.stateNode,a.props=r,a.state=t.memoizedState,a.refs={},Ha(t),o=n.contextType,a.context=typeof o==`object`&&o?na(o):ui,a.state=t.memoizedState,o=n.getDerivedStateFromProps,typeof o==`function`&&(Vs(t,n,o,r),a.state=t.memoizedState),typeof n.getDerivedStateFromProps==`function`||typeof a.getSnapshotBeforeUpdate==`function`||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(o=a.state,typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount(),o!==a.state&&Hs.enqueueReplaceState(a,a.state,null),Xa(t,r,a,i),Ya(),a.state=t.memoizedState),typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!0}else if(e===null){a=t.stateNode;var s=t.memoizedProps,c=Gs(n,s);a.props=c;var l=a.context,u=n.contextType;o=ui,typeof u==`object`&&u&&(o=na(u));var d=n.getDerivedStateFromProps;u=typeof d==`function`||typeof a.getSnapshotBeforeUpdate==`function`,s=t.pendingProps!==s,u||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(s||l!==o)&&Ws(t,a,r,o),Va=!1;var f=t.memoizedState;a.state=f,Xa(t,r,a,i),Ya(),l=t.memoizedState,s||f!==l||Va?(typeof d==`function`&&(Vs(t,n,d,r),l=t.memoizedState),(c=Va||Us(t,n,c,r,f,l,o))?(u||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount()),typeof a.componentDidMount==`function`&&(t.flags|=4194308)):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),t.memoizedProps=r,t.memoizedState=l),a.props=r,a.state=l,a.context=o,r=c):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!1)}else{a=t.stateNode,Ua(e,t),o=t.memoizedProps,u=Gs(n,o),a.props=u,d=t.pendingProps,f=a.context,l=n.contextType,c=ui,typeof l==`object`&&l&&(c=na(l)),s=n.getDerivedStateFromProps,(l=typeof s==`function`||typeof a.getSnapshotBeforeUpdate==`function`)||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(o!==d||f!==c)&&Ws(t,a,r,c),Va=!1,f=t.memoizedState,a.state=f,Xa(t,r,a,i),Ya();var p=t.memoizedState;o!==d||f!==p||Va||e!==null&&e.dependencies!==null&&ea(e.dependencies)?(typeof s==`function`&&(Vs(t,n,s,r),p=t.memoizedState),(u=Va||Us(t,n,u,r,f,p,c)||e!==null&&e.dependencies!==null&&ea(e.dependencies))?(l||typeof a.UNSAFE_componentWillUpdate!=`function`&&typeof a.componentWillUpdate!=`function`||(typeof a.componentWillUpdate==`function`&&a.componentWillUpdate(r,p,c),typeof a.UNSAFE_componentWillUpdate==`function`&&a.UNSAFE_componentWillUpdate(r,p,c)),typeof a.componentDidUpdate==`function`&&(t.flags|=4),typeof a.getSnapshotBeforeUpdate==`function`&&(t.flags|=1024)):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),t.memoizedProps=r,t.memoizedState=p),a.props=r,a.state=p,a.context=c,r=u):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),r=!1)}return a=r,fc(e,t),r=(t.flags&128)!=0,a||r?(a=t.stateNode,n=r&&typeof n.getDerivedStateFromError!=`function`?null:a.render(),t.flags|=1,e!==null&&r?(t.child=za(t,e.child,null,i),t.child=za(t,null,n,i)):G(e,t,n,i),t.memoizedState=a.state,e=t.child):e=Dc(e,t,i),e}function gc(e,t,n,r){return Ui(),t.flags|=256,G(e,t,n,r),t.child}var _c={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function vc(e){return{baseLanes:e,cachePool:Sa()}}function yc(e,t,n){return e=e===null?0:e.childLanes&~n,t&&(e|=Jl),e}function bc(e,t,n){var r=t.pendingProps,i=!1,o=(t.flags&128)!=0,s;if((s=o)||(s=e!==null&&e.memoizedState===null?!1:(lo.current&2)!=0),s&&(i=!0,t.flags&=-129),s=(t.flags&32)!=0,t.flags&=-33,e===null){if(R){if(i?io(t):so(t),(e=L)?(e=rf(e,Li),e=e!==null&&e.data!==`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:ki===null?null:{id:F,overflow:Ai},retryLane:536870912,hydrationErrors:null},n=yi(e),n.return=t,t.child=n,I=t,L=null)):e=null,e===null)throw zi(t);return of(e)?t.lanes=32:t.lanes=536870912,null}var c=r.children;return r=r.fallback,i?(so(t),i=t.mode,c=Sc({mode:`hidden`,children:c},i),r=_i(r,i,n,null),c.return=t,r.return=t,c.sibling=r,t.child=c,r=t.child,r.memoizedState=vc(n),r.childLanes=yc(e,s,n),t.memoizedState=_c,sc(null,r)):(io(t),xc(t,c))}var l=e.memoizedState;if(l!==null&&(c=l.dehydrated,c!==null)){if(o)t.flags&256?(io(t),t.flags&=-257,t=Cc(e,t,n)):t.memoizedState===null?(so(t),c=r.fallback,i=t.mode,r=Sc({mode:`visible`,children:r.children},i),c=_i(c,i,n,null),c.flags|=2,r.return=t,c.return=t,r.sibling=c,t.child=r,za(t,e.child,null,n),r=t.child,r.memoizedState=vc(n),r.childLanes=yc(e,s,n),t.memoizedState=_c,t=sc(null,r)):(so(t),t.child=e.child,t.flags|=128,t=null);else if(io(t),of(c)){if(s=c.nextSibling&&c.nextSibling.dataset,s)var u=s.dgst;s=u,r=Error(a(419)),r.stack=``,r.digest=s,Gi({value:r,source:null,stack:null}),t=Cc(e,t,n)}else if(nc||$i(e,t,n,!1),s=(n&e.childLanes)!==0,nc||s){if(s=q,s!==null&&(r=ot(s,n),r!==0&&r!==l.retryLane))throw l.retryLane=r,si(e,r),hu(s,e,r),tc;af(c)||Du(),t=Cc(e,t,n)}else af(c)?(t.flags|=192,t.child=e.child,t=null):(e=l.treeContext,L=cf(c.nextSibling),I=t,R=!0,Ii=null,Li=!1,e!==null&&Fi(t,e),t=xc(t,r.children),t.flags|=4096);return t}return i?(so(t),c=r.fallback,i=t.mode,l=e.child,u=l.sibling,r=mi(l,{mode:`hidden`,children:r.children}),r.subtreeFlags=l.subtreeFlags&65011712,u===null?(c=_i(c,i,n,null),c.flags|=2):c=mi(u,c),c.return=t,r.return=t,r.sibling=c,t.child=r,sc(null,r),r=t.child,c=e.child.memoizedState,c===null?c=vc(n):(i=c.cachePool,i===null?i=Sa():(l=ca._currentValue,i=i.parent===l?i:{parent:l,pool:l}),c={baseLanes:c.baseLanes|n,cachePool:i}),r.memoizedState=c,r.childLanes=yc(e,s,n),t.memoizedState=_c,sc(e.child,r)):(io(t),n=e.child,e=n.sibling,n=mi(n,{mode:`visible`,children:r.children}),n.return=t,n.sibling=null,e!==null&&(s=t.deletions,s===null?(t.deletions=[e],t.flags|=16):s.push(e)),t.child=n,t.memoizedState=null,n)}function xc(e,t){return t=Sc({mode:`visible`,children:t},e.mode),t.return=e,e.child=t}function Sc(e,t){return e=fi(22,e,null,t),e.lanes=0,e}function Cc(e,t,n){return za(t,e.child,null,n),e=xc(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function wc(e,t,n){e.lanes|=t;var r=e.alternate;r!==null&&(r.lanes|=t),Zi(e.return,t,n)}function Tc(e,t,n,r,i,a){var o=e.memoizedState;o===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:r,tail:n,tailMode:i,treeForkCount:a}:(o.isBackwards=t,o.rendering=null,o.renderingStartTime=0,o.last=r,o.tail=n,o.tailMode=i,o.treeForkCount=a)}function Ec(e,t,n){var r=t.pendingProps,i=r.revealOrder,a=r.tail;r=r.children;var o=lo.current,s=(o&2)!=0;if(s?(o=o&1|2,t.flags|=128):o&=1,M(lo,o),G(e,t,r,n),r=R?Ei:0,!s&&e!==null&&e.flags&128)a:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&wc(e,n,t);else if(e.tag===19)wc(e,n,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break a;for(;e.sibling===null;){if(e.return===null||e.return===t)break a;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(i){case`forwards`:for(n=t.child,i=null;n!==null;)e=n.alternate,e!==null&&uo(e)===null&&(i=n),n=n.sibling;n=i,n===null?(i=t.child,t.child=null):(i=n.sibling,n.sibling=null),Tc(t,!1,i,n,a,r);break;case`backwards`:case`unstable_legacy-backwards`:for(n=null,i=t.child,t.child=null;i!==null;){if(e=i.alternate,e!==null&&uo(e)===null){t.child=i;break}e=i.sibling,i.sibling=n,n=i,i=e}Tc(t,!0,n,null,a,r);break;case`together`:Tc(t,!1,null,null,void 0,r);break;default:t.memoizedState=null}return t.child}function Dc(e,t,n){if(e!==null&&(t.dependencies=e.dependencies),Gl|=t.lanes,(n&t.childLanes)===0)if(e!==null){if($i(e,t,n,!1),(n&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(a(153));if(t.child!==null){for(e=t.child,n=mi(e,e.pendingProps),t.child=n,n.return=t;e.sibling!==null;)e=e.sibling,n=n.sibling=mi(e,e.pendingProps),n.return=t;n.sibling=null}return t.child}function Oc(e,t){return(e.lanes&t)===0?(e=e.dependencies,!!(e!==null&&ea(e))):!0}function kc(e,t,n){switch(t.tag){case 3:he(t,t.stateNode.containerInfo),Yi(t,ca,e.memoizedState.cache),Ui();break;case 27:case 5:_e(t);break;case 4:he(t,t.stateNode.containerInfo);break;case 10:Yi(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,ao(t),null;break;case 13:var r=t.memoizedState;if(r!==null)return r.dehydrated===null?(n&t.child.childLanes)===0?(io(t),e=Dc(e,t,n),e===null?null:e.sibling):bc(e,t,n):(io(t),t.flags|=128,null);io(t);break;case 19:var i=(e.flags&128)!=0;if(r=(n&t.childLanes)!==0,r||=($i(e,t,n,!1),(n&t.childLanes)!==0),i){if(r)return Ec(e,t,n);t.flags|=128}if(i=t.memoizedState,i!==null&&(i.rendering=null,i.tail=null,i.lastEffect=null),M(lo,lo.current),r)break;return null;case 22:return t.lanes=0,oc(e,t,n,t.pendingProps);case 24:Yi(t,ca,e.memoizedState.cache)}return Dc(e,t,n)}function Ac(e,t,n){if(e!==null)if(e.memoizedProps!==t.pendingProps)nc=!0;else{if(!Oc(e,n)&&!(t.flags&128))return nc=!1,kc(e,t,n);nc=!!(e.flags&131072)}else nc=!1,R&&t.flags&1048576&&Mi(t,Ei,t.index);switch(t.lanes=0,t.tag){case 16:a:{var r=t.pendingProps;if(e=ka(t.elementType),t.type=e,typeof e==`function`)pi(e)?(r=Gs(e,r),t.tag=1,t=hc(null,t,e,r,n)):(t.tag=0,t=pc(null,t,e,r,n));else{if(e!=null){var i=e.$$typeof;if(i===C){t.tag=11,t=rc(null,t,e,r,n);break a}else if(i===ne){t.tag=14,t=ic(null,t,e,r,n);break a}}throw t=D(e)||e,Error(a(306,t,``))}}return t;case 0:return pc(e,t,t.type,t.pendingProps,n);case 1:return r=t.type,i=Gs(r,t.pendingProps),hc(e,t,r,i,n);case 3:a:{if(he(t,t.stateNode.containerInfo),e===null)throw Error(a(387));r=t.pendingProps;var o=t.memoizedState;i=o.element,Ua(e,t),Xa(t,r,null,n);var s=t.memoizedState;if(r=s.cache,Yi(t,ca,r),r!==o.cache&&Qi(t,[ca],n,!0),Ya(),r=s.element,o.isDehydrated)if(o={element:r,isDehydrated:!1,cache:s.cache},t.updateQueue.baseState=o,t.memoizedState=o,t.flags&256){t=gc(e,t,r,n);break a}else if(r!==i){i=Si(Error(a(424)),t),Gi(i),t=gc(e,t,r,n);break a}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName===`HTML`?e.ownerDocument.body:e}for(L=cf(e.firstChild),I=t,R=!0,Ii=null,Li=!0,n=Ba(t,null,r,n),t.child=n;n;)n.flags=n.flags&-3|4096,n=n.sibling}else{if(Ui(),r===i){t=Dc(e,t,n);break a}G(e,t,r,n)}t=t.child}return t;case 26:return fc(e,t),e===null?(n=kf(t.type,null,t.pendingProps,null))?t.memoizedState=n:R||(n=t.type,e=t.pendingProps,r=Bd(pe.current).createElement(n),r[ft]=t,r[N]=e,Pd(r,n,e),wt(r),t.stateNode=r):t.memoizedState=kf(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return _e(t),e===null&&R&&(r=t.stateNode=ff(t.type,t.pendingProps,pe.current),I=t,Li=!0,i=L,Zd(t.type)?(lf=i,L=cf(r.firstChild)):L=i),G(e,t,t.pendingProps.children,n),fc(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&R&&((i=r=L)&&(r=tf(r,t.type,t.pendingProps,Li),r===null?i=!1:(t.stateNode=r,I=t,L=cf(r.firstChild),Li=!1,i=!0)),i||zi(t)),_e(t),i=t.type,o=t.pendingProps,s=e===null?null:e.memoizedProps,r=o.children,Ud(i,o)?r=null:s!==null&&Ud(i,s)&&(t.flags|=32),t.memoizedState!==null&&(i=So(e,t,To,null,null,n),Qf._currentValue=i),fc(e,t),G(e,t,r,n),t.child;case 6:return e===null&&R&&((e=n=L)&&(n=nf(n,t.pendingProps,Li),n===null?e=!1:(t.stateNode=n,I=t,L=null,e=!0)),e||zi(t)),null;case 13:return bc(e,t,n);case 4:return he(t,t.stateNode.containerInfo),r=t.pendingProps,e===null?t.child=za(t,null,r,n):G(e,t,r,n),t.child;case 11:return rc(e,t,t.type,t.pendingProps,n);case 7:return G(e,t,t.pendingProps,n),t.child;case 8:return G(e,t,t.pendingProps.children,n),t.child;case 12:return G(e,t,t.pendingProps.children,n),t.child;case 10:return r=t.pendingProps,Yi(t,t.type,r.value),G(e,t,r.children,n),t.child;case 9:return i=t.type._context,r=t.pendingProps.children,ta(t),i=na(i),r=r(i),t.flags|=1,G(e,t,r,n),t.child;case 14:return ic(e,t,t.type,t.pendingProps,n);case 15:return ac(e,t,t.type,t.pendingProps,n);case 19:return Ec(e,t,n);case 31:return dc(e,t,n);case 22:return oc(e,t,n,t.pendingProps);case 24:return ta(t),r=na(ca),e===null?(i=ba(),i===null&&(i=q,o=la(),i.pooledCache=o,o.refCount++,o!==null&&(i.pooledCacheLanes|=n),i=o),t.memoizedState={parent:r,cache:i},Ha(t),Yi(t,ca,i)):((e.lanes&n)!==0&&(Ua(e,t),Xa(t,null,null,n),Ya()),i=e.memoizedState,o=t.memoizedState,i.parent===r?(r=o.cache,Yi(t,ca,r),r!==i.cache&&Qi(t,[ca],n,!0)):(i={parent:r,cache:r},t.memoizedState=i,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=i),Yi(t,ca,r))),G(e,t,t.pendingProps.children,n),t.child;case 29:throw t.pendingProps}throw Error(a(156,t.tag))}function jc(e){e.flags|=4}function Mc(e,t,n,r,i){if((t=(e.mode&32)!=0)&&(t=!1),t){if(e.flags|=16777216,(i&335544128)===i)if(e.stateNode.complete)e.flags|=8192;else if(wu())e.flags|=8192;else throw Aa=Ea,wa}else e.flags&=-16777217}function Nc(e,t){if(t.type!==`stylesheet`||t.state.loading&4)e.flags&=-16777217;else if(e.flags|=16777216,!Wf(t))if(wu())e.flags|=8192;else throw Aa=Ea,wa}function Pc(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag===22?536870912:et(),e.lanes|=t,Yl|=t)}function Fc(e,t){if(!R)switch(e.tailMode){case`hidden`:t=e.tail;for(var n=null;t!==null;)t.alternate!==null&&(n=t),t=t.sibling;n===null?e.tail=null:n.sibling=null;break;case`collapsed`:n=e.tail;for(var r=null;n!==null;)n.alternate!==null&&(r=n),n=n.sibling;r===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:r.sibling=null}}function Ic(e){var t=e.alternate!==null&&e.alternate.child===e.child,n=0,r=0;if(t)for(var i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags&65011712,r|=i.flags&65011712,i.return=e,i=i.sibling;else for(i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags,r|=i.flags,i.return=e,i=i.sibling;return e.subtreeFlags|=r,e.childLanes=n,t}function Lc(e,t,n){var r=t.pendingProps;switch(Pi(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return Ic(t),null;case 1:return Ic(t),null;case 3:return n=t.stateNode,r=null,e!==null&&(r=e.memoizedState.cache),t.memoizedState.cache!==r&&(t.flags|=2048),Xi(ca),ge(),n.pendingContext&&(n.context=n.pendingContext,n.pendingContext=null),(e===null||e.child===null)&&(Hi(t)?jc(t):e===null||e.memoizedState.isDehydrated&&!(t.flags&256)||(t.flags|=1024,Wi())),Ic(t),null;case 26:var i=t.type,o=t.memoizedState;return e===null?(jc(t),o===null?(Ic(t),Mc(t,i,null,r,n)):(Ic(t),Nc(t,o))):o?o===e.memoizedState?(Ic(t),t.flags&=-16777217):(jc(t),Ic(t),Nc(t,o)):(e=e.memoizedProps,e!==r&&jc(t),Ic(t),Mc(t,i,e,r,n)),null;case 27:if(ve(t),n=pe.current,i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&jc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Ic(t),null}e=de.current,Hi(t)?Bi(t,e):(e=ff(i,r,n),t.stateNode=e,jc(t))}return Ic(t),null;case 5:if(ve(t),i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&jc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Ic(t),null}if(o=de.current,Hi(t))Bi(t,o);else{var s=Bd(pe.current);switch(o){case 1:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case 2:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;default:switch(i){case`svg`:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case`math`:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;case`script`:o=s.createElement(`div`),o.innerHTML=`<script><\/script>`,o=o.removeChild(o.firstChild);break;case`select`:o=typeof r.is==`string`?s.createElement(`select`,{is:r.is}):s.createElement(`select`),r.multiple?o.multiple=!0:r.size&&(o.size=r.size);break;default:o=typeof r.is==`string`?s.createElement(i,{is:r.is}):s.createElement(i)}}o[ft]=t,o[N]=r;a:for(s=t.child;s!==null;){if(s.tag===5||s.tag===6)o.appendChild(s.stateNode);else if(s.tag!==4&&s.tag!==27&&s.child!==null){s.child.return=s,s=s.child;continue}if(s===t)break a;for(;s.sibling===null;){if(s.return===null||s.return===t)break a;s=s.return}s.sibling.return=s.return,s=s.sibling}t.stateNode=o;a:switch(Pd(o,i,r),i){case`button`:case`input`:case`select`:case`textarea`:r=!!r.autoFocus;break a;case`img`:r=!0;break a;default:r=!1}r&&jc(t)}}return Ic(t),Mc(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,n),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==r&&jc(t);else{if(typeof r!=`string`&&t.stateNode===null)throw Error(a(166));if(e=pe.current,Hi(t)){if(e=t.stateNode,n=t.memoizedProps,r=null,i=I,i!==null)switch(i.tag){case 27:case 5:r=i.memoizedProps}e[ft]=t,e=!!(e.nodeValue===n||r!==null&&!0===r.suppressHydrationWarning||Md(e.nodeValue,n)),e||zi(t,!0)}else e=Bd(e).createTextNode(r),e[ft]=t,t.stateNode=e}return Ic(t),null;case 31:if(n=t.memoizedState,e===null||e.memoizedState!==null){if(r=Hi(t),n!==null){if(e===null){if(!r)throw Error(a(318));if(e=t.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(557));e[ft]=t}else Ui(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Ic(t),e=!1}else n=Wi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=n),e=!0;if(!e)return t.flags&256?(co(t),t):(co(t),null);if(t.flags&128)throw Error(a(558))}return Ic(t),null;case 13:if(r=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(i=Hi(t),r!==null&&r.dehydrated!==null){if(e===null){if(!i)throw Error(a(318));if(i=t.memoizedState,i=i===null?null:i.dehydrated,!i)throw Error(a(317));i[ft]=t}else Ui(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Ic(t),i=!1}else i=Wi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=i),i=!0;if(!i)return t.flags&256?(co(t),t):(co(t),null)}return co(t),t.flags&128?(t.lanes=n,t):(n=r!==null,e=e!==null&&e.memoizedState!==null,n&&(r=t.child,i=null,r.alternate!==null&&r.alternate.memoizedState!==null&&r.alternate.memoizedState.cachePool!==null&&(i=r.alternate.memoizedState.cachePool.pool),o=null,r.memoizedState!==null&&r.memoizedState.cachePool!==null&&(o=r.memoizedState.cachePool.pool),o!==i&&(r.flags|=2048)),n!==e&&n&&(t.child.flags|=8192),Pc(t,t.updateQueue),Ic(t),null);case 4:return ge(),e===null&&Sd(t.stateNode.containerInfo),Ic(t),null;case 10:return Xi(t.type),Ic(t),null;case 19:if(j(lo),r=t.memoizedState,r===null)return Ic(t),null;if(i=(t.flags&128)!=0,o=r.rendering,o===null)if(i)Fc(r,!1);else{if(Wl!==0||e!==null&&e.flags&128)for(e=t.child;e!==null;){if(o=uo(e),o!==null){for(t.flags|=128,Fc(r,!1),e=o.updateQueue,t.updateQueue=e,Pc(t,e),t.subtreeFlags=0,e=n,n=t.child;n!==null;)hi(n,e),n=n.sibling;return M(lo,lo.current&1|2),R&&ji(t,r.treeForkCount),t.child}e=e.sibling}r.tail!==null&&je()>tu&&(t.flags|=128,i=!0,Fc(r,!1),t.lanes=4194304)}else{if(!i)if(e=uo(o),e!==null){if(t.flags|=128,i=!0,e=e.updateQueue,t.updateQueue=e,Pc(t,e),Fc(r,!0),r.tail===null&&r.tailMode===`hidden`&&!o.alternate&&!R)return Ic(t),null}else 2*je()-r.renderingStartTime>tu&&n!==536870912&&(t.flags|=128,i=!0,Fc(r,!1),t.lanes=4194304);r.isBackwards?(o.sibling=t.child,t.child=o):(e=r.last,e===null?t.child=o:e.sibling=o,r.last=o)}return r.tail===null?(Ic(t),null):(e=r.tail,r.rendering=e,r.tail=e.sibling,r.renderingStartTime=je(),e.sibling=null,n=lo.current,M(lo,i?n&1|2:n&1),R&&ji(t,r.treeForkCount),e);case 22:case 23:return co(t),no(),r=t.memoizedState!==null,e===null?r&&(t.flags|=8192):e.memoizedState!==null!==r&&(t.flags|=8192),r?n&536870912&&!(t.flags&128)&&(Ic(t),t.subtreeFlags&6&&(t.flags|=8192)):Ic(t),n=t.updateQueue,n!==null&&Pc(t,n.retryQueue),n=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),r=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(r=t.memoizedState.cachePool.pool),r!==n&&(t.flags|=2048),e!==null&&j(ya),null;case 24:return n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),Xi(ca),Ic(t),null;case 25:return null;case 30:return null}throw Error(a(156,t.tag))}function Rc(e,t){switch(Pi(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return Xi(ca),ge(),e=t.flags,e&65536&&!(e&128)?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return ve(t),null;case 31:if(t.memoizedState!==null){if(co(t),t.alternate===null)throw Error(a(340));Ui()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(co(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(a(340));Ui()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return j(lo),null;case 4:return ge(),null;case 10:return Xi(t.type),null;case 22:case 23:return co(t),no(),e!==null&&j(ya),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return Xi(ca),null;case 25:return null;default:return null}}function zc(e,t){switch(Pi(t),t.tag){case 3:Xi(ca),ge();break;case 26:case 27:case 5:ve(t);break;case 4:ge();break;case 31:t.memoizedState!==null&&co(t);break;case 13:co(t);break;case 19:j(lo);break;case 10:Xi(t.type);break;case 22:case 23:co(t),no(),e!==null&&j(ya);break;case 24:Xi(ca)}}function Bc(e,t){try{var n=t.updateQueue,r=n===null?null:n.lastEffect;if(r!==null){var i=r.next;n=i;do{if((n.tag&e)===e){r=void 0;var a=n.create,o=n.inst;r=a(),o.destroy=r}n=n.next}while(n!==i)}}catch(e){Z(t,t.return,e)}}function Vc(e,t,n){try{var r=t.updateQueue,i=r===null?null:r.lastEffect;if(i!==null){var a=i.next;r=a;do{if((r.tag&e)===e){var o=r.inst,s=o.destroy;if(s!==void 0){o.destroy=void 0,i=t;var c=n,l=s;try{l()}catch(e){Z(i,c,e)}}}r=r.next}while(r!==a)}}catch(e){Z(t,t.return,e)}}function Hc(e){var t=e.updateQueue;if(t!==null){var n=e.stateNode;try{Qa(t,n)}catch(t){Z(e,e.return,t)}}}function Uc(e,t,n){n.props=Gs(e.type,e.memoizedProps),n.state=e.memoizedState;try{n.componentWillUnmount()}catch(n){Z(e,t,n)}}function Wc(e,t){try{var n=e.ref;if(n!==null){switch(e.tag){case 26:case 27:case 5:var r=e.stateNode;break;case 30:r=e.stateNode;break;default:r=e.stateNode}typeof n==`function`?e.refCleanup=n(r):n.current=r}}catch(n){Z(e,t,n)}}function Gc(e,t){var n=e.ref,r=e.refCleanup;if(n!==null)if(typeof r==`function`)try{r()}catch(n){Z(e,t,n)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof n==`function`)try{n(null)}catch(n){Z(e,t,n)}else n.current=null}function Kc(e){var t=e.type,n=e.memoizedProps,r=e.stateNode;try{a:switch(t){case`button`:case`input`:case`select`:case`textarea`:n.autoFocus&&r.focus();break a;case`img`:n.src?r.src=n.src:n.srcSet&&(r.srcset=n.srcSet)}}catch(t){Z(e,e.return,t)}}function qc(e,t,n){try{var r=e.stateNode;Fd(r,e.type,n,t),r[N]=t}catch(t){Z(e,e.return,t)}}function Jc(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&Zd(e.type)||e.tag===4}function Yc(e){a:for(;;){for(;e.sibling===null;){if(e.return===null||Jc(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&Zd(e.type)||e.flags&2||e.child===null||e.tag===4)continue a;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function Xc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?(n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n).insertBefore(e,t):(t=n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n,t.appendChild(e),n=n._reactRootContainer,n!=null||t.onclick!==null||(t.onclick=an));else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode,t=null),e=e.child,e!==null))for(Xc(e,t,n),e=e.sibling;e!==null;)Xc(e,t,n),e=e.sibling}function Zc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?n.insertBefore(e,t):n.appendChild(e);else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode),e=e.child,e!==null))for(Zc(e,t,n),e=e.sibling;e!==null;)Zc(e,t,n),e=e.sibling}function Qc(e){var t=e.stateNode,n=e.memoizedProps;try{for(var r=e.type,i=t.attributes;i.length;)t.removeAttributeNode(i[0]);Pd(t,r,n),t[ft]=e,t[N]=n}catch(t){Z(e,e.return,t)}}var $c=!1,el=!1,tl=!1,nl=typeof WeakSet==`function`?WeakSet:Set,rl=null;function il(e,t){if(e=e.containerInfo,Rd=sp,e=jr(e),Mr(e)){if(`selectionStart`in e)var n={start:e.selectionStart,end:e.selectionEnd};else a:{n=(n=e.ownerDocument)&&n.defaultView||window;var r=n.getSelection&&n.getSelection();if(r&&r.rangeCount!==0){n=r.anchorNode;var i=r.anchorOffset,o=r.focusNode;r=r.focusOffset;try{n.nodeType,o.nodeType}catch{n=null;break a}var s=0,c=-1,l=-1,u=0,d=0,f=e,p=null;b:for(;;){for(var m;f!==n||i!==0&&f.nodeType!==3||(c=s+i),f!==o||r!==0&&f.nodeType!==3||(l=s+r),f.nodeType===3&&(s+=f.nodeValue.length),(m=f.firstChild)!==null;)p=f,f=m;for(;;){if(f===e)break b;if(p===n&&++u===i&&(c=s),p===o&&++d===r&&(l=s),(m=f.nextSibling)!==null)break;f=p,p=f.parentNode}f=m}n=c===-1||l===-1?null:{start:c,end:l}}else n=null}n||={start:0,end:0}}else n=null;for(zd={focusedElem:e,selectionRange:n},sp=!1,rl=t;rl!==null;)if(t=rl,e=t.child,t.subtreeFlags&1028&&e!==null)e.return=t,rl=e;else for(;rl!==null;){switch(t=rl,o=t.alternate,e=t.flags,t.tag){case 0:if(e&4&&(e=t.updateQueue,e=e===null?null:e.events,e!==null))for(n=0;n<e.length;n++)i=e[n],i.ref.impl=i.nextImpl;break;case 11:case 15:break;case 1:if(e&1024&&o!==null){e=void 0,n=t,i=o.memoizedProps,o=o.memoizedState,r=n.stateNode;try{var h=Gs(n.type,i);e=r.getSnapshotBeforeUpdate(h,o),r.__reactInternalSnapshotBeforeUpdate=e}catch(e){Z(n,n.return,e)}}break;case 3:if(e&1024){if(e=t.stateNode.containerInfo,n=e.nodeType,n===9)ef(e);else if(n===1)switch(e.nodeName){case`HEAD`:case`HTML`:case`BODY`:ef(e);break;default:e.textContent=``}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if(e&1024)throw Error(a(163))}if(e=t.sibling,e!==null){e.return=t.return,rl=e;break}rl=t.return}}function al(e,t,n){var r=n.flags;switch(n.tag){case 0:case 11:case 15:bl(e,n),r&4&&Bc(5,n);break;case 1:if(bl(e,n),r&4)if(e=n.stateNode,t===null)try{e.componentDidMount()}catch(e){Z(n,n.return,e)}else{var i=Gs(n.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(i,t,e.__reactInternalSnapshotBeforeUpdate)}catch(e){Z(n,n.return,e)}}r&64&&Hc(n),r&512&&Wc(n,n.return);break;case 3:if(bl(e,n),r&64&&(e=n.updateQueue,e!==null)){if(t=null,n.child!==null)switch(n.child.tag){case 27:case 5:t=n.child.stateNode;break;case 1:t=n.child.stateNode}try{Qa(e,t)}catch(e){Z(n,n.return,e)}}break;case 27:t===null&&r&4&&Qc(n);case 26:case 5:bl(e,n),t===null&&r&4&&Kc(n),r&512&&Wc(n,n.return);break;case 12:bl(e,n);break;case 31:bl(e,n),r&4&&dl(e,n);break;case 13:bl(e,n),r&4&&fl(e,n),r&64&&(e=n.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(n=Ju.bind(null,n),sf(e,n))));break;case 22:if(r=n.memoizedState!==null||$c,!r){t=t!==null&&t.memoizedState!==null||el,i=$c;var a=el;$c=r,(el=t)&&!a?Sl(e,n,(n.subtreeFlags&8772)!=0):bl(e,n),$c=i,el=a}break;case 30:break;default:bl(e,n)}}function ol(e){var t=e.alternate;t!==null&&(e.alternate=null,ol(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&yt(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var sl=null,cl=!1;function ll(e,t,n){for(n=n.child;n!==null;)ul(e,t,n),n=n.sibling}function ul(e,t,n){if(Ve&&typeof Ve.onCommitFiberUnmount==`function`)try{Ve.onCommitFiberUnmount(Be,n)}catch{}switch(n.tag){case 26:el||Gc(n,t),ll(e,t,n),n.memoizedState?n.memoizedState.count--:n.stateNode&&(n=n.stateNode,n.parentNode.removeChild(n));break;case 27:el||Gc(n,t);var r=sl,i=cl;Zd(n.type)&&(sl=n.stateNode,cl=!1),ll(e,t,n),pf(n.stateNode),sl=r,cl=i;break;case 5:el||Gc(n,t);case 6:if(r=sl,i=cl,sl=null,ll(e,t,n),sl=r,cl=i,sl!==null)if(cl)try{(sl.nodeType===9?sl.body:sl.nodeName===`HTML`?sl.ownerDocument.body:sl).removeChild(n.stateNode)}catch(e){Z(n,t,e)}else try{sl.removeChild(n.stateNode)}catch(e){Z(n,t,e)}break;case 18:sl!==null&&(cl?(e=sl,Qd(e.nodeType===9?e.body:e.nodeName===`HTML`?e.ownerDocument.body:e,n.stateNode),Np(e)):Qd(sl,n.stateNode));break;case 4:r=sl,i=cl,sl=n.stateNode.containerInfo,cl=!0,ll(e,t,n),sl=r,cl=i;break;case 0:case 11:case 14:case 15:Vc(2,n,t),el||Vc(4,n,t),ll(e,t,n);break;case 1:el||(Gc(n,t),r=n.stateNode,typeof r.componentWillUnmount==`function`&&Uc(n,t,r)),ll(e,t,n);break;case 21:ll(e,t,n);break;case 22:el=(r=el)||n.memoizedState!==null,ll(e,t,n),el=r;break;default:ll(e,t,n)}}function dl(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{Np(e)}catch(e){Z(t,t.return,e)}}}function fl(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{Np(e)}catch(e){Z(t,t.return,e)}}function pl(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new nl),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new nl),t;default:throw Error(a(435,e.tag))}}function ml(e,t){var n=pl(e);t.forEach(function(t){if(!n.has(t)){n.add(t);var r=Yu.bind(null,e,t);t.then(r,r)}})}function hl(e,t){var n=t.deletions;if(n!==null)for(var r=0;r<n.length;r++){var i=n[r],o=e,s=t,c=s;a:for(;c!==null;){switch(c.tag){case 27:if(Zd(c.type)){sl=c.stateNode,cl=!1;break a}break;case 5:sl=c.stateNode,cl=!1;break a;case 3:case 4:sl=c.stateNode.containerInfo,cl=!0;break a}c=c.return}if(sl===null)throw Error(a(160));ul(o,s,i),sl=null,cl=!1,o=i.alternate,o!==null&&(o.return=null),i.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)_l(t,e),t=t.sibling}var gl=null;function _l(e,t){var n=e.alternate,r=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:hl(t,e),vl(e),r&4&&(Vc(3,e,e.return),Bc(3,e),Vc(5,e,e.return));break;case 1:hl(t,e),vl(e),r&512&&(el||n===null||Gc(n,n.return)),r&64&&$c&&(e=e.updateQueue,e!==null&&(r=e.callbacks,r!==null&&(n=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=n===null?r:n.concat(r))));break;case 26:var i=gl;if(hl(t,e),vl(e),r&512&&(el||n===null||Gc(n,n.return)),r&4){var o=n===null?null:n.memoizedState;if(r=e.memoizedState,n===null)if(r===null)if(e.stateNode===null){a:{r=e.type,n=e.memoizedProps,i=i.ownerDocument||i;b:switch(r){case`title`:o=i.getElementsByTagName(`title`)[0],(!o||o[vt]||o[ft]||o.namespaceURI===`http://www.w3.org/2000/svg`||o.hasAttribute(`itemprop`))&&(o=i.createElement(r),i.head.insertBefore(o,i.querySelector(`head > title`))),Pd(o,r,n),o[ft]=e,wt(o),r=o;break a;case`link`:var s=Vf(`link`,`href`,i).get(r+(n.href||``));if(s){for(var c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`href`)===(n.href==null||n.href===``?null:n.href)&&o.getAttribute(`rel`)===(n.rel==null?null:n.rel)&&o.getAttribute(`title`)===(n.title==null?null:n.title)&&o.getAttribute(`crossorigin`)===(n.crossOrigin==null?null:n.crossOrigin)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;case`meta`:if(s=Vf(`meta`,`content`,i).get(r+(n.content||``))){for(c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`content`)===(n.content==null?null:``+n.content)&&o.getAttribute(`name`)===(n.name==null?null:n.name)&&o.getAttribute(`property`)===(n.property==null?null:n.property)&&o.getAttribute(`http-equiv`)===(n.httpEquiv==null?null:n.httpEquiv)&&o.getAttribute(`charset`)===(n.charSet==null?null:n.charSet)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;default:throw Error(a(468,r))}o[ft]=e,wt(o),r=o}e.stateNode=r}else Hf(i,e.type,e.stateNode);else e.stateNode=If(i,r,e.memoizedProps);else o===r?r===null&&e.stateNode!==null&&qc(e,e.memoizedProps,n.memoizedProps):(o===null?n.stateNode!==null&&(n=n.stateNode,n.parentNode.removeChild(n)):o.count--,r===null?Hf(i,e.type,e.stateNode):If(i,r,e.memoizedProps))}break;case 27:hl(t,e),vl(e),r&512&&(el||n===null||Gc(n,n.return)),n!==null&&r&4&&qc(e,e.memoizedProps,n.memoizedProps);break;case 5:if(hl(t,e),vl(e),r&512&&(el||n===null||Gc(n,n.return)),e.flags&32){i=e.stateNode;try{Xt(i,``)}catch(t){Z(e,e.return,t)}}r&4&&e.stateNode!=null&&(i=e.memoizedProps,qc(e,i,n===null?i:n.memoizedProps)),r&1024&&(tl=!0);break;case 6:if(hl(t,e),vl(e),r&4){if(e.stateNode===null)throw Error(a(162));r=e.memoizedProps,n=e.stateNode;try{n.nodeValue=r}catch(t){Z(e,e.return,t)}}break;case 3:if(Bf=null,i=gl,gl=gf(t.containerInfo),hl(t,e),gl=i,vl(e),r&4&&n!==null&&n.memoizedState.isDehydrated)try{Np(t.containerInfo)}catch(t){Z(e,e.return,t)}tl&&(tl=!1,yl(e));break;case 4:r=gl,gl=gf(e.stateNode.containerInfo),hl(t,e),vl(e),gl=r;break;case 12:hl(t,e),vl(e);break;case 31:hl(t,e),vl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 13:hl(t,e),vl(e),e.child.flags&8192&&e.memoizedState!==null!=(n!==null&&n.memoizedState!==null)&&($l=je()),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 22:i=e.memoizedState!==null;var l=n!==null&&n.memoizedState!==null,u=$c,d=el;if($c=u||i,el=d||l,hl(t,e),el=d,$c=u,vl(e),r&8192)a:for(t=e.stateNode,t._visibility=i?t._visibility&-2:t._visibility|1,i&&(n===null||l||$c||el||xl(e)),n=null,t=e;;){if(t.tag===5||t.tag===26){if(n===null){l=n=t;try{if(o=l.stateNode,i)s=o.style,typeof s.setProperty==`function`?s.setProperty(`display`,`none`,`important`):s.display=`none`;else{c=l.stateNode;var f=l.memoizedProps.style,p=f!=null&&f.hasOwnProperty(`display`)?f.display:null;c.style.display=p==null||typeof p==`boolean`?``:(``+p).trim()}}catch(e){Z(l,l.return,e)}}}else if(t.tag===6){if(n===null){l=t;try{l.stateNode.nodeValue=i?``:l.memoizedProps}catch(e){Z(l,l.return,e)}}}else if(t.tag===18){if(n===null){l=t;try{var m=l.stateNode;i?$d(m,!0):$d(l.stateNode,!1)}catch(e){Z(l,l.return,e)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break a;for(;t.sibling===null;){if(t.return===null||t.return===e)break a;n===t&&(n=null),t=t.return}n===t&&(n=null),t.sibling.return=t.return,t=t.sibling}r&4&&(r=e.updateQueue,r!==null&&(n=r.retryQueue,n!==null&&(r.retryQueue=null,ml(e,n))));break;case 19:hl(t,e),vl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 30:break;case 21:break;default:hl(t,e),vl(e)}}function vl(e){var t=e.flags;if(t&2){try{for(var n,r=e.return;r!==null;){if(Jc(r)){n=r;break}r=r.return}if(n==null)throw Error(a(160));switch(n.tag){case 27:var i=n.stateNode;Zc(e,Yc(e),i);break;case 5:var o=n.stateNode;n.flags&32&&(Xt(o,``),n.flags&=-33),Zc(e,Yc(e),o);break;case 3:case 4:var s=n.stateNode.containerInfo;Xc(e,Yc(e),s);break;default:throw Error(a(161))}}catch(t){Z(e,e.return,t)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function yl(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;yl(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function bl(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)al(e,t.alternate,t),t=t.sibling}function xl(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:Vc(4,t,t.return),xl(t);break;case 1:Gc(t,t.return);var n=t.stateNode;typeof n.componentWillUnmount==`function`&&Uc(t,t.return,n),xl(t);break;case 27:pf(t.stateNode);case 26:case 5:Gc(t,t.return),xl(t);break;case 22:t.memoizedState===null&&xl(t);break;case 30:xl(t);break;default:xl(t)}e=e.sibling}}function Sl(e,t,n){for(n&&=(t.subtreeFlags&8772)!=0,t=t.child;t!==null;){var r=t.alternate,i=e,a=t,o=a.flags;switch(a.tag){case 0:case 11:case 15:Sl(i,a,n),Bc(4,a);break;case 1:if(Sl(i,a,n),r=a,i=r.stateNode,typeof i.componentDidMount==`function`)try{i.componentDidMount()}catch(e){Z(r,r.return,e)}if(r=a,i=r.updateQueue,i!==null){var s=r.stateNode;try{var c=i.shared.hiddenCallbacks;if(c!==null)for(i.shared.hiddenCallbacks=null,i=0;i<c.length;i++)Za(c[i],s)}catch(e){Z(r,r.return,e)}}n&&o&64&&Hc(a),Wc(a,a.return);break;case 27:Qc(a);case 26:case 5:Sl(i,a,n),n&&r===null&&o&4&&Kc(a),Wc(a,a.return);break;case 12:Sl(i,a,n);break;case 31:Sl(i,a,n),n&&o&4&&dl(i,a);break;case 13:Sl(i,a,n),n&&o&4&&fl(i,a);break;case 22:a.memoizedState===null&&Sl(i,a,n),Wc(a,a.return);break;case 30:break;default:Sl(i,a,n)}t=t.sibling}}function Cl(e,t){var n=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==n&&(e!=null&&e.refCount++,n!=null&&ua(n))}function wl(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ua(e))}function Tl(e,t,n,r){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)El(e,t,n,r),t=t.sibling}function El(e,t,n,r){var i=t.flags;switch(t.tag){case 0:case 11:case 15:Tl(e,t,n,r),i&2048&&Bc(9,t);break;case 1:Tl(e,t,n,r);break;case 3:Tl(e,t,n,r),i&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ua(e)));break;case 12:if(i&2048){Tl(e,t,n,r),e=t.stateNode;try{var a=t.memoizedProps,o=a.id,s=a.onPostCommit;typeof s==`function`&&s(o,t.alternate===null?`mount`:`update`,e.passiveEffectDuration,-0)}catch(e){Z(t,t.return,e)}}else Tl(e,t,n,r);break;case 31:Tl(e,t,n,r);break;case 13:Tl(e,t,n,r);break;case 23:break;case 22:a=t.stateNode,o=t.alternate,t.memoizedState===null?a._visibility&2?Tl(e,t,n,r):(a._visibility|=2,Dl(e,t,n,r,(t.subtreeFlags&10256)!=0||!1)):a._visibility&2?Tl(e,t,n,r):Ol(e,t),i&2048&&Cl(o,t);break;case 24:Tl(e,t,n,r),i&2048&&wl(t.alternate,t);break;default:Tl(e,t,n,r)}}function Dl(e,t,n,r,i){for(i&&=(t.subtreeFlags&10256)!=0||!1,t=t.child;t!==null;){var a=e,o=t,s=n,c=r,l=o.flags;switch(o.tag){case 0:case 11:case 15:Dl(a,o,s,c,i),Bc(8,o);break;case 23:break;case 22:var u=o.stateNode;o.memoizedState===null?(u._visibility|=2,Dl(a,o,s,c,i)):u._visibility&2?Dl(a,o,s,c,i):Ol(a,o),i&&l&2048&&Cl(o.alternate,o);break;case 24:Dl(a,o,s,c,i),i&&l&2048&&wl(o.alternate,o);break;default:Dl(a,o,s,c,i)}t=t.sibling}}function Ol(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var n=e,r=t,i=r.flags;switch(r.tag){case 22:Ol(n,r),i&2048&&Cl(r.alternate,r);break;case 24:Ol(n,r),i&2048&&wl(r.alternate,r);break;default:Ol(n,r)}t=t.sibling}}var kl=8192;function Al(e,t,n){if(e.subtreeFlags&kl)for(e=e.child;e!==null;)jl(e,t,n),e=e.sibling}function jl(e,t,n){switch(e.tag){case 26:Al(e,t,n),e.flags&kl&&e.memoizedState!==null&&Gf(n,gl,e.memoizedState,e.memoizedProps);break;case 5:Al(e,t,n);break;case 3:case 4:var r=gl;gl=gf(e.stateNode.containerInfo),Al(e,t,n),gl=r;break;case 22:e.memoizedState===null&&(r=e.alternate,r!==null&&r.memoizedState!==null?(r=kl,kl=16777216,Al(e,t,n),kl=r):Al(e,t,n));break;default:Al(e,t,n)}}function Ml(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function Nl(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Il(r,e)}Ml(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)Pl(e),e=e.sibling}function Pl(e){switch(e.tag){case 0:case 11:case 15:Nl(e),e.flags&2048&&Vc(9,e,e.return);break;case 3:Nl(e);break;case 12:Nl(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,Fl(e)):Nl(e);break;default:Nl(e)}}function Fl(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Il(r,e)}Ml(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:Vc(8,t,t.return),Fl(t);break;case 22:n=t.stateNode,n._visibility&2&&(n._visibility&=-3,Fl(t));break;default:Fl(t)}e=e.sibling}}function Il(e,t){for(;rl!==null;){var n=rl;switch(n.tag){case 0:case 11:case 15:Vc(8,n,t);break;case 23:case 22:if(n.memoizedState!==null&&n.memoizedState.cachePool!==null){var r=n.memoizedState.cachePool.pool;r!=null&&r.refCount++}break;case 24:ua(n.memoizedState.cache)}if(r=n.child,r!==null)r.return=n,rl=r;else a:for(n=e;rl!==null;){r=rl;var i=r.sibling,a=r.return;if(ol(r),r===n){rl=null;break a}if(i!==null){i.return=a,rl=i;break a}rl=a}}}var Ll={getCacheForType:function(e){var t=na(ca),n=t.data.get(e);return n===void 0&&(n=e(),t.data.set(e,n)),n},cacheSignal:function(){return na(ca).controller.signal}},Rl=typeof WeakMap==`function`?WeakMap:Map,K=0,q=null,J=null,Y=0,X=0,zl=null,Bl=!1,Vl=!1,Hl=!1,Ul=0,Wl=0,Gl=0,Kl=0,ql=0,Jl=0,Yl=0,Xl=null,Zl=null,Ql=!1,$l=0,eu=0,tu=1/0,nu=null,ru=null,iu=0,au=null,ou=null,su=0,cu=0,lu=null,uu=null,du=0,fu=null;function pu(){return K&2&&Y!==0?Y&-Y:O.T===null?lt():dd()}function mu(){if(Jl===0)if(!(Y&536870912)||R){var e=Je;Je<<=1,!(Je&3932160)&&(Je=262144),Jl=e}else Jl=536870912;return e=ro.current,e!==null&&(e.flags|=32),Jl}function hu(e,t,n){(e===q&&(X===2||X===9)||e.cancelPendingCommit!==null)&&(Su(e,0),yu(e,Y,Jl,!1)),nt(e,n),(!(K&2)||e!==q)&&(e===q&&(!(K&2)&&(Kl|=n),Wl===4&&yu(e,Y,Jl,!1)),rd(e))}function gu(e,t,n){if(K&6)throw Error(a(327));var r=!n&&(t&127)==0&&(t&e.expiredLanes)===0||Qe(e,t),i=r?Au(e,t):Ou(e,t,!0),o=r;do{if(i===0){Vl&&!r&&yu(e,t,0,!1);break}else{if(n=e.current.alternate,o&&!vu(n)){i=Ou(e,t,!1),o=!1;continue}if(i===2){if(o=t,e.errorRecoveryDisabledLanes&o)var s=0;else s=e.pendingLanes&-536870913,s=s===0?s&536870912?536870912:0:s;if(s!==0){t=s;a:{var c=e;i=Xl;var l=c.current.memoizedState.isDehydrated;if(l&&(Su(c,s).flags|=256),s=Ou(c,s,!1),s!==2){if(Hl&&!l){c.errorRecoveryDisabledLanes|=o,Kl|=o,i=4;break a}o=Zl,Zl=i,o!==null&&(Zl===null?Zl=o:Zl.push.apply(Zl,o))}i=s}if(o=!1,i!==2)continue}}if(i===1){Su(e,0),yu(e,t,0,!0);break}a:{switch(r=e,o=i,o){case 0:case 1:throw Error(a(345));case 4:if((t&4194048)!==t)break;case 6:yu(r,t,Jl,!Bl);break a;case 2:Zl=null;break;case 3:case 5:break;default:throw Error(a(329))}if((t&62914560)===t&&(i=$l+300-je(),10<i)){if(yu(r,t,Jl,!Bl),Ze(r,0,!0)!==0)break a;su=t,r.timeoutHandle=Kd(_u.bind(null,r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,`Throttled`,-0,0),i);break a}_u(r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,null,-0,0)}}break}while(1);rd(e)}function _u(e,t,n,r,i,a,o,s,c,l,u,d,f,p){if(e.timeoutHandle=-1,d=t.subtreeFlags,d&8192||(d&16785408)==16785408){d={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:an},jl(t,a,d);var m=(a&62914560)===a?$l-je():(a&4194048)===a?eu-je():0;if(m=qf(d,m),m!==null){su=a,e.cancelPendingCommit=m(Lu.bind(null,e,t,a,n,r,i,o,s,c,u,d,null,f,p)),yu(e,a,o,!l);return}}Lu(e,t,a,n,r,i,o,s,c)}function vu(e){for(var t=e;;){var n=t.tag;if((n===0||n===11||n===15)&&t.flags&16384&&(n=t.updateQueue,n!==null&&(n=n.stores,n!==null)))for(var r=0;r<n.length;r++){var i=n[r],a=i.getSnapshot;i=i.value;try{if(!Er(a(),i))return!1}catch{return!1}}if(n=t.child,t.subtreeFlags&16384&&n!==null)n.return=t,t=n;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function yu(e,t,n,r){t&=~ql,t&=~Kl,e.suspendedLanes|=t,e.pingedLanes&=~t,r&&(e.warmLanes|=t),r=e.expirationTimes;for(var i=t;0<i;){var a=31-Ue(i),o=1<<a;r[a]=-1,i&=~o}n!==0&&it(e,n,t)}function bu(){return K&6?!0:(id(0,!1),!1)}function xu(){if(J!==null){if(X===0)var e=J.return;else e=J,Ji=qi=null,Oo(e),Na=null,Pa=0,e=J;for(;e!==null;)zc(e.alternate,e),e=e.return;J=null}}function Su(e,t){var n=e.timeoutHandle;n!==-1&&(e.timeoutHandle=-1,qd(n)),n=e.cancelPendingCommit,n!==null&&(e.cancelPendingCommit=null,n()),su=0,xu(),q=e,J=n=mi(e.current,null),Y=t,X=0,zl=null,Bl=!1,Vl=Qe(e,t),Hl=!1,Yl=Jl=ql=Kl=Gl=Wl=0,Zl=Xl=null,Ql=!1,t&8&&(t|=t&32);var r=e.entangledLanes;if(r!==0)for(e=e.entanglements,r&=t;0<r;){var i=31-Ue(r),a=1<<i;t|=e[i],r&=~a}return Ul=t,ii(),n}function Cu(e,t){H=null,O.H=Ls,t===Ca||t===Ta?(t=ja(),X=3):t===wa?(t=ja(),X=4):X=t===tc?8:typeof t==`object`&&t&&typeof t.then==`function`?6:1,zl=t,J===null&&(Wl=1,Ys(e,Si(t,e.current)))}function wu(){var e=ro.current;return e===null?!0:(Y&4194048)===Y?B===null:(Y&62914560)===Y||Y&536870912?e===B:!1}function Tu(){var e=O.H;return O.H=Ls,e===null?Ls:e}function Eu(){var e=O.A;return O.A=Ll,e}function Du(){Wl=4,Bl||(Y&4194048)!==Y&&ro.current!==null||(Vl=!0),!(Gl&134217727)&&!(Kl&134217727)||q===null||yu(q,Y,Jl,!1)}function Ou(e,t,n){var r=K;K|=2;var i=Tu(),a=Eu();(q!==e||Y!==t)&&(nu=null,Su(e,t)),t=!1;var o=Wl;a:do try{if(X!==0&&J!==null){var s=J,c=zl;switch(X){case 8:xu(),o=6;break a;case 3:case 2:case 9:case 6:ro.current===null&&(t=!0);var l=X;if(X=0,zl=null,Pu(e,s,c,l),n&&Vl){o=0;break a}break;default:l=X,X=0,zl=null,Pu(e,s,c,l)}}ku(),o=Wl;break}catch(t){Cu(e,t)}while(1);return t&&e.shellSuspendCounter++,Ji=qi=null,K=r,O.H=i,O.A=a,J===null&&(q=null,Y=0,ii()),o}function ku(){for(;J!==null;)Mu(J)}function Au(e,t){var n=K;K|=2;var r=Tu(),i=Eu();q!==e||Y!==t?(nu=null,tu=je()+500,Su(e,t)):Vl=Qe(e,t);a:do try{if(X!==0&&J!==null){t=J;var o=zl;b:switch(X){case 1:X=0,zl=null,Pu(e,t,o,1);break;case 2:case 9:if(Da(o)){X=0,zl=null,Nu(t);break}t=function(){X!==2&&X!==9||q!==e||(X=7),rd(e)},o.then(t,t);break a;case 3:X=7;break a;case 4:X=5;break a;case 7:Da(o)?(X=0,zl=null,Nu(t)):(X=0,zl=null,Pu(e,t,o,7));break;case 5:var s=null;switch(J.tag){case 26:s=J.memoizedState;case 5:case 27:var c=J;if(s?Wf(s):c.stateNode.complete){X=0,zl=null;var l=c.sibling;if(l!==null)J=l;else{var u=c.return;u===null?J=null:(J=u,Fu(u))}break b}}X=0,zl=null,Pu(e,t,o,5);break;case 6:X=0,zl=null,Pu(e,t,o,6);break;case 8:xu(),Wl=6;break a;default:throw Error(a(462))}}ju();break}catch(t){Cu(e,t)}while(1);return Ji=qi=null,O.H=r,O.A=i,K=n,J===null?(q=null,Y=0,ii(),Wl):0}function ju(){for(;J!==null&&!ke();)Mu(J)}function Mu(e){var t=Ac(e.alternate,e,Ul);e.memoizedProps=e.pendingProps,t===null?Fu(e):J=t}function Nu(e){var t=e,n=t.alternate;switch(t.tag){case 15:case 0:t=mc(n,t,t.pendingProps,t.type,void 0,Y);break;case 11:t=mc(n,t,t.pendingProps,t.type.render,t.ref,Y);break;case 5:Oo(t);default:zc(n,t),t=J=hi(t,Ul),t=Ac(n,t,Ul)}e.memoizedProps=e.pendingProps,t===null?Fu(e):J=t}function Pu(e,t,n,r){Ji=qi=null,Oo(t),Na=null,Pa=0;var i=t.return;try{if(ec(e,i,t,n,Y)){Wl=1,Ys(e,Si(n,e.current)),J=null;return}}catch(t){if(i!==null)throw J=i,t;Wl=1,Ys(e,Si(n,e.current)),J=null;return}t.flags&32768?(R||r===1?e=!0:Vl||Y&536870912?e=!1:(Bl=e=!0,(r===2||r===9||r===3||r===6)&&(r=ro.current,r!==null&&r.tag===13&&(r.flags|=16384))),Iu(t,e)):Fu(t)}function Fu(e){var t=e;do{if(t.flags&32768){Iu(t,Bl);return}e=t.return;var n=Lc(t.alternate,t,Ul);if(n!==null){J=n;return}if(t=t.sibling,t!==null){J=t;return}J=t=e}while(t!==null);Wl===0&&(Wl=5)}function Iu(e,t){do{var n=Rc(e.alternate,e);if(n!==null){n.flags&=32767,J=n;return}if(n=e.return,n!==null&&(n.flags|=32768,n.subtreeFlags=0,n.deletions=null),!t&&(e=e.sibling,e!==null)){J=e;return}J=e=n}while(e!==null);Wl=6,J=null}function Lu(e,t,n,r,i,o,s,c,l){e.cancelPendingCommit=null;do Hu();while(iu!==0);if(K&6)throw Error(a(327));if(t!==null){if(t===e.current)throw Error(a(177));if(o=t.lanes|t.childLanes,o|=ri,rt(e,n,o,s,c,l),e===q&&(J=q=null,Y=0),ou=t,au=e,su=n,cu=o,lu=i,uu=r,t.subtreeFlags&10256||t.flags&10256?(e.callbackNode=null,e.callbackPriority=0,Xu(Fe,function(){return Uu(),null})):(e.callbackNode=null,e.callbackPriority=0),r=(t.flags&13878)!=0,t.subtreeFlags&13878||r){r=O.T,O.T=null,i=k.p,k.p=2,s=K,K|=4;try{il(e,t,n)}finally{K=s,k.p=i,O.T=r}}iu=1,Ru(),zu(),Bu()}}function Ru(){if(iu===1){iu=0;var e=au,t=ou,n=(t.flags&13878)!=0;if(t.subtreeFlags&13878||n){n=O.T,O.T=null;var r=k.p;k.p=2;var i=K;K|=4;try{_l(t,e);var a=zd,o=jr(e.containerInfo),s=a.focusedElem,c=a.selectionRange;if(o!==s&&s&&s.ownerDocument&&Ar(s.ownerDocument.documentElement,s)){if(c!==null&&Mr(s)){var l=c.start,u=c.end;if(u===void 0&&(u=l),`selectionStart`in s)s.selectionStart=l,s.selectionEnd=Math.min(u,s.value.length);else{var d=s.ownerDocument||document,f=d&&d.defaultView||window;if(f.getSelection){var p=f.getSelection(),m=s.textContent.length,h=Math.min(c.start,m),g=c.end===void 0?h:Math.min(c.end,m);!p.extend&&h>g&&(o=g,g=h,h=o);var _=kr(s,h),v=kr(s,g);if(_&&v&&(p.rangeCount!==1||p.anchorNode!==_.node||p.anchorOffset!==_.offset||p.focusNode!==v.node||p.focusOffset!==v.offset)){var y=d.createRange();y.setStart(_.node,_.offset),p.removeAllRanges(),h>g?(p.addRange(y),p.extend(v.node,v.offset)):(y.setEnd(v.node,v.offset),p.addRange(y))}}}}for(d=[],p=s;p=p.parentNode;)p.nodeType===1&&d.push({element:p,left:p.scrollLeft,top:p.scrollTop});for(typeof s.focus==`function`&&s.focus(),s=0;s<d.length;s++){var b=d[s];b.element.scrollLeft=b.left,b.element.scrollTop=b.top}}sp=!!Rd,zd=Rd=null}finally{K=i,k.p=r,O.T=n}}e.current=t,iu=2}}function zu(){if(iu===2){iu=0;var e=au,t=ou,n=(t.flags&8772)!=0;if(t.subtreeFlags&8772||n){n=O.T,O.T=null;var r=k.p;k.p=2;var i=K;K|=4;try{al(e,t.alternate,t)}finally{K=i,k.p=r,O.T=n}}iu=3}}function Bu(){if(iu===4||iu===3){iu=0,Ae();var e=au,t=ou,n=su,r=uu;t.subtreeFlags&10256||t.flags&10256?iu=5:(iu=0,ou=au=null,Vu(e,e.pendingLanes));var i=e.pendingLanes;if(i===0&&(ru=null),ct(n),t=t.stateNode,Ve&&typeof Ve.onCommitFiberRoot==`function`)try{Ve.onCommitFiberRoot(Be,t,void 0,(t.current.flags&128)==128)}catch{}if(r!==null){t=O.T,i=k.p,k.p=2,O.T=null;try{for(var a=e.onRecoverableError,o=0;o<r.length;o++){var s=r[o];a(s.value,{componentStack:s.stack})}}finally{O.T=t,k.p=i}}su&3&&Hu(),rd(e),i=e.pendingLanes,n&261930&&i&42?e===fu?du++:(du=0,fu=e):du=0,id(0,!1)}}function Vu(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,ua(t)))}function Hu(){return Ru(),zu(),Bu(),Uu()}function Uu(){if(iu!==5)return!1;var e=au,t=cu;cu=0;var n=ct(su),r=O.T,i=k.p;try{k.p=32>n?32:n,O.T=null,n=lu,lu=null;var o=au,s=su;if(iu=0,ou=au=null,su=0,K&6)throw Error(a(331));var c=K;if(K|=4,Pl(o.current),El(o,o.current,s,n),K=c,id(0,!1),Ve&&typeof Ve.onPostCommitFiberRoot==`function`)try{Ve.onPostCommitFiberRoot(Be,o)}catch{}return!0}finally{k.p=i,O.T=r,Vu(e,t)}}function Wu(e,t,n){t=Si(n,t),t=Zs(e.stateNode,t,2),e=Ga(e,t,2),e!==null&&(nt(e,2),rd(e))}function Z(e,t,n){if(e.tag===3)Wu(e,e,n);else for(;t!==null;){if(t.tag===3){Wu(t,e,n);break}else if(t.tag===1){var r=t.stateNode;if(typeof t.type.getDerivedStateFromError==`function`||typeof r.componentDidCatch==`function`&&(ru===null||!ru.has(r))){e=Si(n,e),n=Qs(2),r=Ga(t,n,2),r!==null&&($s(n,r,t,e),nt(r,2),rd(r));break}}t=t.return}}function Gu(e,t,n){var r=e.pingCache;if(r===null){r=e.pingCache=new Rl;var i=new Set;r.set(t,i)}else i=r.get(t),i===void 0&&(i=new Set,r.set(t,i));i.has(n)||(Hl=!0,i.add(n),e=Ku.bind(null,e,t,n),t.then(e,e))}function Ku(e,t,n){var r=e.pingCache;r!==null&&r.delete(t),e.pingedLanes|=e.suspendedLanes&n,e.warmLanes&=~n,q===e&&(Y&n)===n&&(Wl===4||Wl===3&&(Y&62914560)===Y&&300>je()-$l?!(K&2)&&Su(e,0):ql|=n,Yl===Y&&(Yl=0)),rd(e)}function qu(e,t){t===0&&(t=et()),e=si(e,t),e!==null&&(nt(e,t),rd(e))}function Ju(e){var t=e.memoizedState,n=0;t!==null&&(n=t.retryLane),qu(e,n)}function Yu(e,t){var n=0;switch(e.tag){case 31:case 13:var r=e.stateNode,i=e.memoizedState;i!==null&&(n=i.retryLane);break;case 19:r=e.stateNode;break;case 22:r=e.stateNode._retryCache;break;default:throw Error(a(314))}r!==null&&r.delete(t),qu(e,n)}function Xu(e,t){return De(e,t)}var Zu=null,Qu=null,$u=!1,ed=!1,td=!1,nd=0;function rd(e){e!==Qu&&e.next===null&&(Qu===null?Zu=Qu=e:Qu=Qu.next=e),ed=!0,$u||($u=!0,ud())}function id(e,t){if(!td&&ed){td=!0;do for(var n=!1,r=Zu;r!==null;){if(!t)if(e!==0){var i=r.pendingLanes;if(i===0)var a=0;else{var o=r.suspendedLanes,s=r.pingedLanes;a=(1<<31-Ue(42|e)+1)-1,a&=i&~(o&~s),a=a&201326741?a&201326741|1:a?a|2:0}a!==0&&(n=!0,ld(r,a))}else a=Y,a=Ze(r,r===q?a:0,r.cancelPendingCommit!==null||r.timeoutHandle!==-1),!(a&3)||Qe(r,a)||(n=!0,ld(r,a));r=r.next}while(n);td=!1}}function ad(){od()}function od(){ed=$u=!1;var e=0;nd!==0&&Gd()&&(e=nd);for(var t=je(),n=null,r=Zu;r!==null;){var i=r.next,a=sd(r,t);a===0?(r.next=null,n===null?Zu=i:n.next=i,i===null&&(Qu=n)):(n=r,(e!==0||a&3)&&(ed=!0)),r=i}iu!==0&&iu!==5||id(e,!1),nd!==0&&(nd=0)}function sd(e,t){for(var n=e.suspendedLanes,r=e.pingedLanes,i=e.expirationTimes,a=e.pendingLanes&-62914561;0<a;){var o=31-Ue(a),s=1<<o,c=i[o];c===-1?((s&n)===0||(s&r)!==0)&&(i[o]=$e(s,t)):c<=t&&(e.expiredLanes|=s),a&=~s}if(t=q,n=Y,n=Ze(e,e===t?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r=e.callbackNode,n===0||e===t&&(X===2||X===9)||e.cancelPendingCommit!==null)return r!==null&&r!==null&&Oe(r),e.callbackNode=null,e.callbackPriority=0;if(!(n&3)||Qe(e,n)){if(t=n&-n,t===e.callbackPriority)return t;switch(r!==null&&Oe(r),ct(n)){case 2:case 8:n=Pe;break;case 32:n=Fe;break;case 268435456:n=Le;break;default:n=Fe}return r=cd.bind(null,e),n=De(n,r),e.callbackPriority=t,e.callbackNode=n,t}return r!==null&&r!==null&&Oe(r),e.callbackPriority=2,e.callbackNode=null,2}function cd(e,t){if(iu!==0&&iu!==5)return e.callbackNode=null,e.callbackPriority=0,null;var n=e.callbackNode;if(Hu()&&e.callbackNode!==n)return null;var r=Y;return r=Ze(e,e===q?r:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r===0?null:(gu(e,r,t),sd(e,je()),e.callbackNode!=null&&e.callbackNode===n?cd.bind(null,e):null)}function ld(e,t){if(Hu())return null;gu(e,t,!0)}function ud(){Yd(function(){K&6?De(Ne,ad):od()})}function dd(){if(nd===0){var e=pa;e===0&&(e=qe,qe<<=1,!(qe&261888)&&(qe=256)),nd=e}return nd}function fd(e){return e==null||typeof e==`symbol`||typeof e==`boolean`?null:typeof e==`function`?e:rn(``+e)}function pd(e,t){var n=t.ownerDocument.createElement(`input`);return n.name=t.name,n.value=t.value,e.id&&n.setAttribute(`form`,e.id),t.parentNode.insertBefore(n,t),e=new FormData(e),n.parentNode.removeChild(n),e}function md(e,t,n,r,i){if(t===`submit`&&n&&n.stateNode===i){var a=fd((i[N]||null).action),o=r.submitter;o&&(t=(t=o[N]||null)?fd(t.formAction):o.getAttribute(`formAction`),t!==null&&(a=t,o=null));var s=new En(`action`,`action`,null,r,i);e.push({event:s,listeners:[{instance:null,listener:function(){if(r.defaultPrevented){if(nd!==0){var e=o?pd(i,o):new FormData(i);Cs(n,{pending:!0,data:e,method:i.method,action:a},null,e)}}else typeof a==`function`&&(s.preventDefault(),e=o?pd(i,o):new FormData(i),Cs(n,{pending:!0,data:e,method:i.method,action:a},a,e))},currentTarget:i}]})}}for(var hd=0;hd<Qr.length;hd++){var gd=Qr[hd];$r(gd.toLowerCase(),`on`+(gd[0].toUpperCase()+gd.slice(1)))}$r(Wr,`onAnimationEnd`),$r(Gr,`onAnimationIteration`),$r(Kr,`onAnimationStart`),$r(`dblclick`,`onDoubleClick`),$r(`focusin`,`onFocus`),$r(`focusout`,`onBlur`),$r(qr,`onTransitionRun`),$r(Jr,`onTransitionStart`),$r(Yr,`onTransitionCancel`),$r(Xr,`onTransitionEnd`),Ot(`onMouseEnter`,[`mouseout`,`mouseover`]),Ot(`onMouseLeave`,[`mouseout`,`mouseover`]),Ot(`onPointerEnter`,[`pointerout`,`pointerover`]),Ot(`onPointerLeave`,[`pointerout`,`pointerover`]),Dt(`onChange`,`change click focusin focusout input keydown keyup selectionchange`.split(` `)),Dt(`onSelect`,`focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(` `)),Dt(`onBeforeInput`,[`compositionend`,`keypress`,`textInput`,`paste`]),Dt(`onCompositionEnd`,`compositionend focusout keydown keypress keyup mousedown`.split(` `)),Dt(`onCompositionStart`,`compositionstart focusout keydown keypress keyup mousedown`.split(` `)),Dt(`onCompositionUpdate`,`compositionupdate focusout keydown keypress keyup mousedown`.split(` `));var _d=`abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting`.split(` `),vd=new Set(`beforetoggle cancel close invalid load scroll scrollend toggle`.split(` `).concat(_d));function yd(e,t){t=(t&4)!=0;for(var n=0;n<e.length;n++){var r=e[n],i=r.event;r=r.listeners;a:{var a=void 0;if(t)for(var o=r.length-1;0<=o;o--){var s=r[o],c=s.instance,l=s.currentTarget;if(s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){ei(e)}i.currentTarget=null,a=c}else for(o=0;o<r.length;o++){if(s=r[o],c=s.instance,l=s.currentTarget,s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){ei(e)}i.currentTarget=null,a=c}}}}function Q(e,t){var n=t[mt];n===void 0&&(n=t[mt]=new Set);var r=e+`__bubble`;n.has(r)||(Cd(t,e,2,!1),n.add(r))}function bd(e,t,n){var r=0;t&&(r|=4),Cd(n,e,r,t)}var xd=`_reactListening`+Math.random().toString(36).slice(2);function Sd(e){if(!e[xd]){e[xd]=!0,Tt.forEach(function(t){t!==`selectionchange`&&(vd.has(t)||bd(t,!1,e),bd(t,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[xd]||(t[xd]=!0,bd(`selectionchange`,!1,t))}}function Cd(e,t,n,r){switch(mp(t)){case 2:var i=cp;break;case 8:i=lp;break;default:i=up}n=i.bind(null,t,n,e),i=void 0,!hn||t!==`touchstart`&&t!==`touchmove`&&t!==`wheel`||(i=!0),r?i===void 0?e.addEventListener(t,n,!0):e.addEventListener(t,n,{capture:!0,passive:i}):i===void 0?e.addEventListener(t,n,!1):e.addEventListener(t,n,{passive:i})}function wd(e,t,n,r,i){var a=r;if(!(t&1)&&!(t&2)&&r!==null)a:for(;;){if(r===null)return;var o=r.tag;if(o===3||o===4){var c=r.stateNode.containerInfo;if(c===i)break;if(o===4)for(o=r.return;o!==null;){var l=o.tag;if((l===3||l===4)&&o.stateNode.containerInfo===i)return;o=o.return}for(;c!==null;){if(o=bt(c),o===null)return;if(l=o.tag,l===5||l===6||l===26||l===27){r=a=o;continue a}c=c.parentNode}}r=r.return}fn(function(){var r=a,i=sn(n),o=[];a:{var c=Zr.get(e);if(c!==void 0){var l=En,u=e;switch(e){case`keypress`:if(xn(n)===0)break a;case`keydown`:case`keyup`:l=Wn;break;case`focusin`:u=`focus`,l=Fn;break;case`focusout`:u=`blur`,l=Fn;break;case`beforeblur`:case`afterblur`:l=Fn;break;case`click`:if(n.button===2)break a;case`auxclick`:case`dblclick`:case`mousedown`:case`mousemove`:case`mouseup`:case`mouseout`:case`mouseover`:case`contextmenu`:l=Nn;break;case`drag`:case`dragend`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`dragstart`:case`drop`:l=Pn;break;case`touchcancel`:case`touchend`:case`touchmove`:case`touchstart`:l=Kn;break;case Wr:case Gr:case Kr:l=In;break;case Xr:l=qn;break;case`scroll`:case`scrollend`:l=On;break;case`wheel`:l=Jn;break;case`copy`:case`cut`:case`paste`:l=Ln;break;case`gotpointercapture`:case`lostpointercapture`:case`pointercancel`:case`pointerdown`:case`pointermove`:case`pointerout`:case`pointerover`:case`pointerup`:l=Gn;break;case`toggle`:case`beforetoggle`:l=Yn}var d=(t&4)!=0,f=!d&&(e===`scroll`||e===`scrollend`),p=d?c===null?null:c+`Capture`:c;d=[];for(var m=r,h;m!==null;){var g=m;if(h=g.stateNode,g=g.tag,g!==5&&g!==26&&g!==27||h===null||p===null||(g=pn(m,p),g!=null&&d.push(Td(m,g,h))),f)break;m=m.return}0<d.length&&(c=new l(c,u,null,n,i),o.push({event:c,listeners:d}))}}if(!(t&7)){a:{if(c=e===`mouseover`||e===`pointerover`,l=e===`mouseout`||e===`pointerout`,c&&n!==on&&(u=n.relatedTarget||n.fromElement)&&(bt(u)||u[pt]))break a;if((l||c)&&(c=i.window===i?i:(c=i.ownerDocument)?c.defaultView||c.parentWindow:window,l?(u=n.relatedTarget||n.toElement,l=r,u=u?bt(u):null,u!==null&&(f=s(u),d=u.tag,u!==f||d!==5&&d!==27&&d!==6)&&(u=null)):(l=null,u=r),l!==u)){if(d=Nn,g=`onMouseLeave`,p=`onMouseEnter`,m=`mouse`,(e===`pointerout`||e===`pointerover`)&&(d=Gn,g=`onPointerLeave`,p=`onPointerEnter`,m=`pointer`),f=l==null?c:St(l),h=u==null?c:St(u),c=new d(g,m+`leave`,l,n,i),c.target=f,c.relatedTarget=h,g=null,bt(i)===r&&(d=new d(p,m+`enter`,u,n,i),d.target=h,d.relatedTarget=f,g=d),f=g,l&&u)b:{for(d=Dd,p=l,m=u,h=0,g=p;g;g=d(g))h++;g=0;for(var _=m;_;_=d(_))g++;for(;0<h-g;)p=d(p),h--;for(;0<g-h;)m=d(m),g--;for(;h--;){if(p===m||m!==null&&p===m.alternate){d=p;break b}p=d(p),m=d(m)}d=null}else d=null;l!==null&&Od(o,c,l,d,!1),u!==null&&f!==null&&Od(o,f,u,d,!0)}}a:{if(c=r?St(r):window,l=c.nodeName&&c.nodeName.toLowerCase(),l===`select`||l===`input`&&c.type===`file`)var v=hr;else if(lr(c))if(gr)v=wr;else{v=Sr;var y=xr}else l=c.nodeName,!l||l.toLowerCase()!==`input`||c.type!==`checkbox`&&c.type!==`radio`?r&&en(r.elementType)&&(v=hr):v=Cr;if(v&&=v(e,r)){ur(o,v,n,i);break a}y&&y(e,c,r),e===`focusout`&&r&&c.type===`number`&&r.memoizedProps.value!=null&&Kt(c,`number`,c.value)}switch(y=r?St(r):window,e){case`focusin`:(lr(y)||y.contentEditable===`true`)&&(Pr=y,Fr=r,Ir=null);break;case`focusout`:Ir=Fr=Pr=null;break;case`mousedown`:Lr=!0;break;case`contextmenu`:case`mouseup`:case`dragend`:Lr=!1,Rr(o,n,i);break;case`selectionchange`:if(Nr)break;case`keydown`:case`keyup`:Rr(o,n,i)}var b;if(Zn)b:{switch(e){case`compositionstart`:var x=`onCompositionStart`;break b;case`compositionend`:x=`onCompositionEnd`;break b;case`compositionupdate`:x=`onCompositionUpdate`;break b}x=void 0}else ar?rr(e,n)&&(x=`onCompositionEnd`):e===`keydown`&&n.keyCode===229&&(x=`onCompositionStart`);x&&(er&&n.locale!==`ko`&&(ar||x!==`onCompositionStart`?x===`onCompositionEnd`&&ar&&(b=bn()):(_n=i,vn=`value`in _n?_n.value:_n.textContent,ar=!0)),y=Ed(r,x),0<y.length&&(x=new Rn(x,e,null,n,i),o.push({event:x,listeners:y}),b?x.data=b:(b=ir(n),b!==null&&(x.data=b)))),(b=$n?or(e,n):sr(e,n))&&(x=Ed(r,`onBeforeInput`),0<x.length&&(y=new Rn(`onBeforeInput`,`beforeinput`,null,n,i),o.push({event:y,listeners:x}),y.data=b)),md(o,e,r,n,i)}yd(o,t)})}function Td(e,t,n){return{instance:e,listener:t,currentTarget:n}}function Ed(e,t){for(var n=t+`Capture`,r=[];e!==null;){var i=e,a=i.stateNode;if(i=i.tag,i!==5&&i!==26&&i!==27||a===null||(i=pn(e,n),i!=null&&r.unshift(Td(e,i,a)),i=pn(e,t),i!=null&&r.push(Td(e,i,a))),e.tag===3)return r;e=e.return}return[]}function Dd(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function Od(e,t,n,r,i){for(var a=t._reactName,o=[];n!==null&&n!==r;){var s=n,c=s.alternate,l=s.stateNode;if(s=s.tag,c!==null&&c===r)break;s!==5&&s!==26&&s!==27||l===null||(c=l,i?(l=pn(n,a),l!=null&&o.unshift(Td(n,l,c))):i||(l=pn(n,a),l!=null&&o.push(Td(n,l,c)))),n=n.return}o.length!==0&&e.push({event:t,listeners:o})}var kd=/\r\n?/g,Ad=/\u0000|\uFFFD/g;function jd(e){return(typeof e==`string`?e:``+e).replace(kd,`
`).replace(Ad,``)}function Md(e,t){return t=jd(t),jd(e)===t}function $(e,t,n,r,i,o){switch(n){case`children`:typeof r==`string`?t===`body`||t===`textarea`&&r===``||Xt(e,r):(typeof r==`number`||typeof r==`bigint`)&&t!==`body`&&Xt(e,``+r);break;case`className`:Pt(e,`class`,r);break;case`tabIndex`:Pt(e,`tabindex`,r);break;case`dir`:case`role`:case`viewBox`:case`width`:case`height`:Pt(e,n,r);break;case`style`:$t(e,r,o);break;case`data`:if(t!==`object`){Pt(e,`data`,r);break}case`src`:case`href`:if(r===``&&(t!==`a`||n!==`href`)){e.removeAttribute(n);break}if(r==null||typeof r==`function`||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=rn(``+r),e.setAttribute(n,r);break;case`action`:case`formAction`:if(typeof r==`function`){e.setAttribute(n,`javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')`);break}else typeof o==`function`&&(n===`formAction`?(t!==`input`&&$(e,t,`name`,i.name,i,null),$(e,t,`formEncType`,i.formEncType,i,null),$(e,t,`formMethod`,i.formMethod,i,null),$(e,t,`formTarget`,i.formTarget,i,null)):($(e,t,`encType`,i.encType,i,null),$(e,t,`method`,i.method,i,null),$(e,t,`target`,i.target,i,null)));if(r==null||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=rn(``+r),e.setAttribute(n,r);break;case`onClick`:r!=null&&(e.onclick=an);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`multiple`:e.multiple=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`muted`:e.muted=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`defaultValue`:case`defaultChecked`:case`innerHTML`:case`ref`:break;case`autoFocus`:break;case`xlinkHref`:if(r==null||typeof r==`function`||typeof r==`boolean`||typeof r==`symbol`){e.removeAttribute(`xlink:href`);break}n=rn(``+r),e.setAttributeNS(`http://www.w3.org/1999/xlink`,`xlink:href`,n);break;case`contentEditable`:case`spellCheck`:case`draggable`:case`value`:case`autoReverse`:case`externalResourcesRequired`:case`focusable`:case`preserveAlpha`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``+r):e.removeAttribute(n);break;case`inert`:case`allowFullScreen`:case`async`:case`autoPlay`:case`controls`:case`default`:case`defer`:case`disabled`:case`disablePictureInPicture`:case`disableRemotePlayback`:case`formNoValidate`:case`hidden`:case`loop`:case`noModule`:case`noValidate`:case`open`:case`playsInline`:case`readOnly`:case`required`:case`reversed`:case`scoped`:case`seamless`:case`itemScope`:r&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``):e.removeAttribute(n);break;case`capture`:case`download`:!0===r?e.setAttribute(n,``):!1!==r&&r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,r):e.removeAttribute(n);break;case`cols`:case`rows`:case`size`:case`span`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`&&!isNaN(r)&&1<=r?e.setAttribute(n,r):e.removeAttribute(n);break;case`rowSpan`:case`start`:r==null||typeof r==`function`||typeof r==`symbol`||isNaN(r)?e.removeAttribute(n):e.setAttribute(n,r);break;case`popover`:Q(`beforetoggle`,e),Q(`toggle`,e),Nt(e,`popover`,r);break;case`xlinkActuate`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:actuate`,r);break;case`xlinkArcrole`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:arcrole`,r);break;case`xlinkRole`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:role`,r);break;case`xlinkShow`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:show`,r);break;case`xlinkTitle`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:title`,r);break;case`xlinkType`:Ft(e,`http://www.w3.org/1999/xlink`,`xlink:type`,r);break;case`xmlBase`:Ft(e,`http://www.w3.org/XML/1998/namespace`,`xml:base`,r);break;case`xmlLang`:Ft(e,`http://www.w3.org/XML/1998/namespace`,`xml:lang`,r);break;case`xmlSpace`:Ft(e,`http://www.w3.org/XML/1998/namespace`,`xml:space`,r);break;case`is`:Nt(e,`is`,r);break;case`innerText`:case`textContent`:break;default:(!(2<n.length)||n[0]!==`o`&&n[0]!==`O`||n[1]!==`n`&&n[1]!==`N`)&&(n=tn.get(n)||n,Nt(e,n,r))}}function Nd(e,t,n,r,i,o){switch(n){case`style`:$t(e,r,o);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`children`:typeof r==`string`?Xt(e,r):(typeof r==`number`||typeof r==`bigint`)&&Xt(e,``+r);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`onClick`:r!=null&&(e.onclick=an);break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`innerHTML`:case`ref`:break;case`innerText`:case`textContent`:break;default:if(!Et.hasOwnProperty(n))a:{if(n[0]===`o`&&n[1]===`n`&&(i=n.endsWith(`Capture`),t=n.slice(2,i?n.length-7:void 0),o=e[N]||null,o=o==null?null:o[n],typeof o==`function`&&e.removeEventListener(t,o,i),typeof r==`function`)){typeof o!=`function`&&o!==null&&(n in e?e[n]=null:e.hasAttribute(n)&&e.removeAttribute(n)),e.addEventListener(t,r,i);break a}n in e?e[n]=r:!0===r?e.setAttribute(n,``):Nt(e,n,r)}}}function Pd(e,t,n){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`img`:Q(`error`,e),Q(`load`,e);var r=!1,i=!1,o;for(o in n)if(n.hasOwnProperty(o)){var s=n[o];if(s!=null)switch(o){case`src`:r=!0;break;case`srcSet`:i=!0;break;case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:$(e,t,o,s,n,null)}}i&&$(e,t,`srcSet`,n.srcSet,n,null),r&&$(e,t,`src`,n.src,n,null);return;case`input`:Q(`invalid`,e);var c=o=s=i=null,l=null,u=null;for(r in n)if(n.hasOwnProperty(r)){var d=n[r];if(d!=null)switch(r){case`name`:i=d;break;case`type`:s=d;break;case`checked`:l=d;break;case`defaultChecked`:u=d;break;case`value`:o=d;break;case`defaultValue`:c=d;break;case`children`:case`dangerouslySetInnerHTML`:if(d!=null)throw Error(a(137,t));break;default:$(e,t,r,d,n,null)}}Gt(e,o,c,l,u,s,i,!1);return;case`select`:for(i in Q(`invalid`,e),r=s=o=null,n)if(n.hasOwnProperty(i)&&(c=n[i],c!=null))switch(i){case`value`:o=c;break;case`defaultValue`:s=c;break;case`multiple`:r=c;default:$(e,t,i,c,n,null)}t=o,n=s,e.multiple=!!r,t==null?n!=null&&qt(e,!!r,n,!0):qt(e,!!r,t,!1);return;case`textarea`:for(s in Q(`invalid`,e),o=i=r=null,n)if(n.hasOwnProperty(s)&&(c=n[s],c!=null))switch(s){case`value`:r=c;break;case`defaultValue`:i=c;break;case`children`:o=c;break;case`dangerouslySetInnerHTML`:if(c!=null)throw Error(a(91));break;default:$(e,t,s,c,n,null)}Yt(e,r,i,o);return;case`option`:for(l in n)if(n.hasOwnProperty(l)&&(r=n[l],r!=null))switch(l){case`selected`:e.selected=r&&typeof r!=`function`&&typeof r!=`symbol`;break;default:$(e,t,l,r,n,null)}return;case`dialog`:Q(`beforetoggle`,e),Q(`toggle`,e),Q(`cancel`,e),Q(`close`,e);break;case`iframe`:case`object`:Q(`load`,e);break;case`video`:case`audio`:for(r=0;r<_d.length;r++)Q(_d[r],e);break;case`image`:Q(`error`,e),Q(`load`,e);break;case`details`:Q(`toggle`,e);break;case`embed`:case`source`:case`link`:Q(`error`,e),Q(`load`,e);case`area`:case`base`:case`br`:case`col`:case`hr`:case`keygen`:case`meta`:case`param`:case`track`:case`wbr`:case`menuitem`:for(u in n)if(n.hasOwnProperty(u)&&(r=n[u],r!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:$(e,t,u,r,n,null)}return;default:if(en(t)){for(d in n)n.hasOwnProperty(d)&&(r=n[d],r!==void 0&&Nd(e,t,d,r,n,void 0));return}}for(c in n)n.hasOwnProperty(c)&&(r=n[c],r!=null&&$(e,t,c,r,n,null))}function Fd(e,t,n,r){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`input`:var i=null,o=null,s=null,c=null,l=null,u=null,d=null;for(m in n){var f=n[m];if(n.hasOwnProperty(m)&&f!=null)switch(m){case`checked`:break;case`value`:break;case`defaultValue`:l=f;default:r.hasOwnProperty(m)||$(e,t,m,null,r,f)}}for(var p in r){var m=r[p];if(f=n[p],r.hasOwnProperty(p)&&(m!=null||f!=null))switch(p){case`type`:o=m;break;case`name`:i=m;break;case`checked`:u=m;break;case`defaultChecked`:d=m;break;case`value`:s=m;break;case`defaultValue`:c=m;break;case`children`:case`dangerouslySetInnerHTML`:if(m!=null)throw Error(a(137,t));break;default:m!==f&&$(e,t,p,m,r,f)}}Wt(e,s,c,l,u,d,o,i);return;case`select`:for(o in m=s=c=p=null,n)if(l=n[o],n.hasOwnProperty(o)&&l!=null)switch(o){case`value`:break;case`multiple`:m=l;default:r.hasOwnProperty(o)||$(e,t,o,null,r,l)}for(i in r)if(o=r[i],l=n[i],r.hasOwnProperty(i)&&(o!=null||l!=null))switch(i){case`value`:p=o;break;case`defaultValue`:c=o;break;case`multiple`:s=o;default:o!==l&&$(e,t,i,o,r,l)}t=c,n=s,r=m,p==null?!!r!=!!n&&(t==null?qt(e,!!n,n?[]:``,!1):qt(e,!!n,t,!0)):qt(e,!!n,p,!1);return;case`textarea`:for(c in m=p=null,n)if(i=n[c],n.hasOwnProperty(c)&&i!=null&&!r.hasOwnProperty(c))switch(c){case`value`:break;case`children`:break;default:$(e,t,c,null,r,i)}for(s in r)if(i=r[s],o=n[s],r.hasOwnProperty(s)&&(i!=null||o!=null))switch(s){case`value`:p=i;break;case`defaultValue`:m=i;break;case`children`:break;case`dangerouslySetInnerHTML`:if(i!=null)throw Error(a(91));break;default:i!==o&&$(e,t,s,i,r,o)}Jt(e,p,m);return;case`option`:for(var h in n)if(p=n[h],n.hasOwnProperty(h)&&p!=null&&!r.hasOwnProperty(h))switch(h){case`selected`:e.selected=!1;break;default:$(e,t,h,null,r,p)}for(l in r)if(p=r[l],m=n[l],r.hasOwnProperty(l)&&p!==m&&(p!=null||m!=null))switch(l){case`selected`:e.selected=p&&typeof p!=`function`&&typeof p!=`symbol`;break;default:$(e,t,l,p,r,m)}return;case`img`:case`link`:case`area`:case`base`:case`br`:case`col`:case`embed`:case`hr`:case`keygen`:case`meta`:case`param`:case`source`:case`track`:case`wbr`:case`menuitem`:for(var g in n)p=n[g],n.hasOwnProperty(g)&&p!=null&&!r.hasOwnProperty(g)&&$(e,t,g,null,r,p);for(u in r)if(p=r[u],m=n[u],r.hasOwnProperty(u)&&p!==m&&(p!=null||m!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:if(p!=null)throw Error(a(137,t));break;default:$(e,t,u,p,r,m)}return;default:if(en(t)){for(var _ in n)p=n[_],n.hasOwnProperty(_)&&p!==void 0&&!r.hasOwnProperty(_)&&Nd(e,t,_,void 0,r,p);for(d in r)p=r[d],m=n[d],!r.hasOwnProperty(d)||p===m||p===void 0&&m===void 0||Nd(e,t,d,p,r,m);return}}for(var v in n)p=n[v],n.hasOwnProperty(v)&&p!=null&&!r.hasOwnProperty(v)&&$(e,t,v,null,r,p);for(f in r)p=r[f],m=n[f],!r.hasOwnProperty(f)||p===m||p==null&&m==null||$(e,t,f,p,r,m)}function Id(e){switch(e){case`css`:case`script`:case`font`:case`img`:case`image`:case`input`:case`link`:return!0;default:return!1}}function Ld(){if(typeof performance.getEntriesByType==`function`){for(var e=0,t=0,n=performance.getEntriesByType(`resource`),r=0;r<n.length;r++){var i=n[r],a=i.transferSize,o=i.initiatorType,s=i.duration;if(a&&s&&Id(o)){for(o=0,s=i.responseEnd,r+=1;r<n.length;r++){var c=n[r],l=c.startTime;if(l>s)break;var u=c.transferSize,d=c.initiatorType;u&&Id(d)&&(c=c.responseEnd,o+=u*(c<s?1:(s-l)/(c-l)))}if(--r,t+=8*(a+o)/(i.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e==`number`)?e:5}var Rd=null,zd=null;function Bd(e){return e.nodeType===9?e:e.ownerDocument}function Vd(e){switch(e){case`http://www.w3.org/2000/svg`:return 1;case`http://www.w3.org/1998/Math/MathML`:return 2;default:return 0}}function Hd(e,t){if(e===0)switch(t){case`svg`:return 1;case`math`:return 2;default:return 0}return e===1&&t===`foreignObject`?0:e}function Ud(e,t){return e===`textarea`||e===`noscript`||typeof t.children==`string`||typeof t.children==`number`||typeof t.children==`bigint`||typeof t.dangerouslySetInnerHTML==`object`&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var Wd=null;function Gd(){var e=window.event;return e&&e.type===`popstate`?e===Wd?!1:(Wd=e,!0):(Wd=null,!1)}var Kd=typeof setTimeout==`function`?setTimeout:void 0,qd=typeof clearTimeout==`function`?clearTimeout:void 0,Jd=typeof Promise==`function`?Promise:void 0,Yd=typeof queueMicrotask==`function`?queueMicrotask:Jd===void 0?Kd:function(e){return Jd.resolve(null).then(e).catch(Xd)};function Xd(e){setTimeout(function(){throw e})}function Zd(e){return e===`head`}function Qd(e,t){var n=t,r=0;do{var i=n.nextSibling;if(e.removeChild(n),i&&i.nodeType===8)if(n=i.data,n===`/$`||n===`/&`){if(r===0){e.removeChild(i),Np(t);return}r--}else if(n===`$`||n===`$?`||n===`$~`||n===`$!`||n===`&`)r++;else if(n===`html`)pf(e.ownerDocument.documentElement);else if(n===`head`){n=e.ownerDocument.head,pf(n);for(var a=n.firstChild;a;){var o=a.nextSibling,s=a.nodeName;a[vt]||s===`SCRIPT`||s===`STYLE`||s===`LINK`&&a.rel.toLowerCase()===`stylesheet`||n.removeChild(a),a=o}}else n===`body`&&pf(e.ownerDocument.body);n=i}while(n);Np(t)}function $d(e,t){var n=e;e=0;do{var r=n.nextSibling;if(n.nodeType===1?t?(n._stashedDisplay=n.style.display,n.style.display=`none`):(n.style.display=n._stashedDisplay||``,n.getAttribute(`style`)===``&&n.removeAttribute(`style`)):n.nodeType===3&&(t?(n._stashedText=n.nodeValue,n.nodeValue=``):n.nodeValue=n._stashedText||``),r&&r.nodeType===8)if(n=r.data,n===`/$`){if(e===0)break;e--}else n!==`$`&&n!==`$?`&&n!==`$~`&&n!==`$!`||e++;n=r}while(n)}function ef(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var n=t;switch(t=t.nextSibling,n.nodeName){case`HTML`:case`HEAD`:case`BODY`:ef(n),yt(n);continue;case`SCRIPT`:case`STYLE`:continue;case`LINK`:if(n.rel.toLowerCase()===`stylesheet`)continue}e.removeChild(n)}}function tf(e,t,n,r){for(;e.nodeType===1;){var i=n;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!r&&(e.nodeName!==`INPUT`||e.type!==`hidden`))break}else if(!r)if(t===`input`&&e.type===`hidden`){var a=i.name==null?null:``+i.name;if(i.type===`hidden`&&e.getAttribute(`name`)===a)return e}else return e;else if(!e[vt])switch(t){case`meta`:if(!e.hasAttribute(`itemprop`))break;return e;case`link`:if(a=e.getAttribute(`rel`),a===`stylesheet`&&e.hasAttribute(`data-precedence`)||a!==i.rel||e.getAttribute(`href`)!==(i.href==null||i.href===``?null:i.href)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin)||e.getAttribute(`title`)!==(i.title==null?null:i.title))break;return e;case`style`:if(e.hasAttribute(`data-precedence`))break;return e;case`script`:if(a=e.getAttribute(`src`),(a!==(i.src==null?null:i.src)||e.getAttribute(`type`)!==(i.type==null?null:i.type)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin))&&a&&e.hasAttribute(`async`)&&!e.hasAttribute(`itemprop`))break;return e;default:return e}if(e=cf(e.nextSibling),e===null)break}return null}function nf(e,t,n){if(t===``)return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!n||(e=cf(e.nextSibling),e===null))return null;return e}function rf(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!t||(e=cf(e.nextSibling),e===null))return null;return e}function af(e){return e.data===`$?`||e.data===`$~`}function of(e){return e.data===`$!`||e.data===`$?`&&e.ownerDocument.readyState!==`loading`}function sf(e,t){var n=e.ownerDocument;if(e.data===`$~`)e._reactRetry=t;else if(e.data!==`$?`||n.readyState!==`loading`)t();else{var r=function(){t(),n.removeEventListener(`DOMContentLoaded`,r)};n.addEventListener(`DOMContentLoaded`,r),e._reactRetry=r}}function cf(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t===`$`||t===`$!`||t===`$?`||t===`$~`||t===`&`||t===`F!`||t===`F`)break;if(t===`/$`||t===`/&`)return null}}return e}var lf=null;function uf(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`/$`||n===`/&`){if(t===0)return cf(e.nextSibling);t--}else n!==`$`&&n!==`$!`&&n!==`$?`&&n!==`$~`&&n!==`&`||t++}e=e.nextSibling}return null}function df(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`$`||n===`$!`||n===`$?`||n===`$~`||n===`&`){if(t===0)return e;t--}else n!==`/$`&&n!==`/&`||t++}e=e.previousSibling}return null}function ff(e,t,n){switch(t=Bd(n),e){case`html`:if(e=t.documentElement,!e)throw Error(a(452));return e;case`head`:if(e=t.head,!e)throw Error(a(453));return e;case`body`:if(e=t.body,!e)throw Error(a(454));return e;default:throw Error(a(451))}}function pf(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);yt(e)}var mf=new Map,hf=new Set;function gf(e){return typeof e.getRootNode==`function`?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var _f=k.d;k.d={f:vf,r:yf,D:Sf,C:Cf,L:wf,m:Tf,X:Df,S:Ef,M:Of};function vf(){var e=_f.f(),t=bu();return e||t}function yf(e){var t=xt(e);t!==null&&t.tag===5&&t.type===`form`?Ts(t):_f.r(e)}var bf=typeof document>`u`?null:document;function xf(e,t,n){var r=bf;if(r&&typeof t==`string`&&t){var i=Ut(t);i=`link[rel="`+e+`"][href="`+i+`"]`,typeof n==`string`&&(i+=`[crossorigin="`+n+`"]`),hf.has(i)||(hf.add(i),e={rel:e,crossOrigin:n,href:t},r.querySelector(i)===null&&(t=r.createElement(`link`),Pd(t,`link`,e),wt(t),r.head.appendChild(t)))}}function Sf(e){_f.D(e),xf(`dns-prefetch`,e,null)}function Cf(e,t){_f.C(e,t),xf(`preconnect`,e,t)}function wf(e,t,n){_f.L(e,t,n);var r=bf;if(r&&e&&t){var i=`link[rel="preload"][as="`+Ut(t)+`"]`;t===`image`&&n&&n.imageSrcSet?(i+=`[imagesrcset="`+Ut(n.imageSrcSet)+`"]`,typeof n.imageSizes==`string`&&(i+=`[imagesizes="`+Ut(n.imageSizes)+`"]`)):i+=`[href="`+Ut(e)+`"]`;var a=i;switch(t){case`style`:a=Af(e);break;case`script`:a=Pf(e)}mf.has(a)||(e=m({rel:`preload`,href:t===`image`&&n&&n.imageSrcSet?void 0:e,as:t},n),mf.set(a,e),r.querySelector(i)!==null||t===`style`&&r.querySelector(jf(a))||t===`script`&&r.querySelector(Ff(a))||(t=r.createElement(`link`),Pd(t,`link`,e),wt(t),r.head.appendChild(t)))}}function Tf(e,t){_f.m(e,t);var n=bf;if(n&&e){var r=t&&typeof t.as==`string`?t.as:`script`,i=`link[rel="modulepreload"][as="`+Ut(r)+`"][href="`+Ut(e)+`"]`,a=i;switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:a=Pf(e)}if(!mf.has(a)&&(e=m({rel:`modulepreload`,href:e},t),mf.set(a,e),n.querySelector(i)===null)){switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:if(n.querySelector(Ff(a)))return}r=n.createElement(`link`),Pd(r,`link`,e),wt(r),n.head.appendChild(r)}}}function Ef(e,t,n){_f.S(e,t,n);var r=bf;if(r&&e){var i=Ct(r).hoistableStyles,a=Af(e);t||=`default`;var o=i.get(a);if(!o){var s={loading:0,preload:null};if(o=r.querySelector(jf(a)))s.loading=5;else{e=m({rel:`stylesheet`,href:e,"data-precedence":t},n),(n=mf.get(a))&&Rf(e,n);var c=o=r.createElement(`link`);wt(c),Pd(c,`link`,e),c._p=new Promise(function(e,t){c.onload=e,c.onerror=t}),c.addEventListener(`load`,function(){s.loading|=1}),c.addEventListener(`error`,function(){s.loading|=2}),s.loading|=4,Lf(o,t,r)}o={type:`stylesheet`,instance:o,count:1,state:s},i.set(a,o)}}}function Df(e,t){_f.X(e,t);var n=bf;if(n&&e){var r=Ct(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),wt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function Of(e,t){_f.M(e,t);var n=bf;if(n&&e){var r=Ct(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0,type:`module`},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),wt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function kf(e,t,n,r){var i=(i=pe.current)?gf(i):null;if(!i)throw Error(a(446));switch(e){case`meta`:case`title`:return null;case`style`:return typeof n.precedence==`string`&&typeof n.href==`string`?(t=Af(n.href),n=Ct(i).hoistableStyles,r=n.get(t),r||(r={type:`style`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};case`link`:if(n.rel===`stylesheet`&&typeof n.href==`string`&&typeof n.precedence==`string`){e=Af(n.href);var o=Ct(i).hoistableStyles,s=o.get(e);if(s||(i=i.ownerDocument||i,s={type:`stylesheet`,instance:null,count:0,state:{loading:0,preload:null}},o.set(e,s),(o=i.querySelector(jf(e)))&&!o._p&&(s.instance=o,s.state.loading=5),mf.has(e)||(n={rel:`preload`,as:`style`,href:n.href,crossOrigin:n.crossOrigin,integrity:n.integrity,media:n.media,hrefLang:n.hrefLang,referrerPolicy:n.referrerPolicy},mf.set(e,n),o||Nf(i,e,n,s.state))),t&&r===null)throw Error(a(528,``));return s}if(t&&r!==null)throw Error(a(529,``));return null;case`script`:return t=n.async,n=n.src,typeof n==`string`&&t&&typeof t!=`function`&&typeof t!=`symbol`?(t=Pf(n),n=Ct(i).hoistableScripts,r=n.get(t),r||(r={type:`script`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};default:throw Error(a(444,e))}}function Af(e){return`href="`+Ut(e)+`"`}function jf(e){return`link[rel="stylesheet"][`+e+`]`}function Mf(e){return m({},e,{"data-precedence":e.precedence,precedence:null})}function Nf(e,t,n,r){e.querySelector(`link[rel="preload"][as="style"][`+t+`]`)?r.loading=1:(t=e.createElement(`link`),r.preload=t,t.addEventListener(`load`,function(){return r.loading|=1}),t.addEventListener(`error`,function(){return r.loading|=2}),Pd(t,`link`,n),wt(t),e.head.appendChild(t))}function Pf(e){return`[src="`+Ut(e)+`"]`}function Ff(e){return`script[async]`+e}function If(e,t,n){if(t.count++,t.instance===null)switch(t.type){case`style`:var r=e.querySelector(`style[data-href~="`+Ut(n.href)+`"]`);if(r)return t.instance=r,wt(r),r;var i=m({},n,{"data-href":n.href,"data-precedence":n.precedence,href:null,precedence:null});return r=(e.ownerDocument||e).createElement(`style`),wt(r),Pd(r,`style`,i),Lf(r,n.precedence,e),t.instance=r;case`stylesheet`:i=Af(n.href);var o=e.querySelector(jf(i));if(o)return t.state.loading|=4,t.instance=o,wt(o),o;r=Mf(n),(i=mf.get(i))&&Rf(r,i),o=(e.ownerDocument||e).createElement(`link`),wt(o);var s=o;return s._p=new Promise(function(e,t){s.onload=e,s.onerror=t}),Pd(o,`link`,r),t.state.loading|=4,Lf(o,n.precedence,e),t.instance=o;case`script`:return o=Pf(n.src),(i=e.querySelector(Ff(o)))?(t.instance=i,wt(i),i):(r=n,(i=mf.get(o))&&(r=m({},n),zf(r,i)),e=e.ownerDocument||e,i=e.createElement(`script`),wt(i),Pd(i,`link`,r),e.head.appendChild(i),t.instance=i);case`void`:return null;default:throw Error(a(443,t.type))}else t.type===`stylesheet`&&!(t.state.loading&4)&&(r=t.instance,t.state.loading|=4,Lf(r,n.precedence,e));return t.instance}function Lf(e,t,n){for(var r=n.querySelectorAll(`link[rel="stylesheet"][data-precedence],style[data-precedence]`),i=r.length?r[r.length-1]:null,a=i,o=0;o<r.length;o++){var s=r[o];if(s.dataset.precedence===t)a=s;else if(a!==i)break}a?a.parentNode.insertBefore(e,a.nextSibling):(t=n.nodeType===9?n.head:n,t.insertBefore(e,t.firstChild))}function Rf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.title??=t.title}function zf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.integrity??=t.integrity}var Bf=null;function Vf(e,t,n){if(Bf===null){var r=new Map,i=Bf=new Map;i.set(n,r)}else i=Bf,r=i.get(n),r||(r=new Map,i.set(n,r));if(r.has(e))return r;for(r.set(e,null),n=n.getElementsByTagName(e),i=0;i<n.length;i++){var a=n[i];if(!(a[vt]||a[ft]||e===`link`&&a.getAttribute(`rel`)===`stylesheet`)&&a.namespaceURI!==`http://www.w3.org/2000/svg`){var o=a.getAttribute(t)||``;o=e+o;var s=r.get(o);s?s.push(a):r.set(o,[a])}}return r}function Hf(e,t,n){e=e.ownerDocument||e,e.head.insertBefore(n,t===`title`?e.querySelector(`head > title`):null)}function Uf(e,t,n){if(n===1||t.itemProp!=null)return!1;switch(e){case`meta`:case`title`:return!0;case`style`:if(typeof t.precedence!=`string`||typeof t.href!=`string`||t.href===``)break;return!0;case`link`:if(typeof t.rel!=`string`||typeof t.href!=`string`||t.href===``||t.onLoad||t.onError)break;switch(t.rel){case`stylesheet`:return e=t.disabled,typeof t.precedence==`string`&&e==null;default:return!0}case`script`:if(t.async&&typeof t.async!=`function`&&typeof t.async!=`symbol`&&!t.onLoad&&!t.onError&&t.src&&typeof t.src==`string`)return!0}return!1}function Wf(e){return!(e.type===`stylesheet`&&!(e.state.loading&3))}function Gf(e,t,n,r){if(n.type===`stylesheet`&&(typeof r.media!=`string`||!1!==matchMedia(r.media).matches)&&!(n.state.loading&4)){if(n.instance===null){var i=Af(r.href),a=t.querySelector(jf(i));if(a){t=a._p,typeof t==`object`&&t&&typeof t.then==`function`&&(e.count++,e=Jf.bind(e),t.then(e,e)),n.state.loading|=4,n.instance=a,wt(a);return}a=t.ownerDocument||t,r=Mf(r),(i=mf.get(i))&&Rf(r,i),a=a.createElement(`link`),wt(a);var o=a;o._p=new Promise(function(e,t){o.onload=e,o.onerror=t}),Pd(a,`link`,r),n.instance=a}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(n,t),(t=n.state.preload)&&!(n.state.loading&3)&&(e.count++,n=Jf.bind(e),t.addEventListener(`load`,n),t.addEventListener(`error`,n))}}var Kf=0;function qf(e,t){return e.stylesheets&&e.count===0&&Xf(e,e.stylesheets),0<e.count||0<e.imgCount?function(n){var r=setTimeout(function(){if(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend){var t=e.unsuspend;e.unsuspend=null,t()}},6e4+t);0<e.imgBytes&&Kf===0&&(Kf=62500*Ld());var i=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend)){var t=e.unsuspend;e.unsuspend=null,t()}},(e.imgBytes>Kf?50:800)+t);return e.unsuspend=n,function(){e.unsuspend=null,clearTimeout(r),clearTimeout(i)}}:null}function Jf(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)Xf(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var Yf=null;function Xf(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,Yf=new Map,t.forEach(Zf,e),Yf=null,Jf.call(e))}function Zf(e,t){if(!(t.state.loading&4)){var n=Yf.get(e);if(n)var r=n.get(null);else{n=new Map,Yf.set(e,n);for(var i=e.querySelectorAll(`link[data-precedence],style[data-precedence]`),a=0;a<i.length;a++){var o=i[a];(o.nodeName===`LINK`||o.getAttribute(`media`)!==`not all`)&&(n.set(o.dataset.precedence,o),r=o)}r&&n.set(null,r)}i=t.instance,o=i.getAttribute(`data-precedence`),a=n.get(o)||r,a===r&&n.set(null,i),n.set(o,i),this.count++,r=Jf.bind(this),i.addEventListener(`load`,r),i.addEventListener(`error`,r),a?a.parentNode.insertBefore(i,a.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(i,e.firstChild)),t.state.loading|=4}}var Qf={$$typeof:S,Provider:null,Consumer:null,_currentValue:ce,_currentValue2:ce,_threadCount:0};function $f(e,t,n,r,i,a,o,s,c){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=tt(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=tt(0),this.hiddenUpdates=tt(null),this.identifierPrefix=r,this.onUncaughtError=i,this.onCaughtError=a,this.onRecoverableError=o,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=c,this.incompleteTransitions=new Map}function ep(e,t,n,r,i,a,o,s,c,l,u,d){return e=new $f(e,t,n,o,c,l,u,d,s),t=1,!0===a&&(t|=24),a=fi(3,null,null,t),e.current=a,a.stateNode=e,t=la(),t.refCount++,e.pooledCache=t,t.refCount++,a.memoizedState={element:r,isDehydrated:n,cache:t},Ha(a),e}function tp(e){return e?(e=ui,e):ui}function np(e,t,n,r,i,a){i=tp(i),r.context===null?r.context=i:r.pendingContext=i,r=Wa(t),r.payload={element:n},a=a===void 0?null:a,a!==null&&(r.callback=a),n=Ga(e,r,t),n!==null&&(hu(n,e,t),Ka(n,e,t))}function rp(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var n=e.retryLane;e.retryLane=n!==0&&n<t?n:t}}function ip(e,t){rp(e,t),(e=e.alternate)&&rp(e,t)}function ap(e){if(e.tag===13||e.tag===31){var t=si(e,67108864);t!==null&&hu(t,e,67108864),ip(e,67108864)}}function op(e){if(e.tag===13||e.tag===31){var t=pu();t=st(t);var n=si(e,t);n!==null&&hu(n,e,t),ip(e,t)}}var sp=!0;function cp(e,t,n,r){var i=O.T;O.T=null;var a=k.p;try{k.p=2,up(e,t,n,r)}finally{k.p=a,O.T=i}}function lp(e,t,n,r){var i=O.T;O.T=null;var a=k.p;try{k.p=8,up(e,t,n,r)}finally{k.p=a,O.T=i}}function up(e,t,n,r){if(sp){var i=dp(r);if(i===null)wd(e,t,r,fp,n),Cp(e,r);else if(Tp(i,e,t,n,r))r.stopPropagation();else if(Cp(e,r),t&4&&-1<Sp.indexOf(e)){for(;i!==null;){var a=xt(i);if(a!==null)switch(a.tag){case 3:if(a=a.stateNode,a.current.memoizedState.isDehydrated){var o=Xe(a.pendingLanes);if(o!==0){var s=a;for(s.pendingLanes|=2,s.entangledLanes|=2;o;){var c=1<<31-Ue(o);s.entanglements[1]|=c,o&=~c}rd(a),!(K&6)&&(tu=je()+500,id(0,!1))}}break;case 31:case 13:s=si(a,2),s!==null&&hu(s,a,2),bu(),ip(a,2)}if(a=dp(r),a===null&&wd(e,t,r,fp,n),a===i)break;i=a}i!==null&&r.stopPropagation()}else wd(e,t,r,null,n)}}function dp(e){return e=sn(e),pp(e)}var fp=null;function pp(e){if(fp=null,e=bt(e),e!==null){var t=s(e);if(t===null)e=null;else{var n=t.tag;if(n===13){if(e=c(t),e!==null)return e;e=null}else if(n===31){if(e=l(t),e!==null)return e;e=null}else if(n===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return fp=e,null}function mp(e){switch(e){case`beforetoggle`:case`cancel`:case`click`:case`close`:case`contextmenu`:case`copy`:case`cut`:case`auxclick`:case`dblclick`:case`dragend`:case`dragstart`:case`drop`:case`focusin`:case`focusout`:case`input`:case`invalid`:case`keydown`:case`keypress`:case`keyup`:case`mousedown`:case`mouseup`:case`paste`:case`pause`:case`play`:case`pointercancel`:case`pointerdown`:case`pointerup`:case`ratechange`:case`reset`:case`resize`:case`seeked`:case`submit`:case`toggle`:case`touchcancel`:case`touchend`:case`touchstart`:case`volumechange`:case`change`:case`selectionchange`:case`textInput`:case`compositionstart`:case`compositionend`:case`compositionupdate`:case`beforeblur`:case`afterblur`:case`beforeinput`:case`blur`:case`fullscreenchange`:case`focus`:case`hashchange`:case`popstate`:case`select`:case`selectstart`:return 2;case`drag`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`mousemove`:case`mouseout`:case`mouseover`:case`pointermove`:case`pointerout`:case`pointerover`:case`scroll`:case`touchmove`:case`wheel`:case`mouseenter`:case`mouseleave`:case`pointerenter`:case`pointerleave`:return 8;case`message`:switch(Me()){case Ne:return 2;case Pe:return 8;case Fe:case Ie:return 32;case Le:return 268435456;default:return 32}default:return 32}}var hp=!1,gp=null,_p=null,vp=null,yp=new Map,bp=new Map,xp=[],Sp=`mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset`.split(` `);function Cp(e,t){switch(e){case`focusin`:case`focusout`:gp=null;break;case`dragenter`:case`dragleave`:_p=null;break;case`mouseover`:case`mouseout`:vp=null;break;case`pointerover`:case`pointerout`:yp.delete(t.pointerId);break;case`gotpointercapture`:case`lostpointercapture`:bp.delete(t.pointerId)}}function wp(e,t,n,r,i,a){return e===null||e.nativeEvent!==a?(e={blockedOn:t,domEventName:n,eventSystemFlags:r,nativeEvent:a,targetContainers:[i]},t!==null&&(t=xt(t),t!==null&&ap(t)),e):(e.eventSystemFlags|=r,t=e.targetContainers,i!==null&&t.indexOf(i)===-1&&t.push(i),e)}function Tp(e,t,n,r,i){switch(t){case`focusin`:return gp=wp(gp,e,t,n,r,i),!0;case`dragenter`:return _p=wp(_p,e,t,n,r,i),!0;case`mouseover`:return vp=wp(vp,e,t,n,r,i),!0;case`pointerover`:var a=i.pointerId;return yp.set(a,wp(yp.get(a)||null,e,t,n,r,i)),!0;case`gotpointercapture`:return a=i.pointerId,bp.set(a,wp(bp.get(a)||null,e,t,n,r,i)),!0}return!1}function Ep(e){var t=bt(e.target);if(t!==null){var n=s(t);if(n!==null){if(t=n.tag,t===13){if(t=c(n),t!==null){e.blockedOn=t,ut(e.priority,function(){op(n)});return}}else if(t===31){if(t=l(n),t!==null){e.blockedOn=t,ut(e.priority,function(){op(n)});return}}else if(t===3&&n.stateNode.current.memoizedState.isDehydrated){e.blockedOn=n.tag===3?n.stateNode.containerInfo:null;return}}}e.blockedOn=null}function Dp(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var n=dp(e.nativeEvent);if(n===null){n=e.nativeEvent;var r=new n.constructor(n.type,n);on=r,n.target.dispatchEvent(r),on=null}else return t=xt(n),t!==null&&ap(t),e.blockedOn=n,!1;t.shift()}return!0}function Op(e,t,n){Dp(e)&&n.delete(t)}function kp(){hp=!1,gp!==null&&Dp(gp)&&(gp=null),_p!==null&&Dp(_p)&&(_p=null),vp!==null&&Dp(vp)&&(vp=null),yp.forEach(Op),bp.forEach(Op)}function Ap(e,n){e.blockedOn===n&&(e.blockedOn=null,hp||(hp=!0,t.unstable_scheduleCallback(t.unstable_NormalPriority,kp)))}var jp=null;function Mp(e){jp!==e&&(jp=e,t.unstable_scheduleCallback(t.unstable_NormalPriority,function(){jp===e&&(jp=null);for(var t=0;t<e.length;t+=3){var n=e[t],r=e[t+1],i=e[t+2];if(typeof r!=`function`){if(pp(r||n)===null)continue;break}var a=xt(n);a!==null&&(e.splice(t,3),t-=3,Cs(a,{pending:!0,data:i,method:n.method,action:r},r,i))}}))}function Np(e){function t(t){return Ap(t,e)}gp!==null&&Ap(gp,e),_p!==null&&Ap(_p,e),vp!==null&&Ap(vp,e),yp.forEach(t),bp.forEach(t);for(var n=0;n<xp.length;n++){var r=xp[n];r.blockedOn===e&&(r.blockedOn=null)}for(;0<xp.length&&(n=xp[0],n.blockedOn===null);)Ep(n),n.blockedOn===null&&xp.shift();if(n=(e.ownerDocument||e).$$reactFormReplay,n!=null)for(r=0;r<n.length;r+=3){var i=n[r],a=n[r+1],o=i[N]||null;if(typeof a==`function`)o||Mp(n);else if(o){var s=null;if(a&&a.hasAttribute(`formAction`)){if(i=a,o=a[N]||null)s=o.formAction;else if(pp(i)!==null)continue}else s=o.action;typeof s==`function`?n[r+1]=s:(n.splice(r,3),r-=3),Mp(n)}}}function Pp(){function e(e){e.canIntercept&&e.info===`react-transition`&&e.intercept({handler:function(){return new Promise(function(e){return i=e})},focusReset:`manual`,scroll:`manual`})}function t(){i!==null&&(i(),i=null),r||setTimeout(n,20)}function n(){if(!r&&!navigation.transition){var e=navigation.currentEntry;e&&e.url!=null&&navigation.navigate(e.url,{state:e.getState(),info:`react-transition`,history:`replace`})}}if(typeof navigation==`object`){var r=!1,i=null;return navigation.addEventListener(`navigate`,e),navigation.addEventListener(`navigatesuccess`,t),navigation.addEventListener(`navigateerror`,t),setTimeout(n,100),function(){r=!0,navigation.removeEventListener(`navigate`,e),navigation.removeEventListener(`navigatesuccess`,t),navigation.removeEventListener(`navigateerror`,t),i!==null&&(i(),i=null)}}}function Fp(e){this._internalRoot=e}Ip.prototype.render=Fp.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(a(409));var n=t.current;np(n,pu(),e,t,null,null)},Ip.prototype.unmount=Fp.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;np(e.current,2,null,e,null,null),bu(),t[pt]=null}};function Ip(e){this._internalRoot=e}Ip.prototype.unstable_scheduleHydration=function(e){if(e){var t=lt();e={blockedOn:null,target:e,priority:t};for(var n=0;n<xp.length&&t!==0&&t<xp[n].priority;n++);xp.splice(n,0,e),n===0&&Ep(e)}};var Lp=r.version;if(Lp!==`19.2.8`)throw Error(a(527,Lp,`19.2.8`));k.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render==`function`?Error(a(188)):(e=Object.keys(e).join(`,`),Error(a(268,e)));return e=f(t),e=e===null?null:p(e),e=e===null?null:e.stateNode,e};var Rp={bundleType:0,version:`19.2.8`,rendererPackageName:`react-dom`,currentDispatcherRef:O,reconcilerVersion:`19.2.8`};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<`u`){var zp=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!zp.isDisabled&&zp.supportsFiber)try{Be=zp.inject(Rp),Ve=zp}catch{}}e.createRoot=function(e,t){if(!o(e))throw Error(a(299));var n=!1,r=``,i=Ks,s=qs,c=Js;return t!=null&&(!0===t.unstable_strictMode&&(n=!0),t.identifierPrefix!==void 0&&(r=t.identifierPrefix),t.onUncaughtError!==void 0&&(i=t.onUncaughtError),t.onCaughtError!==void 0&&(s=t.onCaughtError),t.onRecoverableError!==void 0&&(c=t.onRecoverableError)),t=ep(e,1,!1,null,null,n,r,null,i,s,c,Pp),e[pt]=t.current,Sd(e),new Fp(t)}})),re=t(((e,t)=>{function n(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>`u`||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!=`function`))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n)}catch(e){console.error(e)}}n(),t.exports=T()})),E=`modulepreload`,ie=function(e){return`/`+e},ae={},oe=function(e,t,n){let r=Promise.resolve();if(t&&t.length>0){let e=document.getElementsByTagName(`link`),i=document.querySelector(`meta[property=csp-nonce]`),a=i?.nonce||i?.getAttribute(`nonce`);function o(e){return Promise.all(e.map(e=>Promise.resolve(e).then(e=>({status:`fulfilled`,value:e}),e=>({status:`rejected`,reason:e}))))}function s(e){return import.meta.resolve?import.meta.resolve(e):new URL(e,import.meta.url).href}r=o(t.map(t=>{if(t=ie(t,n),t=s(t),t in ae)return;ae[t]=!0;let r=t.endsWith(`.css`);for(let n=e.length-1;n>=0;n--){let i=e[n];if(i.href===t&&(!r||i.rel===`stylesheet`))return}let i=document.createElement(`link`);if(i.rel=r?`stylesheet`:E,r||(i.as=`script`),i.crossOrigin=``,i.href=t,a&&i.setAttribute(`nonce`,a),document.head.appendChild(i),r)return new Promise((e,n)=>{i.addEventListener(`load`,e),i.addEventListener(`error`,()=>n(Error(`Unable to preload CSS for ${t}`)))})}))}function i(e){let t=new Event(`vite:preloadError`,{cancelable:!0});if(t.payload=e,window.dispatchEvent(t),!t.defaultPrevented)throw e}return r.then(t=>{for(let e of t||[])e.status===`rejected`&&i(e.reason);return e().catch(i)})},D=e(n(),1),se=/^(?:[a-z][a-z0-9+.-]*:|[\\/]{2})/i,O=/^[\\/]{2}/;function k(e,t){return t+e.replace(/\\/g,`/`)}var ce=`popstate`;function le(e){return typeof e==`object`&&!!e&&`pathname`in e&&`search`in e&&`hash`in e&&`state`in e&&`key`in e}function ue(e={}){function t(e,t){let n=t.state?.masked,{pathname:r,search:i,hash:a}=n||e.location;return fe(``,{pathname:r,search:i,hash:a},t.state&&t.state.usr||null,t.state&&t.state.key||`default`,n?{pathname:e.location.pathname,search:e.location.search,hash:e.location.hash}:void 0)}function n(e,t){return typeof t==`string`?t:pe(t)}return he(t,n,null,e)}function A(e,t){if(e===!1||e==null)throw Error(t)}function j(e,t){if(!e){typeof console<`u`&&console.warn(t);try{throw Error(t)}catch{}}}function M(){return Math.random().toString(36).substring(2,10)}function de(e,t){return{usr:e.state,key:e.key,idx:t,masked:e.mask?{pathname:e.pathname,search:e.search,hash:e.hash}:void 0}}function fe(e,t,n=null,r,i){return{pathname:typeof e==`string`?e:e.pathname,search:``,hash:``,...typeof t==`string`?me(t):t,state:n,key:t&&t.key||r||M(),mask:i}}function pe({pathname:e=`/`,search:t=``,hash:n=``}){return t&&t!==`?`&&(e+=t.charAt(0)===`?`?t:`?`+t),n&&n!==`#`&&(e+=n.charAt(0)===`#`?n:`#`+n),e}function me(e){let t={};if(e){let n=e.indexOf(`#`);n>=0&&(t.hash=e.substring(n),e=e.substring(0,n));let r=e.indexOf(`?`);r>=0&&(t.search=e.substring(r),e=e.substring(0,r)),e&&(t.pathname=e)}return t}function he(e,t,n,r={}){let{window:i=document.defaultView,v5Compat:a=!1}=r,o=i.history,s=`POP`,c=null,l=u();l??(l=0,o.replaceState({...o.state,idx:l},``));function u(){return(o.state||{idx:null}).idx}function d(){s=`POP`;let e=u(),t=e==null?null:e-l;l=e,c&&c({action:s,location:h.location,delta:t})}function f(e,t){s=`PUSH`;let r=le(e)?e:fe(h.location,e,t);n&&n(r,e),l=u()+1;let d=de(r,l),f=h.createHref(r.mask||r);try{o.pushState(d,``,f)}catch(e){if(e instanceof DOMException&&e.name===`DataCloneError`)throw e;i.location.assign(f)}a&&c&&c({action:s,location:h.location,delta:1})}function p(e,t){s=`REPLACE`;let r=le(e)?e:fe(h.location,e,t);n&&n(r,e),l=u();let i=de(r,l),d=h.createHref(r.mask||r);o.replaceState(i,``,d),a&&c&&c({action:s,location:h.location,delta:0})}function m(e){return ge(i,e)}let h={get action(){return s},get location(){return e(i,o)},listen(e){if(c)throw Error(`A history only accepts one active listener`);return i.addEventListener(ce,d),c=e,()=>{i.removeEventListener(ce,d),c=null}},createHref(e){return t(i,e)},createURL:m,encodeLocation(e){let t=m(e);return{pathname:t.pathname,search:t.search,hash:t.hash}},push:f,replace:p,go(e){return o.go(e)}};return h}function ge(e,t,n=!1){let r=`http://localhost`;e&&(r=e.location.origin===`null`?e.location.href:e.location.origin),A(r,`No window.location.(origin|href) available to create URL`);let i=typeof t==`string`?t:pe(t);return i=i.replace(/ $/,`%20`),!n&&O.test(i)&&(i=r+i),new URL(i,r)}function _e(e,t,n=`/`){return ve(e,t,n,!1)}function ve(e,t,n,r,i){let a=Re((typeof t==`string`?me(t):t).pathname||`/`,n);if(a==null)return null;let o=i??be(e),s=null,c=Le(a);for(let e=0;s==null&&e<o.length;++e)s=Ne(o[e],c,r);return s}function ye(e,t){let{route:n,pathname:r,params:i}=e;return{id:n.id,pathname:r,params:i,data:t[n.id],loaderData:t[n.id],handle:n.handle}}function be(e){let t=xe(e);return Ce(t),t}function xe(e,t=[],n=[],r=``,i=!1){let a=(e,a,o=i,s)=>{let c={relativePath:s===void 0?e.path||``:s,caseSensitive:e.caseSensitive===!0,childrenIndex:a,route:e};if(c.relativePath.startsWith(`/`)){if(!c.relativePath.startsWith(r)&&o)return;A(c.relativePath.startsWith(r),`Absolute route path "${c.relativePath}" nested under path "${r}" is not valid. An absolute child route path must start with the combined path of all its parent routes.`),c.relativePath=c.relativePath.slice(r.length)}let l=Ke([r,c.relativePath]),u=n.concat(c);e.children&&e.children.length>0&&(A(e.index!==!0,`Index routes must not have child routes. Please remove all child routes from route path "${l}".`),xe(e.children,t,u,l,o)),!(e.path==null&&!e.index)&&t.push({path:l,score:je(l,e.index),routesMeta:u.map((e,t)=>{let[n,r]=Ie(e.relativePath,e.caseSensitive,t===u.length-1);return{...e,matcher:n,compiledParams:r}})})};return e.forEach((e,t)=>{if(e.path===``||!e.path?.includes(`?`))a(e,t);else for(let n of Se(e.path))a(e,t,!0,n)}),t}function Se(e){let t=e.split(`/`);if(t.length===0)return[];let[n,...r]=t,i=n.endsWith(`?`),a=n.replace(/\?$/,``);if(r.length===0)return i?[a,``]:[a];let o=Se(r.join(`/`)),s=[];return s.push(...o.map(e=>e===``?a:[a,e].join(`/`))),i&&s.push(...o),s.map(t=>e.startsWith(`/`)&&t===``?`/`:t)}function Ce(e){e.sort((e,t)=>e.score===t.score?Me(e.routesMeta.map(e=>e.childrenIndex),t.routesMeta.map(e=>e.childrenIndex)):t.score-e.score)}var we=/^:[\w-]+$/,Te=3,Ee=2,De=1,Oe=10,ke=-2,Ae=e=>e===`*`;function je(e,t){let n=e.split(`/`),r=n.length;return n.some(Ae)&&(r+=ke),t&&(r+=Ee),n.filter(e=>!Ae(e)).reduce((e,t)=>e+(we.test(t)?Te:t===``?De:Oe),r)}function Me(e,t){return e.length===t.length&&e.slice(0,-1).every((e,n)=>e===t[n])?e[e.length-1]-t[t.length-1]:0}function Ne(e,t,n=!1){let{routesMeta:r}=e,i={},a=`/`,o=[];for(let e=0;e<r.length;++e){let s=r[e],c=e===r.length-1,l=a===`/`?t:t.slice(a.length)||`/`,u={path:s.relativePath,caseSensitive:s.caseSensitive,end:c},d=s.matcher&&s.compiledParams?Fe(u,l,s.matcher,s.compiledParams):Pe(u,l),f=s.route;if(!d&&c&&n&&!r[r.length-1].route.index&&(d=Pe({path:s.relativePath,caseSensitive:s.caseSensitive,end:!1},l)),!d)return null;Object.assign(i,d.params),o.push({params:i,pathname:Ke([a,d.pathname]),pathnameBase:Je(Ke([a,d.pathnameBase])),route:f}),d.pathnameBase!==`/`&&(a=Ke([a,d.pathnameBase]))}return o}function Pe(e,t){typeof e==`string`&&(e={path:e,caseSensitive:!1,end:!0});let[n,r]=Ie(e.path,e.caseSensitive,e.end);return Fe(e,t,n,r)}function Fe(e,t,n,r){let i=t.match(n);if(!i)return null;let a=i[0],o=a.replace(/(.)\/+$/,`$1`),s=i.slice(1);return{params:r.reduce((e,{paramName:t,isOptional:n},r)=>{if(t===`*`){let e=s[r]||``;o=a.slice(0,a.length-e.length).replace(/(.)\/+$/,`$1`)}let i=s[r];return n&&!i?e[t]=void 0:e[t]=(i||``).replace(/%2F/g,`/`),e},{}),pathname:a,pathnameBase:o,pattern:e}}function Ie(e,t=!1,n=!0){j(e===`*`||!e.endsWith(`*`)||e.endsWith(`/*`),`Route path "${e}" will be treated as if it were "${e.replace(/\*$/,`/*`)}" because the \`*\` character must always follow a \`/\` in the pattern. To get rid of this warning, please change the route path to "${e.replace(/\*$/,`/*`)}".`);let r=[],i=`^`+e.replace(/\/*\*?$/,``).replace(/^\/*/,`/`).replace(/[\\.*+^${}|()[\]]/g,`\\$&`).replace(/\/:([\w-]+)(\?)?/g,(e,t,n,i,a)=>{if(r.push({paramName:t,isOptional:n!=null}),n){let t=a.charAt(i+e.length);return t&&t!==`/`?`/([^\\/]*)`:`(?:/([^\\/]*))?`}return`/([^\\/]+)`}).replace(/\/([\w-]+)\?(\/|$)/g,`(/$1)?$2`);return e.endsWith(`*`)?(r.push({paramName:`*`}),i+=e===`*`||e===`/*`?`(.*)$`:`(?:\\/(.+)|\\/*)$`):n?i+=`\\/*$`:e!==``&&e!==`/`&&(i+=`(?:(?=\\/|$))`),[new RegExp(i,t?void 0:`i`),r]}function Le(e){try{return e.split(`/`).map(e=>decodeURIComponent(e).replace(/\//g,`%2F`)).join(`/`)}catch(t){return j(!1,`The URL path "${e}" could not be decoded because it is a malformed URL segment. This is probably due to a bad percent encoding (${t}).`),e}}function Re(e,t){if(t===`/`)return e;if(!e.toLowerCase().startsWith(t.toLowerCase()))return null;let n=t.endsWith(`/`)?t.length-1:t.length,r=e.charAt(n);return r&&r!==`/`?null:e.slice(n)||`/`}function ze(e,t=`/`){let{pathname:n,search:r=``,hash:i=``}=typeof e==`string`?me(e):e,a;return n?(n=Ge(n),a=n.startsWith(`/`)?Be(n.substring(1),`/`):Be(n,t)):a=t,{pathname:a,search:Ye(r),hash:Xe(i)}}function Be(e,t){let n=qe(t).split(`/`);return e.split(`/`).forEach(e=>{e===`..`?n.length>1&&n.pop():e!==`.`&&n.push(e)}),n.length>1?n.join(`/`):`/`}function Ve(e,t,n,r){return`Cannot include a '${e}' character in a manually specified \`to.${t}\` field [${JSON.stringify(r)}].  Please separate it out to the \`to.${n}\` field. Alternatively you may provide the full path as a string in <Link to="..."> and the router will parse it for you.`}function He(e){return e.filter((e,t)=>t===0||e.route.path&&e.route.path.length>0)}function Ue(e){let t=He(e);return t.map((e,n)=>n===t.length-1?e.pathname:e.pathnameBase)}function We(e,t,n,r=!1){let i;typeof e==`string`?i=me(e):(i={...e},A(!i.pathname||!i.pathname.includes(`?`),Ve(`?`,`pathname`,`search`,i)),A(!i.pathname||!i.pathname.includes(`#`),Ve(`#`,`pathname`,`hash`,i)),A(!i.search||!i.search.includes(`#`),Ve(`#`,`search`,`hash`,i)));let a=e===``||i.pathname===``,o=a?`/`:i.pathname,s;if(o==null)s=n;else{let e=t.length-1;if(!r&&o.startsWith(`..`)){let t=o.split(`/`);for(;t[0]===`..`;)t.shift(),--e;i.pathname=t.join(`/`)}s=e>=0?t[e]:`/`}let c=ze(i,s),l=o&&o!==`/`&&o.endsWith(`/`),u=(a||o===`.`)&&n.endsWith(`/`);return!c.pathname.endsWith(`/`)&&(l||u)&&(c.pathname+=`/`),c}var Ge=e=>e.replace(/[\\/]{2,}/g,`/`),Ke=e=>Ge(e.join(`/`)),qe=e=>e.replace(/\/+$/,``),Je=e=>qe(e).replace(/^\/*/,`/`),Ye=e=>!e||e===`?`?``:e.startsWith(`?`)?e:`?`+e,Xe=e=>!e||e===`#`?``:e.startsWith(`#`)?e:`#`+e,Ze=class{constructor(e,t,n,r=!1){this.status=e,this.statusText=t||``,this.internal=r,n instanceof Error?(this.data=n.toString(),this.error=n):this.data=n}};function Qe(e){return e!=null&&typeof e.status==`number`&&typeof e.statusText==`string`&&typeof e.internal==`boolean`&&`data`in e}function $e(e){return Ke(e.map(e=>e.route.path).filter(Boolean))||`/`}var et=typeof window<`u`&&window.document!==void 0&&window.document.createElement!==void 0;function tt(e,t){let n=e;if(typeof n!=`string`||!se.test(n))return{absoluteURL:void 0,isExternal:!1,to:n};let r=n,i=!1;if(et)try{let e=new URL(window.location.href),r=O.test(n)?new URL(k(n,e.protocol)):new URL(n),a=Re(r.pathname,t);r.origin===e.origin&&a!=null?n=a+r.search+r.hash:i=!0}catch{j(!1,`<Link to="${n}"> contains an invalid URL which will probably break when clicked - please update to a valid URL path.`)}return{absoluteURL:r,isExternal:i,to:n}}Object.getOwnPropertyNames(Object.prototype).sort().join(`\0`);var nt=[`POST`,`PUT`,`PATCH`,`DELETE`];new Set(nt);var rt=[`GET`,...nt];new Set(rt);var it=[`about:`,`blob:`,`chrome:`,`chrome-untrusted:`,`content:`,`data:`,`devtools:`,`file:`,`filesystem:`,`javascript:`];function at(e){try{return it.includes(new URL(e).protocol)}catch{return!1}}var ot=D.createContext(null);ot.displayName=`DataRouter`;var st=D.createContext(null);st.displayName=`DataRouterState`;var ct=D.createContext(!1);function lt(){return D.useContext(ct)}var ut=D.createContext({isTransitioning:!1});ut.displayName=`ViewTransition`;var dt=D.createContext(new Map);dt.displayName=`Fetchers`;var ft=D.createContext(null);ft.displayName=`Await`;var N=D.createContext(null);N.displayName=`Navigation`;var pt=D.createContext(null);pt.displayName=`Location`;var mt=D.createContext({outlet:null,matches:[],isDataRoute:!1});mt.displayName=`Route`;var ht=D.createContext(null);ht.displayName=`RouteError`;var gt=`REACT_ROUTER_ERROR`,_t=`REDIRECT`,vt=`ROUTE_ERROR_RESPONSE`;function yt(e){if(e.startsWith(`${gt}:${_t}:{`))try{let t=JSON.parse(e.slice(28));if(typeof t==`object`&&t&&typeof t.status==`number`&&typeof t.statusText==`string`&&typeof t.location==`string`&&typeof t.reloadDocument==`boolean`&&typeof t.replace==`boolean`)return t}catch{}}function bt(e){if(e.startsWith(`${gt}:${vt}:{`))try{let t=JSON.parse(e.slice(40));if(typeof t==`object`&&t&&typeof t.status==`number`&&typeof t.statusText==`string`)return new Ze(t.status,t.statusText,t.data)}catch{}}function xt(e,{relative:t}={}){A(St(),`useHref() may be used only in the context of a <Router> component.`);let{basename:n,navigator:r}=D.useContext(N),{hash:i,pathname:a,search:o}=Ot(e,{relative:t}),s=a;return n!==`/`&&(s=a===`/`?n:Ke([n,a])),r.createHref({pathname:s,search:o,hash:i})}function St(){return D.useContext(pt)!=null}function Ct(){return A(St(),`useLocation() may be used only in the context of a <Router> component.`),D.useContext(pt).location}var wt=`You should call navigate() in a React.useEffect(), not when your component is first rendered.`;function Tt(e){D.useContext(N).static||D.useLayoutEffect(e)}function Et(){let{isDataRoute:e}=D.useContext(mt);return e?qt():Dt()}function Dt(){A(St(),`useNavigate() may be used only in the context of a <Router> component.`);let e=D.useContext(ot),{basename:t,navigator:n}=D.useContext(N),{matches:r}=D.useContext(mt),{pathname:i}=Ct(),a=JSON.stringify(Ue(r)),o=D.useRef(!1);return Tt(()=>{o.current=!0}),D.useCallback((r,s={})=>{if(j(o.current,wt),!o.current)return;if(typeof r==`number`){n.go(r);return}let c=We(r,JSON.parse(a),i,s.relative===`path`);e==null&&t!==`/`&&(c.pathname=c.pathname===`/`?t:Ke([t,c.pathname])),(s.replace?n.replace:n.push)(c,s.state,s)},[t,n,a,i,e])}D.createContext(null);function Ot(e,{relative:t}={}){let{matches:n}=D.useContext(mt),{pathname:r}=Ct(),i=JSON.stringify(Ue(n));return D.useMemo(()=>We(e,JSON.parse(i),r,t===`path`),[e,i,r,t])}function kt(e,t){return At(e,t)}function At(e,t,n){A(St(),`useRoutes() may be used only in the context of a <Router> component.`);let{navigator:r}=D.useContext(N),{matches:i}=D.useContext(mt),a=i[i.length-1],o=a?a.params:{},s=a?a.pathname:`/`,c=a?a.pathnameBase:`/`,l=a&&a.route;{let e=l&&l.path||``;Yt(s,!l||e.endsWith(`*`)||e.endsWith(`*?`),`You rendered descendant <Routes> (or called \`useRoutes()\`) at "${s}" (under <Route path="${e}">) but the parent route path has no trailing "*". This means if you navigate deeper, the parent won't match anymore and therefore the child routes will never render.

Please change the parent <Route path="${e}"> to <Route path="${e===`/`?`*`:`${e}/*`}">.`)}let u=Ct(),d;if(t){let e=typeof t==`string`?me(t):t;A(c===`/`||e.pathname?.startsWith(c),`When overriding the location using \`<Routes location>\` or \`useRoutes(routes, location)\`, the location pathname must begin with the portion of the URL pathname that was matched by all parent routes. The current pathname base is "${c}" but pathname "${e.pathname}" was given in the \`location\` prop.`),d=e}else d=u;let f=d.pathname||`/`,p=f;if(c!==`/`){let e=c.replace(/^\//,``).split(`/`);p=`/`+f.replace(/^\//,``).split(`/`).slice(e.length).join(`/`)}let m=n&&n.state.matches.length?n.state.matches.map(e=>Object.assign(e,{route:n.manifest[e.route.id]||e.route})):_e(e,{pathname:p});j(l||m!=null,`No routes matched location "${d.pathname}${d.search}${d.hash}" `),j(m==null||m[m.length-1].route.element!==void 0||m[m.length-1].route.Component!==void 0||m[m.length-1].route.lazy!==void 0,`Matched leaf route at location "${d.pathname}${d.search}${d.hash}" does not have an element or Component. This means it will render an <Outlet /> with a null value by default resulting in an "empty" page.`);let h=Lt(m&&m.map(e=>Object.assign({},e,{params:Object.assign({},o,e.params),pathname:Ke([c,r.encodeLocation?r.encodeLocation(e.pathname.replace(/%/g,`%25`).replace(/\?/g,`%3F`).replace(/#/g,`%23`)).pathname:e.pathname]),pathnameBase:e.pathnameBase===`/`?c:Ke([c,r.encodeLocation?r.encodeLocation(e.pathnameBase.replace(/%/g,`%25`).replace(/\?/g,`%3F`).replace(/#/g,`%23`)).pathname:e.pathnameBase])})),i,n);return t&&h?D.createElement(pt.Provider,{value:{location:{pathname:`/`,search:``,hash:``,state:null,key:`default`,mask:void 0,...d},navigationType:`POP`}},h):h}function jt(){let e=Kt(),t=Qe(e)?`${e.status} ${e.statusText}`:e instanceof Error?e.message:JSON.stringify(e),n=e instanceof Error?e.stack:null,r=`rgba(200,200,200, 0.5)`,i={padding:`0.5rem`,backgroundColor:r},a={padding:`2px 4px`,backgroundColor:r},o=null;return console.error(`Error handled by React Router default ErrorBoundary:`,e),o=D.createElement(D.Fragment,null,D.createElement(`p`,null,`💿 Hey developer 👋`),D.createElement(`p`,null,`You can provide a way better UX than this when your app throws errors by providing your own `,D.createElement(`code`,{style:a},`ErrorBoundary`),` or`,` `,D.createElement(`code`,{style:a},`errorElement`),` prop on your route.`)),D.createElement(D.Fragment,null,D.createElement(`h2`,null,`Unexpected Application Error!`),D.createElement(`h3`,{style:{fontStyle:`italic`}},t),n?D.createElement(`pre`,{style:i},n):null,o)}var Mt=D.createElement(jt,null),Nt=class extends D.Component{constructor(e){super(e),this.state={location:e.location,revalidation:e.revalidation,error:e.error}}static getDerivedStateFromError(e){return{error:e}}static getDerivedStateFromProps(e,t){return t.location!==e.location||t.revalidation!==`idle`&&e.revalidation===`idle`?{error:e.error,location:e.location,revalidation:e.revalidation}:{error:e.error===void 0?t.error:e.error,location:t.location,revalidation:e.revalidation||t.revalidation}}componentDidCatch(e,t){this.props.onError?this.props.onError(e,t):console.error(`React Router caught the following error during render`,e)}render(){let e=this.state.error;if(this.context&&typeof e==`object`&&e&&`digest`in e&&typeof e.digest==`string`){let t=bt(e.digest);t&&(e=t)}let t=e===void 0?this.props.children:D.createElement(mt.Provider,{value:this.props.routeContext},D.createElement(ht.Provider,{value:e,children:this.props.component}));return this.context?D.createElement(Ft,{error:e},t):t}};Nt.contextType=ct;var Pt=new WeakMap;function Ft({children:e,error:t}){let{basename:n}=D.useContext(N);if(typeof t==`object`&&t&&`digest`in t&&typeof t.digest==`string`){let e=yt(t.digest);if(e){let r=Pt.get(t);if(r)throw r;let i=tt(e.location,n),a=i.absoluteURL||i.to;if(at(a))throw Error(`Invalid redirect location`);if(et&&!Pt.get(t))if(i.isExternal||e.reloadDocument)window.location.href=a;else{let n=Promise.resolve().then(()=>window.__reactRouterDataRouter.navigate(i.to,{replace:e.replace}));throw Pt.set(t,n),n}return D.createElement(`meta`,{httpEquiv:`refresh`,content:`0;url=${a}`})}}return e}function It({routeContext:e,match:t,children:n}){let r=D.useContext(ot);return r&&r.static&&r.staticContext&&(t.route.errorElement||t.route.ErrorBoundary)&&(r.staticContext._deepestRenderedBoundaryId=t.route.id),D.createElement(mt.Provider,{value:e},n)}function Lt(e,t=[],n){let r=n?.state;if(e==null){if(!r)return null;if(r.errors)e=r.matches;else if(t.length===0&&!r.initialized&&r.matches.length>0)e=r.matches;else return null}let i=e,a=r?.errors;if(a!=null){let e=i.findIndex(e=>e.route.id&&a?.[e.route.id]!==void 0);A(e>=0,`Could not find a matching route for errors on route IDs: ${Object.keys(a).join(`,`)}`),i=i.slice(0,Math.min(i.length,e+1))}let o=!1,s=-1;if(n&&r){o=r.renderFallback;for(let e=0;e<i.length;e++){let t=i[e];if((t.route.HydrateFallback||t.route.hydrateFallbackElement)&&(s=e),t.route.id){let{loaderData:e,errors:a}=r,c=t.route.loader&&!e.hasOwnProperty(t.route.id)&&(!a||a[t.route.id]===void 0);if(t.route.lazy||c){n.isStatic&&(o=!0),i=s>=0?i.slice(0,s+1):[i[0]];break}}}}let c=n?.onError,l=r&&c?(e,t)=>{c(e,{location:r.location,params:r.matches?.[0]?.params??{},pattern:$e(r.matches),errorInfo:t})}:void 0;return i.reduceRight((e,n,c)=>{let u,d=!1,f=null,p=null;r&&(u=a&&n.route.id?a[n.route.id]:void 0,f=n.route.errorElement||Mt,o&&(s<0&&c===0?(Yt(`route-fallback`,!1,"No `HydrateFallback` element provided to render during initial hydration"),d=!0,p=null):s===c&&(d=!0,p=n.route.hydrateFallbackElement||null)));let m=t.concat(i.slice(0,c+1)),h=()=>{let t;return t=u?f:d?p:n.route.Component?D.createElement(n.route.Component,null):n.route.element?n.route.element:e,D.createElement(It,{match:n,routeContext:{outlet:e,matches:m,isDataRoute:r!=null},children:t})};return r&&(n.route.ErrorBoundary||n.route.errorElement||c===0)?D.createElement(Nt,{location:r.location,revalidation:r.revalidation,component:f,error:u,children:h(),routeContext:{outlet:null,matches:m,isDataRoute:!0},onError:l}):h()},null)}function Rt(e){return`${e} must be used within a data router.  See https://reactrouter.com/en/main/routers/picking-a-router.`}function zt(e){let t=D.useContext(ot);return A(t,Rt(e)),t}function Bt(e){let t=D.useContext(st);return A(t,Rt(e)),t}function Vt(e){let t=D.useContext(mt);return A(t,Rt(e)),t}function Ht(e){let t=Vt(e),n=t.matches[t.matches.length-1];return A(n.route.id,`${e} can only be used on routes that contain a unique "id"`),n.route.id}function Ut(){return Ht(`useRouteId`)}function Wt(){let e=Bt(`useNavigation`);return D.useMemo(()=>{let{matches:t,historyAction:n,...r}=e.navigation;return r},[e.navigation])}function Gt(){let{matches:e,loaderData:t}=Bt(`useMatches`);return D.useMemo(()=>e.map(e=>ye(e,t)),[e,t])}function Kt(){let e=D.useContext(ht),t=Bt(`useRouteError`),n=Ht(`useRouteError`);return e===void 0?t.errors?.[n]:e}function qt(){let{router:e}=zt(`useNavigate`),t=Ht(`useNavigate`),n=D.useRef(!1);return Tt(()=>{n.current=!0}),D.useCallback(async(r,i={})=>{j(n.current,wt),n.current&&(typeof r==`number`?await e.navigate(r):await e.navigate(r,{fromRouteId:t,...i}))},[e,t])}var Jt={};function Yt(e,t,n){!t&&!Jt[e]&&(Jt[e]=!0,j(!1,n))}D.memo(Xt);function Xt({routes:e,manifest:t,future:n,state:r,isStatic:i,onError:a}){return At(e,void 0,{manifest:t,state:r,isStatic:i,onError:a,future:n})}function Zt({to:e,replace:t,state:n,relative:r}){A(St(),`<Navigate> may be used only in the context of a <Router> component.`);let{static:i}=D.useContext(N);j(!i,`<Navigate> must not be used on the initial render in a <StaticRouter>. This is a no-op, but you should modify your code so the <Navigate> is only ever rendered in response to some user interaction or state change.`);let{matches:a}=D.useContext(mt),{pathname:o}=Ct(),s=Et(),c=We(e,Ue(a),o,r===`path`),l=JSON.stringify(c);return D.useEffect(()=>{s(JSON.parse(l),{replace:t,state:n,relative:r})},[s,l,r,t,n]),null}function Qt(e){A(!1,`A <Route> is only ever to be used as the child of <Routes> element, never rendered directly. Please wrap your <Route> in a <Routes>.`)}function $t({basename:e=`/`,children:t=null,location:n,navigationType:r=`POP`,navigator:i,static:a=!1,useTransitions:o}){A(!St(),`You cannot render a <Router> inside another <Router>. You should never have more than one in your app.`);let s=e.replace(/^\/*/,`/`),c=D.useMemo(()=>({basename:s,navigator:i,static:a,useTransitions:o,future:{}}),[s,i,a,o]);typeof n==`string`&&(n=me(n));let{pathname:l=`/`,search:u=``,hash:d=``,state:f=null,key:p=`default`,mask:m}=n,h=D.useMemo(()=>{let e=Re(l,s);return e==null?null:{location:{pathname:e,search:u,hash:d,state:f,key:p,mask:m},navigationType:r}},[s,l,u,d,f,p,r,m]);return j(h!=null,`<Router basename="${s}"> is not able to match the URL "${l}${u}${d}" because it does not start with the basename, so the <Router> won't render anything.`),h==null?null:D.createElement(N.Provider,{value:c},D.createElement(pt.Provider,{children:t,value:h}))}function en({children:e,location:t}){return kt(tn(e),t)}D.Component;function tn(e,t=[]){let n=[];return D.Children.forEach(e,(e,r)=>{if(!D.isValidElement(e))return;let i=[...t,r];if(e.type===D.Fragment){n.push.apply(n,tn(e.props.children,i));return}A(e.type===Qt,`[${typeof e.type==`string`?e.type:e.type.name}] is not a <Route> component. All component children of <Routes> must be a <Route> or <React.Fragment>`),A(!e.props.index||!e.props.children,`An index route cannot have child routes.`);let a={id:e.props.id||i.join(`-`),caseSensitive:e.props.caseSensitive,element:e.props.element,Component:e.props.Component,index:e.props.index,path:e.props.path,middleware:e.props.middleware,loader:e.props.loader,action:e.props.action,hydrateFallbackElement:e.props.hydrateFallbackElement,HydrateFallback:e.props.HydrateFallback,errorElement:e.props.errorElement,ErrorBoundary:e.props.ErrorBoundary,hasErrorBoundary:e.props.hasErrorBoundary===!0||e.props.ErrorBoundary!=null||e.props.errorElement!=null,shouldRevalidate:e.props.shouldRevalidate,handle:e.props.handle,lazy:e.props.lazy};e.props.children&&(a.children=tn(e.props.children,i)),n.push(a)}),n}var nn=`get`,rn=`application/x-www-form-urlencoded`;function an(e){return typeof HTMLElement<`u`&&e instanceof HTMLElement}function on(e){return an(e)&&e.tagName.toLowerCase()===`button`}function sn(e){return an(e)&&e.tagName.toLowerCase()===`form`}function cn(e){return an(e)&&e.tagName.toLowerCase()===`input`}function ln(e){return!!(e.metaKey||e.altKey||e.ctrlKey||e.shiftKey)}function un(e,t){return e.button===0&&(!t||t===`_self`)&&!ln(e)}var dn=null;function fn(){if(dn===null)try{new FormData(document.createElement(`form`),0),dn=!1}catch{dn=!0}return dn}var pn=new Set([`application/x-www-form-urlencoded`,`multipart/form-data`,`text/plain`]);function mn(e){return e!=null&&!pn.has(e)?(j(!1,`"${e}" is not a valid \`encType\` for \`<Form>\`/\`<fetcher.Form>\` and will default to "${rn}"`),null):e}function hn(e,t){let n,r,i,a,o;if(sn(e)){let o=e.getAttribute(`action`);r=o?Re(o,t):null,n=e.getAttribute(`method`)||nn,i=mn(e.getAttribute(`enctype`))||rn,a=new FormData(e)}else if(on(e)||cn(e)&&(e.type===`submit`||e.type===`image`)){let o=e.form;if(o==null)throw Error(`Cannot submit a <button> or <input type="submit"> without a <form>`);let s=e.getAttribute(`formaction`)||o.getAttribute(`action`);if(r=s?Re(s,t):null,n=e.getAttribute(`formmethod`)||o.getAttribute(`method`)||nn,i=mn(e.getAttribute(`formenctype`))||mn(o.getAttribute(`enctype`))||rn,a=new FormData(o,e),!fn()){let{name:t,type:n,value:r}=e;if(n===`image`){let e=t?`${t}.`:``;a.append(`${e}x`,`0`),a.append(`${e}y`,`0`)}else t&&a.append(t,r)}}else if(an(e))throw Error(`Cannot submit element that is not <form>, <button>, or <input type="submit|image">`);else n=nn,r=null,i=rn,o=e;return a&&i===`text/plain`&&(o=a,a=void 0),{action:r,method:n.toLowerCase(),encType:i,formData:a,body:o}}Object.getOwnPropertyNames(Object.prototype).sort().join(`\0`);var gn={"&":`\\u0026`,">":`\\u003e`,"<":`\\u003c`,"\u2028":`\\u2028`,"\u2029":`\\u2029`},_n=/[&><\u2028\u2029]/g;function vn(e){return e.replace(_n,e=>gn[e])}function yn(e,t){if(e===!1||e==null)throw Error(t)}function bn(e,t,n,r){let i=typeof e==`string`?new URL(e,typeof window>`u`?`server://singlefetch/`:window.location.origin):e;return n?i.pathname.endsWith(`/`)?i.pathname=`${i.pathname}_.${r}`:i.pathname=`${i.pathname}.${r}`:i.pathname===`/`?i.pathname=`_root.${r}`:t&&Re(i.pathname,t)===`/`?i.pathname=`${qe(t)}/_root.${r}`:i.pathname=`${qe(i.pathname)}.${r}`,i}async function xn(e,t){if(e.id in t)return t[e.id];try{let n=await oe(()=>import(e.module),[]);return t[e.id]=n,n}catch(t){return console.error(`Error loading route module \`${e.module}\`, reloading page...`),console.error(t),window.__reactRouterContext&&window.__reactRouterContext.isSpaMode,window.location.reload(),new Promise(()=>{})}}function Sn(e){return e!=null&&typeof e.page==`string`}function Cn(e){return e==null?!1:e.href==null?e.rel===`preload`&&typeof e.imageSrcSet==`string`&&typeof e.imageSizes==`string`:typeof e.rel==`string`&&typeof e.href==`string`}async function wn(e,t,n){return kn((await Promise.all(e.map(async e=>{let r=t.routes[e.route.id];if(r){let e=await xn(r,n);return e.links?e.links():[]}return[]}))).flat(1).filter(Cn).filter(e=>e.rel===`stylesheet`||e.rel===`preload`).map(e=>e.rel===`stylesheet`?{...e,rel:`prefetch`,as:`style`}:{...e,rel:`prefetch`}))}function Tn(e,t,n,r,i,a){let o=(e,t)=>!n[t]||e.route.id!==n[t].route.id,s=(e,t)=>n[t].pathname!==e.pathname||n[t].route.path?.endsWith(`*`)&&n[t].params[`*`]!==e.params[`*`];return a===`assets`?t.filter((e,t)=>o(e,t)||s(e,t)):a===`data`?t.filter((t,a)=>{let c=r.routes[t.route.id];if(!c||!c.hasLoader)return!1;if(o(t,a)||s(t,a))return!0;if(t.route.shouldRevalidate){let r=t.route.shouldRevalidate({currentUrl:new URL(i.pathname+i.search+i.hash,window.origin),currentParams:n[0]?.params||{},nextUrl:new URL(e,window.origin),nextParams:t.params,defaultShouldRevalidate:!0});if(typeof r==`boolean`)return r}return!0}):[]}function En(e,t,{includeHydrateFallback:n}={}){return Dn(e.map(e=>{let r=t.routes[e.route.id];if(!r)return[];let i=[r.module];return r.clientActionModule&&(i=i.concat(r.clientActionModule)),r.clientLoaderModule&&(i=i.concat(r.clientLoaderModule)),n&&r.hydrateFallbackModule&&(i=i.concat(r.hydrateFallbackModule)),r.imports&&(i=i.concat(r.imports)),i}).flat(1))}function Dn(e){return[...new Set(e)]}function On(e){let t={},n=Object.keys(e).sort();for(let r of n)t[r]=e[r];return t}function kn(e,t){let n=new Set,r=new Set(t);return e.reduce((e,i)=>{if(t&&!Sn(i)&&i.as===`script`&&i.href&&r.has(i.href))return e;let a=JSON.stringify(On(i));return n.has(a)||(n.add(a),e.push({key:a,link:i})),e},[])}function An(){let e=D.useContext(ot);return yn(e,`You must render this element inside a <DataRouterContext.Provider> element`),e}function jn(){let e=D.useContext(st);return yn(e,`You must render this element inside a <DataRouterStateContext.Provider> element`),e}var Mn=D.createContext(void 0);Mn.displayName=`FrameworkContext`;function Nn(){let e=D.useContext(Mn);return yn(e,`You must render this element inside a <HydratedRouter> element`),e}function Pn(e,t){let n=D.useContext(Mn),[r,i]=D.useState(!1),[a,o]=D.useState(!1),{onFocus:s,onBlur:c,onMouseEnter:l,onMouseLeave:u,onTouchStart:d}=t,f=D.useRef(null);D.useEffect(()=>{if(e===`render`&&o(!0),e===`viewport`){let e=new IntersectionObserver(e=>{e.forEach(e=>{o(e.isIntersecting)})},{threshold:.5});return f.current&&e.observe(f.current),()=>{e.disconnect()}}},[e]),D.useEffect(()=>{if(r){let e=setTimeout(()=>{o(!0)},100);return()=>{clearTimeout(e)}}},[r]);let p=()=>{i(!0)},m=()=>{i(!1),o(!1)};return n?e===`intent`?[a,f,{onFocus:Fn(s,p),onBlur:Fn(c,m),onMouseEnter:Fn(l,p),onMouseLeave:Fn(u,m),onTouchStart:Fn(d,p)}]:[a,f,{}]:[!1,f,{}]}function Fn(e,t){return n=>{e&&e(n),n.defaultPrevented||t(n)}}function In({page:e,...t}){let n=lt(),{nonce:r}=Nn(),{router:i}=An(),a=D.useMemo(()=>_e(i.routes,e,i.basename),[i.routes,e,i.basename]);return a?(t.nonce==null&&r&&(t={...t,nonce:r}),n?D.createElement(Rn,{page:e,matches:a,...t}):D.createElement(zn,{page:e,matches:a,...t})):null}function Ln(e){let{manifest:t,routeModules:n}=Nn(),[r,i]=D.useState([]);return D.useEffect(()=>{let r=!1;return wn(e,t,n).then(e=>{r||i(e)}),()=>{r=!0}},[e,t,n]),r}function Rn({page:e,matches:t,...n}){let r=Ct(),{future:i}=Nn(),{basename:a}=An(),o=D.useMemo(()=>{if(e===r.pathname+r.search+r.hash)return[];let n=bn(e,a,i.v8_trailingSlashAwareDataRequests,`rsc`),o=!1,s=[];for(let e of t)typeof e.route.shouldRevalidate==`function`?o=!0:s.push(e.route.id);return o&&s.length>0&&n.searchParams.set(`_routes`,s.join(`,`)),[n.pathname+n.search]},[a,i.v8_trailingSlashAwareDataRequests,e,r,t]);return D.createElement(D.Fragment,null,o.map(e=>D.createElement(`link`,{key:e,rel:`prefetch`,as:`fetch`,href:e,...n})))}function zn({page:e,matches:t,...n}){let r=Ct(),{future:i,manifest:a,routeModules:o}=Nn(),{basename:s}=An(),{loaderData:c,matches:l}=jn(),u=D.useMemo(()=>Tn(e,t,l,a,r,`data`),[e,t,l,a,r]),d=D.useMemo(()=>Tn(e,t,l,a,r,`assets`),[e,t,l,a,r]),f=D.useMemo(()=>{if(e===r.pathname+r.search+r.hash)return[];let n=new Set,l=!1;if(t.forEach(e=>{let t=a.routes[e.route.id];!t||!t.hasLoader||(!u.some(t=>t.route.id===e.route.id)&&e.route.id in c&&o[e.route.id]?.shouldRevalidate||t.hasClientLoader?l=!0:n.add(e.route.id))}),n.size===0)return[];let d=bn(e,s,i.v8_trailingSlashAwareDataRequests,`data`);return l&&n.size>0&&d.searchParams.set(`_routes`,t.filter(e=>n.has(e.route.id)).map(e=>e.route.id).join(`,`)),[d.pathname+d.search]},[s,i.v8_trailingSlashAwareDataRequests,c,r,a,u,t,e,o]),p=D.useMemo(()=>En(d,a),[d,a]),m=Ln(d);return D.createElement(D.Fragment,null,f.map(e=>D.createElement(`link`,{key:e,rel:`prefetch`,as:`fetch`,href:e,...n})),p.map(e=>D.createElement(`link`,{key:e,rel:`modulepreload`,href:e,...n})),m.map(({key:e,link:t})=>D.createElement(`link`,{key:e,nonce:n.nonce,...t,crossOrigin:t.crossOrigin??n.crossOrigin})))}function Bn(...e){return t=>{e.forEach(e=>{typeof e==`function`?e(t):e!=null&&(e.current=t)})}}D.Component;var Vn=typeof window<`u`&&window.document!==void 0&&window.document.createElement!==void 0;try{Vn&&(window.__reactRouterVersion=`7.18.1`)}catch{}function Hn({basename:e,children:t,useTransitions:n,window:r}){let i=D.useRef();i.current??=ue({window:r,v5Compat:!0});let a=i.current,[o,s]=D.useState({action:a.action,location:a.location}),c=D.useCallback(e=>{n===!1?s(e):D.startTransition(()=>s(e))},[n]);return D.useLayoutEffect(()=>a.listen(c),[a,c]),D.createElement($t,{basename:e,children:t,location:o.location,navigationType:o.action,navigator:a,useTransitions:n})}function Un({basename:e,children:t,history:n,useTransitions:r}){let[i,a]=D.useState({action:n.action,location:n.location}),o=D.useCallback(e=>{r===!1?a(e):D.startTransition(()=>a(e))},[r]);return D.useLayoutEffect(()=>n.listen(o),[n,o]),D.createElement($t,{basename:e,children:t,location:i.location,navigationType:i.action,navigator:n,useTransitions:r})}Un.displayName=`unstable_HistoryRouter`;var Wn=D.forwardRef(function({onClick:e,discover:t=`render`,prefetch:n=`none`,relative:r,reloadDocument:i,replace:a,mask:o,state:s,target:c,to:l,preventScrollReset:u,viewTransition:d,defaultShouldRevalidate:f,...p},m){let{basename:h,navigator:g,useTransitions:_}=D.useContext(N),v=typeof l==`string`&&se.test(l),y=tt(l,h);l=y.to;let b=xt(l,{relative:r}),x=Ct(),S=null;if(o){let e=We(o,[],x.mask?x.mask.pathname:`/`,!0);h!==`/`&&(e.pathname=e.pathname===`/`?h:Ke([h,e.pathname])),S=g.createHref(e)}let[C,ee,te]=Pn(n,p),ne=Zn(l,{replace:a,mask:o,state:s,target:c,preventScrollReset:u,relative:r,viewTransition:d,defaultShouldRevalidate:f,useTransitions:_});function w(t){e&&e(t),t.defaultPrevented||ne(t)}let T=!(y.isExternal||i),re=D.createElement(`a`,{...p,...te,href:(T?S:void 0)||y.absoluteURL||b,onClick:T?w:e,ref:Bn(m,ee),target:c,"data-discover":!v&&t===`render`?`true`:void 0});return C&&!v?D.createElement(D.Fragment,null,re,D.createElement(In,{page:b})):re});Wn.displayName=`Link`;var Gn=D.forwardRef(function({"aria-current":e=`page`,caseSensitive:t=!1,className:n=``,end:r=!1,style:i,to:a,viewTransition:o,children:s,...c},l){let u=Ot(a,{relative:c.relative}),d=Ct(),f=D.useContext(st),{navigator:p,basename:m}=D.useContext(N),h=f!=null&&sr(u)&&o===!0,g=p.encodeLocation?p.encodeLocation(u).pathname:u.pathname,_=d.pathname,v=f&&f.navigation&&f.navigation.location?f.navigation.location.pathname:null;t||(_=_.toLowerCase(),v=v?v.toLowerCase():null,g=g.toLowerCase()),v&&m&&(v=Re(v,m)||v);let y=g!==`/`&&g.endsWith(`/`)?g.length-1:g.length,b=_===g||!r&&_.startsWith(g)&&_.charAt(y)===`/`,x=v!=null&&(v===g||!r&&v.startsWith(g)&&v.charAt(g.length)===`/`),S={isActive:b,isPending:x,isTransitioning:h},C=b?e:void 0,ee;ee=typeof n==`function`?n(S):[n,b?`active`:null,x?`pending`:null,h?`transitioning`:null].filter(Boolean).join(` `);let te=typeof i==`function`?i(S):i;return D.createElement(Wn,{...c,"aria-current":C,className:ee,ref:l,style:te,to:a,viewTransition:o},typeof s==`function`?s(S):s)});Gn.displayName=`NavLink`;var Kn=D.forwardRef(({discover:e=`render`,fetcherKey:t,navigate:n,reloadDocument:r,replace:i,state:a,method:o=nn,action:s,onSubmit:c,relative:l,preventScrollReset:u,viewTransition:d,defaultShouldRevalidate:f,...p},m)=>{let{useTransitions:h}=D.useContext(N),g=er(),_=tr(s,{relative:l}),v=o.toLowerCase()===`get`?`get`:`post`,y=typeof s==`string`&&se.test(s);return D.createElement(`form`,{ref:m,method:v,action:_,onSubmit:r?c:e=>{if(c&&c(e),e.defaultPrevented)return;e.preventDefault();let r=e.nativeEvent.submitter,s=r?.getAttribute(`formmethod`)||o,p=()=>g(r||e.currentTarget,{fetcherKey:t,method:s,navigate:n,replace:i,state:a,relative:l,preventScrollReset:u,viewTransition:d,defaultShouldRevalidate:f});h&&n!==!1?D.startTransition(()=>p()):p()},...p,"data-discover":!y&&e===`render`?`true`:void 0})});Kn.displayName=`Form`;function qn({getKey:e,storageKey:t,...n}){let r=D.useContext(Mn),{basename:i}=D.useContext(N),a=Ct(),o=Gt();ar({getKey:e,storageKey:t});let s=D.useMemo(()=>{if(!r||!e)return null;let t=ir(a,o,i,e);return t===a.key?null:t},[]);if(!r||r.isSpaMode)return null;let c=((e,t)=>{if(!window.history.state||!window.history.state.key){let e=Math.random().toString(32).slice(2);window.history.replaceState({key:e},``)}try{let n=JSON.parse(sessionStorage.getItem(e)||`{}`)[t||window.history.state.key];typeof n==`number`&&window.scrollTo(0,n)}catch(t){console.error(t),sessionStorage.removeItem(e)}}).toString();return n.nonce==null&&r?.nonce&&(n.nonce=r.nonce),D.createElement(`script`,{...n,suppressHydrationWarning:!0,dangerouslySetInnerHTML:{__html:`(${c})(${vn(JSON.stringify(t||nr))}, ${vn(JSON.stringify(s))})`}})}qn.displayName=`ScrollRestoration`;function Jn(e){return`${e} must be used within a data router.  See https://reactrouter.com/en/main/routers/picking-a-router.`}function Yn(e){let t=D.useContext(ot);return A(t,Jn(e)),t}function Xn(e){let t=D.useContext(st);return A(t,Jn(e)),t}function Zn(e,{target:t,replace:n,mask:r,state:i,preventScrollReset:a,relative:o,viewTransition:s,defaultShouldRevalidate:c,useTransitions:l}={}){let u=Et(),d=Ct(),f=Ot(e,{relative:o});return D.useCallback(p=>{if(un(p,t)){p.preventDefault();let t=n===void 0?pe(d)===pe(f):n,m=()=>u(e,{replace:t,mask:r,state:i,preventScrollReset:a,relative:o,viewTransition:s,defaultShouldRevalidate:c});l?D.startTransition(()=>m()):m()}},[d,u,f,n,r,i,t,e,a,o,s,c,l])}var Qn=0,$n=()=>`__${String(++Qn)}__`;function er(){let{router:e}=Yn(`useSubmit`),{basename:t}=D.useContext(N),n=Ut(),r=e.fetch,i=e.navigate;return D.useCallback(async(e,a={})=>{let{action:o,method:s,encType:c,formData:l,body:u}=hn(e,t);if(a.navigate===!1){let e=a.fetcherKey||$n();await r(e,n,a.action||o,{defaultShouldRevalidate:a.defaultShouldRevalidate,preventScrollReset:a.preventScrollReset,formData:l,body:u,formMethod:a.method||s,formEncType:a.encType||c,flushSync:a.flushSync})}else await i(a.action||o,{defaultShouldRevalidate:a.defaultShouldRevalidate,preventScrollReset:a.preventScrollReset,formData:l,body:u,formMethod:a.method||s,formEncType:a.encType||c,replace:a.replace,state:a.state,fromRouteId:n,flushSync:a.flushSync,viewTransition:a.viewTransition})},[r,i,t,n])}function tr(e,{relative:t}={}){let{basename:n}=D.useContext(N),r=D.useContext(mt);A(r,`useFormAction must be used inside a RouteContext`);let[i]=r.matches.slice(-1),a={...Ot(e||`.`,{relative:t})},o=Ct();if(e==null){a.search=o.search;let e=new URLSearchParams(a.search),t=e.getAll(`index`);if(t.some(e=>e===``)){e.delete(`index`),t.filter(e=>e).forEach(t=>e.append(`index`,t));let n=e.toString();a.search=n?`?${n}`:``}}return(!e||e===`.`)&&i.route.index&&(a.search=a.search?a.search.replace(/^\?/,`?index&`):`?index`),n!==`/`&&(a.pathname=a.pathname===`/`?n:Ke([n,a.pathname])),pe(a)}var nr=`react-router-scroll-positions`,rr={};function ir(e,t,n,r){let i=null;return r&&(i=r(n===`/`?e:{...e,pathname:Re(e.pathname,n)||e.pathname},t)),i??=e.key,i}function ar({getKey:e,storageKey:t}={}){let{router:n}=Yn(`useScrollRestoration`),{restoreScrollPosition:r,preventScrollReset:i}=Xn(`useScrollRestoration`),{basename:a}=D.useContext(N),o=Ct(),s=Gt(),c=Wt();D.useEffect(()=>(window.history.scrollRestoration=`manual`,()=>{window.history.scrollRestoration=`auto`}),[]),or(D.useCallback(()=>{if(c.state===`idle`){let t=ir(o,s,a,e);rr[t]=window.scrollY}try{sessionStorage.setItem(t||nr,JSON.stringify(rr))}catch(e){j(!1,`Failed to save scroll positions in sessionStorage, <ScrollRestoration /> will not work properly (${e}).`)}window.history.scrollRestoration=`auto`},[c.state,e,a,o,s,t])),typeof document<`u`&&(D.useLayoutEffect(()=>{try{let e=sessionStorage.getItem(t||nr);e&&(rr=JSON.parse(e))}catch{}},[t]),D.useLayoutEffect(()=>{let t=n?.enableScrollRestoration(rr,()=>window.scrollY,e?(t,n)=>ir(t,n,a,e):void 0);return()=>t&&t()},[n,a,e]),D.useLayoutEffect(()=>{if(r!==!1){if(typeof r==`number`){window.scrollTo(0,r);return}try{if(o.hash){let e=document.getElementById(decodeURIComponent(o.hash.slice(1)));if(e){e.scrollIntoView();return}}}catch{j(!1,`"${o.hash.slice(1)}" is not a decodable element ID. The view will not scroll to it.`)}i!==!0&&window.scrollTo(0,0)}},[o,r,i]))}function or(e,t){let{capture:n}=t||{};D.useEffect(()=>{let t=n==null?void 0:{capture:n};return window.addEventListener(`pagehide`,e,t),()=>{window.removeEventListener(`pagehide`,e,t)}},[e,n])}function sr(e,{relative:t}={}){let n=D.useContext(ut);A(n!=null,"`useViewTransitionState` must be used within `react-router-dom`'s `RouterProvider`.  Did you accidentally import `RouterProvider` from `react-router`?");let{basename:r}=Yn(`useViewTransitionState`),i=Ot(e,{relative:t});if(!n.isTransitioning)return!1;let a=Re(n.currentLocation.pathname,r)||n.currentLocation.pathname,o=Re(n.nextLocation.pathname,r)||n.nextLocation.pathname;return Pe(i.pathname,o)!=null||Pe(i.pathname,a)!=null}var cr=re(),lr={wrapper:`_wrapper_bt1w8_2`,header:`_header_bt1w8_10`,headerActions:`_headerActions_bt1w8_21`,title:`_title_bt1w8_27`,panelGroup:`_panelGroup_bt1w8_36`,clipboardToggle:`_clipboardToggle_bt1w8_43`,helpToggle:`_helpToggle_bt1w8_66`,helpButtonWrapper:`_helpButtonWrapper_bt1w8_93`,helpTogglePulsing:`_helpTogglePulsing_bt1w8_97`,helpPulse:`_helpPulse_bt1w8_1`,helpHint:`_helpHint_bt1w8_112`,helpHintFading:`_helpHintFading_bt1w8_139`,helpHintKbd:`_helpHintKbd_bt1w8_144`,resizeHandle:`_resizeHandle_bt1w8_153`},ur=e=>{try{return!new DOMParser().parseFromString(e.trim(),`text/xml`).querySelector(`parsererror`)}catch{return!1}},dr=e=>{try{return JSON.parse(e),!0}catch{return!1}},fr=e=>e.trim()?dr(e)?{valid:!0,error:null,type:`json`}:ur(e)?{valid:!0,error:null,type:`xml`}:{valid:!1,error:`Invalid JSON/XML format`,type:null}:{valid:!0,error:null,type:null},pr=e=>{try{let t=JSON.parse(e);return JSON.stringify(t,null,2)}catch{return e}},mr=()=>{let[e,t]=(0,D.useState)([]),n=(0,D.useRef)(0),r=(0,D.useRef)(new Set);return(0,D.useEffect)(()=>()=>{r.current.forEach(clearTimeout)},[]),{toasts:e,addToast:(0,D.useCallback)((e,i=`info`)=>{let a=++n.current;t(t=>[...t,{id:a,message:e,type:i}]);let o=setTimeout(()=>{r.current.delete(o),t(e=>e.filter(e=>e.id!==a))},3e3);r.current.add(o)},[]),removeToast:(0,D.useCallback)(e=>{t(t=>t.filter(t=>t.id!==e))},[])}},hr=(e,t)=>{let n=(0,D.useCallback)(()=>{try{let n=window.localStorage.getItem(e);return n?JSON.parse(n):t}catch{return t}},[e]),[r,i]=(0,D.useState)(n);return(0,D.useEffect)(()=>{i(n())},[e]),(0,D.useEffect)(()=>{try{window.localStorage.setItem(e,JSON.stringify(r))}catch(t){console.error(`Error setting localStorage key "${e}":`,t)}},[e,r]),(0,D.useEffect)(()=>{let t=t=>{(t.key===e||t.key===null)&&i(n())};return window.addEventListener(`storage`,t),()=>window.removeEventListener(`storage`,t)},[e,n]),(0,D.useEffect)(()=>{let e=()=>i(n());return window.addEventListener(`focus`,e),document.addEventListener(`visibilitychange`,e),()=>{window.removeEventListener(`focus`,e),document.removeEventListener(`visibilitychange`,e)}},[n]),[r,i]},gr=2e4,_r=[{path:`/json-path`,label:`JSON-Path`,title:`JSON-Path Playground`,wsPath:`/ws/json/path`,storageKeyPayload:`jsonpath-last-payload`,storageKeyHistory:`jsonpath-command-history`,storageKeyTab:`jsonpath-right-tab`,supportsUpload:!0,tabs:[`payload`,`graph`,`graph-data`]},{path:`/`,label:`Minigraph`,title:`Minigraph Playground`,wsPath:`/ws/graph/playground`,storageKeyPayload:`minigraph-last-payload`,storageKeyHistory:`minigraph-command-history`,storageKeyTab:`minigraph-right-tab`,storageKeySavedGraphs:`minigraph-saved-graphs`,storageKeyHelpTopic:`minigraph-help-topic`,supportsClipboard:!0,supportsHelp:!0,supportsAuthoring:!0,tabs:[`graph`,`graph-data`]}],vr={json_simple:JSON.stringify({name:`John Doe`,age:30,city:`New York`},null,2),json_nested:JSON.stringify({user:{name:`Jane Smith`,profile:{email:`jane@example.com`,address:{city:`San Francisco`,country:`USA`}}}},null,2),json_array:JSON.stringify([{id:1,name:`Item 1`,status:`active`},{id:2,name:`Item 2`,status:`pending`},{id:3,name:`Item 3`,status:`inactive`}],null,2),xml_simple:`<?xml version="1.0" encoding="UTF-8"?>
<person>
  <name>John Doe</name>
  <age>30</age>
  <city>New York</city>
</person>`,xml_nested:`<?xml version="1.0" encoding="UTF-8"?>
<user>
  <name>Jane Smith</name>
  <profile>
    <email>jane@example.com</email>
    <address>
      <city>San Francisco</city>
      <country>USA</country>
    </address>
  </profile>
</user>`,xml_array:`<?xml version="1.0" encoding="UTF-8"?>
<items>
  <item>
    <id>1</id>
    <name>Item 1</name>
    <status>active</status>
  </item>
  <item>
    <id>2</id>
    <name>Item 2</name>
    <status>pending</status>
  </item>
  <item>
    <id>3</id>
    <name>Item 3</name>
    <status>inactive</status>
  </item>
</items>`};function yr(e){return`ws://${window.location.host}${e}`}var P=b();function br(e,t,n,r){let i=e[t]??{phase:`idle`,messages:[]},a=[...i.messages,{id:n,raw:r}];return a.length>200&&a.shift(),{...e,[t]:{...i,messages:a}}}function xr(e,t){let n=e[t.path]??{phase:`idle`,messages:[]};switch(t.type){case`CONNECTING`:return{...e,[t.path]:{...n,phase:`connecting`}};case`CONNECTED`:return br({...e,[t.path]:{...n,phase:`connected`}},t.path,t.id,t.msg);case`MESSAGE_RECEIVED`:return br(e,t.path,t.id,t.msg);case`DISCONNECTED`:return br({...e,[t.path]:{...n,phase:`idle`}},t.path,t.id,t.msg);case`CONNECT_ERROR`:return{...e,[t.path]:{...n,phase:`idle`}};case`CLEAR_MESSAGES`:return{...e,[t.path]:{...n,messages:[]}};default:return e}}var Sr=(0,D.createContext)(null);function Cr({children:e}){let[t,n]=(0,D.useReducer)(xr,{}),r=(0,D.useRef)({}),i=(0,D.useRef)({}),a=(0,D.useRef)({});(0,D.useEffect)(()=>()=>{Object.entries(r.current).forEach(([e,t])=>{t?.close();let n=i.current[e];n&&clearInterval(n)})},[]);let o=e=>yr(e),s=e=>(a.current[e]=(a.current[e]??0)+1,a.current[e]),c=()=>{let e=new Date().toString(),t=e.indexOf(`GMT`);return t>0?e.substring(0,t).trim():e},l=(e,t)=>JSON.stringify({type:e,message:t,time:c()}),u=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t){let e=t.type;return e===`ping`||e===`pong`}}catch{}return!1},d=(0,D.useCallback)((e,t)=>{if(!window.WebSocket){t?.(`WebSocket not supported by your browser`,`error`);return}let a=r.current[e];if(a&&(a.readyState===WebSocket.OPEN||a.readyState===WebSocket.CONNECTING)){t?.(`Already connected`,`error`);return}n({type:`CONNECTING`,path:e});let c=new WebSocket(o(e));r.current[e]=c,c.onopen=()=>{n({type:`CONNECTED`,path:e,id:s(e),msg:l(`info`,`connected`)}),t?.(`Connected to WebSocket`,`success`),c.send(JSON.stringify({type:`welcome`})),i.current[e]=setInterval(()=>{c.readyState===WebSocket.OPEN&&c.send(l(`ping`,`keep alive`))},gr)},c.onmessage=t=>{u(t.data)||n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t.data})},c.onerror=()=>{n({type:`CONNECT_ERROR`,path:e})},c.onclose=a=>{let o=i.current[e];o&&(clearInterval(o),i.current[e]=null),n({type:`DISCONNECTED`,path:e,id:s(e),msg:l(`info`,`disconnected - (${a.code}) ${a.reason}`)}),t?.(`Disconnected from WebSocket`,`info`),r.current[e]===c&&(r.current[e]=null)}},[]),f=(0,D.useCallback)(e=>{let t=r.current[e];t?t.close():n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:l(`error`,`already disconnected`)})},[]);(0,D.useEffect)(()=>(_r.forEach(e=>{d(e.wsPath)}),()=>{_r.forEach(e=>{let t=r.current[e.wsPath];t&&t.close()})}),[]);let p=(0,D.useCallback)((e,t)=>{let n=r.current[e];return n&&n.readyState===WebSocket.OPEN?(n.send(t),!0):!1},[]),m=(0,D.useCallback)((e,t)=>{n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t})},[]),h=(0,D.useCallback)(e=>{n({type:`CLEAR_MESSAGES`,path:e})},[]),[g,_]=(0,D.useState)({}),v=(0,D.useCallback)((e,t)=>{_(n=>{if(t===null){let t={...n};return delete t[e],t}return{...n,[e]:t}})},[]),y=(0,D.useCallback)(e=>g[e]??null,[g]),b=(0,D.useCallback)(e=>{let t=g[e]??null;return t!==null&&_(t=>{let n={...t};return delete n[e],n}),t},[g]),x=(0,D.useCallback)(e=>t[e]??{phase:`idle`,messages:[]},[t]),S=(0,D.useMemo)(()=>({getSlot:x,connect:d,disconnect:f,send:p,appendMessage:m,clearMessages:h,setPendingPayload:v,peekPendingPayload:y,takePendingPayload:b}),[x,d,f,p,m,h,v,y,b]);return(0,P.jsx)(Sr.Provider,{value:S,children:e})}function wr(){let e=(0,D.useContext)(Sr);if(!e)throw Error(`useWebSocketContext must be used inside <WebSocketProvider>`);return e}var Tr=e=>{try{let t=JSON.parse(e);return{type:t.type||`info`,message:t.message||e,time:t.time,raw:e}}catch{return{type:`raw`,message:e,time:null,raw:e}}},Er=e=>({info:`ℹ️`,error:`❌`,ping:`🔄`,welcome:`👋`,raw:``})[e]??`•`,Dr=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t)return{isJSON:!0,data:t}}catch{}return{isJSON:!1,data:null}};function Or(e){if(!e.includes(`Graph exported to `))return null;let t=jr(e);if(!t)return null;let n=t.split(`/`)[4];return n?{graphName:n,apiPath:t}:null}function kr(e){return e.includes(`Invalid filename`)?{reason:`invalid-name`}:e.includes(`Expect root node name`)?{reason:`root-name-conflict`}:null}function Ar(e){let t=Dr(e);return t.isJSON?(t.data.type,!1):!0}function jr(e){let t=e.match(/\/api\/graph\/model\/([^\s'"]+)/);return t?t[0]:null}function Mr(e){return Ar(e)?jr(e)!==null:!1}function Nr(e){let t=e.match(/\/api\/json\/content\/([\w-]+)/);return t?t[0]:null}function Pr(e){let t=e.match(/Large payload \((\d+)\)\s*->\s*GET\s+(\/api\/inspect\/[^\s]+)/i);if(!t)return null;let n=parseInt(t[1],10),r=t[2];return{apiPath:r,byteSize:n,filename:`${r.split(`/`).filter(Boolean).pop()??`payload`}.json`}}function Fr(e){let t=e.match(/You may upload .*?->\s*POST\s+(\/api\/mock\/[\w-]+)/i);return t?t[1]:null}function Ir(e){if(!e.startsWith(`> `))return!1;let t=e.slice(2).trim().toLowerCase();return t===`help`||t.startsWith(`help `)?!0:t.startsWith(`describe `)?!t.slice(9).trim().startsWith(`graph`):!1}function Lr(e){if(!e.startsWith(`> `)||!e.slice(2).trimStart().toLowerCase().startsWith(`import graph from `))return null;let t=e.slice(2).trimStart().slice(18).trim();return t.length>0?t:null}var Rr=/^node ([A-Za-z0-9_-]+) created$/i,zr=/^node ([A-Za-z0-9_-]+) already exists$/i,Br=/^node ([A-Za-z0-9_-]+) updated$/i,Vr=/^node ([A-Za-z0-9_-]+) deleted$/i,Hr=/^node ([A-Za-z0-9_-]+) not found$/i,Ur=/^ERROR: (.+)$/;function Wr(e){let t=e.trim();if(t.startsWith(`> `))return null;let n=t.match(Rr);if(n)return{status:`accepted`,action:`create-node`,alias:n[1],message:t};let r=t.match(zr);if(r)return{status:`rejected`,action:`create-node`,alias:r[1],message:t};let i=t.match(Br);if(i)return{status:`accepted`,action:`edit-node`,alias:i[1],message:t};let a=t.match(Vr);if(a)return{status:`accepted`,action:`delete-node`,alias:a[1],message:t};let o=t.match(Hr);return o?{status:`rejected`,action:null,alias:o[1],message:t}:t.match(Ur)?{status:`error`,action:null,alias:null,message:t}:null}function Gr(e){if(!Ar(e)||e.startsWith(`> `)||Mr(e))return null;let t=e.toLowerCase();return t.includes(`graph model imported as draft`)?`import-graph`:t.includes(` -> `)&&t.includes(`removed`)||t.startsWith(`node `)&&(t.includes(` created`)||t.includes(` updated`)||t.includes(` deleted`)||t.includes(` connected to `)||t.includes(` imported from `)||t.includes(` overwritten by node from `))?`node-mutation`:null}var Kr={command:``,historyIndex:-1,draftCommand:``};function qr(e,t){switch(t.type){case`SET_COMMAND`:return{...e,command:t.value,historyIndex:-1,draftCommand:``};case`CLEAR_COMMAND`:return{...e,command:``,historyIndex:-1,draftCommand:``};case`SET_HISTORY_INDEX`:return{...e,historyIndex:t.index,command:t.command};case`ENTER_HISTORY`:return{...e,historyIndex:0,command:t.command,draftCommand:e.command};case`EXIT_HISTORY`:return{...e,historyIndex:-1,command:e.draftCommand,draftCommand:``};default:return e}}function Jr({wsPath:e,storageKeyHistory:t,payload:n,addToast:r,bus:i,handleLocalCommand:a}){let o=wr(),{phase:s,messages:c}=o.getSlot(e),l=s===`connected`,u=s===`connecting`,[d,f]=(0,D.useReducer)(qr,Kr),{command:p,historyIndex:m}=d,[h,g]=hr(t,[]),_=(0,D.useRef)(null),v=(0,D.useRef)(!1);(0,D.useEffect)(()=>{_.current&&(_.current.scrollTop=_.current.scrollHeight)},[c]);let y=(0,D.useCallback)(()=>{o.connect(e,r)},[o,e,r]),b=(0,D.useCallback)(()=>{o.disconnect(e)},[o,e]),x=(0,D.useCallback)(()=>{if(s!==`connected`)return;let t=p.trim();if(t.length!==0){if(a?.(t)===!0){h[0]!==t&&g(e=>[t,...e].slice(0,50)),o.appendMessage(e,`> `+t),f({type:`CLEAR_COMMAND`});return}o.send(e,t),h[0]!==t&&g(e=>[t,...e].slice(0,50)),t===`load`&&(n.length===0?o.appendMessage(e,`ERROR: please paste JSON/XML payload in input text area`):o.send(e,n)),f({type:`CLEAR_COMMAND`})}},[o,e,s,p,n,h,g,a]),S=(0,D.useCallback)(e=>{if(e.key===`ArrowUp`){if(e.preventDefault(),h.length===0)return;if(m===-1)f({type:`ENTER_HISTORY`,command:h[0]});else if(m<h.length-1){let e=m+1;f({type:`SET_HISTORY_INDEX`,index:e,command:h[e]})}}else if(e.key===`ArrowDown`)if(e.preventDefault(),m<=0)m===0&&f({type:`EXIT_HISTORY`});else{let e=m-1;f({type:`SET_HISTORY_INDEX`,index:e,command:h[e]})}},[h,m]);(0,D.useEffect)(()=>{if(i)return i.on(`upload.contentPath`,t=>{if(!v.current)return;if(v.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let i;try{i=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(t.uploadPath,{method:`POST`,headers:{"Content-Type":`application/json`},body:i}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})})},[i,n,e,o,r]),(0,D.useEffect)(()=>{if(i||!v.current||c.length===0)return;let t=c[c.length-1].raw,a=Nr(t);if(!a)return;if(v.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let s;try{s=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(a,{method:`POST`,headers:{"Content-Type":`application/json`},body:s}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})},[i,c,n,e,o,r]);let C=(0,D.useCallback)(()=>{if(s===`connected`){if(n.length===0){r(`Nothing to upload — paste a JSON payload first`,`error`);return}v.current=!0,o.send(e,`upload`)}},[o,e,s,n,r]),ee=(0,D.useCallback)(t=>s===`connected`&&o.send(e,t),[o,e,s]),te=(0,D.useCallback)(()=>{navigator.clipboard.writeText(c.map(e=>e.raw).join(`
`)),r(`Console copied to clipboard!`,`success`)},[c,r]),ne=(0,D.useCallback)(()=>{o.clearMessages(e),r(`Console cleared`,`info`)},[o,e,r]),w=(0,D.useCallback)(t=>{o.appendMessage(e,t)},[o,e]);return{connected:l,connecting:u,messages:c,command:p,setCommand:(0,D.useCallback)(e=>f({type:`SET_COMMAND`,value:e}),[]),connect:y,disconnect:b,sendCommand:x,handleKeyDown:S,consoleRef:_,copyMessages:te,clearMessages:ne,uploadPayload:C,sendRawText:ee,appendMessage:w,history:h}}function Yr(e){let[t,n]=(0,D.useState)(()=>window.matchMedia(e).matches);return(0,D.useEffect)(()=>{let t=window.matchMedia(e),r=e=>n(e.matches);return t.addEventListener(`change`,r),()=>t.removeEventListener(`change`,r)},[e]),t}function Xr(e){if(typeof e!=`object`||!e)return!1;let t=e;return Array.isArray(t.nodes)}function Zr(e,t,n){let r=t.includes(n)?n:t[0]??`graph`;return typeof e==`string`&&t.includes(e)?e:r}function Qr(e,t,n,r,i){let[a,o]=(0,D.useState)(null),[s,c]=hr(i,n),l=Zr(s,r,n),[u,d]=(0,D.useState)(!1),f=(0,D.useCallback)(e=>{c(t=>{let i=Zr(t,r,n);return Zr(typeof e==`function`?e(i):e,r,n)})},[c,r,n]);(0,D.useEffect)(()=>{s!==l&&c(l)},[s,l,c]);let p=(0,D.useRef)(e);(0,D.useEffect)(()=>{p.current=e},[e]);let m=(0,D.useRef)(null);(0,D.useEffect)(()=>{if(!e){o(null);return}let n=new AbortController;return o(null),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{Xr(e)&&(o(e),f(`graph`))}).catch(e=>{e.name!==`AbortError`&&t(`Graph fetch failed: ${e.message}`,`error`)}),()=>{n.abort()}},[e,t]);let h=(0,D.useCallback)(()=>{let e=p.current;if(!e)return;m.current?.abort();let n=new AbortController;m.current=n,d(!0),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{Xr(e)&&o(e),d(!1)}).catch(e=>{e.name!==`AbortError`&&(t(`Graph refresh failed: ${e.message}`,`error`),d(!1))})},[]);return(0,D.useEffect)(()=>()=>{m.current?.abort()},[]),{graphData:a,setGraphData:o,rightTab:l,setRightTab:f,isRefreshing:u,refetchGraph:h}}function $r({bus:e,pinnedGraphPath:t,setPinnedGraphPath:n,connected:r,sendRawText:i,addToast:a}){let o=(0,D.useRef)(null),s=(0,D.useRef)(!1),c=(0,D.useRef)(t),l=(0,D.useRef)(r),u=(0,D.useRef)(i);(0,D.useEffect)(()=>{c.current=t},[t]),(0,D.useEffect)(()=>{l.current=r},[r]),(0,D.useEffect)(()=>{u.current=i},[i]),(0,D.useEffect)(()=>{r||(s.current=!1,o.current!==null&&(clearTimeout(o.current),o.current=null))},[r]),(0,D.useEffect)(()=>e.on(`graph.link`,e=>{s.current&&(s.current=!1,n(e.apiPath))}),[e,n]),(0,D.useEffect)(()=>e.on(`graph.mutation`,e=>{if(l.current){if(e.mutationType===`import-graph`){o.current!==null&&(clearTimeout(o.current),o.current=null),s.current=!0,u.current(`describe graph`),a(`Graph imported — refreshing view…`,`info`);return}s.current=!0,o.current!==null&&clearTimeout(o.current),o.current=setTimeout(()=>{o.current=null,l.current&&(s.current=!0,u.current(`describe graph`),a(c.current===null?`Graph updated — opening Graph tab…`:`Graph updated — refreshing…`,`info`))},300)}}),[e,a]),(0,D.useEffect)(()=>e.on(`session.reset`,()=>{o.current!==null&&(clearTimeout(o.current),o.current=null),s.current=!1,n(null)}),[e,n]),(0,D.useEffect)(()=>()=>{o.current!==null&&clearTimeout(o.current)},[])}var ei=Object.assign({"../../../resources/help/help connect.md":`Connect two nodes
-----------------
Create a directional connection from one node to another with a descriptive
relation label.

Syntax
------
\`\`\`
connect {node-A} to {node-B} with {relation}
\`\`\`

Example
-------
\`\`\`
connect root to fetcher with fetch
\`\`\`

Notes
-----
- Connections are directional: 'connect a to b' is different from
  'connect b to a'.
- The relation is a free-form descriptive label (e.g. done, fetch, provider);
  it is not interpreted for skill routing. For data-entity nodes, meaningful
  relation names capture enterprise knowledge.
- Multiple outgoing connections from one node fork traversal into parallel
  branches, one per connection. Synchronize them with a graph.join node
  (see 'help graph-join').
- Every node must connect to at least one other node: a graph with orphan
  nodes cannot be exported for deployment (see 'help export'). Wire config
  nodes (Dictionary, Provider) and data entities under a graph.island node
  so no node is left unconnected (see 'help graph-island').
`,"../../../resources/help/help create.md":`Create a node
-------------
Add a node to the current graph model. This is a multi-line command: enter
all lines as one block.

Syntax
------
\`\`\`
create node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
\`\`\`

Example
-------
\`\`\`
create node root
with type Root
with properties
name=helloworld
purpose=Demo graph
\`\`\`

Notes
-----
- Node names use lowercase letters, digits and hyphen. The names 'root' and
  'end' are reserved: the root node must be named 'root' and the end node
  must be named 'end'.
- Types are descriptive labels, conventionally Capitalized (e.g. Root, End,
  Provider, Dictionary, Fetcher, Island). The type and properties are used
  and validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional. Property values act as
  defaults for the instance model.
- A property key may be composite, using the dot-bracket format; a
  key[]=entry line appends one entry to the list "key" (repeat per entry).
  Values may use the Event Script constant syntax, e.g. text(hello),
  int(100), boolean(true).
- Wrap a multi-line value in triple single quotes (''').
- Best practice: give the root node a "name" property (the graph name) and a
  "purpose" property describing the use case as a one-liner.
`,"../../../resources/help/help data-dictionary.md":`Data Dictionary
---------------
The data-dictionary method separates WHAT data to get from WHERE it comes
from. It uses three kinds of nodes:

1. Dictionary - defines one data attribute (or set of attributes)
   retrievable from a provider
2. Provider - defines the HTTP endpoint that supplies it
3. Fetcher - a node with skill=graph.api.fetcher that names dictionaries and
   makes the call(s) at run time (see 'help graph-api-fetcher')

Dictionary and Provider are CONFIGURATION nodes: they never execute and are
referenced by name (dictionary[]=..., provider=...). Do not leave them
floating - wire them into the knowledge layer under a graph.island node so
the graph carries its own entity-relationship diagram:
root -[contains]-> island -[data]-> dictionary -[provider]-> provider.
See 'help graph-island'.

Dictionary node
---------------
Defines one data attribute retrievable through a Provider.

\`\`\`
create node {name}
with type Dictionary
with properties
purpose={description}
provider={provider-node-name}
input[]={parameter}
input[]={parameter}:{default}
output[]=response.{path} -> result.{key}
\`\`\`

- input[] entries are BARE parameter names, not source -> target mappings
  (the one exception to the mapping rule). An optional :{default} suffix
  supplies a default value, e.g. input[]=detail:true - that is the ONLY
  meaning of ':' here.
- output[] maps the provider's raw HTTP response body (the response.*
  namespace) into the result set (result.{key}) that the fetcher exposes.
  The source path may be a leaf OR an interior node - an interior path maps
  the WHOLE subtree: response.profile.name -> result.name extracts one
  field, while response.profile -> result.profile captures the entire
  profile object and response.accounts -> result.account_numbers an entire
  array.

Example:

\`\`\`
create node person-profile
with type Dictionary
with properties
purpose=full profile record of a person
provider=mdm-profile
input[]=person_id
input[]=detail:true
output[]=response.profile.name -> result.name
output[]=response.profile.address -> result.address
\`\`\`

Provider node
-------------
Defines the HTTP call - the communication contract with the target system.

\`\`\`
create node {name}
with type Provider
with properties
purpose={description}
url={target url}
method={GET | POST | PUT | PATCH | DELETE | HEAD}
feature[]={feature flag}
input[]={source} -> {target}
\`\`\`

- The url may embed {name} path placeholders - each one is filled by an
  input[] line targeting path_parameter.{name}. Standard
  \${config.key:default} substitution also applies to the url.
- input[] sources: a constant (e.g. text(application/json)), a Dictionary
  parameter name (bare), or a state-machine value (model.*). Targets:
  header.{name}, query.{name}, path_parameter.{name}, body.{key} - or the
  whole "body" (e.g. to send a string or an array as the request body).
- feature[] entries declare capabilities the calling fetcher must support
  (e.g. an auth mechanism). Built-ins: log-request-headers and
  log-response-headers - the fetcher logs request/response headers into the
  "header" section of its properties. graph.api.fetcher prints a warning
  for a feature it does not support (a custom fetcher may enforce it).

GET example - a URL path placeholder filled from a dictionary parameter,
plus a JSON accept header:

\`\`\`
create node mdm-profile
with type Provider
with properties
purpose=MDM profile endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
\`\`\`

POST example - body.{key} targets build the JSON request body; set the
content-type header (no URL placeholder - the parameters travel in the
body):

\`\`\`
create node account-api
with type Provider
with properties
purpose=account management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/account/details
method=POST
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
\`\`\`

Putting it together
-------------------
The fetcher names the dictionary; the dictionary names the provider:

\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

The fetcher's input[] targets must match the dictionary parameter names
exactly, or execution fails. Full fetcher semantics (iterative fetching,
failure routing, deduplication): 'help graph-api-fetcher'.

Notes
-----
- Several Dictionary nodes may share one Provider - e.g. a provider returns
  a complex structure and each dictionary extracts different attributes.
  Identical calls (same provider + same input values) are deduplicated into
  a single HTTP request within a graph instance; only successful responses
  are cached.
- Dictionary and Provider nodes hold configuration only; the result set is
  stored on the FETCHER node ({fetcher}.result), not on the dictionary.
- Wire every dictionary and provider under the island knowledge layer
  (connect island to {dictionary} with data, connect {dictionary} to
  {provider} with provider) - leave no node unconnected.
`,"../../../resources/help/help delete.md":`Delete a node, a connection or the fetch cache
----------------------------------------------
Remove a node or the connections between two nodes from the current graph
model, or clear the API-fetcher response cache of the current graph instance.

Syntax
------
\`\`\`
delete node {name}
delete connection {node-A} and {node-B}
delete cache
\`\`\`

Example
-------
\`\`\`
delete node fetcher
delete connection root and fetcher
\`\`\`

Notes
-----
- Deleting a node also removes every connection touching it.
- 'delete connection' removes the connections between the two nodes in both
  directions, if any.
- 'delete cache' requires a graph instance (see 'help instantiate'). It
  clears the cache of successful API-fetcher responses, so the next
  identical call makes a real HTTP request instead of reusing a cached
  response.
- 'clear' is an alias of 'delete' (e.g. 'clear cache').
`,"../../../resources/help/help describe.md":`Describe graph, node, connection or skill
-----------------------------------------
Print the structure of the current graph model, the detail of a node or a
connection, or the documentation of a skill.

Syntax
------
\`\`\`
describe graph
describe graph {graph-id}
describe node {name}
describe connection {node-A} and {node-B}
describe skill {skill.route.name}
\`\`\`

- 'describe graph' (no id) describes the CURRENT DRAFT of this session.
- 'describe graph {graph-id}' (discovery, read-only) shows a DEPLOYED
  model's contract view: its purpose, node/connection counts, and the
  input.*/output.* data surface derived from the model's own mappings -
  everything needed to wire an extension= delegation without trial
  execution. Find the available ids with 'list graphs'.

Example
-------
\`\`\`
describe node fetcher
describe skill graph.api.fetcher
\`\`\`

Notes
-----
- 'describe graph' shows the structure of the current draft graph model.
- 'describe node' prints a node's type and properties.
- 'describe connection' reports the connections between the two nodes in
  either direction, or that they are not connected.
- 'describe skill' prints the shipped documentation of a skill by its route
  name - the same content as the hyphenated help topic (e.g.
  'help graph-api-fetcher').
`,"../../../resources/help/help edit.md":`Edit a node
-----------
A convenience command: prints an existing node as a complete 'update node'
command so you can copy it, edit the text, and submit the update.

Syntax
------
\`\`\`
edit node {name}
\`\`\`

Example
-------
\`\`\`
edit node demo-node
\`\`\`

Sample output
-------------
\`\`\`
update node demo-node
with type Demo
with properties
hello=world
test='''
this is a sample multi-line value
line two
line three
'''
good=day
\`\`\`

Notes
-----
- The printed command carries the node's current type and all properties,
  flattened to one key per line; list properties print one key[]=entry line
  per element, in order.
- Multi-line values are wrapped in triple single quotes.
- Edit the printed text and submit it as-is to apply the change (see
  'help update'). The node must exist, or the command reports an error.
`,"../../../resources/help/help execute.md":`Execute a single node
---------------------
Run one node's skill in isolation. Graph traversal is paused, so you can
functionally verify a node without walking the whole graph.

Syntax
------
\`\`\`
execute node {name}
execute {name}
\`\`\`

Example
-------
\`\`\`
execute fetcher
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- The node must have a 'skill' property with exactly one skill route, and
  that route must exist at runtime.
- The node reads from and writes to the instance's state machine exactly as
  it would during a run; use 'inspect' to check the outcome (see
  'help inspect').
- On success the console reports the execution time and the node's exit
  path; the node is marked as seen (see 'help seen').
`,"../../../resources/help/help export.md":`Export a graph model
--------------------
Write the current graph model as a JSON file for deployment or later
re-import.

Syntax
------
\`\`\`
export graph as {name}
\`\`\`

Example
-------
\`\`\`
export graph as helloworld
\`\`\`

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- The file is written to the Playground temp folder (configuration key
  location.graph.temp, default /tmp/graph).
- The export sets name={name} on the root node. If the root node's "name"
  property differs from {name} and the target file already exists, the
  export is refused - update the root node's name to overwrite the existing
  model. If no root node exists, one is created automatically.
- Export fails when the graph has orphan nodes: every node must connect to
  at least one other node (see 'help connect').
- The reply includes "Described in /api/graph/model/{name}/{token}", a
  read-only HTTP view of the exported model.
`,"../../../resources/help/help graph-api-fetcher.md":`Skill: Graph API Fetcher
------------------------
Calls an external HTTP API declaratively. The node never holds a URL itself:
it names one or more Dictionary nodes (data attributes), and each Dictionary
names the Provider node (endpoint definition) that supplies it. When
traversal reaches the node, the fetcher resolves the provider through the
dictionary, makes the call(s), and collects the result set into the node's
"result" property.

Authoring the Dictionary and Provider configuration nodes is covered in
'help data-dictionary' - read that first.

Route name
----------
"graph.api.fetcher"

Properties
----------
\`\`\`
skill=graph.api.fetcher
dictionary[]={dictionary-node-name}
input[]={source} -> {dictionary-parameter}
output[]={source} -> {target}
\`\`\`

- dictionary[] (required) - one or more Dictionary node names configured in
  the same graph model. This is the only hard-required property.
- input[] - required whenever the dictionaries declare parameters (the usual
  case). Each entry's TARGET must match a dictionary parameter name exactly,
  or execution fails.
- output[] (optional) - maps the result set onward (e.g. to output.* or
  model.*). Optional because the result set always lands at {node}.result,
  where a later data mapper can pick it up.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}   (iterative fetching - see below)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
\`\`\`

Result set
----------
On success the result set - the values the Dictionary's output[] mappings
produced as result.{key} - is stored at {node}.result. In this node's own
output[] mappings, result.{key} reads from that set; later nodes read
{node}.result.{key}.

Example
-------
\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

Iterative fetching (for_each)
-----------------------------
A fetcher can execute once per element of a runtime array - the mechanism
for "fetch details for each item in a list obtained from a previous call":

\`\`\`
create node accounts-fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=account-detail
for_each[]=profile-fetcher.result.accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.detail -> model.account_details
\`\`\`

- The for_each source MUST resolve to a list - typically a prior fetcher's
  result ({fetcher}.result.{key}) or a model.* array. Multiple for_each[]
  lines iterate multiple parameters in lock-step.
- Wire the current element into each call with an ordinary input mapping:
  input[]=model.{var} -> {dictionary-parameter}. Non-iterated inputs (like
  person_id above) pass unchanged to every call.
- concurrency bounds the parallel fan-out (1-30, default 3); calls run in
  batches of that size to avoid overwhelming the target service.
- Aggregation is GUARANTEED and ordered: each iteration's result.{key}
  values are appended into a single array on this node's result set - after
  N iterations, result.detail above is an array of N - and the aggregated
  array preserves the source list's order regardless of concurrency.

Failure routing (exception)
---------------------------
On a failed call (HTTP status >= 400):

- {node}.status and {node}.error are set (the engine's error record)
- the output[] mappings are SKIPPED
- with exception={handler-node}, traversal JUMPS to the handler; without
  it, the run ABORTS and the error is returned to the caller.

The handler is typically a graph.math decision node that inspects the
fetcher's status/error, counts attempts, and retries with a bound:

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]=RESET: fetcher, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: recovery-node
ELSE: next
'''
statement[]=NEXT: fetcher
statement[]=DELAY: 50
\`\`\`

RESET comes first among the action statements so it runs on every path (a
taken IF jump ends the list) - the attempt counters live in the "model"
namespace, which RESET never touches. If the handler also carries a defensive
check on the failed node's status, that check must come BEFORE the RESET (it
reads state the reset wipes).

Wire the handler back explicitly (connect error-handler to fetcher with
retry) - no node left unconnected. See 'help graph-math' for the statement
grammar and the engine's loop guard.

Notes
-----
- One Provider call is exactly one HTTP request - redirects are never
  followed. A 3xx answer is a non-failure: its status and body are captured
  and traversal proceeds (only >= 400 triggers failure routing). Point the
  Provider url at the redirect target to land on it.
- {node}.status always carries the HTTP status of the fetch, success
  included (a 200 or a 301 is readable there, not just failures). The
  response.* namespace in a Dictionary output[] addresses the BODY only;
  the bare root (response -> result.page) captures a whole non-JSON body
  such as an HTML page.
- Deduplication: identical requests (same provider + same input values)
  within one graph instance are deduplicated into a single HTTP call. Only
  SUCCESSFUL responses are cached - a failed call is never cached, so a
  retry after RESET makes a real call, while an identical successful call
  reuses the cached response.
- Provider feature[] flags declare capabilities this fetcher must support.
  Built-ins: log-request-headers and log-response-headers - the fetcher
  logs request/response headers into the "header" section of its
  properties. An unsupported feature produces a warning (a custom fetcher
  may enforce it).
- Keep chains minimalist: fetchers can be chained to make multiple API
  calls, but an overly complex chain means slow performance. Take only the
  minimal set of data your application requires - don't abuse the
  flexibility of the API fetcher.
- Wire the Dictionary and Provider nodes into the island knowledge layer so
  no node is left unconnected - see 'help graph-island'.
`,"../../../resources/help/help graph-data-mapper.md":`Skill: Graph Data Mapper
------------------------
Copies and transforms data between state-machine namespaces. Each mapping[]
entry moves one value from a source to a target when the node executes. This
is the workhorse skill for shaping inputs, staging intermediate values in
model.*, and assembling the response in output.body.

Route name
----------
"graph.data.mapper"

Properties
----------
\`\`\`
skill=graph.data.mapper
mapping[]={source} -> {target}
\`\`\`

- mapping[] (required) - one entry per line; entries execute in order,
  so a later entry may read an earlier entry's target (the chain idiom:
  ingest -> transform -> publish inside one mapper).

Sources: input.body / input.header, model.*, a node name (its properties),
{node}.result, a constant, an f:plugin(...) call, or a $. JSONPath
expression. Targets: output.body / output.header, model.*, or a node name.

Example
-------
\`\`\`
create node shape-response
with type Mapper
with properties
skill=graph.data.mapper
mapping[]=input.body.hr_id -> employee.id
mapping[]=fetch-one.result.profile -> output.body.profile[0]
mapping[]=fetch-two.result.profile -> output.body.profile[1]
mapping[]=f:now(text(local)) -> output.body.timestamp
\`\`\`

Constants
---------
A constant is valid wherever a source is. This is the full set:

- text(hello world) - string, verbatim (no quoting needed)
- int(100) / long(10000000000) - integer (non-numeric input yields -1; a
  decimal part is dropped)
- float(1.5) / double(1.5) - floating-point number
- boolean(true) - true only for case-insensitive "true"; anything else false
- map(k1=v1, k2=v2) - inline map literal (values are strings)
- map(config.key) - the value of an application-configuration key
- file(text:/tmp/f.txt) / file(json:...) / file(binary:...) - file content
  as text / parsed JSON / bytes
- classpath(text:/data/f.txt) - like file(), resolved against the app's
  resource roots

Beyond constants, two non-constant source forms are valid:

- f:plugin(args...) - a simple-plugin call, e.g. f:uuid(),
  f:now(text(local)), f:concat(model.a, text(!)), f:add(model.n, int(1)),
  f:ternary(...), f:defaultValue(input.body.flag, boolean(false)),
  f:removeKey(model.list, text(key)), f:listOfMap(...).
- $.  - a JSONPath expression over the state machine. Prefer plain
  dot-bracket keys; use JSONPath only when the query needs it.

Notes
-----
- Composite keys use dot-bracket form on both sides. A numeric index in a
  target creates/sets that list slot (profile[0], profile[1]) - the idiom
  for assembling a JSON list deterministically, e.g. after a fork/join. An
  empty index "[]" appends one element to the end of the list (and creates
  the list with that first element when it does not yet exist).
- An interior (non-leaf) source path maps the ENTIRE subtree, not just
  scalars - fetch-one.result.profile above carries the whole profile object.
- An unresolvable source SKIPS the entry: the target is left untouched (not
  nulled). Two idioms for defaults follow from this:
  f:defaultValue(input.body.flag, boolean(false)) -> model.flag, or
  default-then-overlay (boolean(false) -> model.flag followed by
  input.body.flag -> model.flag).
- The legacy colon-type suffix ("simple type matching") is deprecated - use
  the f:plugin forms instead.
- Inside a graph.math node, MAPPING: statements use exactly this syntax; see
  'help graph-math'.
`,"../../../resources/help/help graph-extension.md":`Skill: Graph Extension
----------------------
Delegates to another graph model (a sub-graph) or to an Event Script flow,
so larger capabilities compose from smaller ones. The node passes named
inputs to the target, and the target's response body becomes this node's
result. This is the seam between the knowledge-graph layer and the Event
Script layer beneath it.

Route name
----------
"graph.extension"

Properties
----------
\`\`\`
skill=graph.extension
extension={graph-id}           (a deployed sub-graph ...)
extension=flow://{flow-id}     (... or an Event Script flow)
input[]={source} -> {key}
output[]={source} -> {target}
\`\`\`

- extension (required) - the target. A graph id resolves among DEPLOYED
  graph models only (compiled at startup from the app's resources/graph
  folder - the same ids callable at POST /api/graph/{graph-id}). A session
  draft is NOT addressable: export and deploy it first. A missing id fails
  the node fast at run time. A flow target takes the flow:// prefix, e.g.
  extension=flow://hello-world.
- input[] (required) - each entry's TARGET is a bare key that becomes the
  target's input.body.{key}. There is NO whole-body "*" target on this
  skill - map named keys (the "*" merge idiom is graph.task-only; see
  'help graph-task').
- output[] (optional) - maps the result onward; the result always lands at
  {node}.result regardless.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}   (iterate over a runtime list)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
\`\`\`

Result set
----------
This node's result namespace IS the target's output.body:

- bare "result" in an output[] mapping is the whole response body
- result.{key} is a field of it

The same contract applies to both target kinds: the named input keys feed
the sub-graph's or flow's input.body, and result.* is its output.body.

Example
-------
\`\`\`
create node performance-evaluator
with type Extension
with properties
skill=graph.extension
extension=evaluate-sales-performance
input[]=input.body.department_id -> id
output[]=result.sales_performance -> output.body.sales_performance
\`\`\`

Here input.body.department_id feeds the sub-graph's input.body.id, and the
sub-graph's output.body.sales_performance comes back as
result.sales_performance.

Notes
-----
- Failure routing: on failure, {node}.status and {node}.error are set and
  the output[] mappings are skipped. With exception={handler-node},
  traversal jumps to the handler instead of aborting; without it, the run
  aborts. The bounded-retry pattern is shown under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the target once per
  element of a runtime list, with bounded parallel fan-out (concurrency
  1-30, default 3). The shared iteration rules are under
  'help graph-api-fetcher'.
- Use graph.extension for multi-step orchestration; use graph.task for a
  single composable-function call.
`,"../../../resources/help/help graph-island.md":`Skill: Graph Island
-------------------
Marks an isolated node. A node with this skill always returns ".sink", so
graph traversal never continues through it. That isolation is the point: the
island anchors the graph's knowledge layer. Dictionary, Provider, data-entity,
and reusable Module nodes hang off the island, turning the graph into its own
entity-relationship diagram - living documentation of the enterprise knowledge
behind the execution path.

Route name
----------
"graph.island"

Properties
----------
\`\`\`
skill=graph.island
\`\`\`

No other properties are required or accepted.

Example
-------
\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Wire the knowledge layer under it:

\`\`\`
connect root to dictionary with contains
connect dictionary to person-profile with data
connect dictionary to account-detail with data
connect person-profile to mdm-profile with provider
connect account-detail to account-api with provider
\`\`\`

Notes
-----
- Required convention: leave no node unconnected. Whenever the graph has
  off-path nodes - Dictionary/Provider configuration, data-entity, or
  reusable Module nodes - wire every one of them into the island structure:
  root -[contains]-> island -[data]-> dictionary -[provider]-> provider,
  and island -[module]-> module for reusable graph.math modules.
- Encouraged even for graphs with no off-path nodes: data-entity nodes that
  document the domain model (entities, fields, which fields are
  internal-only) make even a small graph discoverable enterprise knowledge.
- Relation labels are free-form and descriptive; "contains", "data",
  "provider" and "module" are the shipped conventions - choose names that
  capture the real-world relationship.
- Traversal is unaffected: the island sinks, so the run log shows a single
  "Executed ... with skill graph.island" line and the execution path never
  enters the knowledge layer.
- Reusable modules are documented under 'help graph-math'; the Dictionary and
  Provider configuration nodes under 'help data-dictionary'.
`,"../../../resources/help/help graph-join.md":`Skill: Graph Join
-----------------
A synchronization barrier for parallel branches. A node with this skill
returns "next" only when ALL upstream nodes connected to it have completed;
until then it returns ".sink" (the arriving path pauses). Use it to bring
forked branches back together before continuing.

Completion is success-only and current: a branch that failed into its
"exception=" route does not count while it retries, and a RESET node stops
counting until it re-executes successfully - so a retry loop feeding a join
holds the barrier instead of firing it prematurely. A chained upstream join
counts only once it actually FIRED (an evaluation that sank does not count),
so multi-stage joins compose safely.

Route name
----------
"graph.join"

Properties
----------
\`\`\`
skill=graph.join
\`\`\`

No other properties are required or accepted.

Example
-------
\`\`\`
create node join
with type Join
with properties
skill=graph.join
\`\`\`

Fork, then join:

\`\`\`
connect root to fetch-name with fetch
connect root to fetch-address with fetch
connect fetch-name to join with done
connect fetch-address to join with done
connect join to combine with proceed
\`\`\`

Notes
-----
- The fork side needs no special node: multiple outgoing connections from one
  node run their branches in parallel.
- A join is only meaningful with two or more upstream connections. Without a
  join, traversal simply proceeds as each branch completes.
- Data mapping is thread-safe (state-machine operations are serialized), but
  parallel branches must not write the SAME scalar key - the last writer
  wins, nondeterministically. Use disjoint keys (e.g. per-branch model.*
  variables), or append to a shared list with the race-free "[]" target form
  (element order then follows completion order). When the final order must
  be deterministic, assemble with numeric indices after the join, e.g.
  fetch-name.result.profile -> output.body.profile[0]. See
  'help graph-data-mapper'.
`,"../../../resources/help/help graph-js.md":`Skill: Graph JS (retired)
-------------------------
The graph.js skill is RETIRED in this Rust port for security reasons - the
engine deliberately ships no embedded JavaScript runtime.

Any node configured with skill=graph.js is rejected at execution time with:

\`\`\`
Skill graph.js is retired for security reasons - use graph.math or graph.task instead
\`\`\`

What to use instead
-------------------
- graph.math - inline computation and IF/THEN/ELSE decision-making with a
  narrow math/boolean expression dialect. See 'help graph-math'.
- graph.task - invoke a composable function by route name for any logic an
  inline expression cannot express. See 'help graph-task'.

Notes
-----
- Do not author new graph.js nodes.
- When importing an older graph model that contains graph.js nodes, replace
  skill=graph.js with graph.math (compute/branch) or graph.task (custom
  logic) before running it.
`,"../../../resources/help/help graph-math.md":`Skill: Graph Math
-----------------
Fast inline math and boolean evaluation for computation and decision-making.
A node with this skill runs an ordered list of statement[] lines. This is THE
skill for inline compute/branch in this Rust port (graph.js is retired - see
'help graph-js'). For anything richer than the narrow expression dialect
described below, invoke a composable function instead (see 'help graph-task').

Route name
----------
"graph.math"

Properties
----------
\`\`\`
skill=graph.math
statement[]=COMPUTE: {var} -> {expression}
statement[]=IF: / THEN: / ELSE:              (multi-line - see below)
statement[]=MAPPING: {source} -> {target}
statement[]=EXECUTE: {node-name}
statement[]=RESET: {node-name}[, {node-name} ...]
\`\`\`

- statement[] (required) - at least one statement; statements run in order.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}     (iterate a statement block)
statement[]=BEGIN / statement[]=END          (delimit the for_each block)
statement[]=NEXT: {node-name}
statement[]=DELAY: {milliseconds}
\`\`\`

Statements
----------
- COMPUTE: {var} -> {expression} - evaluate the expression; the result is
  stored in THIS node's result namespace, readable as
  {this-node}.result.{var} or moved onward with a MAPPING statement.
- IF - a boolean decision that can redirect traversal (see below).
- MAPPING: {source} -> {target} - data mapping, identical to the data mapper
  (see 'help graph-data-mapper'). Do NOT wrap source/target in curly braces.
  A node with ONLY MAPPING statements is rejected - use graph.data.mapper.
- EXECUTE: {node-name} - run another graph.math node's statements inline, IN
  THE CALLING NODE'S CONTEXT: any COMPUTE results land on the INVOKING node
  ({invoker}.result.{var}); the executed module's own namespace stays empty.
  This is the module-reuse mechanism - author a formula once in an off-path
  Module node reading neutral model.* operands, and any node borrows it.
- RESET: {node-name}[, ...] - forget one or MORE nodes completely (the
  run-once guard, the completion mark, and the node state; comma/space
  list). A reset node stops satisfying a graph.join barrier until it
  re-executes successfully. Resetting a never-executed node is a safe
  no-op. Advanced - see Notes.

Expressions
-----------
{namespace.key} substitutes a value from input.*, model.*, or a node's
properties/result into a COMPUTE or IF expression, e.g.
{input.body.discount}, {book.price}, {model.x}. Substitution is robust to
hyphenated names - {unit-price} is the value of "unit-price", never parsed
as a subtraction - so use communicative hyphenated names freely.

The dialect is a NARROW JavaScript-like subset: arithmetic, comparison and
boolean operators only. No bitwise operators, no function calls (e.g. no
parseInt), no variables inside the expression. COMPUTE yields a double, so
an integer result serializes as e.g. 8.0 (numerically exact).

IF / THEN / ELSE
----------------
IF is the decision construct. It is a multi-line statement - enter it as one
statement[] value wrapped in triple single quotes. THEN: and ELSE: are both
REQUIRED, or the engine aborts the run.

\`\`\`
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: ge-path
ELSE: lt-path
'''
\`\`\`

- THEN: / ELSE: each name the node to jump to, or the keyword "next".
- A taken node-jump ENDS the statement list immediately - later statements
  do not run. A branch resolving to "next" FALLS THROUGH: processing
  continues with the following statements, and natural traversal is
  preserved if nothing else redirects it. Order the list accordingly (e.g.
  an early-exit check first, retry logic after).

Traversal control
-----------------
- NEXT: {node-name} - unconditionally jump to a node BY NAME (a node name,
  not a connection label). Unlike a taken IF jump, NEXT: does not stop
  processing: the remaining statements still run, and the jump applies after
  the whole list completes (the last NEXT: wins).
- DELAY: {milliseconds} - pause after this node completes, before the walk
  continues to the next node. Paces retries; simulates a slow service.
- RESET enables retry loops. A node may reset ITSELF - the run-once mark is
  set before execution, so a self-reset survives and the node can run again.
  Placement rule: put RESET FIRST among the action statements - it then runs
  on every path (a later taken IF jump would skip it) and everything the node
  stores afterwards (such as DELAY's pending pause) survives the self-wipe.
  The one exception: keep RESET after any statement that reads state it would
  wipe - an IF on a just-wiped variable (e.g. {fetcher.status} after
  RESET: fetcher) aborts the run, so a defensive status check goes before it.

Iterating lists (for_each)
--------------------------
for_each[] turns part of the statement list into a loop. Each entry has the
mapping form {source} -> model.{var}; the right-hand side MUST be a model.*
key.

- A LIST-valued source becomes an iteration array: model.{var} is rebound to
  element i on each pass. Multiple list entries advance in LOCKSTEP (parallel
  arrays) and must all have the same length. At least one entry must resolve
  to a list, or the node aborts.
- A SCALAR source binds its model.{var} once, before the loop - even when
  the lists are empty.
- An UNRESOLVABLE source REMOVES the model.{var} key.

BEGIN and END split the statements into three blocks:

\`\`\`
statement[]=...       <- pre-block: runs ONCE, before the loop
statement[]=BEGIN
statement[]=...       <- each-block: runs once PER ELEMENT
statement[]=END
statement[]=...       <- post-block: runs ONCE, after the loop
\`\`\`

- Without BEGIN, the WHOLE statement list is the loop body - seed
  accumulators in a pre-block, or the seeding re-runs on every iteration.
- Iteration is strictly SEQUENTIAL, in list order, inside one node execution
  (a long list does not trip the loop guard). Contrast: the API fetcher's
  for_each fans HTTP calls out concurrently - see 'help graph-api-fetcher'.
- A taken IF jump BREAKS the loop: it ends the current iteration, skips the
  remaining elements and the post-block, and redirects traversal. An
  "ELSE: next" falls through within the iteration.
- Empty lists are fine: the each-block runs zero times; pre/post still run.
- COMPUTE yields doubles; the f:add family uses numeric promotion - inputs
  that are all whole numbers keep exact long arithmetic (including integer
  division), while any decimal argument promotes the whole computation to a
  double. So f:add composes directly with COMPUTE results; accumulate with
  either f:add or a pure-COMPUTE read-back, as below. Tame floating-point
  precision artifacts with f:round(value, int(2)) - half-up rounding on the
  number's decimal representation (1.005 rounds to 1.01 at 2 places).

\`\`\`
create node totaler
with type Loop
with properties
skill=graph.math
for_each[]=input.body.prices -> model.price
for_each[]=input.body.quantities -> model.qty
statement[]=MAPPING: int(0) -> model.total
statement[]=BEGIN
statement[]=COMPUTE: total -> {model.total} + {model.price} * {model.qty}
statement[]=MAPPING: totaler.result.total -> model.total
statement[]=END
statement[]=MAPPING: model.total -> output.body.total
\`\`\`

With prices=[10,20,30] and quantities=[7,8,9] the run yields total: 500.0 -
the pre-block seeds the accumulator once, each pass computes
total + price*qty and writes it back, and the post-block maps the final
value out. The plugin form is equivalent:
COMPUTE: line -> {model.price} * {model.qty} followed by
MAPPING: f:add(model.total, totaler.result.line) -> model.total.

Example
-------
\`\`\`
create node price-check
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
\`\`\`

Reusable module - author the formula once, borrow it anywhere:

\`\`\`
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
\`\`\`

\`\`\`
create node calculate
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: calculate.result.sum -> output.body.sum
\`\`\`

Note "calculate.result.sum", not "addition.result.sum" - the caller borrows
the logic, so the result belongs to the caller. Keep the module off the
execution path and hang it under the island knowledge layer
(island -[module]-> addition) - see 'help graph-island'.

Notes
-----
- A node executes ONCE per run (the run-once guard); a RESET statement is
  the only escape, for advanced re-execution. Use it with care.
- Loop guard: a node executed too frequently (default: more than 10 times
  per second) aborts the traversal - bound every retry loop and pace it
  with DELAY:.
- for_each[]={array-source} -> model.{var} iterates a statement block over a
  runtime array; BEGIN / END delimit the block to iterate (they are for_each
  delimiters, not IF braces) - see "Iterating lists" above. Without
  for_each[], BEGIN/END lines are accepted and ignored.
- The bounded-retry pattern (RESET the failing node and itself first, count
  attempts with f:defaultValue + f:add, exit at the bound via a taken IF
  jump, NEXT: back, DELAY: to pace) is shown under 'help graph-api-fetcher'.
`,"../../../resources/help/help graph-task.md":`Skill: Graph Task
-----------------
Invokes a composable function through its route name and collects the
function's response into the node's "result" property. This is the
lightweight way to plug a small piece of custom business logic into a graph:
your own function becomes, in effect, a custom skill. For multi-step
orchestration, prefer a sub-graph or an Event Script flow via graph.extension
(see 'help graph-extension').

Route name
----------
"graph.task"

Properties
----------
\`\`\`
skill=graph.task
task={function-route}
input[]={source} -> {target}
output[]={source} -> {target}
\`\`\`

- task (required) - the route name of a composable function registered in
  this application.
- input[] (optional) - maps values into the function's request; entries
  apply IN ORDER (see below).
- output[] (optional) - maps the function's result onward; the result always
  lands at {node}.result regardless.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}   (iterate over a runtime list)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
\`\`\`

Input data mapping
------------------
input[] entries apply in order. The target addresses the function's request:

- "*" - the mapped value is MERGED into the whole request body. Because
  entries apply in order, later entries can merge additional fields into a
  body seeded with "*".
- header.{name} - sets a request header of the function call.
- any other key - sets that field in the request body (composite dot-bracket
  keys supported).

Sources are the usual mapping sources: input.*, model.*, another node,
constants such as text(hello), or f:plugin(...) calls. If the function
declares a typed (PoJo) input, the request body converts automatically at
the function boundary.

Output data mapping
-------------------
The function's response body is stored at {node}.result, the response status
at {node}.status, and response headers at {node}.header. In output[]
mappings, bare "result" is the WHOLE function result; result.{key} is a
field of it (same rule as graph.extension):

\`\`\`
output[]=result -> output.body
output[]=result.total -> model.total
\`\`\`

Example
-------
\`\`\`
create node hello-task
with type Task
with properties
skill=graph.task
task=v1.hello.task
input[]=input.body -> *
input[]=text(minigraph) -> header.x-app
output[]=result -> output.body
\`\`\`

Notes
-----
- The task route must exist at runtime, or the node fails fast.
- A call is bounded by the graph instance's time-to-live (model.ttl,
  default 30000 ms).
- Failure routing: on a function error (or timeout), {node}.status and
  {node}.error are set and the output[] mappings are skipped. With
  exception={handler-node}, traversal jumps to the handler instead of
  aborting; without it, the run aborts. The bounded-retry pattern is shown
  under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the function once per
  element of a runtime list (the source must resolve to a list; wire the
  element in with an ordinary input[] mapping from model.{var}), with
  bounded parallel fan-out (concurrency 1-30, default 3). The shared
  iteration rules are under 'help graph-api-fetcher'.
- Writing the composable function itself is a development task done in Rust
  with the #[preload] attribute; from the Playground you only reference its
  route name.
`,"../../../resources/help/help import.md":`Import a graph model or a node
------------------------------
Load an exported graph model into your session as a draft for review and
update, or copy a single node from another graph model.

Syntax
------
\`\`\`
import graph from {name}
import node {node-name} from {graph-name}
\`\`\`

Example
-------
\`\`\`
import graph from helloworld
import node fetcher from helloworld
\`\`\`

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- 'import graph' looks in the Playground temp folder first (where
  'export graph' writes). When the file is not there, it falls back to the
  graph models deployed with the application. The message "Graph model not
  found in /tmp/graph/... Found deployed graph model" is this normal
  fallback, not an error - the deployed model is imported as your draft.
- 'import node' copies one node (its type and properties, not its
  connections) from an exported graph model in the temp folder - export the
  source graph first. If a node with the same name already exists in your
  draft, it is overwritten.
- Best practice: publish a common graph model holding reusable nodes
  (modules and skills) so team members can import them into their own
  graph models.
`,"../../../resources/help/help inspect.md":`Inspect the state machine
-------------------------
Read a value from the current graph instance's state machine: node
properties, and the input, output and model namespaces.

Syntax
------
\`\`\`
inspect {key}
\`\`\`

\`{key}\` is a placeholder - substitute your key and do not type the braces.
A whole namespace (input | output | model) is also valid, e.g.
'inspect output'.

Example
-------
\`\`\`
inspect output
inspect input.body.user_id
inspect model.some_variable
inspect output.body.some_key
inspect book.price
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Keys may be composite (dot-bracket), e.g. output.body.profile[0].name.
- A node's properties and results are addressed by node name, e.g.
  book.price or fetcher.result.name.
- A value too large for the console is redirected: the reply prints a
  GET /api/inspect/... URL to download it instead.
`,"../../../resources/help/help instantiate.md":`Instantiate a graph instance
----------------------------
Create a runnable instance of the current graph model, optionally seeded
with mock input for development and testing. Required before the 'run',
'execute' and 'inspect' commands. This is a multi-line command: enter all
lines as one block.

Syntax
------
\`\`\`
instantiate graph
{constant} -> input.body.{key}
{constant} -> input.header.{key}
{constant} -> model.{key}
\`\`\`

Example
-------
\`\`\`
instantiate graph
int(100) -> input.body.profile_id
text(application/json) -> input.header.content-type
text(world) -> model.hello
\`\`\`

Notes
-----
- The seed lines are optional. Each line assigns a constant (text(...),
  int(...), boolean(...), etc.) to the input.body, input.header or model
  namespace - no other targets are accepted. The model namespace is the
  state machine; seed it only to emulate model variables.
- Seed keys may be composite (dot-bracket), so nested mock payloads seed
  directly:

\`\`\`
instantiate graph
text(Peter) -> input.body.profile.name
text(100 World Blvd) -> input.body.profile.address1
\`\`\`

- The graph must have a root node and an end node.
- Instantiating replaces any previous instance of your session.
- The reply reports the number of mock entries loaded and the instance's
  model.ttl (default 30000 ms), the execution time budget - seed model.ttl
  to change it.
- 'start' is an alias of 'instantiate'.
- To mock a large input.body with a JSON payload, see 'help upload'.
`,"../../../resources/help/help list.md":`List nodes, connections, graphs or flows
----------------------------------------
Show all nodes or all connections of the current graph model - or discover
the deployable graph models and Event Script flows of this server.

Syntax
------
\`\`\`
list nodes
list connections
list graphs
list flows
\`\`\`

Notes
-----
- 'list graphs' (discovery, read-only) enumerates the deployable graph
  models - the valid extension={graph-id} delegation targets - each with
  its root node's "purpose" property, so the listing reads as living
  documentation of the enterprise knowledge on this server. Follow up
  with 'describe graph {graph-id}' for a model's input/output contract.
- 'list flows' (discovery, read-only) enumerates the Event Script flows -
  the valid extension=flow://{flow-id} delegation targets.
- 'list nodes' prints each node with its type: the root node first, the end
  node last, and the other nodes in alphabetical order. A missing root or
  end node is flagged with "(does not exist)".
- 'list connections' prints one line per connection with its relation
  label(s).
- Use 'describe node {name}' for the full detail of a single node (see
  'help describe').
`,"../../../resources/help/help run.md":`Run a graph instance
--------------------
Traverse the current graph instance from the root node to the end node,
executing every node that has a skill along the way.

Syntax
------
\`\`\`
run
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Traversal starts at the root node. Multiple outgoing connections fork into
  parallel branches (synchronize them with graph.join); each node executes
  at most once per run (loop guard).
- Every run ends with either "Graph traversal completed in N ms" or
  "Graph traversal aborted"; on failure, the reason is printed before the
  aborted line.
- 'run' may be repeated on the same instance: each run clears the visited
  set and the output namespace, but model values persist across runs -
  instantiate again for a completely fresh state.
- Use 'seen' to list the nodes visited by the last run, and 'inspect' to
  read the results (e.g. 'inspect output.body').
`,"../../../resources/help/help seen.md":`Display nodes that have been 'seen'
-----------------------------------
List the nodes of the current graph instance that have been seen - visited
by graph traversal or executed directly.

Syntax
------
\`\`\`
seen
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Covers nodes visited by 'run' and nodes tested with 'execute'.
- The visited set is cleared at the start of each run.
`,"../../../resources/help/help session.md":`Session commands
----------------
Manage your Playground session and collaborate with other users by
subscribing to a primary session, so both users see and drive the same graph.

Syntax
------
\`\`\`
session                    show this session's id and subscriptions
session subscribe {id}     mirror a primary session into yours
session unsubscribe        detach from the session you subscribed to
session reset              restart your session
\`\`\`

Example
-------
\`\`\`
> session
Session ws-178443-2 started since 2026-06-02 10:20:32.054
subscribed by [ws-485844-4]
\`\`\`

Notes
-----
- 'session' shows the session id and start time, the session you subscribed
  to (if any), and the sessions subscribed to yours.
- Subscribing mirrors commands both ways: input commands from either user
  run in both sessions, keeping the graphs in sync. On subscribe the graphs
  are aligned - if the primary session is empty, your draft is pushed to it;
  otherwise its graph replaces your draft.
- You can subscribe only to a primary session (one that has not itself
  subscribed to another), and never to yourself. If you are already
  subscribed, do 'session reset' before subscribing to another session.
- 'session unsubscribe' decouples your session from the one you subscribed
  to; your graph is retained so you can continue editing. A primary session
  gets "Nothing to unsubscribe".
- 'session reset' restarts your session. As a primary session it disconnects
  all subscribers (they keep their own graphs); as a subscriber it
  unsubscribes first. It resets subscriptions but does NOT clear your draft
  graph - the UI restores the draft when it reconnects. To start clean,
  delete the nodes explicitly (see 'help delete').
- The companion REST endpoints reject 'session subscribe', 'session
  unsubscribe' and 'session reset': a companion is an assistant to a
  session, not a session of its own. Only the read-only 'session' status
  query works there - session topology is managed from a
  WebSocket-connected session (the browser console) only.
`,"../../../resources/help/help tutorial 1.md":`Tutorial 1
----------
Welcome to the MiniGraph Playground, the self-service user interface for creating
applications with the Active Knowledge Graph.

In this tutorial, you will create the simplest possible application: a graph model
that returns a "hello world" message.

Exercise
--------
If you can see this page, you have successfully started the MiniGraph Playground in a
browser and connected to a designer workbench session.

If your session is disconnected, select the "Tools" dropdown in the top-right corner,
click MiniGraph's start toggle and select "MiniGraph".

Create the starting point of a graph
------------------------------------
**Create a root node** — the starting point of every graph model.
Select multiline and enter the following command in the bottom-right input box.

\`\`\`
create node root
with type Root
with properties
purpose=Tutorial one to return a 'hello world' message
\`\`\`

The console displays:

\`\`\`
> create node root...
Graph with 1 node described in /api/graph/model/ws-875677-2/165-1
\`\`\`

A drawing appears on the right-hand side under the "Graph" tab: a graph with a single
node called "root" has been created.

\`ws-875677-2\` is the session ID of the workbench.
\`165-1\` is a random number for the session that you can ignore.

Create an end node
------------------
An end node is the exit point of a graph model. Enter the following to create one.

\`\`\`
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=text(hello world) -> output.body
\`\`\`

The console displays:

\`\`\`
> create node end...
Graph with 2 nodes described in /api/graph/model/ws-875677-2/061-2
\`\`\`

The \`skill=graph.data.mapper\` line assigns the data mapper skill to the end node.
A data mapper node performs data mapping when it executes.

The mapping statement \`mapping[]=text(hello world) -> output.body\` maps the constant
"hello world" to \`output.body\` — the response payload when the graph is executed.
The \`[]\` suffix means \`mapping\` is a list: each \`mapping[]=\` line appends one statement.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper" in the console.

First attempt to run the graph
------------------------------
To run a graph model, first create an instance of it with the \`instantiate graph\` command.

The console displays:

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
\`\`\`

Now try to run the graph by entering the \`run\` command.

The console displays:

\`\`\`
> run
Walk to root
\`\`\`

The system starts graph traversal from the starting point, i.e. the root node —
and then nothing happens.

What is missing?
----------------
An Active Knowledge Graph is a "property graph" that contains one or more "active"
nodes. An active node carries a "skill" that is backed by a composable function.

The system traverses the graph from the root node. Nothing happened because there is
no further node to reach after the root node: the two nodes are not yet connected,
so traversal stops before it can reach the end node.

Connecting nodes
----------------
Enter the following command to connect the root node to the end node.

\`\`\`
connect root to end with done
\`\`\`

The console displays:

\`\`\`
> connect root to end with done
node root connected to end
Graph with 2 nodes described in /api/graph/model/ws-875677-2/551-3
\`\`\`

The graph drawing on the right panel is updated.

Running the graph
-----------------
You now have a graph with a starting point and an ending point, where one node
carries a skill — the end node with its data mapping statement.

Instantiate the graph again and run it by entering the following commands.

\`\`\`
instantiate graph
run
\`\`\`

The console displays:

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 1.736 ms
{
  "output": {
    "body": "hello world"
  }
}
Graph traversal completed in 9 ms
\`\`\`

Congratulations — you have created your first working MiniGraph.
It returns "hello world" when it runs.

Export the graph
----------------
You may now export the graph so that you can deploy it later.

Enter the export command below:

\`\`\`
export graph as tutorial-1
\`\`\`

This exports the graph model in JSON format with the name \`tutorial-1\`
as "/tmp/graph/tutorial-1.json".

The console displays:

\`\`\`
> export graph as tutorial-1
Added name=tutorial-1 to Root node
Graph exported to /tmp/graph/tutorial-1.json
Described in /api/graph/model/tutorial-1/436-4
\`\`\`

Note that the system adds the graph name (its unique "id") to the root node.
This prevents you from accidentally overwriting a different graph model.

Help pages
----------
To learn more about each command used in this tutorial, enter:

\`\`\`
help create
help connect
help instantiate
help run
help export
\`\`\`

Summary
-------
In this tutorial, you created the simplest graph model — it returns a "hello world"
message when its graph API endpoint is called — exported it, and tried some help pages.

Well done. Let's move on to "Tutorial 2".
`,"../../../resources/help/help tutorial 10.md":`Tutorial 10
-----------
In this tutorial, you will create a graph model that uses another graph model as an extension.

Exercise
--------
You will use an existing graph model as an extension, then create a new graph model that calls it.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a graph extension?
--------------------------
A graph extension is a graph model built to serve some logic that another graph model can reuse.

The \`extension\` property of a graph.extension node names a **deployed** graph model — one compiled
at application startup from the \`resources/graph\` folder (the same ids callable at
POST /api/graph/{graph-id}). A session draft is not addressable as an extension: export and deploy
it first.

Import tutorial 3 as an extension
---------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
\`resources/graph\` folder.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Once the graph model is imported, start the graph with mock data.

\`\`\`
start graph
int(100) -> input.body.person_id
\`\`\`

Then do a 'dry-run'.

\`\`\`
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.982 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

You can see that it fetches data using the input parameter (person_id=100) and returns the name
and address of the person. This is the behavior your new graph will reuse.

Restart playground session
--------------------------
You will clear the current graph session — click the Tools button in the top-right corner and
click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
You will create a new graph model with a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-10
purpose=Demonstrate the use of graph extension
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create a node to use an extension
---------------------------------
Enter the following to create an extension node. The skill is 'graph.extension' and the
'extension' property names the deployed graph model 'tutorial-3'.

The input mapping sets the input parameter(s) of the extension, which is itself a graph model.
The output mapping sets the result from the extension to the output payload.

\`\`\`
create node extension
with type Extension
with properties
skill=graph.extension
extension=tutorial-3
input[]=input.body.person_id -> person_id
output[]=result -> output.body
\`\`\`

Connect the nodes to complete the graph model
---------------------------------------------

\`\`\`
connect root to extension with run
connect extension to end with finish
\`\`\`

Test the graph model
--------------------
Enter the following to instantiate the graph model with mock input.

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

Then do a 'dry-run'.

\`\`\`
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 19.013 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 20 ms
\`\`\`

The input of the current graph instance is mapped as an input parameter to the extension
'tutorial-3', and the result is mapped as the output of the graph.

If you inspect the extension node, you will see:

\`\`\`
> inspect extension
{
  "inspect": "extension",
  "outcome": {
    "result": {
      "address": "100 World Blvd",
      "name": "Peter"
    },
    "live": true,
    "target": "tutorial-3",
    "status": 200
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
\`\`\`

Check the application log
-------------------------
Complete telemetry information is shown in the application log. You will see that 'tutorial-3' is
invoked as an extension and that it fetches data from the data provider with the input parameter
'person_id'.

\`\`\`
Call extension tutorial-3, ttl=30000
GET http://127.0.0.1:8100/api/mdm/profile/100, with [person_id], ttl=30000
\`\`\`

This is a small example, but it demonstrates the pattern: a typical main graph model uses one or
more extensions for API data fetching, then performs decision-making using the retrieved data.

Reusability
-----------
Graph extension promotes reusability. Common use cases can be built as graph models and made
available as "extensions" for other graph models to use.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-10
Graph exported to /tmp/graph/tutorial-10.json
Described in /api/graph/model/tutorial-10/286-8
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-10.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-10 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "person_id": 100
}'
\`\`\`

Summary
-------
In this tutorial, you have created a graph model that uses a graph extension: the 'extension'
property names a deployed graph model, input mappings feed its input.body, and the extension's
output.body comes back as the node's result.
`,"../../../resources/help/help tutorial 11.md":`Tutorial 11
-----------
In this tutorial, you will create a graph model that uses an "event flow" as an extension.

Pre-requisite
-------------
You would need some working knowledge of Event Script. For more details, see
docs/guides/event-script/ in this repository.

Assuming you already know how to create an event flow (configuration plus composable functions as
tasks), it is easy to use an event flow as an extension.

What is a flow extension?
-------------------------
A flow extension is an event flow built to serve some logic that a graph model can reuse. The same
graph.extension skill from tutorial 10 is used — only the target changes: the "flow://" protocol
prefix tells the system to execute an event flow instead of another graph model.

Import the graph model from tutorial 10
---------------------------------------
In tutorial 10, you created a main graph that calls another graph as an extension and exported it
as tutorial-10. Import it back as your starting point:

\`\`\`
import graph from tutorial-10
\`\`\`

The import loads the version you exported in tutorial 10 from the temporary graph folder — or
falls back to the preloaded copy in classpath:/graph if you have not exported one.

Edit the root node
------------------
Enter 'edit node root' and copy-n-paste the content into the input box. Change the name and
purpose for tutorial 11.

\`\`\`
update node root
with type Root
with properties
name=tutorial-11
purpose=Demonstrate the use of flow extension
\`\`\`

Edit the extension node
-----------------------
Enter 'edit node extension' and copy-n-paste the content into the input box. Update the extension
to "flow://flow-11" and change the input statements to pass "hello" and "message" as parameters.
The flow protocol prefix tells the system to execute the flow with the identifier "flow-11".

\`\`\`
update node extension
with type Extension
with properties
extension=flow://flow-11
input[]=input.body.hello -> hello
input[]=input.body.message -> message
output[]=result -> output.body
skill=graph.extension
\`\`\`

About flow 11
-------------
For your convenience, "flow-11" is preloaded. You can review the configuration files "flows.yaml"
and "flow-11.yml" in the resources folder. The event flow "flow-11" is an echo program: the task
"no.op" echoes everything from the input and passes it as output. Below is an extract of the event
flow's first task.

\`\`\`yaml
tasks:
  - input:
      # pass all input parameters as arguments
      - 'input.body -> *'
    process: 'no.op'
    output:
      - 'result -> output.body'
    description: 'echo everything in the input payload'
    execution: end
\`\`\`

Perform a dry-run
-----------------
To test the updated graph model, instantiate the graph with the two inputs "hello" and "message"
as follows:

\`\`\`
instantiate graph
text(world) -> input.body.hello
text(this is a good day) -> input.body.message
\`\`\`

Then enter 'run' to execute the graph.

\`\`\`
> start graph...
Graph instance created. Loaded 2 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 5.46 ms
Walk to end
{
  "output": {
    "body": {
      "hello": "world",
      "message": "this is a good day"
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

You can also check the application log, where telemetry and tracing information are shown.

\`\`\`
Call extension flow://flow-11, ttl=30000
{trace={path=/graph/playground, service=graph.extension...
{trace={path=/graph/playground, service=no.op...
{trace={path=/graph/playground, service=task.executor...
{trace={path=/graph/playground, service=event.script.manager...
\`\`\`

This validates that the event flow instance for "flow-11" was executed by the graph instance for
tutorial-11.

Why extend a graph model with an event flow?
--------------------------------------------
While the graph extension discussed in tutorial 10 can compose sophisticated and powerful graph
models, extending a graph with an event flow lets you go beyond API fetching, data mapping,
computation and decision-making.

With an event flow, you can model very complex transaction processing in "pro-code". Combining
graph modeling with Event Script programming gives you the best of both worlds — no-code and
pro-code — to tackle the most demanding use cases.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-11
Graph exported to /tmp/graph/tutorial-11.json
Described in /api/graph/model/tutorial-11/794-6
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-11.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-11 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "hello": "world",
    "message": "this is a good day"
}'
\`\`\`

Summary
-------
In this tutorial, you have used an event flow as an extension to a graph model, selected with the
flow protocol prefix "flow://". The delegation contract is the same as for a sub-graph: the input
mappings feed the flow's input.body, and the flow's output.body comes back as the node's result.
`,"../../../resources/help/help tutorial 12.md":`Tutorial 12
-----------
In this tutorial, you will create a graph model with custom error handling.

Exercise
--------
You will import tutorial 3 and add an error-handler node that retries an API failure.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

Import tutorial 3 as a template
-------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
\`resources/graph\` folder.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Update the root node
--------------------
Enter the following to update the root node. It assigns the skill "graph.data.mapper" to the node
and maps the input parameter "exception" to the model variable with the same name.

The \`f:defaultValue()\` plugin function sets the variable "model.exception" to false when the input
parameter is not given.

We will use the model.exception parameter to trigger a simulated exception in the mdm-profile
service.

\`\`\`
update node root
with type Root
with properties
mapping[]=f:defaultValue(input.body.exception, boolean(false)) -> model.exception
name=tutorial-12
purpose=Demonstrate custom error handling
skill=graph.data.mapper
\`\`\`

Update the dictionary
---------------------
For person-address, you will add the input parameter \`exception:false\`, where ":false" is the
default value of the parameter when it is not given.

\`\`\`
update node person-address
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.address -> result.address
provider=mdm-profile
purpose=address of a person
\`\`\`

and do the same for person-name

\`\`\`
update node person-name
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.name -> result.name
provider=mdm-profile
purpose=name of a person
\`\`\`

Update the data provider
------------------------
You will add the input data mapping \`exception -> header.x-exception\` to the mdm-profile node. The
input parameter "exception" is used to set the HTTP request header "X-Exception".

\`\`\`
update node mdm-profile
with type Provider
with properties
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=exception -> header.x-exception
input[]=person_id -> path_parameter.id
method=GET
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
\`\`\`

Update the fetcher node
-----------------------
You will add the input data mapping \`model.exception -> exception\` to set the parameter
"exception" when retrieving the two data dictionary items (person-name and person-address).

You also add the property \`exception=error-handler\`. This tells the system to route the flow to
the "error-handler" node when a call fails, instead of aborting the graph traversal.

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
exception=error-handler
input[]=input.body.person_id -> person_id
input[]=model.exception -> exception
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

The dev-mode mock endpoint contains this:

\`\`\`rust
// extract of the dev mock endpoint (mock.mdm.profile)
if request["headers"]["x-exception"] == "true" {
    return Err(AppError::new(401, "simulated exception"));
}
// business logic not shown
\`\`\`

Create the error-handler node
-----------------------------
You will now create the error-handler node referenced by the fetcher above.

When the "exception" property is configured on a fetcher, a failed call — an error status is
always a value of 400 or higher — does not abort the graph traversal: the engine sets the node's
"status" and "error" variables, skips its output mappings, and routes the flow to the named error
handler.

The handler's statements run in order:

1. The first IF tests "fetcher.status". It is good practice to test for exactly 200 so an
   unintended configuration error cannot slip through. On HTTP-200 the THEN branch jumps to the
   end node — a taken node-jump ends the statement list immediately. Otherwise the ELSE branch
   resolves to "next" and falls through to the following statements.
2. RESET comes **first among the action statements**: it clears the run-once guard and state of a
   comma-separated list of nodes so they can be executed again — here the fetcher and the
   error-handler itself (a node may reset itself because the run-once mark is set before its
   statements execute). Placing RESET early guarantees it runs on every path — a later taken IF
   jump would skip it — and everything the node stores afterwards (such as the pending DELAY)
   survives the self-wipe. Keep RESET **after** any check that reads state it would wipe: the
   status IF above must run first, because RESET clears "fetcher.status" and an IF on a wiped
   variable aborts the run.
3. The two MAPPING statements increment the retry counter "model.attempts" (\`f:defaultValue()\`
   seeds it to 0 on the first pass). The "model" namespace is not touched by RESET.
4. The second IF bounds the retry loop: after 3 attempts it jumps to the "clear-exception" node.
5. "NEXT: fetcher" tells the traversal system to jump to the fetcher node. Unlike a taken IF jump,
   NEXT does not stop the statement list — the jump is applied after the whole list completes.
6. "DELAY: 50" pauses for 50 milliseconds after this node completes, before the next retry. Pacing
   retries is a best practice: it avoids very rapid retries that can cause a "recovery storm" — an
   unintended denial-of-service attack on the target service.

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]='''
IF: {fetcher.status} == 200
THEN: end
ELSE: next
'''
statement[]=RESET: fetcher, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: clear-exception
ELSE: next
'''
statement[]=NEXT: fetcher
statement[]=DELAY: 50
\`\`\`

Create the clear-exception node
-------------------------------
In the clear-exception node, the RESET comes first (nothing before it reads node state), clearing
the fetcher and the clear-exception node itself so that the system can execute them again. You
then set the variable "model.exception" to false so that the mock service returns a normal
response instead of an exception, and clear "model.attempts" to zero.

\`\`\`
create node clear-exception
with type Decision
with properties
skill=graph.math
statement[]=RESET: fetcher, clear-exception
statement[]=MAPPING: boolean(false) -> model.exception
statement[]=MAPPING: int(0) -> model.attempts
\`\`\`

Connections for error-handler and clear-exception nodes
-------------------------------------------------------
Create the connections to complete the retry loop.

\`\`\`
connect error-handler to fetcher with retry
connect clear-exception to fetcher with reset
\`\`\`

Do a dry-run
------------
Enter the following to start the graph with mock input data. You are setting the integer 100 to
person_id and the boolean value "true" to exception in the input payload.

\`\`\`
start graph
int(100) -> input.body.person_id
boolean(true) -> input.body.exception
\`\`\`

Execute the run command.

\`\`\`
> run
Walk to root
Executed root with skill graph.data.mapper in 0.231 ms
Walk to fetcher
Walk to dictionary
Executed dictionary with skill graph.island in 0.014 ms
Executed fetcher with skill graph.api.fetcher in 21.83 ms
Walk to error-handler
Executed error-handler with skill graph.math in 52.242 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.025 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.824 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.264 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.837 ms
Walk to clear-exception
Executed clear-exception with skill graph.math in 0.132 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.547 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 201 ms
\`\`\`

The graph traversal log shows that the "error-handler" node executed 3 times before the
clear-exception node ran. After the exception is cleared, the mock service returns a correct
result set as "output".

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-12
Graph exported to /tmp/graph/tutorial-12.json
Described in /api/graph/model/tutorial-12/591-5
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-12.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-12 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "person_id": 100,
    "exception": true
}'
\`\`\`

Summary
-------
In this tutorial, you have used tutorial-3 as a template and enhanced it with custom error
handling.

You have used the keywords "RESET", "NEXT" and "DELAY" to clear the state of visited nodes, to
tell the graph traversal system to route to a specific node, and to introduce an artificial delay
that avoids overwhelming the target service.

IMPORTANT: Graph traversal loops
--------------------------------
The graph traversal system is designed to allow a node to be executed only once per run.

When you use the keyword "RESET: node-name", the "seen" status and all state information are
cleared so that the node can be executed again. This creates the potential for an endless loop in
graph traversal.

Therefore, always include decision logic that bounds the looping or retries — like the
"model.attempts" counter in this tutorial.

As a protection mechanism, the system has built-in loop detection. When a node is executed too
frequently, the graph traversal is aborted.

The default parameters in \`application.properties\` allow 10 visits per second for the same node.

\`\`\`properties
graph.max.loop.interval=1000
graph.node.high.frequency=10
\`\`\`
`,"../../../resources/help/help tutorial 13.md":`Tutorial 13
-----------
In this tutorial, you will create a graph model that invokes a composable function using the
"graph.task" skill.

Pre-requisite
-------------
You would need some working knowledge of composable functions. A composable function is a struct
implementing ComposableFunction, registered with the #[preload] attribute. For more details, see
docs/guides/event-driven/ai-agent-guide.md in this repository (the composable-function authoring
guide).

What is a task?
---------------
A task is a node that invokes a composable function through its route name. MiniGraph is designed
to be zero-code with built-in skills for data mapping, decision-making and API fetching. More
complex business logic is delegated to a flow extension or a subgraph (tutorials 10 and 11). A
task node sits in between: it provides a lightweight method to extend a knowledge graph's
capability with a small piece of business logic, without writing a new skill.

Create the graph model
----------------------
Create the root node:

\`\`\`
create node root
with type Root
with properties
name=tutorial-13
purpose=Demonstrate the graph.task skill - invoking a composable function through its route name
\`\`\`

Create the task node. The "task" property is the route name of the composable function:

\`\`\`
create node hello-task
with type Task
with properties
task=v1.hello.task
input[]=input.body -> *
input[]=text(minigraph) -> header.x-app
output[]=result -> output.body
skill=graph.task
\`\`\`

Create the end node and connect the three nodes:

\`\`\`
create node end
with type End
\`\`\`

\`\`\`
connect root to hello-task with run
connect hello-task to end with finish
\`\`\`

For your convenience, this graph model is also preloaded. You can import it with
'import graph from tutorial-13' instead of creating the nodes manually.

About the input data mapping
----------------------------
The input data mapping follows the Event Script syntax and is applied in declaration order:

1. \`input.body -> *\` maps the whole request body as the request body of the composable function.
   Since data mapping entries are processed in order, later entries can merge additional
   key-values into a request body that was seeded with \`*\`.
2. \`text(minigraph) -> header.x-app\` sets a request header of the function call. You can also map
   individual fields, e.g. \`input.body.amount -> amount\` would set one key-value in the request
   body.

If the composable function declares a typed input, the request body is automatically converted at
the function boundary.

About v1.hello.task
-------------------
For your convenience, the composable function "v1.hello.task" is preloaded in dev mode. It
composes a greeting from the "name" field, doubles the "amount" field and echoes the "x-app"
request header. Below is an extract of the function:

\`\`\`rust
/// The tutorial-13 demo task invoked through the graph.task skill.
#[preload(route = "v1.hello.task", instances = 50)]
#[optional_service("app.env=dev")]
pub struct HelloTask;

#[async_trait]
impl ComposableFunction for HelloTask {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: JsonValue = input.body_as().unwrap_or(JsonValue::Null);
        let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("stranger");
        let mut result = serde_json::json!({"greeting": format!("Hello, {name}")});
        if let Some(amount) = body.get("amount").and_then(|v| v.as_f64()) {
            result["doubled"] = serde_json::json!(amount * 2.0);
        }
        if let Some(app) = headers.get("x-app") {
            result["app"] = serde_json::json!(app);
        }
        EventEnvelope::new().set_body(result)
    }
}
\`\`\`

Perform a dry-run
-----------------
To test the graph model, you can instantiate the graph with mock input as follows:

\`\`\`
instantiate graph
text(world) -> input.body.name
int(21) -> input.body.amount
\`\`\`

Then enter 'run' to execute the graph.

\`\`\`
> start graph...
Graph instance created. Loaded 2 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to hello-task
Executed hello-task with skill graph.task in 4.12 ms
Walk to end
{
  "output": {
    "body": {
      "greeting": "Hello, world",
      "doubled": 42.0,
      "app": "minigraph"
    }
  }
}
Graph traversal completed in 6 ms
\`\`\`

You can also check the application log. Telemetry and tracing information are shown, proving that
the composable function was executed by the graph instance with full trace propagation.

\`\`\`
Call task v1.hello.task, ttl=30000
{trace={path=/graph/playground, service=graph.task...
{trace={path=/graph/playground, service=v1.hello.task...
\`\`\`

Error handling
--------------
If the composable function returns an error (e.g. an AppError with a status code) or the call
times out, the "error" and "status" parameters of the node are set. You can add an "exception"
property to the task node to route the error to a handler node, e.g. \`exception=on-error\`
(tutorial 12 shows the full retry pattern).

Iterative execution
-------------------
Like the API fetcher and the flow extension, a task node supports iterative fork-join execution
with the "for_each" and "concurrency" properties. Please enter 'describe skill graph.task' for
details.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without
writing any code, and flow extensions or subgraphs handle complex orchestration. A task node
completes the picture: any custom business logic can be packaged as a composable function and
plugged into a graph as if it were a custom skill. You can extend a knowledge graph's capability
with the full power of the Mercury Composable programming model, one small function at a time.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-13
Graph exported to /tmp/graph/tutorial-13.json
Described in /api/graph/model/tutorial-13/431-3
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-13 \\
  -H "Content-Type: application/json" \\
  -d '{
    "name": "world",
    "amount": 21
}'
\`\`\`

Summary
-------
In this tutorial, you have used the "graph.task" skill to invoke a composable function through its
route name, with Event Script style input and output data mapping.
`,"../../../resources/help/help tutorial 2.md":`Tutorial 2
----------
In this tutorial, you will deploy the 'hello world' graph model that you created in
tutorial 1, then enhance it into an echo application.

Exercise
--------
To deploy the graph model from tutorial 1, copy the 'tutorial-1.json' file that was
exported earlier into your application's resources/graph folder.

\`\`\`
cp /tmp/graph/tutorial-1.json ~/sandbox/{your_project}/resources/graph
\`\`\`

The default locations of the temp graph folder and the deployed graph folder are set
in the application configuration file (application.properties or application.yml):

\`\`\`properties
#
# temp graph working location
# (temp graph location must use "file:/" prefix because of READ/WRITE requirements)
#
location.graph.temp=file:/tmp/graph
#
# deployed graph model location
# (deployed graph location may use "file:/" or "classpath:/" because it is READ only)
#
location.graph.deployed=classpath:/graph
\`\`\`

Invoke the graph API REST endpoint
----------------------------------
The generic graph API endpoint is \`POST /api/graph/{graph_id}\`, where 'graph_id' is
the name of the graph model.

To make a request to the 'tutorial-1' graph model, enter the following curl command.

\`\`\`
> curl -X POST http://127.0.0.1:8100/api/graph/tutorial-1
hello world
\`\`\`

It returns 'hello world'.

Since the "hello world" graph model does not require any input parameter, you can also
use HTTP GET to execute the graph.

\`\`\`
> curl http://127.0.0.1:8100/api/graph/tutorial-1
hello world
\`\`\`

In the application log, you will see the 'telemetry' of the event flow. The HTTP POST
request is received by the 'http.flow.adapter' that executes a flow called
'graph-executor'.

The Graph Executor creates an instance of the graph, traverses from the "root" node
and comes to the "end" node that contains the "graph.data.mapper" skill. The data
mapper sets the output to "hello world", which is routed to "async.http.response"
and returned to the curl command.

The telemetry entries look like this:

\`\`\`
2026-03-31T22:19:08.052Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"http.flow.adapter","success":true,"from":"http.request","exec_time":0.12,"status":200}}
2026-03-31T22:19:08.055Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"graph.data.mapper","success":true,"from":"graph.executor","exec_time":0.074,"status":200},
    "annotations":{"node":"end"}}
2026-03-31T22:19:08.056Z INFO  [knowledge_graph::services] Graph instance 2c1a00d63f7d4ec2b657db4a75021068
    for model 'tutorial-1' cleared
2026-03-31T22:19:08.056Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"task.executor","success":true,"from":"event.script.manager","exec_time":4.0,"status":200},
    "annotations":{"execution":"Run 1 task in 4 ms","flow":"graph-executor"}}
2026-03-31T22:19:08.057Z INFO  [platform_core::telemetry] {"trace":{"path":"POST /api/graph/tutorial-1",
    "service":"async.http.response","success":true,"from":"task.executor","exec_time":0.224,"status":200}}
\`\`\`

Let's enhance the graph model to echo input.

Import the graph model
----------------------
You can import the tutorial-1 graph model like this:

\`\`\`
> import graph from tutorial-1
Graph model imported as draft
\`\`\`

The graph diagram is shown in the right panel under the "Graph" tab.

Edit the nodes
--------------
Enter an "edit node" command to print out the root node content.

\`\`\`
> edit node root
update node root
with type Root
with properties
name=tutorial-1
purpose=Tutorial one to return a 'hello world' message
\`\`\`

Copy-and-paste the "update node" block into the input box and modify it as:

\`\`\`
update node root
with type Root
with properties
name=tutorial-2
purpose=Tutorial two to echo a user message
\`\`\`

Press enter and you will see:

\`\`\`
> update node root...
node root updated
\`\`\`

Then update the end node in the same fashion. Modify its content like this:

\`\`\`
update node end
with type End
with properties
mapping[]=input.body -> output.body
skill=graph.data.mapper
\`\`\`

Perform a dry-run
-----------------
To run the updated graph model, use the \`instantiate graph\` command with some
mock input content.

\`\`\`
> instantiate graph
  text(it works) -> input.body.message
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

In the above command, you insert the constant value "it works" into the "message"
key of the "input.body" namespace.

Enter "run" to do a dry-run and you will see this:

\`\`\`
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 0.43 ms
{
  "output": {
    "body": {
      "message": "it works"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

Export the updated graph model
------------------------------
You may export the updated graph model as "tutorial-2".

\`\`\`
> export graph as tutorial-2
Graph exported to /tmp/graph/tutorial-2.json
Described in /api/graph/model/tutorial-2/235-7
\`\`\`

Deploy the graph model
----------------------
Repeat the deployment step at the beginning of this tutorial: copy
"/tmp/graph/tutorial-2.json" into your application's resources/graph folder.

Test the deployed graph model
-----------------------------
Restart your application to load the deployed graphs into memory.

Send the following curl command:

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-2 \\
  -H "Content-Type: application/json" \\
  -d '{
    "greeting": "Hello",
    "message": "it is a wonderful day"
  }'
\`\`\`

It responds with:

\`\`\`json
{
  "greeting": "Hello",
  "message": "it is a wonderful day"
}
\`\`\`

Summary
-------
In this tutorial, you have completed the following exercise:

1. deployed the graph model 'tutorial-1' and invoked the API that executes the graph model as an instance
2. enhanced the graph model from a simple 'hello world' application to an echo program
3. performed a dry-run with mock input to test the response
4. exported the updated graph model as 'tutorial-2'
5. deployed the 'tutorial-2' graph model
6. tested the 'tutorial-2' graph model using an HTTP POST request with an input payload
`,"../../../resources/help/help tutorial 3.md":`Tutorial 3
----------
In this tutorial, you will learn the data dictionary method to source data from an
external service.

Exercise
--------
You will create a root node, an end node, two data dictionary nodes, a data provider
node and an API fetcher node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

\`\`\`
create node root
with type Root
with properties
name=tutorial-3
purpose=Demonstrate data sourcing using the Data Dictionary method - fetch one person profile (name and address) by person_id
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create data dictionary items
----------------------------
A data dictionary describes a "data attribute" and its "data provider". Please enter
the following:

\`\`\`
create node person-name
with type Dictionary
with properties
purpose=name of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.name -> result.name

create node person-address
with type Dictionary
with properties
purpose=address of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.address -> result.address
\`\`\`

This creates two Dictionary nodes, "person-name" and "person-address", both served by
a data provider called "mdm-profile".

In a Dictionary node, \`input[]\` entries are **bare parameter names** — not
\`source -> target\` mappings. Here, the parameter required to retrieve these data
attributes is "person_id". If a parameter has a sensible default, supply it with an
optional \`:{default}\` suffix (e.g. \`input[]=detail:true\`) — a default value is the
only meaning of \`:\` in a Dictionary input entry.

The \`output[]\` section maps the provider's response into the dictionary's result set.
The \`response.\` and \`result.\` namespaces represent the response key-values from the
data provider and the result key-values produced by this data dictionary.

In the "person-name" data dictionary, the output mapping extracts the "profile.name"
attribute from the response's data structure and exposes it as the key "name".

Create a data provider
----------------------
The data dictionaries name a data provider "mdm-profile". Create a node for it:

\`\`\`
create node mdm-profile
with type Provider
with properties
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
\`\`\`

The "url" is the REST endpoint of the target service. \`\${rest.server.port:8080}\`
resolves a key-value from the application configuration or an environment variable;
the value after the optional \`:\` is a default.

In this example, the url has a path parameter "id" — filled by the \`input[]\` line
that targets \`path_parameter.id\`.

The "feature" section tells the system to apply pre-processing and/or post-processing
to the HTTP request/response. "log-request-headers" logs the request headers, if any,
and "log-response-headers" logs the HTTP response headers from the target service.
These two features are for demonstration; in a real-world use case, you might
implement an "oauth2-bearer" feature. Custom features are discussed in a subsequent
tutorial.

The input section maps values into the outgoing HTTP request — headers, path
parameters, query and/or body key-values. The target namespaces are:

\`\`\`
header.
query.
path_parameter.
body.
\`\`\`

The left-hand side of a provider input mapping is a constant (e.g.
\`text(application/json)\`) or an input parameter declared by the associated data
dictionary (e.g. \`person_id\`).

Create an API fetcher
---------------------
Create a fetcher node like this:

\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

After this step, you will see 6 nodes in the graph diagram on the right panel.

Connect the fetcher
-------------------
Connect the root node to the fetcher node, then the fetcher to the end node.

\`\`\`
> connect root to fetcher with fetch
node root connected to fetcher
> connect fetcher to end with complete
node fetcher connected to end
\`\`\`

Export the graph model
----------------------
The execution path is complete. Let's export it as 'tutorial-3'.

\`\`\`
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/849-13
\`\`\`

Test the fetcher node
---------------------
Before you do a dry-run, you can test the fetcher alone because it is self-contained:
it maps the input parameter to 'person_id', makes an outgoing HTTP request using the
data dictionary and returns the result as "output.body".

First, instantiate the graph model and mock the input parameter like this:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

The system acknowledges your command as follows:

\`\`\`
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

Before you test the fetcher, check the input and output key-values with the
\`inspect\` command:

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "person_id": 100
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {}
}
\`\`\`

When a graph model is instantiated, the system creates a temporary "state machine"
for the graph instance. The inspect command lets you check the current key-values in
that state machine.

The above output shows that "person_id" with the integer value 100 is stored in
input.body, and there is nothing in the output yet.

You can now test the fetcher with the "execute" command:

\`\`\`
> execute fetcher
node fetcher run for 0.266 ms with exit path 'next'
\`\`\`

The fetcher has been executed and it is ready to continue to the next node.

Now inspect the "output" in the state machine again.

\`\`\`
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
\`\`\`

The result set contains the name and address obtained from the target service.

Dry-run
-------
The fetcher is configured correctly, so you can do a dry-run from beginning to end.

Clear the state machine by instantiating the graph model again:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

\`\`\`
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

Verify that the output key-values are cleared with \`inspect output\`. Then enter \`run\`.

\`\`\`
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 14.456 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 15 ms
\`\`\`

List nodes and connections
--------------------------
Let's check the nodes and connections of the graph model 'tutorial-3'.

Enter the \`list nodes\` and \`list connections\` commands:

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Note that the data dictionary and data provider nodes have no connections yet. They
are "configuration" nodes — not active nodes that execute on their own. The API
fetcher references them by name and uses their configuration to make an external
API call.

For more details of the data dictionary method, enter "help data-dictionary".

Configuration nodes must still not be left floating — the next step wires them into
the graph's knowledge layer.

Create an island to hold the data dictionary
--------------------------------------------
The required convention is: **leave no node unconnected**. Configuration nodes are
wired into the graph's knowledge layer with an "island" node.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and the provider node to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
dictionary -[data]-> person-address
dictionary -[data]-> person-name
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher -[complete]-> end
\`\`\`

A "graph.island" node is isolated from graph traversal: it never hands execution to
the next node, so the execution path is unaffected. Its purpose is knowledge
structure — the island subgraph is the graph's entity-relationship diagram.

Data entities such as person, account and order, and the directional relationships
between them, represent enterprise knowledge. With the dictionaries, providers and
entities wired under the island, the graph becomes living documentation: a new team
member (or an AI agent) can read the knowledge layer to discover the domain model,
not just the execution path.

To save the updated graph model, export it again.

\`\`\`
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/287-4
\`\`\`

Deploy the graph model
----------------------
To deploy, copy "/tmp/graph/tutorial-3.json" into your application's resources/graph
folder and restart the application. You can then invoke the knowledge graph endpoint
with the following curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-3 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
  }'
\`\`\`

Note that input parameters, if any, must be submitted as a POST request body with
content type "application/json".

You will receive the following response:

\`\`\`json
{
  "address": "100 World Blvd",
  "name": "Peter"
}
\`\`\`

If you change the person_id to 10, you will receive an error because the test profile
is set to 100.

\`\`\`json
{
  "message": "Profile 10 not found",
  "type": "error",
  "target": "person-name",
  "status": 400
}
\`\`\`

Well done! You have successfully created a graph model that fetches external data.

API call optimization
---------------------
If you check the application log, you will notice that each graph instance makes only
one HTTP call to \`http://127.0.0.1:8100/api/mdm/profile/10\`.

When multiple data dictionary items share the same target URL, method and input
parameter values, the system avoids making redundant API calls.

Therefore, it is important to configure the data dictionary and provider correctly so
that the system fetches data efficiently.

Summary
-------
In this tutorial, you configured a data dictionary and a data provider, and defined an
API fetcher node that uses them to fetch data. You deployed the graph model and made
an API request with a curl command.

You also organized the data dictionary and provider nodes under an "island" — the
required knowledge-layer convention that leaves no node unconnected.
`,"../../../resources/help/help tutorial 4.md":`Tutorial 4
----------
In this tutorial, you will set up simple mathematics and boolean operations in a
graph model to make a decision.

Exercise
--------
You will create a root node, an end node and a decision node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

\`\`\`
create node root
with type Root
with properties
name=tutorial-4
purpose=Demonstrate decision making using mathematics and boolean operations
\`\`\`

Assume there are two input parameters (a and b). The 'decision' node will add the two
numbers, and the end node will echo the input parameters and the sum.

\`\`\`
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=input.body.a -> output.body.a
mapping[]=input.body.b -> output.body.b
mapping[]=decision.result.c -> output.body.sum
\`\`\`

Create a decision node
----------------------
Create a node with the skill 'graph.math' to do decision-making.

\`\`\`
create node decision
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: c -> {input.body.a} + {input.body.b}
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: next
ELSE: less-than
'''
statement[]=MAPPING: text(a >= b) -> output.body.message
statement[]=MAPPING: boolean(false) -> output.body.less_than
\`\`\`

The skill "graph.math" supports these statement types:

| Type         | Operation                                                       |
|--------------|-----------------------------------------------------------------|
| COMPUTE      | generate a value (LHS) from a mathematics expression (RHS)      |
| IF-THEN-ELSE | evaluate a boolean condition and select the next node           |
| MAPPING      | perform a data mapping operation                                |
| EXECUTE      | run another graph.math node's statements inline (module reuse)  |
| RESET        | reset the current state of one or more nodes                    |

The 'RESET' and 'EXECUTE' features are covered in more advanced tutorials.
Enter "help graph-math" for the full statement grammar.

Use the 'triple single quote' syntax to enter the IF-THEN-ELSE statement as one
multi-line value.

The IF line is a boolean expression. THEN names the next step when the expression is
true; ELSE names the next step when it is false. Each may be a node name or the
keyword 'next'.

Statements are evaluated in order. A branch that resolves to 'next' falls through to
the statements after the IF-THEN-ELSE — in this example, the two MAPPING statements
that set the positive-case output key-values. A branch that jumps to a named node
(here 'less-than') ends the statement list immediately, so those mappings do not run.

The curly brace syntax \`{key}\` substitutes the value of the bracketed key inside a
COMPUTE or IF expression. A MAPPING statement does not use curly braces — it is data
mapping only, where the left-hand side is a constant, an input parameter or a model
variable, and the right-hand side is a model or output variable.

Create a node to handle the negative case
-----------------------------------------
Create a node called "less-than" to handle the negative case from the decision node.

\`\`\`
create node less-than
with type Reject
with properties
mapping[]=text(a < b) -> output.body.message
mapping[]=boolean(true) -> output.body.less_than
skill=graph.data.mapper
\`\`\`

Connect the nodes
-----------------

\`\`\`
connect root to decision with evaluate
connect less-than to end with negative
connect decision to end with positive
\`\`\`

The "less-than" node is reached only when the decision node evaluates "a < b", so it
does not need a connection from the root. When it finishes, it hands off to the "end"
node. A "list connections" command shows:

\`\`\`
> list connections
root -[evaluate]-> decision
decision -[positive]-> end
less-than -[negative]-> end
\`\`\`

You can also use the "describe node" command to see a node's content and connections:

\`\`\`
> describe node decision
{
  "node": {
    "types": [
      "Decision"
    ],
    "alias": "decision",
    "id": "c9b30d7d8a6c4d49a88b5a9254fe44e2",
    "properties": {
      "skill": "graph.math",
      "statement": [
        "COMPUTE: c -> {input.body.a} + {input.body.b}",
        "IF: {input.body.a} >= {input.body.b}
         THEN: next
         ELSE: less-than",
        "MAPPING: text(a >= b) -> output.body.message",
        "MAPPING: boolean(false) -> output.body.less_than"
      ]
    }
  },
  "from": [
    "root"
  ],
  "to": [
    "end"
  ]
}
\`\`\`

Test the positive case
----------------------
To test the positive case, mock the input values and instantiate the graph model.
Note that "start" is an alias of "instantiate".

\`\`\`
start graph
int(100) -> input.body.a
int(50) -> input.body.b
\`\`\`

Then test the graph model with the "run" command:

\`\`\`
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.824 ms
Walk to end
Executed end with skill graph.data.mapper in 0.099 ms
{
  "output": {
    "body": {
      "a": 100,
      "b": 50,
      "less_than": false,
      "sum": 150.0,
      "message": "a >= b"
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

Note that "sum" is 150.0 — a COMPUTE statement evaluates to a floating-point number,
so an integer result serializes with a decimal point (it is numerically exact).

Test the negative case
----------------------

\`\`\`
start graph
int(180) -> input.body.a
int(250) -> input.body.b
\`\`\`

When you do a dry-run, it shows the following:

\`\`\`
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.394 ms
Walk to less-than
Executed less-than with skill graph.data.mapper in 0.054 ms
Walk to end
Executed end with skill graph.data.mapper in 0.051 ms
{
  "output": {
    "body": {
      "a": 180,
      "b": 250,
      "less_than": true,
      "sum": 430.0,
      "message": "a < b"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-4
Graph exported to /tmp/graph/tutorial-4.json
Described in /api/graph/model/tutorial-4/804-24
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-4.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

Summary
-------
In this tutorial, you created a graph model that adds two numbers, compares them and
returns a decision.

While this is a trivial example, it demonstrates that you can build useful computation
and evaluation logic in an Active Knowledge Graph using just simple mathematics and
boolean operation statements.
`,"../../../resources/help/help tutorial 5.md":`Tutorial 5
----------
In this tutorial, you will explore parallel processing and graph navigation using a
node with the skill 'graph.join'.

Exercise
--------
You will import the graph model from tutorial-3 and update it to fetch two user
profiles at the same time.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
\`\`\`

If you have not exported tutorial-3 earlier, the system imports it from a demo graph.

Examine the graph model
-----------------------
Examine the graph model with the 'list nodes' and 'list connections' commands.

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Review the fetcher node
-----------------------
Enter 'edit node fetcher' to review the configuration of the node. The system
displays the following:

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

Create two new fetchers
-----------------------
Assume the use case is to fetch two user profiles at the same time. Create two
fetchers like this:

\`\`\`
create node fetcher-1
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person1 -> person_id
output[]=result.name -> model.fetcher-1.name
output[]=result.address -> model.fetcher-1.address
output[]=model.fetcher-1 -> output.body.profile[]
skill=graph.api.fetcher
\`\`\`

\`\`\`
create node fetcher-2
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person2 -> person_id
output[]=result.name -> model.fetcher-2.name
output[]=result.address -> model.fetcher-2.address
output[]=model.fetcher-2 -> output.body.profile[]
skill=graph.api.fetcher
\`\`\`

When two skilled nodes execute in parallel, pay attention to how they share the state
machine. Data mapping itself is thread-safe — state-machine operations are
serialized — but parallel branches must not write to the same scalar key: the last
writer wins, nondeterministically. Write to disjoint keys instead. Here, each fetcher
assembles its profile under its own temporary variable in the "model" namespace:
\`model.fetcher-1\` and \`model.fetcher-2\`.

The final output mapping uses the array append syntax \`[]\`, which appends the map
containing name and address to the 'profile' array. Appending with \`[]\` from parallel
branches is race-free, but the element order follows completion order — undetermined
across parallel branches. If you must guarantee that person1's result goes to array
element 0 and person2's to element 1, set the array element index directly:

\`\`\`
output[]=model.fetcher-1 -> output.body.profile[0]
\`\`\`

\`\`\`
output[]=model.fetcher-2 -> output.body.profile[1]
\`\`\`

Since profile order does not matter in this tutorial, we will use the append form \`[]\`.

Create a join node
------------------
Create a "join" node to synchronize the two parallel branches:

\`\`\`
create node join
with type Join
with properties
skill=graph.join
\`\`\`

Remove the original fetcher node
--------------------------------
Enter 'delete node fetcher' to remove the original fetcher node.

\`\`\`
> delete node fetcher
node fetcher deleted
\`\`\`

When the original fetcher is deleted, its connections to the root node and end node
are removed too.

Connect the new fetchers
------------------------
Enter the following to define the graph navigation.

\`\`\`
connect root to fetcher-1 with one
connect root to fetcher-2 with two
connect fetcher-1 to join with join
connect fetcher-2 to join with join
connect join to end with done
\`\`\`

Do a 'list connections' to confirm the setup.

\`\`\`
> list connections
root -[one]-> fetcher-1
root -[two]-> fetcher-2
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
join -[done]-> end
\`\`\`

Perform a dry-run
-----------------
Start the graph model with this mock input:

\`\`\`
start graph
int(100) -> input.body.person1
int(200) -> input.body.person2
\`\`\`

Then enter 'run' to execute the graph instance.

\`\`\`
> run
Walk to root
Walk to fetcher-2
Walk to fetcher-1
Executed fetcher-1 with skill graph.api.fetcher in 1.048 ms
Walk to join
Executed fetcher-2 with skill graph.api.fetcher in 0.931 ms
Walk to join
Executed join with skill graph.join in 0.04 ms
Walk to end
Executed join with skill graph.join in 0.017 ms
{
  "output": {
    "body": {
      "profile": [
        {
          "address": "100 World Blvd",
          "name": "Mary"
        },
        {
          "address": "100 World Blvd",
          "name": "Peter"
        }
      ]
    }
  }
}
Graph traversal completed in 6 ms
\`\`\`

If you check the application log, you will see the two fetchers executed in parallel.

\`\`\`
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8100/api/mdm/profile/100, with ["person_id"], ttl=30000
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8100/api/mdm/profile/200, with ["person_id"], ttl=30000
\`\`\`

Create an island to hold the data dictionary
--------------------------------------------
Just like tutorial 3, wire the data dictionary and provider nodes into the graph's
knowledge layer with an island node. This is the required convention — leave no node
unconnected: the island subgraph is the graph's entity-relationship diagram, turning
the graph into living documentation of enterprise knowledge.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and the provider node to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> list connections
root -[contains]-> dictionary
root -[one]-> fetcher-1
root -[two]-> fetcher-2
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
join -[done]-> end
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-5
Graph exported to /tmp/graph/tutorial-5.json
Described in /api/graph/model/tutorial-5/920-28
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-5.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-5 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person1": 100,
    "person2": 200
  }'
\`\`\`

Summary
-------
In this tutorial, you created a graph model capable of parallel processing. It makes
two API requests at the same time; the two branches then converge into a "join" node
before reaching the "end" node.

The execution of a graph instance is guided by graph traversal: it follows the
connections you define between nodes. If a node has a skill, the graph executor runs
the composable function that provides the skill; if not, the graph executor continues
to the next downstream node.
`,"../../../resources/help/help tutorial 6.md":`Tutorial 6
----------
In this tutorial, you will create a graph model that fetches an array list from one
service and iterates over the elements of the array to fetch more details from
another service, using the "for_each" keyword.

Exercise
--------
You will import the graph model from tutorial-3 as a template and expand it to handle
a multi-step data fetch use case.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
\`\`\`

If you have not exported tutorial-3 earlier, the system imports it from a demo graph.

Examine the graph model
-----------------------
Examine the graph model with the 'list nodes' and 'list connections' commands.

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Create a new data dictionary node
---------------------------------
Enter the following to create a new data dictionary node "person-accounts". It uses
the same data provider "mdm-profile" to retrieve the list of accounts for a person —
an array of account numbers.

\`\`\`
create node person-accounts
with type Dictionary
with properties
input[]=person_id
output[]=response.accounts -> result.account_numbers
provider=mdm-profile
purpose=accounts of a person
\`\`\`

Update the fetcher
------------------
Add the dictionary item "person-accounts" to the original fetcher.

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
dictionary[]=person-accounts
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

Create one more data dictionary node
------------------------------------
Create a data dictionary node "account-details", associated with the data provider
"account-details-provider", to retrieve account details based on person_id and
account_id.

\`\`\`
create node account-details
with type Dictionary
with properties
input[]=person_id
input[]=account_id
output[]=response.account.details -> result.accounts
provider=account-details-provider
purpose=Account details
\`\`\`

Create a new data provider
--------------------------
Enter the following to create the data provider that retrieves account details.

Its feature section declares oauth2-bearer, log-request-headers and
log-response-headers. The "oauth2-bearer" entry is a placeholder — implement it
according to your organization's security guidelines. Functionally, it would acquire
an OAuth2 bearer token from a security authority using a client id and secret
configured in the deployed environment, cache and refresh the access token as
required, and insert the "authorization" header in a pre-processing step of the Graph
API Fetcher. The log-request-headers and log-response-headers features can serve as
templates for implementing your own pre-processing and post-processing features.

\`\`\`
create node account-details-provider
with type Provider
with properties
feature[]=oauth2-bearer
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
method=POST
purpose=Account Management Endpoint
url=http://127.0.0.1:\${rest.server.port}/api/account/details
\`\`\`

Note that this is a POST provider: the \`body.{key}\` input targets build the JSON
request body, and the parameters travel in the body rather than the URL.

Create a second fetcher
-----------------------
Create a second fetcher as follows. The \`for_each\` statement iterates over the array
in the first fetcher's result set (\`fetcher.result.account_numbers\`), mapping each
element into "model.account_number".

For each element, the input statement block runs to populate the input parameters:
"person_id" is passed unchanged to every call, while "account_id" takes the current
element.

\`\`\`
create node fetcher-2
with type Fetcher
with properties
dictionary[]=account-details
for_each[]=fetcher.result.account_numbers -> model.account_number
input[]=input.body.person_id -> person_id
input[]=model.account_number -> account_id
output[]=result.accounts -> output.body.accounts
skill=graph.api.fetcher
\`\`\`

Each iteration's \`result.accounts\` value is appended into a single array on this
node's result set — with five account numbers, "output.body.accounts" becomes an
array of five account detail records.

Rearrange the connections
-------------------------
Connect the first fetcher to the second fetcher, delete the original connection
between the fetcher and the end node, then connect the second fetcher to the end node.

Enter 'list connections' to show the updated connections.

\`\`\`
> connect fetcher to fetcher-2 with details
node fetcher connected to fetcher-2
> delete connection fetcher and end
fetcher -> end removed
> connect fetcher-2 to end with complete
node fetcher-2 connected to end
> list connections
root -[fetch]-> fetcher
fetcher -[details]-> fetcher-2
fetcher-2 -[complete]-> end
\`\`\`

Update the root node
--------------------
Since you are using the tutorial-3 graph model as a template, it is good practice to
update the root node to describe the new purpose of tutorial-6. Enter the following.

\`\`\`
update node root
with type Root
with properties
name=tutorial-6
purpose=Demonstrate multi-step API fetching and the "for_each" method
\`\`\`

Perform a dry-run
-----------------
Enter the following to mock the input parameter "person_id = 100".

\`\`\`
start graph
int(100) -> input.body.person_id
\`\`\`

Then enter \`run\` to do a dry-run. You will see the following:

\`\`\`
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 12.085 ms
Walk to fetcher-2
Executed fetcher-2 with skill graph.api.fetcher in 14.326 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter",
      "accounts": [
        {
          "balance": 25032.13,
          "id": "a101",
          "type": "Saving"
        },
        {
          "balance": 6020.68,
          "id": "b202",
          "type": "Current"
        },
        {
          "balance": 120000.0,
          "id": "c303",
          "type": "C/D"
        },
        {
          "balance": 6000.0,
          "id": "d400",
          "type": "apple"
        },
        {
          "balance": 8200.0,
          "id": "e500",
          "type": "google"
        }
      ]
    }
  }
}
Graph traversal completed in 28 ms
\`\`\`

Parallelism
-----------
With the "for_each" method, the system performs the API fetches in parallel. The
default concurrency is 3; set "concurrency" in "fetcher-2" (1-30) to try other
values.

With a concurrency of 3 and five accounts, the system makes a batch of 3 followed by
a batch of 2 API requests. When you change the concurrency setting, the batch size
adjusts accordingly.

Aggregation order is guaranteed: batches execute in source-list order and responses
join in request order, so the aggregated result array preserves the order of the
source account numbers — regardless of the concurrency setting. You can see this in
the dry-run above: the account details appear in the same order as the account
numbers (a101 to e500).

Create an island to hold the data dictionary
--------------------------------------------
Wire the data dictionary and provider nodes into the graph's knowledge layer with an
island node. This is the required convention — leave no node unconnected: the island
subgraph is the graph's entity-relationship diagram, turning the graph into living
documentation of enterprise knowledge.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and provider nodes to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect dictionary to person-accounts with data
node dictionary connected to person-accounts
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> connect person-accounts to mdm-profile with provider
node person-accounts connected to mdm-profile
> connect dictionary to account-details with data
node dictionary connected to account-details
> connect account-details to account-details-provider with provider
node account-details connected to account-details-provider
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
account-details -[provider]-> account-details-provider
dictionary -[data]-> account-details
dictionary -[data]-> person-accounts
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher -[details]-> fetcher-2
person-accounts -[provider]-> mdm-profile
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher-2 -[complete]-> end
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-6
Graph exported to /tmp/graph/tutorial-6.json
Described in /api/graph/model/tutorial-6/775-18
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-6.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-6 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
  }'
\`\`\`

Summary
-------
In this tutorial, you created a graph model that performs two steps of API fetching.
The first step gets the name, address and list of account numbers. The second step
uses the "for_each" method to fetch the account details for each account number, and
aggregates the results into a single array in source-list order.
`,"../../../resources/help/help tutorial 7.md":`Tutorial 7
----------
In this tutorial, you will explore data mapping in more detail.

Exercise
--------
You will create a new graph model to test various data mapping methods.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-7
purpose=Demonstrate various data mapping methods
\`\`\`

\`\`\`
create node end
with type End
with properties
\`\`\`

Create a data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

\`\`\`
create node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
\`\`\`

\`mapping[]\` builds the node's data mapping statement list in "append mode": the
statements are evaluated in the order provided.

Each data mapping statement has a left-hand side (the source) and a right-hand side
(the target), separated by the "map to" indicator (\`->\`). The value of the source is
mapped to the target key.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper"; the full syntax lives in
docs/guides/event-script/syntax.md in this repository.

*Constant* — \`text(world)\` means a constant of "world". \`output.body.\` is the
namespace for the output payload when a graph finishes execution. In this example,
output.body is populated with "hello=world".

*Input* — \`input.body\` is the namespace for the input payload provided to a graph
instance when it starts.

Assuming the input payload looks like this:

\`\`\`json
{
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}
\`\`\`

The value "Peter" is mapped to the "name" field, and address1 and address2 become the
first and second elements of an array in "model.address". The \`model.\` namespace is a
temporary state machine that lives for the duration of the graph instance — use it as
a scratch buffer for data transformation.

*Output* — the mapping statement \`model.address -> output.body.address\` maps the
two-element address array into the output payload of the graph instance.

Building an array — two techniques
----------------------------------
*Direct addressing (preferred)* — set the array element index explicitly:

\`\`\`
mapping[]=input.body.profile.address1 -> model.address[0]
mapping[]=input.body.profile.address2 -> model.address[1]
mapping[]=model.address -> output.body.address
\`\`\`

Numeric indices write each value into a known slot, so the result is deterministic
and the mapping is idempotent — executing the node again simply overwrites the same
slots. Use direct addressing whenever you know where each value belongs.

*Append + clear (for append-mode workflows)* — the array append syntax (\`[]\`) adds
one element to the end of the array on every execution. That is what you want when a
workflow accumulates an unknown number of elements — but it is not idempotent: during
testing, you may execute the same node several times, and each pass would append
duplicate entries. To make an append sequence repeatable, clear the array first by
mapping a non-existent key (conventionally \`model.none\`) to it:

\`\`\`
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
\`\`\`

Mapping a source that does not exist removes the target key — the \`model.none\` clear
idiom. This exercise deliberately uses the append + clear form so you can observe the
idiom at work.

*Plugin functions* — the left-hand side of \`f:now(text(local)) -> output.body.time\`
uses the \`f:\` syntax to execute a "plugin" function called "now". It takes the
constant value "local" and returns a local timestamp. A number of built-in data
mapping plugins are available — see the simple-plugin catalog in
docs/guides/event-script/syntax.md in this repository.

Test the data mapper
--------------------
You can test the data mapper before you complete the whole graph model.

Enter the following to instantiate the graph and open a dialog box for the mock
input data.

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
\`\`\`

When you enter the "upload mock data" command, an input dialog box opens. Paste the
sample input payload for the "profile" of "Peter" listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "address2": "New York",
        "address1": "100 World Blvd",
        "name": "Peter"
      }
    }
  }
}
\`\`\`

You can now test the data mapper by "executing" it. Enter "execute data-mapper".

\`\`\`
> execute data-mapper
ERROR: node data-mapper does not have a skill property
\`\`\`

The system rejects the request with an error message: the data-mapper node is missing
a skill.

Enter 'edit node data-mapper', copy the printed "update node" block into the input
box, add "skill=graph.data.mapper" and submit.

\`\`\`
> edit node data-mapper
update node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
skill=graph.data.mapper
\`\`\`

The system will display "node data-mapper updated".

To activate the updated node, restart the graph instance by entering
'instantiate graph' and 'upload mock data'. Submit the mock input payload again.

Then execute the data-mapper again.

\`\`\`
> execute data-mapper
node data-mapper run for 0.488 ms with exit path 'next'
\`\`\`

The data-mapper runs successfully.

Inspect the model and output
----------------------------
Inspect the model and the output key-values to see what values were mapped.

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "address": [
      "100 World Blvd",
      "New York"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": [
        "100 World Blvd",
        "New York"
      ],
      "name": "Peter",
      "hello": "world",
      "time": "2026-04-11 19:52:22.527"
    }
  }
}
\`\`\`

Connect the nodes to complete the graph model
---------------------------------------------
Enter the two connect commands below.

\`\`\`
> connect root to data-mapper with mapping
node root connected to data-mapper
> connect data-mapper to end with complete
node data-mapper connected to end
\`\`\`

The graph model is shown in the right panel.

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-7
Graph exported to /tmp/graph/tutorial-7.json
Described in /api/graph/model/tutorial-7/152-13
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-7.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-7 \\
  -H "Content-Type: application/json" \\
  -d '{
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}'
\`\`\`

Summary
-------
In this tutorial, you created a graph model that performs data mapping. You compared
the two array-building techniques — direct addressing (preferred) and append + clear
with the \`model.none\` idiom — transformed address1 and address2 into an array, and
applied the "f:now()" plugin function to return the current time.
`,"../../../resources/help/help tutorial 8.md":`Tutorial 8
----------
In this tutorial, you will use the JSON-Path search feature to retrieve key-values from the input
payload, then reshape the result with the f:listOfMap() and f:removeKey() plugins. Reshaping a
third-party API response into your own internal data contract — "impedance matching" — is one of
the most common jobs for a data mapper, and these tools let you do it without writing code.

Exercise
--------
You will import tutorial-7 and replace some data mapping statements with JSON-Path search requests.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

Import tutorial-7
-----------------
Enter 'import graph from tutorial-7' first.

\`\`\`
> import graph from tutorial-7
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Input payload
-------------
The account holder "Peter" has 2 accounts. We will assume the following input payload data
structure. You will copy-n-paste this JSON dataset when the "upload mock data" dialog box opens
later in this exercise.

\`\`\`json
{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}
\`\`\`

Edit the data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.type
mapping[]=$.input.body.profile.account[*].id -> model.id
mapping[]=$.input.body.profile.account[*].amount -> model.amount
skill=graph.data.mapper
\`\`\`

A mapping source that starts with "$." is a JSON-Path expression evaluated over the state machine.
The three JSON-Path statements above use the [*] wildcard to extract the type, id and amount from
every element of the account list in the input payload. For a simple key, prefer the plain
dot-bracket form (like the first statement) and save JSON-Path for queries that need it.

Test the data mapper
--------------------
Enter the following to instantiate the graph and open a dialog box to enter the mock input data.

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
\`\`\`

The first data mapping statement maps input.body.profile.name into the "name" field of the output
body. The JSON-Path statements extract the type, id and amount key-values from the account list
and map them into the model variables type, id and amount accordingly.

When you enter the "upload mock data" command, an input dialog box will open. Please paste the
sample input payload listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "name": "Peter",
        "account": [
          {
            "amount": 18000.3,
            "description": "Time deposit",
            "id": "100",
            "type": "C/D"
          },
          {
            "amount": 62050.8,
            "description": "Saving account",
            "id": "200",
            "type": "Saving"
          }
        ]
      }
    }
  }
}
\`\`\`

You can now test the data mapper by executing it. Enter "execute data-mapper".

\`\`\`
> execute data-mapper
node data-mapper run for 0.589 ms with exit path 'next'
\`\`\`

The data-mapper runs successfully.

Inspect the model and output
----------------------------
You can inspect the model and the output key-values to see what values are mapped.

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "amount": [
      18000.3,
      62050.8
    ],
    "id": [
      "100",
      "200"
    ],
    "type": [
      "C/D",
      "Saving"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter"
    }
  }
}
\`\`\`

This confirms that the JSON-Path statements have extracted the key-values from the account list
successfully. However, three parallel lists — a "map of lists" — is usually not a good schema
design: easy for an application to parse, but harder for a human to read. Let's turn it into a
proper list of maps.

Using the listOfMap plugin
--------------------------
For proper data structure representation, use the plugin f:listOfMap() to consolidate the maps of
lists into a list of maps. Update the data mapper like this:

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
skill=graph.data.mapper
\`\`\`

Note the extra level of key called "account" that holds the 3 lists for type, id and amount. The
f:listOfMap() plugin then consolidates the maps of lists into a list of maps.

Instantiate the graph, upload the same mock data and execute the data-mapper again. When you enter
'inspect model' and 'inspect output', you will see:

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "account": {
      "amount": [
        18000.3,
        62050.8
      ],
      "id": [
        "100",
        "200"
      ],
      "type": [
        "C/D",
        "Saving"
      ]
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
\`\`\`

This illustrates that the listOfMap plugin can perform simple data transformation. It is handy
when your graph model uses API fetchers to retrieve data from multiple sources: without writing
code, you can group data from different data structures into the shape your consumers expect.

Using the removeKey plugin
--------------------------
When the data comes from a single source, it is even easier to use the f:removeKey() plugin to
drop the unwanted keys directly. Its form is:

\`\`\`
f:removeKey(source, text(key1), text(key2), ...)
\`\`\`

It removes the named keys from a map — or from every map in a list — and returns a copy of the
data structure. Here it strips the "description" field from every account:

\`\`\`
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account
\`\`\`

Let's prove this by editing the data-mapper again. We add a new data mapping statement at the end
to map the alternative solution to the "account2" field in the output payload.

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account2
skill=graph.data.mapper
\`\`\`

Do 'instantiate graph' and 'upload mock data' with the same input payload. Then
'execute data-mapper' and 'inspect output' to see the outcome.

\`\`\`
> execute data-mapper
node data-mapper run for 2.826 ms with exit path 'next'
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account2": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ],
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
\`\`\`

Note that "account" and "account2" have the same key-values and data structure. This confirms that
the "description" key-value has been removed from each map in the list successfully.

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-8
Graph exported to /tmp/graph/tutorial-8.json
Described in /api/graph/model/tutorial-8/315-6
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-8.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-8 \\
  -H "Content-Type: application/json" \\
  -d '{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}'
\`\`\`

Summary
-------
In this tutorial, you have used JSON-Path retrieval to extract key-values from a list, applied the
f:listOfMap() plugin to consolidate maps of lists into a list of maps, and used the f:removeKey()
plugin to remove unwanted key-values from a list of maps — the building blocks for reshaping a
third-party API response into your internal data contract.

Note that JSON-Path also supports value comparison for selective key-value retrieval. Please refer
to a JSON-Path syntax reference on the web for more details.
`,"../../../resources/help/help tutorial 9.md":`Tutorial 9
----------
In this tutorial, you will create a reusable module — a formula authored once, then borrowed by
any node that needs it.

Exercise
--------
You will create a reusable "addition" module, call it from a compute node with the EXECUTE
statement, and organize the module under an island node.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a reusable module?
--------------------------
A module is a node with the graph.math skill that stays off the execution path. For a frequently
used math formula or boolean operation, you can save the "common logic" in one or more module
nodes and export them as a common graph model. When you design a new graph model, you can import
the modules you need from that common model.

This is a best practice for common computation and decision logic: developers do not re-invent the
same formula, and the shared modules encourage quality control and governance.

For this tutorial, we will skip exporting a common graph model and focus on creating a reusable
module and using it in a graph model.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-9
purpose=Demonstrate use of modules
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create a reusable module
-------------------------
You will create a simple "addition" module that adds two numbers and saves the result in a
variable called "sum".

\`\`\`
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
\`\`\`

Test the module
---------------
Enter the following to start the graph model and set two numbers in the variables "a" and "b" of
the state machine's "model" namespace.

\`\`\`
instantiate graph
int(10) -> model.a
int(20) -> model.b
\`\`\`

You can then test the module using 'execute addition'.

\`\`\`
> execute addition
node addition run for 0.312 ms with exit path 'next'
\`\`\`

Then you can inspect the node.

\`\`\`
> inspect addition
{
  "inspect": "addition",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
\`\`\`

The module adds the two numbers and saves the result "30.0" into the variable "sum" in the node's
result set. (When executed directly, the result lands on the module itself — the next step shows
what changes when another node executes it.)

Using the new module
--------------------
You will create a new node that uses the module.

\`\`\`
create node compute
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: compute.result.sum -> output.body.sum
\`\`\`

This node maps the input parameters "a" and "b" into the model variables "a" and "b", executes the
module "addition", then maps the computed value to the output payload "output.body.sum". Note the
last statement reads compute.result.sum — not addition.result.sum — for the reason shown next.

Test the compute node
---------------------
You will instantiate the graph model like this:

\`\`\`
instantiate graph
int(10) -> input.body.a
int(20) -> input.body.b
\`\`\`

Then enter 'execute compute'. It maps the input parameters to the model variables and executes the
module "addition" that adds the two model variables together.

Inspect the result
------------------
The result is saved to the variable "sum" under the "compute" node instead of the module
"addition". EXECUTE runs the module's statements in the caller's context: any COMPUTE result lands
on the invoking node (compute.result.sum here), and the module's own namespace stays empty — the
compute node just borrows the logic from the module.

\`\`\`
> inspect compute
{
  "inspect": "compute",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
> inspect model
{
  "inspect": "model",
  "outcome": {
    "a": 10,
    "b": 20
  }
}
> inspect addition
{
  "inspect": "addition",
  "outcome": {}
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "sum": 30.0
    }
  }
}
\`\`\`

The module works as expected.

Connect the nodes
-----------------
You will connect the nodes with the following commands:

\`\`\`
connect root to compute with calculate
connect compute to end with finish
\`\`\`

Test the completed model
------------------------
You will enter the following to test the whole model ('start' is an alias of 'instantiate').

\`\`\`
start graph
int(10) -> input.body.a
int(20) -> input.body.b
\`\`\`

Then enter 'run' to do a 'dry-run' from the root to the end node.

\`\`\`
> run
Walk to root
Walk to compute
Executed compute with skill graph.math in 0.387 ms
Walk to end
{
  "output": {
    "body": {
      "sum": 30.0
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

Check the nodes and connections
-------------------------------
Enter the following to show the nodes and connections.

\`\`\`
> list nodes
root [Root]
addition [Module]
compute [Compute]
end [End]
> list connections
root -[calculate]-> compute
compute -[finish]-> end
\`\`\`

Note that the module "addition" is not part of the traversal path — the compute node that executes
it is. However, the convention is to leave no node unconnected: 'export' fails if any node is an
orphan, and off-path nodes belong in the graph's knowledge structure so the model documents
itself. The next step wires the module in.

Create an island to hold modules
--------------------------------
You will create an island node to organize one or more module nodes. An island is isolated from
graph traversal, so the execution path is unaffected.

\`\`\`
create node modules
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the root to the island, and the island to the module.

\`\`\`
> connect root to modules with contains
node root connected to modules
> connect modules to addition with contains
node modules connected to addition
> list connections
root -[calculate]-> compute
root -[contains]-> modules
modules -[contains]-> addition
compute -[finish]-> end
\`\`\`

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-9
Graph exported to /tmp/graph/tutorial-9.json
Described in /api/graph/model/tutorial-9/359-15
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-9.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-9 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "a": 10,
    "b": 20
}'
\`\`\`

Summary
-------
In this tutorial, you have created a graph model with a compute node that executes a reusable
module. You have seen that EXECUTE runs the module's statements in the caller's context — the
result lands on the invoking node — and you have organized the module under an island so that no
node is left unconnected.
`,"../../../resources/help/help update.md":`Update a node
-------------
Replace the definition of an existing node. This multi-line command has the
same shape as 'create node' (see 'help create'): enter all lines as one
block, and the node takes the type and properties you provide.

Syntax
------
\`\`\`
update node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
\`\`\`

Example
-------
\`\`\`
update node greeting
with type Task
with properties
skill=graph.task
task=v1.hello.task
input[]=input.body -> *
output[]=result -> output.body
\`\`\`

Notes
-----
- Node names use lowercase letters, digits and hyphen ('root' and 'end' are
  reserved for the root and end nodes).
- Types are descriptive labels, conventionally Capitalized; the type and
  properties are validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional; a key[]=entry line
  appends one entry to the list "key"; wrap a multi-line value in triple
  single quotes ('''). Values may use the Event Script constant syntax.
- Tip: 'edit node {name}' prints an existing node as a ready-to-edit
  'update node' command (see 'help edit').
`,"../../../resources/help/help upload.md":`Upload mock data
----------------
Print a URL for uploading a JSON payload as the mock 'input.body' of the
current graph instance - convenient when the mock input is too large to seed
line by line.

Syntax
------
\`\`\`
upload mock data
\`\`\`

Example
-------
\`\`\`
> upload mock data
You may upload JSON payload -> POST /api/mock/{name}
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload to the given URL replaces the instance's
  'input.body'; the console confirms with "Mock data loaded into
  'input.body' namespace".
- Only 'input.body' can be uploaded. To mock input headers or model
  variables, seed them with the 'instantiate graph' command before
  uploading.
`,"../../../resources/help/help.md":`MiniGraph
---------
A mini-graph is a property graph designed to run entirely in memory
(default capacity: 750 nodes).

A graph model describes a business use case using graph methodology.
Optionally, you may give a node a special skill so it reacts to incoming
events. A skill is a property with the label "skill" whose value is a
composable function route name.

An instance model is an instance of a graph model used to process one
specific business use case or transaction. In the Playground you create it
with the "instantiate" command, optionally seeding mock input; in a deployed
application it is created when an incoming event arrives. Input data
attributes map to properties of one or more nodes.

Execution of an instance model starts from the root node and walks the graph
until it reaches the end node. The result of the end node is returned to the
calling party.

For a model to be meaningful, at least one node should have a skill to
process the data attributes of other nodes (the "data entities").

For more information about each feature, try the following help topics.

For graph model
---------------
- help create (node)
- help update (node)
- help edit (node)
- help delete (node, connection or cache)
- help connect (node-A to node-B)
- help list (nodes, connections, graphs, flows)
- help export (graph model as JSON for deployment)
- help import (graph or node)
- help describe (graph, node, connection or skill)
- help data-dictionary
- help session (display, subscribe or reset session)

For instance model
------------------
- help instantiate (create an instance from the current graph model)
- help upload (mock data)
- help execute (the skill of one node in isolation, for functional testing)
- help inspect (state machine: node properties, input, output and model namespaces)
- help run (traverse a graph instance from the root node to the end node)
- help seen (display the nodes that have been seen or executed)

Built-in skills
---------------
1. graph.data.mapper - map data from one node or namespace to another
2. graph.math - compute and branch with a fast built-in math/boolean expression engine
3. graph.js - retired in this port; use graph.math or graph.task instead
4. graph.api.fetcher - make API calls to other systems via Dictionary and Provider nodes
5. graph.extension - delegate to another graph model or an Event Script flow
6. graph.island - marks the knowledge layer; the node leads to isolated nodes and traversal pauses there
7. graph.join - wait for completion of all nodes that connect to it (parallel-branch barrier)
8. graph.task - invoke a composable function through its route name

For skill details, use the hyphenated help topics, e.g. 'help graph-math',
'help graph-api-fetcher', or 'describe skill {route}'.

Tutorials
---------
- help tutorial 1 (your first 'hello world' graph model)
- help tutorial 2 (deploying a graph model)
- help tutorial 3 (data dictionary, provider and API fetcher)
- help tutorial 4 (decision-making with math and boolean expressions)
- help tutorial 5 (parallel processing with a join barrier)
- help tutorial 6 (iterative API fetching with the 'for_each' keyword)
- help tutorial 7 (data mapping)
- help tutorial 8 (JSON-Path key-value retrieval and search)
- help tutorial 9 (reusable 'modules')
- help tutorial 10 (graph extension)
- help tutorial 11 (flow extension)
- help tutorial 12 (custom error handling)
- help tutorial 13 (invoking a composable function with the graph.task skill)
`});function ti(e){let t=e.split(`/`);return(t[t.length-1]??e).replace(/\.md$/,``)}var ni=Object.fromEntries(Object.entries(ei).map(([e,t])=>[ti(e),t]));function ri(e){return ni[e===``?`help`:`help ${e}`]??null}var ii=Object.keys(ni).filter(e=>e!==`help`).map(e=>e.replace(/^help\s+/,``)).sort(),ai=[{id:`overview`,label:`Overview`},{id:`graph-model`,label:`Graph Model`},{id:`graph-skills`,label:`Graph Skills`},{id:`instance-model`,label:`Instance Model`},{id:`tutorials`,label:`Tutorials`,chipStripLabel:`Chapters`}],oi=new Set([`execute`,`inspect`,`instantiate`,`run`,`seen`,`upload`]);function si(e){return e===``?`overview`:e.startsWith(`tutorial `)?`tutorials`:e.startsWith(`graph-`)?`graph-skills`:oi.has(e)?`instance-model`:`graph-model`}function ci(e){if(e===`overview`)return[``];let t=ii.filter(t=>si(t)===e);return e===`tutorials`?[...t].sort((e,t)=>parseInt(e.replace(/^tutorial\s+/,``),10)-parseInt(t.replace(/^tutorial\s+/,``),10)):t}function li(e,t){return e===``?`Overview`:t===`tutorials`?e.replace(/^tutorial\s+/,``):e}var ui=ai.flatMap(e=>ci(e.id));function di(e){return e.replace(/^help\s*/i,``).trim().toLowerCase()}function fi({bus:e,setHelpTopic:t,onTabSwitch:n}){let r=(0,D.useRef)(n);(0,D.useEffect)(()=>{r.current=n}),(0,D.useEffect)(()=>e.on(`command.helpOrDescribe`,e=>{if(!e.commandText.trim().toLowerCase().startsWith(`help`))return;let n=di(e.commandText);ri(n)!==null&&(t(n),r.current())}),[e,t])}function pi({ctx:e,navigate:t,addToast:n,wsPath:r}){let i=_r.find(e=>e.tabs.includes(`payload`)&&e.supportsUpload),a=(0,D.useRef)(null),o=i?.wsPath;(0,D.useEffect)(()=>{if(!(!o||!a.current)&&e.getSlot(o).phase===`connected`){let{wsPath:r,json:o}=a.current;a.current=null,e.setPendingPayload(r,o),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)}},[o,e,t,n,i]);let s=(0,D.useCallback)(r=>{if(!i)return;let o=e.getSlot(i.wsPath);o.phase===`connected`?(e.setPendingPayload(i.wsPath,r),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)):o.phase===`connecting`?(a.current={wsPath:i.wsPath,json:r},n(`Updated pending JSON transfer — latest payload will open when connected`,`info`)):(a.current={wsPath:i.wsPath,json:r},e.connect(i.wsPath,n),n(`Connecting to JSON-Path Playground…`,`info`))},[e,t,n,i]);return{handleSendToJsonPath:i&&r!==i.wsPath?s:void 0}}function mi({bus:e,onOpenModal:t,modalOpen:n}){let r=(0,D.useRef)(!1);(0,D.useEffect)(()=>{n||(r.current=!1)},[n]),(0,D.useEffect)(()=>e.on(`upload.invitation`,e=>{r.current||(r.current=!0,t(e.uploadPath))}),[e,t])}function hi({bus:e,addToast:t}){let[n,r]=(0,D.useState)(null),i=(0,D.useRef)(null),[a,o]=(0,D.useState)(new Set),s=(0,D.useCallback)(e=>{i.current=document.activeElement,r(e)},[]),c=(0,D.useCallback)(()=>{r(null),setTimeout(()=>i.current?.focus(),0)},[]),l=(0,D.useCallback)(e=>{o(e=>new Set([...e,n])),r(null),setTimeout(()=>i.current?.focus(),0),t(`Mock data uploaded successfully ✓`,`success`)},[n,t]),u=(0,D.useCallback)(e=>{t(`Upload failed: ${e}`,`error`)},[t]),d=(0,D.useCallback)(()=>{o(new Set)},[]);return mi({bus:e,onOpenModal:s,modalOpen:n!==null}),{modalUploadPath:n,successfulUploadPaths:a,handleOpenUploadModal:s,handleCloseUploadModal:c,handleUploadSuccess:l,handleUploadError:u,resetSuccessfulPaths:d}}function gi({bus:e,connected:t,appendMessage:n,addToast:r}){let i=(0,D.useRef)(null),a=(0,D.useRef)(!1),o=(0,D.useRef)(n);(0,D.useEffect)(()=>{o.current=n},[n]);let s=(0,D.useRef)(r);(0,D.useEffect)(()=>{s.current=r},[r]),(0,D.useEffect)(()=>{t||(i.current?.abort(),i.current=null,a.current=!1)},[t]),(0,D.useEffect)(()=>()=>{i.current?.abort()},[]),(0,D.useEffect)(()=>e.on(`payload.large`,e=>{if(a.current)return;let{apiPath:t,byteSize:n}=e;i.current?.abort();let r=new AbortController;i.current=r;let c=(n/(1024*1024)).toFixed(2);s.current(`Fetching large payload (${c} MB)…`,`info`),a.current=!0,fetch(t,{signal:r.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.text()}).then(e=>{if(!e.trim())throw Error(`empty response body`);let t=e;try{t=JSON.stringify(JSON.parse(e),null,2)}catch{}o.current(t),a.current=!1,i.current=null}).catch(e=>{e.name!==`AbortError`&&(a.current=!1,i.current=null,o.current(`ERROR: payload fetch failed — ${e.message}`),s.current(`Payload fetch failed: ${e.message}`,`error`))})}),[e])}function _i(e){let[t,n]=hr(e,{}),r=(0,D.useCallback)(e=>{n(t=>({...t,[e]:{name:e,savedAt:new Date().toISOString()}}))},[n]),i=(0,D.useCallback)(e=>{n(t=>{let n={...t};return delete n[e],n})},[n]),a=(0,D.useCallback)(e=>Object.prototype.hasOwnProperty.call(t,e),[t]);return{savedGraphs:(0,D.useMemo)(()=>Object.values(t).sort((e,t)=>new Date(t.savedAt).getTime()-new Date(e.savedAt).getTime()),[t]),saveGraph:r,deleteGraph:i,hasGraph:a}}function vi(e,t){let[n,r]=hr(e,1),i=(0,D.useRef)(!1),[a,o]=(0,D.useState)(null),[s,c]=(0,D.useState)(null);(0,D.useEffect)(()=>t.on(`command.importGraph`,e=>{o(e.graphName),c(null)}),[t]);let l=(0,D.useCallback)(e=>{c(e),e===`untitled-${n}`&&(i.current=!0)},[n]),u=(0,D.useCallback)(()=>{o(null),c(null),i.current&&r(e=>e+1),i.current=!1},[r]);return{defaultName:s??a??`untitled-${n}`,setLastSavedName:l,resetName:u}}function yi({bus:e,connected:t,sendRawText:n,saveGraph:r,setLastSavedName:i,addToast:a}){let o=(0,D.useRef)(null),s=(0,D.useCallback)(e=>{if(!t){a(`Save failed: connection required to export graph`,`error`);return}let r=setTimeout(()=>{o.current!==null&&(o.current=null,a(`Save failed: export confirmation timed out`,`error`))},1e4);o.current={graphName:e,timeoutId:r},n(`export graph as ${e}`)},[t,n,a]);return(0,D.useEffect)(()=>e.on(`graph.exported`,e=>{if(o.current===null||e.graphName!==o.current.graphName)return;clearTimeout(o.current.timeoutId);let t=o.current.graphName;o.current=null,r(t),i(t),a(`Graph saved as "${t}"`,`success`)}),[e,r,i,a]),(0,D.useEffect)(()=>e.on(`graph.export.failed`,e=>{o.current!==null&&(clearTimeout(o.current.timeoutId),o.current=null,e.reason===`invalid-name`?a(`Save failed: invalid filename (a–z, A–Z, 0–9, hyphen only)`,`error`):a(`Save failed: root node name does not match existing graph`,`error`))}),[e,a]),(0,D.useEffect)(()=>{!t&&o.current!==null&&(clearTimeout(o.current.timeoutId),o.current=null,a(`Save failed: connection closed before export confirmation`,`error`))},[t,a]),(0,D.useEffect)(()=>()=>{o.current!==null&&clearTimeout(o.current.timeoutId)},[]),{handleSaveGraph:s,handleLoadGraph:(0,D.useCallback)(e=>{t&&(n(`import graph from ${e}`),a(`Importing graph "${e}"…`,`info`))},[t,n,a])}}var bi=new Map;function xi(e){let[t,n]=(0,D.useState)(()=>bi.get(e)??null);return[t,(0,D.useCallback)(t=>{n(t),t===null?bi.delete(e):bi.set(e,t)},[e])]}function Si(e){if(e==null)return``;let t=typeof e==`string`?e:JSON.stringify(e);return t.includes(`'''`)&&console.warn(`[commandBuilder] Property value contains "'''" which cannot be escaped in the backend grammar. The value may be truncated on paste.`),t.includes(`
`)?`'''\n${t}\n'''`:t}function Ci(e,t){let n=[`${e} node ${t.alias}`];t.types.length>0&&n.push(`with type ${t.types[0]}`);let r=Object.entries(t.properties).filter(([,e])=>e!=null);if(r.length>0){n.push(`with properties`);for(let[e,t]of r)if(Array.isArray(t))for(let r of t)n.push(`${e}[]=${Si(r)}`);else n.push(`${e}[]=${Si(t)}`)}return n.join(`
`)}function wi(e,t){let n=t?.nodes.some(t=>t.alias===e.node.alias)?`update`:`create`;return{verb:n,command:Ci(n,e.node)}}function Ti(e){return{execute(t){return e(t)}}}var Ei={toastContainer:`_toastContainer_hhy5k_1`,toast:`_toast_hhy5k_1`,slideIn:`_slideIn_hhy5k_1`,success:`_success_hhy5k_36`,error:`_error_hhy5k_40`,info:`_info_hhy5k_44`,toastIcon:`_toastIcon_hhy5k_48`,toastMessage:`_toastMessage_hhy5k_53`},Di=({toasts:e,onRemove:t})=>e.length===0?null:(0,P.jsx)(`div`,{className:Ei.toastContainer,children:e.map(e=>(0,P.jsxs)(`div`,{className:`${Ei.toast} ${Ei[e.type]}`,onClick:()=>t(e.id),children:[(0,P.jsxs)(`span`,{className:Ei.toastIcon,children:[e.type===`success`&&`✅`,e.type===`error`&&`❌`,e.type===`info`&&`ℹ️`]}),(0,P.jsx)(`span`,{className:Ei.toastMessage,children:e.message})]},e.id))}),Oi={container:`_container_9dbh2_3`,trigger:`_trigger_9dbh2_7`,chevron:`_chevron_9dbh2_37`,chevronOpen:`_chevronOpen_9dbh2_43`,dot:`_dot_9dbh2_49`,dotIdle:`_dotIdle_9dbh2_56`,dotConnecting:`_dotConnecting_9dbh2_57`,pulse:`_pulse_9dbh2_1`,dotConnected:`_dotConnected_9dbh2_58`,dotPartial:`_dotPartial_9dbh2_59`,dropdown:`_dropdown_9dbh2_65`,fadeIn:`_fadeIn_9dbh2_1`};function ki({label:e,dotStatus:t,children:n}){let[r,i]=(0,D.useState)(!1),a=(0,D.useRef)(null);(0,D.useEffect)(()=>{if(!r)return;let e=e=>{a.current&&!a.current.contains(e.target)&&i(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[r]);let o=e=>{e.key===`Escape`&&(i(!1),a.current?.querySelector(`button[aria-haspopup]`)?.focus())},s=t===`connected`?Oi.dotConnected:t===`connecting`?Oi.dotConnecting:t===`partial`?Oi.dotPartial:t===`idle`?Oi.dotIdle:void 0;return(0,P.jsxs)(`div`,{className:Oi.container,ref:a,onKeyDown:o,children:[(0,P.jsxs)(`button`,{className:Oi.trigger,onClick:()=>i(e=>!e),"aria-haspopup":`true`,"aria-expanded":r,children:[t!==void 0&&(0,P.jsx)(`span`,{className:`${Oi.dot} ${s??``}`,"aria-hidden":`true`}),(0,P.jsx)(`span`,{children:e}),(0,P.jsx)(`span`,{className:`${Oi.chevron} ${r?Oi.chevronOpen:``}`,"aria-hidden":`true`,children:`▾`})]}),r&&(0,P.jsx)(`div`,{className:Oi.dropdown,role:`menu`,children:n})]})}var F={nav:`_nav_1hfby_3`,menuList:`_menuList_1hfby_11`,menuItem:`_menuItem_1hfby_19`,toolRow:`_toolRow_1hfby_56`,toolLink:`_toolLink_1hfby_67`,toolLinkActive:`_toolLinkActive_1hfby_92`,toolDot:`_toolDot_1hfby_99`,toolDotIdle:`_toolDotIdle_1hfby_106`,toolDotConnecting:`_toolDotConnecting_1hfby_107`,pulse:`_pulse_1hfby_1`,toolDotConnected:`_toolDotConnected_1hfby_108`,connectAllRow:`_connectAllRow_1hfby_112`,connectAllBtn:`_connectAllBtn_1hfby_118`,connectAllBtnStop:`_connectAllBtnStop_1hfby_142`,toolConnectBtn:`_toolConnectBtn_1hfby_154`,toolConnectBtnStop:`_toolConnectBtnStop_1hfby_180`,externalIcon:`_externalIcon_1hfby_192`};function Ai(e){return e.every(e=>e===`connected`)?`connected`:e.every(e=>e===`idle`)?`idle`:e.some(e=>e===`connecting`)?`connecting`:`partial`}function ji(e){return e===`connected`?`connected`:e===`connecting`?`connecting`:`idle`}var Mi=[{href:`/info`,label:`Info`},{href:`/info/lib`,label:`Libraries`},{href:`/info/routes`,label:`Services`},{href:`/health`,label:`Health`},{href:`/env`,label:`Environment`},{href:`http://localhost:8085/api/ws/json`,label:`Legacy JSON`},{href:`http://localhost:8085/api/ws/graph`,label:`Legacy Graph`}];function Ni({addToast:e}){let t=wr(),n=_r.map(e=>t.getSlot(e.wsPath).phase),r=Ai(n),i=n.every(e=>e===`connected`),a=n.some(e=>e===`connecting`);function o(){_r.forEach(n=>{t.getSlot(n.wsPath).phase===`idle`&&t.connect(n.wsPath,e)})}function s(){_r.forEach(e=>{let{phase:n}=t.getSlot(e.wsPath);(n===`connected`||n===`connecting`)&&t.disconnect(e.wsPath)})}return(0,P.jsxs)(`nav`,{className:F.nav,"aria-label":`Main navigation`,children:[(0,P.jsxs)(ki,{label:`Tools`,dotStatus:r,children:[(0,P.jsx)(`div`,{className:F.connectAllRow,children:(0,P.jsx)(`button`,{className:`${F.connectAllBtn} ${i?F.connectAllBtnStop:``}`,onClick:i?s:o,disabled:a,"aria-label":a?`Connecting…`:i?`Disconnect all WebSockets`:`Connect all WebSockets`,children:a?`Connecting…`:i?`Disconnect All`:`Connect All`})}),(0,P.jsx)(`ul`,{className:F.menuList,role:`none`,children:_r.map(n=>{let{phase:r}=t.getSlot(n.wsPath),i=ji(r),a=r===`connected`,o=r===`connecting`,s=i===`connected`?F.toolDotConnected:i===`connecting`?F.toolDotConnecting:F.toolDotIdle;return(0,P.jsxs)(`li`,{role:`none`,className:F.toolRow,children:[(0,P.jsxs)(Gn,{to:n.path,role:`menuitem`,className:({isActive:e})=>`${F.toolLink} ${e?F.toolLinkActive:``}`,children:[(0,P.jsx)(`span`,{className:`${F.toolDot} ${s}`,"aria-hidden":`true`}),(0,P.jsx)(`span`,{className:F.toolLabel,children:n.label})]}),(0,P.jsx)(`button`,{className:`${F.toolConnectBtn} ${a?F.toolConnectBtnStop:``}`,onClick:()=>a||o?t.disconnect(n.wsPath):t.connect(n.wsPath,e),disabled:o,"aria-label":o?`Connecting…`:a?`Disconnect ${n.label}`:`Connect ${n.label}`,title:o?`Connecting…`:yr(n.wsPath),children:o?`…`:a?`Stop`:`Start`})]},n.path)})})]}),(0,P.jsx)(ki,{label:`Quick Links`,children:(0,P.jsx)(`ul`,{className:F.menuList,role:`none`,children:Mi.map(e=>(0,P.jsx)(`li`,{role:`none`,children:(0,P.jsxs)(`a`,{href:e.href,role:`menuitem`,className:F.menuItem,target:`_blank`,rel:`noopener noreferrer`,children:[e.label,(0,P.jsx)(`span`,{className:F.externalIcon,"aria-hidden":`true`,children:`↗`})]})},e.href))})})]})}var Pi={saveBtn:`_saveBtn_1xd2l_3`,saveForm:`_saveForm_1xd2l_33`,saveInput:`_saveInput_1xd2l_39`,saveInputWarn:`_saveInputWarn_1xd2l_55`,saveWarnLabel:`_saveWarnLabel_1xd2l_59`,saveActionBtn:`_saveActionBtn_1xd2l_65`};function Fi({disabled:e,defaultName:t,onSave:n,nameExists:r,connected:i=!1}){let[a,o]=(0,D.useState)(!1),[s,c]=(0,D.useState)(``),l=(0,D.useRef)(null),u=(0,D.useCallback)(()=>{c(t),o(!0)},[t]),d=(0,D.useCallback)(()=>{o(!1),c(``)},[]),f=(0,D.useCallback)(()=>{let e=s.trim();e&&(n(e),o(!1),c(``))},[s,n]),p=(0,D.useCallback)(e=>{e.key===`Enter`&&(e.preventDefault(),f()),e.key===`Escape`&&(e.preventDefault(),d())},[f,d]);return(0,D.useEffect)(()=>{a&&l.current?.focus()},[a]),a?(0,P.jsxs)(`div`,{className:Pi.saveForm,children:[(0,P.jsx)(`input`,{ref:l,className:`${Pi.saveInput}${r?.(s.trim())?` ${Pi.saveInputWarn}`:``}`,type:`text`,value:s,onChange:e=>c(e.target.value),onKeyDown:p,placeholder:`Enter a name…`,"aria-label":`Graph save name`,maxLength:80}),r?.(s.trim())&&(0,P.jsx)(`span`,{className:Pi.saveWarnLabel,role:`status`,children:`Overwrite?`}),(0,P.jsx)(`button`,{className:Pi.saveActionBtn,onClick:f,disabled:!s.trim(),"aria-label":`Confirm save`,children:`✅`}),(0,P.jsx)(`button`,{className:Pi.saveActionBtn,onClick:d,"aria-label":`Cancel save`,children:`❌`})]}):(0,P.jsx)(`button`,{className:Pi.saveBtn,onClick:u,disabled:e||!i,title:e?`No graph loaded`:i?`Export graph snapshot to server and save bookmark`:`Connect first to save`,"aria-label":`Save graph snapshot`,children:`💾 Save Graph`})}var I={empty:`_empty_tpeii_3`,hint:`_hint_tpeii_12`,list:`_list_tpeii_21`,row:`_row_tpeii_31`,rowInfo:`_rowInfo_tpeii_50`,rowName:`_rowName_tpeii_58`,rowMeta:`_rowMeta_tpeii_67`,rowActions:`_rowActions_tpeii_78`,loadBtn:`_loadBtn_tpeii_84`,deleteBtn:`_deleteBtn_tpeii_85`};function L({savedGraphs:e,onLoad:t,onDelete:n,connected:r}){return(0,P.jsx)(ki,{label:e.length>0?`Load Graph (${e.length})`:`Load Graph`,children:e.length===0?(0,P.jsx)(`p`,{className:I.empty,children:`No saved graphs yet.`}):(0,P.jsxs)(P.Fragment,{children:[!r&&(0,P.jsx)(`p`,{className:I.hint,children:`Connect to load a graph`}),(0,P.jsx)(`ul`,{className:I.list,role:`list`,children:e.map(e=>(0,P.jsxs)(`li`,{className:I.row,children:[(0,P.jsxs)(`div`,{className:I.rowInfo,children:[(0,P.jsx)(`span`,{className:I.rowName,title:e.name,children:e.name}),(0,P.jsx)(`span`,{className:I.rowMeta,children:new Date(e.savedAt).toLocaleString()})]}),(0,P.jsxs)(`div`,{className:I.rowActions,children:[(0,P.jsx)(`button`,{className:I.loadBtn,onClick:()=>t(e.name),disabled:!r,title:r?`Run: import graph from ${e.name}`:`Connect to the playground first`,"aria-label":`Load graph ${e.name}`,children:`Load`}),(0,P.jsx)(`button`,{className:I.deleteBtn,onClick:()=>n(e.name),title:`Remove "${e.name}" from local storage`,"aria-label":`Delete saved graph ${e.name}`,children:`Delete`})]})]},e.name))})]})})}var R={payloadRoot:`_payloadRoot_6u47x_2`,labelRow:`_labelRow_6u47x_10`,label:`_label_6u47x_10`,payloadControls:`_payloadControls_6u47x_26`,charCounter:`_charCounter_6u47x_32`,typeIndicator:`_typeIndicator_6u47x_38`,validationIcon:`_validationIcon_6u47x_49`,formatButton:`_formatButton_6u47x_53`,uploadButton:`_uploadButton_6u47x_67`,textarea:`_textarea_6u47x_82`,textareaError:`_textareaError_6u47x_107`,errorMessage:`_errorMessage_6u47x_109`,sampleButtonsRow:`_sampleButtonsRow_6u47x_117`,sampleButtons:`_sampleButtons_6u47x_117`,sampleLabel:`_sampleLabel_6u47x_130`,sampleGroup:`_sampleGroup_6u47x_136`,sampleGroupLabel:`_sampleGroupLabel_6u47x_143`,sampleButton:`_sampleButton_6u47x_117`};function Ii({onLoad:e}){let t=Object.keys(vr).filter(e=>e.startsWith(`json_`)),n=Object.keys(vr).filter(e=>e.startsWith(`xml_`)),r=e=>e.replace(/^(json|xml)_/,``).replace(/_/g,` `);return(0,P.jsxs)(`div`,{className:R.sampleButtons,children:[(0,P.jsx)(`span`,{className:R.sampleLabel,children:`Quick load:`}),(0,P.jsxs)(`div`,{className:R.sampleGroup,children:[(0,P.jsx)(`span`,{className:R.sampleGroupLabel,children:`JSON:`}),t.map(t=>(0,P.jsx)(`button`,{className:R.sampleButton,onClick:()=>e(vr[t]),children:r(t)},t))]}),(0,P.jsxs)(`div`,{className:R.sampleGroup,children:[(0,P.jsx)(`span`,{className:R.sampleGroupLabel,children:`XML:`}),n.map(t=>(0,P.jsx)(`button`,{className:R.sampleButton,onClick:()=>e(vr[t]),children:r(t)},t))]})]})}function Li({payload:e,onChange:t,validation:n,onFormat:r,onUpload:i}){return(0,P.jsxs)(`div`,{className:R.payloadRoot,children:[(0,P.jsxs)(`div`,{className:R.labelRow,children:[(0,P.jsx)(`label`,{htmlFor:`payload`,className:R.label,children:`JSON/XML Payload`}),(0,P.jsxs)(`div`,{className:R.payloadControls,children:[(0,P.jsxs)(`span`,{className:R.charCounter,children:[`size: `,e.length]}),e&&n.type&&(0,P.jsx)(`span`,{className:R.typeIndicator,children:n.type.toUpperCase()}),e&&(0,P.jsx)(`span`,{className:R.validationIcon,children:n.valid?`✅`:`❌`}),(0,P.jsx)(`button`,{className:R.formatButton,onClick:r,disabled:!e||n.type!==`json`,title:n.type===`xml`?`Format only available for JSON`:`Format JSON`,children:`Format`}),i!==void 0&&(0,P.jsx)(`button`,{className:R.uploadButton,onClick:i,disabled:!e||!n.valid||n.type!==`json`,title:`Upload JSON payload to current session via REST`,children:`Upload`})]})]}),(0,P.jsx)(`textarea`,{id:`payload`,className:`${R.textarea} ${n.valid?``:R.textareaError}`,placeholder:`Paste your JSON/XML payload here`,value:e,onChange:e=>t(e.target.value)}),!n.valid&&(0,P.jsx)(`div`,{className:R.errorMessage,children:n.error}),(0,P.jsx)(`div`,{className:R.sampleButtonsRow,children:(0,P.jsx)(Ii,{onLoad:t})})]})}var Ri={Root:{icon:`🚀`,label:`Root`},End:{icon:`🏁`,label:`End`},Fetcher:{icon:`🌐`,label:`Fetcher`},mapper:{icon:`🗺️`,label:`Mapper`},Math:{icon:`🔢`,label:`Math`},JavaScript:{icon:`📜`,label:`JavaScript`},Provider:{icon:`🔌`,label:`Provider`},Dictionary:{icon:`📖`,label:`Dictionary`},Join:{icon:`🔀`,label:`Join`},Extension:{icon:`🧩`,label:`Extension`},Island:{icon:`🏝️`,label:`Island`},Decision:{icon:`❓`,label:`Decision`}},zi={boxSizing:`border-box`,borderRadius:`8px`,borderWidth:`1.5px`,borderStyle:`solid`,background:`var(--bg-secondary, #1e1e2e)`,color:`var(--text-primary, #cdd6f4)`,fontSize:`0.75rem`,boxShadow:`0 2px 8px rgba(0,0,0,0.45)`,overflow:`visible`,padding:0},Bi={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`},Vi=`#6c7086`;function Hi(e){return Ri[e]??{icon:`📦`,label:e}}function Ui(e){let t=Bi[e]??Vi;return{...zi,borderColor:t,"--node-accent":t}}var Wi={content:`_content_138ap_8`,header:`_header_138ap_22`,icon:`_icon_138ap_42`,alias:`_alias_138ap_47`,badge:`_badge_138ap_53`,body:`_body_138ap_65`,row:`_row_138ap_70`,label:`_label_138ap_83`,value:`_value_138ap_89`,edgeHandle:`_edgeHandle_138ap_103`};function Gi({label:e,value:t}){return(0,P.jsxs)(`div`,{className:Wi.row,children:[(0,P.jsx)(`span`,{className:Wi.label,children:e}),(0,P.jsx)(`span`,{className:Wi.value,title:t,children:t})]})}function Ki({properties:e}){let t=Object.entries(e).filter(([,e])=>e!=null);return t.length===0?null:(0,P.jsx)(P.Fragment,{children:t.map(([e,t])=>Array.isArray(t)?t.map((t,n)=>{let r=typeof t==`string`?t:JSON.stringify(t);return(0,P.jsx)(Gi,{label:n===0?e:``,value:r},`${e}-${n}`)}):(0,P.jsx)(Gi,{label:e,value:typeof t==`string`?t:JSON.stringify(t)},e))})}function qi({alias:e,nodeType:t,properties:n}){let r=Hi(t);return(0,P.jsx)(D.Fragment,{children:(0,P.jsxs)(`div`,{className:Wi.content,children:[(0,P.jsxs)(`div`,{className:Wi.header,children:[(0,P.jsx)(`span`,{className:Wi.icon,children:r.icon}),(0,P.jsx)(`span`,{className:Wi.alias,children:e}),(0,P.jsx)(`span`,{className:Wi.badge,children:r.label})]}),(0,P.jsx)(`div`,{className:Wi.body,children:(0,P.jsx)(Ki,{properties:n})})]})})}function Ji({data:e,isConnectable:t,selected:n}){return(0,P.jsxs)(P.Fragment,{children:[(0,P.jsx)(m,{minWidth:180,minHeight:e.minHeight,isVisible:n}),e.targetHandles.map(({id:e,offset:n})=>(0,P.jsx)(d,{id:e,type:`target`,position:l.Left,isConnectable:t,className:Wi.edgeHandle,style:{top:`calc(50% + ${n}px)`}},e)),e.backSourceHandles.map(({id:e,offset:n})=>(0,P.jsx)(d,{id:e,type:`source`,position:l.Left,isConnectable:t,className:Wi.edgeHandle,style:{top:`calc(50% + ${n}px)`}},e)),(0,P.jsx)(qi,{alias:e.alias,nodeType:e.nodeType,properties:e.properties}),e.sourceHandles.map(({id:e,offset:n})=>(0,P.jsx)(d,{id:e,type:`source`,position:l.Right,isConnectable:t,className:Wi.edgeHandle,style:{top:`calc(50% + ${n}px)`}},e)),e.backTargetHandles.map(({id:e,offset:n})=>(0,P.jsx)(d,{id:e,type:`target`,position:l.Right,isConnectable:t,className:Wi.edgeHandle,style:{top:`calc(50% + ${n}px)`}},e))]})}var Yi={Root:Ji,End:Ji,Fetcher:Ji,mapper:Ji,Math:Ji,JavaScript:Ji,Provider:Ji,Dictionary:Ji,Join:Ji,Extension:Ji,Island:Ji,Decision:Ji,default:Ji},Xi={graphWrapper:`_graphWrapper_zglpq_15`,graphSurface:`_graphSurface_zglpq_24`,empty:`_empty_zglpq_30`,emptyIcon:`_emptyIcon_zglpq_43`,emptyCreateButton:`_emptyCreateButton_zglpq_48`,emptyHint:`_emptyHint_zglpq_70`,refreshingOverlay:`_refreshingOverlay_zglpq_104`,clipboardDropOverlay:`_clipboardDropOverlay_zglpq_116`,clipboardDropMessage:`_clipboardDropMessage_zglpq_129`,refreshingSpinner:`_refreshingSpinner_zglpq_144`,graphRefreshSpin:`_graphRefreshSpin_zglpq_1`},Zi=class extends D.Component{constructor(...e){super(...e),this.state={caughtError:null}}static getDerivedStateFromError(e){return{caughtError:e instanceof Error?e.message:String(e)}}componentDidCatch(e,t){let n=e instanceof Error?e.message:String(e);console.error(`[GraphView] Render error:`,n,t.componentStack),this.props.onRenderError?.(`Graph render failed: ${n}`)}render(){return this.state.caughtError?(0,P.jsxs)(`div`,{className:Xi.empty,children:[(0,P.jsx)(`span`,{className:Xi.emptyIcon,children:`⚠️`}),(0,P.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,P.jsx)(`span`,{children:this.state.caughtError})]}):this.props.children}},Qi=240,$i=100,ea=60,ta=360,na=120,ra=80,ia=`rgba(148, 163, 184, 0.42)`,aa=`var(--bg-secondary)`,oa=24,sa=32,ca=[`#0369a1`,`#15803d`,`#b45309`,`#7e22ce`,`#b91c1c`,`#0f766e`,`#c2410c`,`#a16207`],la={fetch:`#0369a1`,details:`#0369a1`,"ext-call":`#0369a1`,mapping:`#b45309`,compute:`#b45309`,calculate:`#b45309`,evaluate:`#b45309`,fork:`#7e22ce`,join:`#7e22ce`,one:`#7e22ce`,two:`#6d28d9`,three:`#5b21b6`,more:`#4c1d95`,done:`#15803d`,complete:`#15803d`,finish:`#15803d`,positive:`#15803d`,negative:`#b91c1c`};function ua(e){let t=0;for(let n=0;n<e.length;n++)t=(t<<5)-t+e.charCodeAt(n),t|=0;return Math.abs(t)}function da(e){if(e.length===0)return ia;let t=e[0].trim().toLowerCase();return la[t]||ca[ua(t)%ca.length]}function fa(e){return`source-${e}`}function pa(e){return`target-${e}`}function ma(e){return`back-source-${e}`}function ha(e){return`back-target-${e}`}function ga(e,t){return t<=1?0:t===2?e===0?-24:oa:(e-(t-1)/2)*oa}function _a(e){return e<=1?$i:Math.max($i,(e-1)*oa+sa*2)}var va=new Set([`graph.math`,`graph.js`]),ya=[`Dictionary`,`Provider`,`Module`,`Entity`],ba={ROOT_TREE:0,DEFAULT_TREE:1,END_TREE:2};function xa(e){return e.alias.toLowerCase()===`root`||e.types.includes(`Root`)||e.types.includes(`entry_point`)}function Sa(e){return e.alias.toLowerCase()===`end`||e.types.includes(`End`)}function Ca(e){return e.hasRoot?ba.ROOT_TREE:e.hasEnd?ba.END_TREE:ba.DEFAULT_TREE}function wa(e,t){let n=Ca(e)-Ca(t);return n===0?e.sortKey.localeCompare(t.sortKey):n}function Ta(e,t){if(t.has(e.alias))return`flow`;let n=e.types[0]??``,r=typeof e.properties.skill==`string`?e.properties.skill:void 0;return n===`Dictionary`?`Dictionary`:n===`Provider`?`Provider`:r&&va.has(r)?`Module`:r?`__unknown__`:`Entity`}function Ea(e,t,n){let r=new Set;for(let e of t??[])r.add(e.source),r.add(e.target);let i=[],a=[],o=new Map;for(let t of e){let e=Ta(t,r);o.set(t.alias,e),e===`flow`?i.push(t):a.push(t)}let s=new Set(i.map(e=>e.alias)),c=new Map(i.map(e=>[e.alias,e])),l=new Map,u=new Map,d=new Map;for(let e of i)l.set(e.alias,[]),u.set(e.alias,new Set),d.set(e.alias,0);for(let e of t??[])!s.has(e.source)||!s.has(e.target)||(l.get(e.source)?.push(e.target),u.get(e.source)?.add(e.target),u.get(e.target)?.add(e.source),d.set(e.target,(d.get(e.target)??0)+1));let f=i.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||xa(e)).map(e=>e.alias),p=new Set;{let e=new Map;for(let t of i)e.set(t.alias,0);function t(t){if(e.get(t)!==0)return;e.set(t,1);let n=[{node:t,childIdx:0}];for(;n.length>0;){let t=n[n.length-1],r=l.get(t.node)??[];if(t.childIdx>=r.length){e.set(t.node,2),n.pop();continue}let i=r[t.childIdx++],a=e.get(i);a===1?p.add(`${t.node}\t${i}`):a===0&&(e.set(i,1),n.push({node:i,childIdx:0}))}}for(let e of f)t(e);for(let e of i)t(e.alias)}let m=[],h=new Set;for(let e of Array.from(s).sort()){if(h.has(e))continue;let t=[],n=[e];for(h.add(e);n.length>0;){let e=n.pop();t.push(e);for(let t of u.get(e)??[])h.has(t)||(h.add(t),n.push(t))}t.sort();let r=t.map(e=>c.get(e)).filter(e=>!!e);m.push({aliases:t,nodes:r,hasRoot:r.some(xa),hasEnd:r.some(Sa),sortKey:t[0]??``})}m.sort(wa);let g=new Map,_=new Map,v=0,y=0;for(let e of m){let t=new Set(e.aliases),r=e.nodes.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||xa(e)).map(e=>e.alias).sort();r.length===0&&e.aliases.length>0&&r.push(e.aliases[0]);let i=new Map,a=[...r];for(r.forEach(e=>i.set(e,0));a.length>0;){let e=a.shift(),n=i.get(e)??0;for(let r of l.get(e)??[])t.has(r)&&(p.has(`${e}\t${r}`)||(!i.has(r)||i.get(r)<=n)&&(i.set(r,n+1),a.push(r)))}let o=i.size>0?Math.max(...i.values()):0;for(let t of e.aliases)i.has(t)||i.set(t,o+1);let s=new Map;for(let[e,t]of i)s.has(t)||s.set(t,[]),s.get(t).push(e);let c=y;for(let[e,t]of[...s].sort(([e],[t])=>e-t)){let r=t.slice().sort(),i=-(r.reduce((e,t)=>e+(n.get(t)??$i),0)+Math.max(0,r.length-1)*ea)/2,a=v+e,o=y+e*360;c=Math.max(c,o),r.forEach(e=>{let t=n.get(e)??$i;g.set(e,a),_.set(e,{x:o,y:i}),i+=t+ea})}let u=i.size>0?Math.max(...i.values()):0;v+=u+1,y=c+Qi+ta}let b=0;for(let[e,t]of _)b=Math.max(b,t.y+(n.get(e)??$i));let x=b+(_.size>0?na:0),S=new Map;for(let e of ya)S.set(e,[]);S.set(`__unknown__`,[]);for(let e of a){let t=o.get(e.alias);S.get(t).push(e.alias)}for(let e of[...ya,`__unknown__`]){let t=(S.get(e)??[]).slice().sort();if(t.length===0)continue;let r=t.reduce((e,t)=>Math.max(e,n.get(t)??$i),0);t.forEach((e,t)=>{_.set(e,{x:0+t*360,y:x})}),x+=r+ra}return{positions:_,levelOf:g}}function Da(e){let t=e.connections??[],n=new Map,r=new Map;for(let e of t)n.set(e.source,(n.get(e.source)??0)+1),r.set(e.target,(r.get(e.target)??0)+1);let i=new Map(e.nodes.map(e=>[e.alias,_a(Math.max(n.get(e.alias)??0,r.get(e.alias)??0))])),{positions:a,levelOf:o}=Ea(e.nodes,t,i),s=new Set;for(let[e,n]of t.entries()){let t=o.get(n.source),r=o.get(n.target);t!==void 0&&r!==void 0&&t>=r&&s.add(e)}let c=new Map,l=new Map;for(let t of e.nodes)c.set(t.alias,[]),l.set(t.alias,[]);for(let[e,n]of t.entries())s.has(e)?(l.get(n.source).push({connIndex:e,peerAlias:n.target,isBack:!0}),c.get(n.target).push({connIndex:e,peerAlias:n.source,isBack:!0})):(c.get(n.source).push({connIndex:e,peerAlias:n.target,isBack:!1}),l.get(n.target).push({connIndex:e,peerAlias:n.source,isBack:!1}));let u=e=>a.get(e)?.y??0;for(let e of c.values())e.sort((e,t)=>u(e.peerAlias)-u(t.peerAlias));for(let e of l.values())e.sort((e,t)=>u(e.peerAlias)-u(t.peerAlias));let d=new Map,f=new Map,p=e.nodes.map(e=>{let t=c.get(e.alias)??[],n=l.get(e.alias)??[],r=_a(Math.max(t.length,n.length)),i=[],o=[],s=0,u=0;for(let e=0;e<t.length;e++){let n=t[e],r=ga(e,t.length);if(n.isBack){let e=ha(u++);o.push({id:e,offset:r}),f.set(n.connIndex,e)}else{let e=fa(s++);i.push({id:e,offset:r}),d.set(n.connIndex,e)}}let p=[],m=[],h=0,g=0;for(let e=0;e<n.length;e++){let t=n[e],r=ga(e,n.length);if(t.isBack){let e=ma(g++);m.push({id:e,offset:r}),d.set(t.connIndex,e)}else{let e=pa(h++);p.push({id:e,offset:r}),f.set(t.connIndex,e)}}return{id:e.alias,type:e.types[0]??`default`,position:a.get(e.alias)??{x:0,y:0},width:Qi,height:r,style:Ui(e.types[0]??`unknown`),data:{alias:e.alias,nodeType:e.types[0]??`unknown`,properties:e.properties,sourceHandles:i,targetHandles:p,backSourceHandles:m,backTargetHandles:o,minHeight:r}}}),m=[];for(let[e,n]of t.entries()){let t=n.relations.map(e=>e.type),r=`${n.source}__${n.target}__${e}`,i=da(t);m.push({id:r,source:n.source,target:n.target,sourceHandle:d.get(e),targetHandle:f.get(e),label:t.join(`, `),type:`bezier`,markerEnd:{type:v.ArrowClosed,width:16,height:16,color:ia},style:{stroke:ia,strokeWidth:2},labelStyle:{fill:i,fontSize:10,fontWeight:700},labelBgStyle:{fill:aa,fillOpacity:.94,stroke:`rgba(15, 23, 42, 0.16)`,strokeWidth:1},labelBgPadding:[5,2],labelBgBorderRadius:6,data:{relationTypes:t}})}return{nodes:p,edges:m}}var Oa=`application/x-minigraph-clipboard-item`;function ka(e){return e.includes(Oa)}function Aa(e,t){e.effectAllowed=`copy`,e.setData(Oa,t)}function ja(e){let t=e?.getData(`application/x-minigraph-clipboard-item`)??``;return t.trim()?t:null}function Ma(e,t){return e.nodes.find(e=>e.alias===t)}function Na(e,t){return(e.connections??[]).filter(e=>e.source!==e.target&&(e.source===t||e.target===t))}var Pa={toolbar:`_toolbar_117v8_2`,nameGroup:`_nameGroup_117v8_13`,graphName:`_graphName_117v8_20`,stats:`_stats_117v8_29`,toolbarActions:`_toolbarActions_117v8_49`,toolbarButton:`_toolbarButton_117v8_55`};function Fa({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i}){let a=(0,D.useCallback)(()=>{e&&navigator.clipboard.writeText(JSON.stringify(e,null,2)).then(()=>n?.()).catch(()=>r?.())},[e,n,r]),o=e?.nodes.length??0,s=(e?.connections??[]).length;return(0,P.jsxs)(`div`,{className:Pa.toolbar,children:[(0,P.jsxs)(`div`,{className:Pa.nameGroup,children:[(0,P.jsx)(`span`,{className:Pa.graphName,children:t??`Untitled`}),(0,P.jsxs)(`span`,{className:Pa.stats,children:[o,` node`,o===1?``:`s`,` · `,s,` connection`,s===1?``:`s`]})]}),(0,P.jsxs)(`div`,{className:Pa.toolbarActions,children:[i,(0,P.jsx)(`button`,{className:Pa.toolbarButton,onClick:a,title:`Copy raw graph JSON to clipboard`,"aria-label":`Copy raw graph JSON to clipboard`,children:`📑`})]})]})}var Ia={menu:`_menu_13qxg_1`,menuItem:`_menuItem_13qxg_12`};function La({open:e,x:t,y:n,canCreateNode:r,onCreateNode:i,onClose:a}){let o=(0,D.useRef)(null),s=(0,D.useRef)(null);return(0,D.useEffect)(()=>{if(!e)return;s.current?.focus();let t=e=>{o.current&&!o.current.contains(e.target)&&a()},n=e=>{e.key===`Escape`&&(e.preventDefault(),a())};return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n)}},[e,a]),e?(0,P.jsx)(`div`,{ref:o,className:Ia.menu,style:{left:t,top:n},role:`menu`,"aria-label":`Graph actions`,children:(0,P.jsx)(`button`,{ref:s,role:`menuitem`,type:`button`,className:Ia.menuItem,disabled:!r,onClick:()=>{r&&(i(),a())},children:`Create Node`})}):null}var Ra={menu:`_menu_1trgd_1`,menuItem:`_menuItem_1trgd_12`,dangerItem:`_dangerItem_1trgd_38`,confirmation:`_confirmation_1trgd_51`,confirmationText:`_confirmationText_1trgd_57`,confirmationActions:`_confirmationActions_1trgd_65`},za=8;function Ba({open:e,x:t,y:n,nodeAlias:r,canClipNode:i,canEditNode:a,canDeleteNode:o,onClipNode:s,onEditNode:c,onDeleteNode:l,onClose:u}){let[d,f]=(0,D.useState)(!1),[p,m]=(0,D.useState)({left:t,top:n}),h=(0,D.useRef)(null),g=(0,D.useRef)(null),_=(0,D.useRef)(null),v=i||a||o;return(0,D.useLayoutEffect)(()=>{e&&f(!1)},[r,e,t,n]),(0,D.useLayoutEffect)(()=>{if(!e)return;let r=h.current;if(!r){m({left:t,top:n});return}let i=r.getBoundingClientRect(),a=Math.max(za,window.innerWidth-i.width-za),o=Math.max(za,window.innerHeight-i.height-za);m({left:Math.min(Math.max(t,za),a),top:Math.min(Math.max(n,za),o)})},[i,o,a,d,r,e,t,n]),(0,D.useEffect)(()=>{if(!e){f(!1);return}d?_.current?.focus():g.current?.focus()},[d,e]),(0,D.useEffect)(()=>{if(!e)return;let t=e=>{h.current&&!h.current.contains(e.target)&&u()},n=e=>{e.key===`Escape`&&(e.preventDefault(),u())},r=()=>u();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[u,e]),!e||!v?null:(0,P.jsx)(`div`,{ref:h,className:Ra.menu,style:{left:p.left,top:p.top},role:`menu`,"aria-label":`Node actions for ${r}`,children:d?(0,P.jsxs)(`div`,{className:Ra.confirmation,role:`group`,"aria-label":`Confirm delete ${r}`,children:[(0,P.jsxs)(`div`,{className:Ra.confirmationText,children:[`Delete "`,r,`"?`]}),(0,P.jsxs)(`div`,{className:Ra.confirmationActions,children:[(0,P.jsx)(`button`,{ref:_,type:`button`,className:`${Ra.menuItem} ${Ra.dangerItem}`,onClick:()=>{l(),u()},children:`Delete`}),(0,P.jsx)(`button`,{type:`button`,className:Ra.menuItem,onClick:()=>f(!1),children:`Cancel`})]})]}):(0,P.jsxs)(P.Fragment,{children:[i&&(0,P.jsx)(`button`,{ref:g,role:`menuitem`,type:`button`,className:Ra.menuItem,onClick:()=>{s(),u()},children:`Clip to Workspace`}),a&&(0,P.jsx)(`button`,{ref:i?void 0:g,role:`menuitem`,type:`button`,className:Ra.menuItem,onClick:()=>{c(),u()},children:`Edit Node`}),o&&(0,P.jsx)(`button`,{ref:!i&&!a?g:void 0,role:`menuitem`,type:`button`,className:`${Ra.menuItem} ${Ra.dangerItem}`,onClick:()=>f(!0),children:`Delete Node`})]})})}var Va=[],Ha=[];function Ua({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,onRenderError:i,isRefreshing:a=!1,onClipNode:o,onClipboardDrop:l,isConnected:u,supportsAuthoring:d=!1,onCreateNode:m,onEditNode:v,onDeleteNode:y}){let[b,x]=(0,D.useState)(null),[S,C]=(0,D.useState)(null),[ee,te]=(0,D.useState)(!1),ne=(0,D.useRef)(0),w=!!(d&&m&&u),T=!!o,re=!!(d&&v&&u),E=!!(d&&y&&u),ie=T||re||E,ae=!!(l&&u),oe=(0,D.useCallback)(()=>{ne.current=0,te(!1)},[]);(0,D.useEffect)(()=>{if(!S)return;let e=e=>{e.key===`Escape`&&C(null)},t=()=>C(null);return document.addEventListener(`keydown`,e),window.addEventListener(`scroll`,t,!0),window.addEventListener(`resize`,t),()=>{document.removeEventListener(`keydown`,e),window.removeEventListener(`scroll`,t,!0),window.removeEventListener(`resize`,t)}},[S]),(0,D.useEffect)(()=>{let e=()=>oe();return window.addEventListener(`dragend`,e),window.addEventListener(`drop`,e),()=>{window.removeEventListener(`dragend`,e),window.removeEventListener(`drop`,e),oe()}},[oe]);let se=(0,D.useRef)(i);(0,D.useEffect)(()=>{se.current=i},[i]);let{nodes:O,edges:k,transformError:ce}=(0,D.useMemo)(()=>{if(!e)return{nodes:Va,edges:Ha,transformError:null};try{return{...Da(e),transformError:null}}catch(e){return{nodes:Va,edges:Ha,transformError:e instanceof Error?e.message:String(e)}}},[e]);(0,D.useEffect)(()=>{ce&&se.current?.(`Graph render failed: ${ce}`)},[ce]);let le=(0,D.useMemo)(()=>e?JSON.stringify(e.nodes.map(e=>e.alias)):`empty`,[e]),[ue,A,j]=f(O),[M,de,fe]=c(k);(0,D.useEffect)(()=>{A(O),de(k)},[O,k,A,de]);let pe=e=>{ae&&ka(Array.from(e.dataTransfer.types))&&(e.preventDefault(),ne.current+=1,te(!0))},me=e=>{ae&&ka(Array.from(e.dataTransfer.types))&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,te(!0))},he=e=>{ka(Array.from(e.dataTransfer.types))&&(ne.current=Math.max(0,ne.current-1),ne.current===0&&te(!1))},ge=e=>{if(!ae||!ka(Array.from(e.dataTransfer.types)))return;e.preventDefault();let t=ja(e.dataTransfer);oe(),t&&l?.(t)},_e=!!(e&&e.nodes.length>0),ve=b&&e?Ma(e,b.nodeAlias):null;return ce?(0,P.jsxs)(`div`,{className:Xi.empty,children:[(0,P.jsx)(`span`,{className:Xi.emptyIcon,children:`⚠️`}),(0,P.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,P.jsx)(`span`,{children:ce})]}):(0,P.jsx)(Zi,{onRenderError:i,children:(0,P.jsxs)(`div`,{className:Xi.graphWrapper,"aria-busy":a,children:[_e&&e&&(0,P.jsx)(Fa,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:r}),(0,P.jsxs)(`div`,{className:Xi.graphSurface,onDragEnter:pe,onDragOver:me,onDragLeave:he,onDrop:ge,children:[_e?(0,P.jsxs)(g,{nodes:ue,edges:M,onNodesChange:j,onEdgesChange:fe,nodeTypes:Yi,fitView:!0,fitViewOptions:{padding:.25},minZoom:.2,maxZoom:2.5,proOptions:{hideAttribution:!1},onNodeContextMenu:(e,t)=>{e.preventDefault(),e.stopPropagation(),C(null),ie&&x({x:e.clientX,y:e.clientY,nodeAlias:t.data.alias})},onPaneContextMenu:e=>{e.preventDefault(),w&&(x(null),C({x:e.clientX,y:e.clientY}))},onPaneClick:()=>{x(null),C(null)},children:[(0,P.jsx)(_,{variant:p.Dots,gap:18,size:1,color:`rgba(255,255,255,0.07)`}),(0,P.jsx)(h,{showInteractive:!1}),(0,P.jsx)(s,{nodeColor:e=>({Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`})[e.type??``]??`#6c7086`,maskColor:`rgba(0,0,0,0.3)`,style:{background:`#fff`}})]}):(0,P.jsxs)(`div`,{className:Xi.empty,children:[(0,P.jsx)(`span`,{className:Xi.emptyIcon,children:`🕸️`}),(0,P.jsx)(`span`,{children:`No graph data yet.`}),(0,P.jsxs)(`span`,{children:[`Run `,(0,P.jsx)(`strong`,{children:`describe graph`}),` or `,(0,P.jsx)(`strong`,{children:`export graph`}),` in the playground.`]}),d&&m&&(0,P.jsxs)(P.Fragment,{children:[(0,P.jsx)(`button`,{type:`button`,className:Xi.emptyCreateButton,disabled:!u,onClick:()=>m(`empty-graph`),children:`Create Node`}),!u&&(0,P.jsx)(`span`,{className:Xi.emptyHint,children:`Connect WebSocket to create a node.`})]})]}),a&&(0,P.jsx)(`div`,{className:Xi.refreshingOverlay,children:(0,P.jsx)(`div`,{className:Xi.refreshingSpinner,role:`status`,"aria-label":`Graph refreshing`})}),ee&&(0,P.jsx)(`div`,{className:Xi.clipboardDropOverlay,children:(0,P.jsx)(`div`,{className:Xi.clipboardDropMessage,children:`Drop to paste workspace node`})}),(0,P.jsx)(La,{open:S!==null,x:S?.x??0,y:S?.y??0,canCreateNode:w,onCreateNode:()=>m?.(`pane-context-menu`),onClose:()=>C(null)}),(0,P.jsx)(Ba,{open:b!==null&&ve!==null&&ie,x:b?.x??0,y:b?.y??0,nodeAlias:b?.nodeAlias??``,canClipNode:T&&ve!==null,canEditNode:re&&ve!==null,canDeleteNode:E&&ve!==null,onClipNode:()=>{if(!ve||!e)return;let t=Na(e,ve.alias);o?.(ve,t)},onEditNode:()=>{ve&&v?.(ve)},onDeleteNode:()=>{ve&&y?.(ve)},onClose:()=>x(null)})]})]})},le)}var Wa={root:`_root_1yhjs_2`,empty:`_empty_1yhjs_10`,emptyIcon:`_emptyIcon_1yhjs_23`,toolbarButton:`_toolbarButton_1yhjs_29 _toolbarButton_117v8_55`,scrollBody:`_scrollBody_1yhjs_34`,jsonContainer:`_jsonContainer_1yhjs_45`,jsonLabel:`_jsonLabel_1yhjs_46`,jsonString:`_jsonString_1yhjs_47`,jsonNumber:`_jsonNumber_1yhjs_48`,jsonBoolean:`_jsonBoolean_1yhjs_49`,jsonNull:`_jsonNull_1yhjs_50`},Ga={default:e=>e<3,all:i,none:a};function Ka({graphData:e,graphName:t,onCopySuccess:n,onCopyError:i}){let[a,s]=(0,D.useState)(`all`);return e?(0,P.jsxs)(`div`,{className:Wa.root,children:[(0,P.jsx)(Fa,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,extraActions:(0,P.jsxs)(P.Fragment,{children:[(0,P.jsx)(`button`,{className:Wa.toolbarButton,onClick:()=>s(`all`),title:`Expand all nodes`,"aria-label":`Expand all JSON nodes`,"aria-pressed":a===`all`,children:`➖`}),(0,P.jsx)(`button`,{className:Wa.toolbarButton,onClick:()=>s(`none`),title:`Collapse all nodes`,"aria-label":`Collapse all JSON nodes`,"aria-pressed":a===`none`,children:`➕`})]})}),(0,P.jsx)(`div`,{className:Wa.scrollBody,children:(0,P.jsx)(o,{data:e,shouldExpandNode:Ga[a],style:{...r,container:`${r.container} ${Wa.jsonContainer}`,label:Wa.jsonLabel,stringValue:Wa.jsonString,numberValue:Wa.jsonNumber,booleanValue:Wa.jsonBoolean,nullValue:Wa.jsonNull}})})]}):(0,P.jsx)(`div`,{className:Wa.root,children:(0,P.jsxs)(`div`,{className:Wa.empty,children:[(0,P.jsx)(`span`,{className:Wa.emptyIcon,children:`🕸️`}),(0,P.jsx)(`span`,{children:`No graph data yet.`}),(0,P.jsx)(`span`,{children:`Pin a graph-link message in the Console to load the raw data here.`})]})})}var qa={rightPanel:`_rightPanel_1xiht_2`,tabStrip:`_tabStrip_1xiht_10`,tab:`_tab_1xiht_10`,tabActive:`_tabActive_1xiht_38`,tabBadge:`_tabBadge_1xiht_42`,tabBody:`_tabBody_1xiht_48`,tabBodyHidden:`_tabBodyHidden_1xiht_57`,graphContent:`_graphContent_1xiht_61`,rightPanelGroup:`_rightPanelGroup_1xiht_68`,verticalResizeHandle:`_verticalResizeHandle_1xiht_76`},Ja=`help-split-percent`,Ya=`help-split-maximized`,Xa=45,Za=98;function Qa({tabs:e,payload:t,onChange:n,validation:r,onFormat:i,onUpload:a,graphData:o,graphName:s,activeTab:c,onTabChange:l,onGraphRenderError:u,onGraphDataCopySuccess:d,onGraphDataCopyError:f,isGraphRefreshing:p,onClipNode:m,onClipboardDrop:h,isConnected:g,supportsAuthoring:_,onCreateNode:v,onEditNode:y,onDeleteNode:b,helpPanel:x}){let ee=(0,D.useId)(),ne=`${ee}-tab-payload`,w=`${ee}-tab-graph`,T=`${ee}-tab-graph-data`,re=(0,P.jsxs)(`div`,{className:qa.rightPanel,children:[(0,P.jsxs)(`div`,{className:qa.tabStrip,role:`tablist`,"aria-label":`Right panel tabs`,children:[e.includes(`payload`)&&(0,P.jsx)(`button`,{role:`tab`,"aria-selected":c===`payload`,"aria-controls":ne,className:`${qa.tab}${c===`payload`?` ${qa.tabActive}`:``}`,onClick:()=>l(`payload`),children:`Payload Editor`}),e.includes(`graph`)&&(0,P.jsxs)(`button`,{role:`tab`,"aria-selected":c===`graph`,"aria-controls":w,className:`${qa.tab}${c===`graph`?` ${qa.tabActive}`:``}`,onClick:()=>l(`graph`),children:[`Graph`,o!==null&&(0,P.jsx)(`span`,{className:qa.tabBadge,"aria-label":`Graph data available`,children:`🕸️`})]}),e.includes(`graph-data`)&&(0,P.jsx)(`button`,{role:`tab`,"aria-selected":c===`graph-data`,"aria-controls":T,className:`${qa.tab}${c===`graph-data`?` ${qa.tabActive}`:``}`,onClick:()=>l(`graph-data`),children:`Graph Data (Raw)`})]}),e.includes(`payload`)&&(0,P.jsx)(`div`,{role:`tabpanel`,id:ne,tabIndex:c===`payload`?0:-1,className:`${qa.tabBody}${c===`payload`?``:` ${qa.tabBodyHidden}`}`,children:(0,P.jsx)(Li,{payload:t,onChange:n,validation:r,onFormat:i,onUpload:a})}),e.includes(`graph`)&&(0,P.jsx)(`div`,{role:`tabpanel`,id:w,tabIndex:c===`graph`?0:-1,className:`${qa.tabBody}${c===`graph`?``:` ${qa.tabBodyHidden}`}`,children:(0,P.jsx)(`div`,{className:qa.graphContent,children:(0,P.jsx)(Ua,{graphData:o,graphName:s,onRenderError:u,isRefreshing:p,onCopySuccess:d,onCopyError:f,onClipNode:m,onClipboardDrop:h,isConnected:g,supportsAuthoring:_,onCreateNode:v,onEditNode:y,onDeleteNode:b})})}),e.includes(`graph-data`)&&(0,P.jsx)(`div`,{role:`tabpanel`,id:T,tabIndex:c===`graph-data`?0:-1,className:`${qa.tabBody}${c===`graph-data`?``:` ${qa.tabBodyHidden}`}`,children:(0,P.jsx)(Ka,{graphData:o,graphName:s,onCopySuccess:d,onCopyError:f})})]}),E=(0,D.useRef)(Number(sessionStorage.getItem(Ja))||Xa),ie=(0,D.useRef)(null),ae=(0,D.useRef)(null),[oe,se]=(0,D.useState)(()=>sessionStorage.getItem(Ya)===`1`),O=(0,D.useRef)(oe),k=(0,D.useCallback)(e=>{let t=e[`help-split-help`];if(t===void 0)return;let n=t>=Za;n!==O.current&&(O.current=n,se(n),sessionStorage.setItem(Ya,n?`1`:`0`)),n||(E.current=t,sessionStorage.setItem(Ja,String(t)))},[]),ce=(0,D.useCallback)(()=>{let e=!O.current;if(O.current=e,se(e),sessionStorage.setItem(Ya,e?`1`:`0`),e)ae.current?.resize(`0%`),ie.current?.resize(`100%`);else{let e=E.current;ie.current?.resize(`${e}%`),ae.current?.resize(`${100-e}%`)}},[]),le=!!x;if((0,D.useEffect)(()=>{le&&O.current&&requestAnimationFrame(()=>{ae.current?.resize(`0%`),ie.current?.resize(`100%`)})},[le]),!x)return re;let ue=typeof x==`function`?x(ce,oe):x,A=O.current?100:E.current,j=100-A;return(0,P.jsxs)(te,{orientation:`vertical`,className:qa.rightPanelGroup,onLayoutChanged:k,children:[(0,P.jsx)(C,{panelRef:ae,defaultSize:`${j}%`,minSize:`0%`,children:re}),(0,P.jsx)(S,{className:qa.verticalResizeHandle,"aria-label":`Resize help panel`}),(0,P.jsx)(C,{id:`help-split-help`,panelRef:ie,defaultSize:`${A}%`,minSize:`15%`,children:ue})]})}var $a=class extends D.Component{constructor(...e){super(...e),this.state={hasError:!1}}static getDerivedStateFromError(){return{hasError:!0}}componentDidCatch(e,t){console.error(`[ConsoleErrorBoundary] Failed to render message:`,e,t.componentStack)}render(){return this.state.hasError?(0,P.jsx)(`span`,{children:this.props.fallback}):this.props.children}},eo=2e3,to=(e={})=>{let{onSuccess:t,onError:n}=e,[r,i]=(0,D.useState)(!1),a=(0,D.useRef)(null);return(0,D.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current)},[]),{copy:(0,D.useCallback)(async e=>{if(!navigator.clipboard)return console.warn(`useCopyToClipboard: Clipboard API not available in this browser.`),n?.(),!1;try{return await navigator.clipboard.writeText(e),i(!0),a.current!==null&&clearTimeout(a.current),a.current=setTimeout(()=>{a.current=null,i(!1)},eo),t?.(),!0}catch(e){return console.error(`useCopyToClipboard: Failed to write to clipboard.`,e),n?.(),!1}},[t,n]),copied:r}},z={consoleRoot:`_consoleRoot_1lgp1_2`,consoleHeader:`_consoleHeader_1lgp1_10`,consoleTitle:`_consoleTitle_1lgp1_20`,consoleControls:`_consoleControls_1lgp1_25`,controlButton:`_controlButton_1lgp1_30`,console:`_console_1lgp1_2`,emptyConsole:`_emptyConsole_1lgp1_67`,consoleMessage:`_consoleMessage_1lgp1_80`,consoleMessageActivatable:`_consoleMessageActivatable_1lgp1_94`,consoleMessageGraphLink:`_consoleMessageGraphLink_1lgp1_104`,consoleMessageLargePayload:`_consoleMessageLargePayload_1lgp1_115`,consoleMessageMockUpload:`_consoleMessageMockUpload_1lgp1_122`,uploadMockButton:`_uploadMockButton_1lgp1_131`,copyButton:`_copyButton_1lgp1_172`,copyButtonCopied:`_copyButtonCopied_1lgp1_225`,sendToJsonPathButton:`_sendToJsonPathButton_1lgp1_234`,messageIcon:`_messageIcon_1lgp1_268`,messageContent:`_messageContent_1lgp1_272`,messageText:`_messageText_1lgp1_278`,messageTime:`_messageTime_1lgp1_283`,"messageType-error":`_messageType-error_1lgp1_290`,"messageType-info":`_messageType-info_1lgp1_291`,"messageType-welcome":`_messageType-welcome_1lgp1_292`,jsonViewWrapper:`_jsonViewWrapper_1lgp1_295`,jsonContainer:`_jsonContainer_1lgp1_301`,jsonLabel:`_jsonLabel_1lgp1_302`,jsonString:`_jsonString_1lgp1_303`,jsonNumber:`_jsonNumber_1lgp1_304`,jsonBoolean:`_jsonBoolean_1lgp1_305`,jsonNull:`_jsonNull_1lgp1_306`};function no({message:e,msgId:t,classificationMap:n,onGraphLink:i,onCopyMessage:a,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){let u=Tr(e),d=Er(u.type),f=Dr(u.message),p=(t===void 0?void 0:n?.get(t))??[],m=p.some(e=>e.kind===`graph.link`),h=p.some(e=>e.kind===`payload.large`),g=p.some(e=>e.kind===`upload.invitation`),_=p.find(e=>e.kind===`upload.invitation`)?.uploadPath??null,v=!!c&&g&&_!==null,y=v&&!!l?.has(_),b=!!i&&m&&!g&&!h,x=!!s&&f.isJSON,{copy:S,copied:C}=to({onSuccess:a}),ee=t=>{t.stopPropagation(),S(e)},te=t=>{(t.key===`Enter`||t.key===` `)&&(t.preventDefault(),t.stopPropagation(),S(e))},ne=e=>{e.stopPropagation(),!(!s||!f.isJSON)&&s(JSON.stringify(f.data,null,2))},w=e=>{e.stopPropagation(),!(!c||!_)&&c(_)};return(0,P.jsxs)(`div`,{className:[z.consoleMessage,z[`messageType-${u.type}`],b?z.consoleMessageActivatable:``,m?z.consoleMessageGraphLink:``,h?z.consoleMessageLargePayload:``,g?z.consoleMessageMockUpload:``].filter(Boolean).join(` `),onClick:b?()=>i():void 0,title:b?`Click to load graph in Graph View`:void 0,role:b?`button`:void 0,tabIndex:b?0:void 0,onKeyDown:b?e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),i())}:void 0,"aria-label":b?`Load graph in Graph View`:void 0,children:[(0,P.jsx)(`span`,{className:z.messageIcon,children:g?`⬆️`:h?`⬇️`:m?`🕸️`:d}),(0,P.jsx)(`div`,{className:z.messageContent,children:f.isJSON?(0,P.jsx)(`div`,{className:z.jsonViewWrapper,children:(0,P.jsx)(o,{data:f.data,shouldExpandNode:e=>e<1,style:{...r,container:`${r.container} ${z.jsonContainer}`,label:z.jsonLabel,stringValue:z.jsonString,numberValue:z.jsonNumber,booleanValue:z.jsonBoolean,nullValue:z.jsonNull}})}):(0,P.jsxs)(`span`,{className:z.messageText,children:[u.message,y&&(0,P.jsx)(`span`,{title:`Upload succeeded`,children:` ✅`})]})}),(0,P.jsx)(`button`,{className:`${z.copyButton} ${C?z.copyButtonCopied:``}`,onClick:ee,onKeyDown:te,title:C?`Copied!`:`Copy message`,"aria-label":C?`Copied to clipboard`:`Copy message to clipboard`,tabIndex:0,children:C?`✅`:`📄`}),x&&(0,P.jsx)(`button`,{className:z.sendToJsonPathButton,onClick:ne,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&ne(e)},title:`Open in JSON-Path Playground`,"aria-label":`Open this JSON in the JSON-Path Playground`,tabIndex:0,children:`➡️`}),v&&(0,P.jsx)(`button`,{className:z.uploadMockButton,onClick:w,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&w(e)},title:`Re-open upload dialog`,"aria-label":`Re-open mock data upload dialog`,tabIndex:0,children:`⬆️ Upload JSON…`}),u.time&&(0,P.jsx)(`span`,{className:z.messageTime,children:u.time})]})}function ro({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){return(0,P.jsxs)(`div`,{className:z.consoleRoot,children:[(0,P.jsxs)(`div`,{className:z.consoleHeader,children:[(0,P.jsx)(`span`,{className:z.consoleTitle,children:`Console Output`}),(0,P.jsxs)(`div`,{className:z.consoleControls,children:[(0,P.jsx)(`button`,{className:z.controlButton,onClick:n,title:`Copy console output`,"aria-label":`Copy console output to clipboard`,children:`📑`}),(0,P.jsx)(`button`,{className:z.controlButton,onClick:r,title:`Clear console`,"aria-label":`Clear console`,children:`🗑️`})]})]}),(0,P.jsxs)(`div`,{className:z.console,ref:i,role:`log`,"aria-live":`polite`,children:[e.map(e=>(0,P.jsx)($a,{fallback:e.raw,children:(0,P.jsx)(no,{message:e.raw,msgId:e.id,classificationMap:t,onGraphLink:a?()=>a(e):void 0,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l})},e.id)),e.length===0&&(0,P.jsxs)(`div`,{className:z.emptyConsole,children:[`No messages yet. Use the `,(0,P.jsx)(`strong`,{children:`Start`}),` button in the header to connect.`]})]})]})}var B={commandInput:`_commandInput_j85f1_2`,labelRow:`_labelRow_j85f1_8`,labelGroup:`_labelGroup_j85f1_16`,label:`_label_j85f1_8`,infoWrapper:`_infoWrapper_j85f1_28`,paletteToggle:`_paletteToggle_j85f1_34`,paletteToggleActive:`_paletteToggleActive_j85f1_66`,popover:`_popover_j85f1_73`,popoverOpen:`_popoverOpen_j85f1_95`,popoverTitle:`_popoverTitle_j85f1_121`,popoverRow:`_popoverRow_j85f1_135`,popoverKeyword:`_popoverKeyword_j85f1_156`,popoverDesc:`_popoverDesc_j85f1_168`,popoverAlias:`_popoverAlias_j85f1_174`,inputRow:`_inputRow_j85f1_181`,inputWrapper:`_inputWrapper_j85f1_187`,textarea:`_textarea_j85f1_197`,sendButton:`_sendButton_j85f1_226`,hint:`_hint_j85f1_243`,dropup:`_dropup_j85f1_251`,dropupHeader:`_dropupHeader_j85f1_266`,dropupItem:`_dropupItem_j85f1_282`,dropupItemText:`_dropupItemText_j85f1_305`,matchHighlight:`_matchHighlight_j85f1_313`,multilineIndicator:`_multilineIndicator_j85f1_319`},io=[`graph.data.mapper`,`graph.math`,`graph.js`,`graph.api.fetcher`,`graph.extension`,`graph.island`,`graph.join`],ao=[{keyword:`help`,description:`List all help topics, or get help for a specific command`,template:`help`},{keyword:`create`,description:`Create a new graph node`,template:`create node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`update`,description:`Update an existing node`,template:`update node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`edit`,description:`Print raw node data ready for editing and re-submitting`,template:`edit node {name}`},{keyword:`delete node`,description:`Delete a node by name`,alias:`clear node`,template:`delete node {name}`},{keyword:`delete connection`,description:`Delete connection(s) between two nodes`,alias:`clear connection`,template:`delete connection {nodeA} and {nodeB}`},{keyword:`delete cache`,description:`Clear cached API fetcher results`,alias:`clear cache`,template:`delete cache`},{keyword:`connect`,description:`Connect two nodes with a named relation`,template:`connect {node-A} to {node-B} with {relation}`},{keyword:`list nodes`,description:`List all nodes in the current graph`,template:`list nodes`},{keyword:`list connections`,description:`List all connections in the current graph`,template:`list connections`},{keyword:`describe graph`,description:`Describe the current graph model`,template:`describe graph`},{keyword:`describe node`,description:`Describe a specific node and its connections`,template:`describe node {name}`},{keyword:`describe connection`,description:`Describe connection(s) between two nodes`,template:`describe connection {nodeA} and {nodeB}`},{keyword:`describe skill`,description:`Show documentation for a skill by route name`,template:`describe skill {skill.route}`},{keyword:`export`,description:`Export the graph model to a JSON file`,template:`export graph as {name}`},{keyword:`import graph`,description:`Import a graph model from a saved file`,template:`import graph from {name}`},{keyword:`import node`,description:`Import a single node from another saved graph`,template:`import node {node-name} from {graph-name}`},{keyword:`instantiate`,description:`Create a runnable graph instance with mock input`,alias:`start`,template:`instantiate graph
{constant} -> input.body.{key}`,multiline:!0},{keyword:`upload mock data`,description:`Print the URL to POST a JSON payload as mock input.body`,template:`upload mock data`},{keyword:`execute`,description:`Execute a single node skill in isolation`,template:`execute node {name}`},{keyword:`inspect`,description:`Inspect a state-machine variable`,template:`inspect {variable_name}`},{keyword:`run`,description:`Run the graph instance from root to end`,template:`run`}];[...io.map(e=>({tokens:[`describe`,`skill`,e],template:`describe skill ${e}`,hint:`Describe built-in skill: ${e}`}))];function oo(e,t){let[n,r]=(0,D.useState)(!1),[i,a]=(0,D.useState)(-1),o=(0,D.useMemo)(()=>{let n=t.trimStart();if(n.length===0)return[];let r=n.toLowerCase(),i=e.filter(e=>e.toLowerCase().startsWith(r)),a=new Set;return i.filter(e=>a.has(e)?!1:(a.add(e),!0)).slice(0,8)},[e,t]),s=()=>{r(!0),a(-1)},c=e=>{let t=o.length;t!==0&&a(n=>e===1?n<0?0:(n+1)%t:n<=0?t-1:n-1)},l=(e,t)=>{e>=0&&e<o.length&&t(o[e]),r(!1),a(-1)};return{suggestions:o,isOpen:n,activeIndex:i,onCommandChange:s,navigate:c,accept:l,onTab:e=>{!n||o.length===0||l(i>=0?i:0,e)},dismiss:()=>{r(!1),a(-1)}}}var so=e=>(0,P.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:14,height:14,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,P.jsx)(`polyline`,{points:`2,4 6,8 2,12`}),(0,P.jsx)(`line`,{x1:7,y1:12,x2:14,y2:12})]});function co({command:e,onChange:t,onKeyDown:n,onSend:r,sendDisabled:i,disabled:a,history:o}){let s=(0,D.useRef)(null),c=(0,D.useRef)(null),l=(0,D.useRef)(null),[u,d]=(0,D.useState)(!1);(0,D.useEffect)(()=>{if(!u)return;let e=e=>{c.current&&!c.current.contains(e.target)&&d(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[u]);let f=oo(o,e);(0,D.useEffect)(()=>{let e=s.current;e&&(e.style.height=`auto`,e.style.height=`${e.scrollHeight}px`)},[e]);let p=a?`Not connected`:`Enter command (Enter to send · Shift+Enter for new line)`,m=a?`Enter your test message once it is connected`:`Enter to send · Shift+Enter for new line · ↑↓ for history`;return(0,P.jsxs)(`div`,{className:B.commandInput,children:[(0,P.jsx)(`div`,{className:B.labelRow,children:(0,P.jsxs)(`div`,{className:B.labelGroup,children:[(0,P.jsx)(`label`,{htmlFor:`command`,className:B.label,children:`Command`}),(0,P.jsxs)(`span`,{ref:c,className:B.infoWrapper,children:[(0,P.jsx)(`button`,{type:`button`,className:`${B.paletteToggle}${u?` ${B.paletteToggleActive}`:``}`,"aria-label":`Toggle command palette`,"aria-expanded":u,"aria-controls":`command-palette`,onClick:()=>d(e=>!e),onKeyDown:e=>{e.key===`ArrowDown`&&u&&(e.preventDefault(),(l.current?.querySelector(`[role="option"]`))?.focus())},title:`Command palette`,children:(0,P.jsx)(so,{"aria-hidden":`true`,focusable:`false`})}),(0,P.jsxs)(`div`,{id:`command-palette`,ref:l,className:`${B.popover}${u?` ${B.popoverOpen}`:``}`,role:`listbox`,"aria-label":`Command palette`,onKeyDown:e=>{if(e.key===`ArrowDown`||e.key===`ArrowUp`){e.preventDefault();let t=l.current?.querySelectorAll(`[role="option"]`);if(!t||t.length===0)return;let n=Array.from(t).indexOf(document.activeElement);e.key===`ArrowDown`?t[n<0?0:(n+1)%t.length].focus():t[n<=0?t.length-1:n-1].focus()}else e.key===`Escape`&&(e.preventDefault(),d(!1),s.current?.focus())},children:[(0,P.jsx)(`p`,{className:B.popoverTitle,children:`Command palette — click to insert`}),ao.map(({keyword:e,alias:n,description:r,template:i})=>(0,P.jsxs)(`div`,{className:B.popoverRow,role:`option`,"aria-selected":!1,tabIndex:u?0:-1,onMouseDown:e=>e.preventDefault(),onClick:()=>{t(i),d(!1),s.current?.focus()},onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),t(i),d(!1),s.current?.focus())},children:[(0,P.jsx)(`span`,{className:B.popoverKeyword,children:e}),(0,P.jsxs)(`span`,{className:B.popoverDesc,children:[r,n&&(0,P.jsxs)(`span`,{className:B.popoverAlias,children:[` · alias: `,n]})]})]},e))]})]})]})}),(0,P.jsxs)(`div`,{className:B.inputRow,children:[(0,P.jsxs)(`div`,{className:B.inputWrapper,children:[(0,P.jsxs)(`div`,{id:`history-dropup`,role:`listbox`,"aria-label":`Command history suggestions`,className:B.dropup,hidden:!(f.isOpen&&f.suggestions.length>0),children:[(0,P.jsx)(`div`,{className:B.dropupHeader,"aria-hidden":`true`,children:`Recent Commands`}),f.isOpen&&f.suggestions.length>0&&f.suggestions.map((n,r)=>{let i=n.split(`
`)[0],a=n.includes(`
`),o=e.trimStart().split(`
`)[0],c=Math.min(o.length,i.length),l=i.slice(0,c),u=i.slice(c);return(0,P.jsxs)(`div`,{id:`history-option-${r}`,role:`option`,"aria-selected":r===f.activeIndex,className:B.dropupItem,onMouseDown:e=>e.preventDefault(),onClick:()=>{f.accept(r,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)})},children:[(0,P.jsxs)(`span`,{className:B.dropupItemText,children:[c>0&&(0,P.jsx)(`strong`,{className:B.matchHighlight,children:l}),u,a?`…`:``]}),a&&(0,P.jsx)(`span`,{className:B.multilineIndicator,"aria-label":`multi-line command`,children:`↵`})]},n)})]}),(0,P.jsx)(`textarea`,{ref:s,id:`command`,role:`combobox`,"aria-expanded":f.isOpen&&f.suggestions.length>0,"aria-haspopup":`listbox`,"aria-controls":`history-dropup`,"aria-activedescendant":f.isOpen&&f.suggestions.length>0&&f.activeIndex>=0?`history-option-${f.activeIndex}`:void 0,"aria-autocomplete":`list`,className:B.textarea,rows:1,placeholder:p,value:e,disabled:a,onChange:e=>{t(e.target.value),f.onCommandChange()},onKeyDown:e=>{if(e.key===`Tab`){e.preventDefault(),f.isOpen&&f.suggestions.length>0&&(f.onTab(e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}));return}if(e.key===`Enter`){if(e.shiftKey)return;if(e.preventDefault(),f.isOpen&&f.activeIndex>=0){f.accept(f.activeIndex,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}),s.current?.focus();return}r(),s.current?.focus();return}if(e.key===`Escape`){if(f.isOpen){f.dismiss(),e.preventDefault();return}return}if(e.key===`ArrowUp`||e.key===`ArrowDown`){if(f.isOpen&&f.suggestions.length>0){e.preventDefault(),f.navigate(e.key===`ArrowDown`?1:-1);return}let t=s.current;if(t){let{selectionStart:n,value:r}=t,i=!r.slice(0,n).includes(`
`),a=!r.slice(n).includes(`
`);if(!(e.key===`ArrowUp`&&i||e.key===`ArrowDown`&&a))return}n(e),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)});return}n(e)},onBlur:()=>f.dismiss(),autoComplete:`off`,autoCorrect:`off`,spellCheck:!1})]}),(0,P.jsx)(`button`,{className:B.sendButton,onClick:()=>{r(),s.current?.focus()},disabled:i,"aria-label":`Send command`,children:`Send`})]}),m&&(0,P.jsx)(`p`,{className:B.hint,children:m})]})}var lo={root:`_root_1ac49_1`};function uo({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l,command:u,onCommandChange:d,onCommandKeyDown:f,onSend:p,sendDisabled:m,inputDisabled:h,commandHistory:g}){return(0,P.jsxs)(`div`,{className:lo.root,children:[(0,P.jsx)(ro,{messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}),(0,P.jsx)(co,{command:u,onChange:d,onKeyDown:f,onSend:p,disabled:h,sendDisabled:m,history:g})]})}var V={dialog:`_dialog_g80bk_4`,modalInner:`_modalInner_g80bk_26`,modalHeader:`_modalHeader_g80bk_34`,modalTitleGroup:`_modalTitleGroup_g80bk_44`,modalTitle:`_modalTitle_g80bk_44`,modalPath:`_modalPath_g80bk_57`,closeButton:`_closeButton_g80bk_64`,modalBody:`_modalBody_g80bk_95`,dropZone:`_dropZone_g80bk_105`,dropZoneActive:`_dropZoneActive_g80bk_127`,dropZoneIcon:`_dropZoneIcon_g80bk_133`,dropZoneText:`_dropZoneText_g80bk_139`,dropZoneOr:`_dropZoneOr_g80bk_152`,browseButton:`_browseButton_g80bk_159`,fileInputHidden:`_fileInputHidden_g80bk_188`,fileError:`_fileError_g80bk_193`,textareaLabel:`_textareaLabel_g80bk_198`,textarea:`_textarea_g80bk_198`,validationError:`_validationError_g80bk_226`,keyboardHint:`_keyboardHint_g80bk_231`,errorBanner:`_errorBanner_g80bk_236`,modalFooter:`_modalFooter_g80bk_247`,footerActions:`_footerActions_g80bk_257`,formatButton:`_formatButton_g80bk_263`,cancelButton:`_cancelButton_g80bk_264`,uploadButton:`_uploadButton_g80bk_265`,spinner:`_spinner_g80bk_332`,spin:`_spin_g80bk_332`};function H({uploadPath:e,json:t,onSuccess:n,onError:r}){let[i,a]=(0,D.useState)(!1),o=(0,D.useRef)(null),s=(0,D.useCallback)(()=>{o.current?.abort(),o.current=null,a(!1)},[]);return{isUploading:i,upload:(0,D.useCallback)(async()=>{o.current?.abort();let i=new AbortController;o.current=i,a(!0);try{let o=await fetch(e,{method:`POST`,headers:{"Content-Type":`application/json`},body:t,signal:i.signal}),s=await o.text();if(!o.ok){a(!1),r(`HTTP ${o.status} — ${s}`);return}a(!1),n(s)}catch(e){if(e.name===`AbortError`){a(!1);return}a(!1),r(e.message??`Network error`)}},[e,t,n,r]),cancel:s}}var U=(navigator.userAgentData?.platform??navigator.platform).toLowerCase().includes(`mac`);function fo(e){return new Promise((t,n)=>{let r=new FileReader;r.onload=()=>t(r.result),r.onerror=()=>n(Error(`Could not read file "${e.name}"`)),r.readAsText(e,`utf-8`)})}function po(e){let t=e.name.toLowerCase().endsWith(`.json`),n=e.type===`application/json`||e.type===`text/plain`;return!t&&!n?`"${e.name}" does not appear to be a JSON file. Only .json files are accepted.`:null}function mo({uploadPath:e,onSuccess:t,onClose:n,onError:r}){let[i,a]=(0,D.useState)(``),[o,s]=(0,D.useState)(null),[c,l]=(0,D.useState)(null),[u,d]=(0,D.useState)(!1),f=(0,D.useRef)(null),p=(0,D.useRef)(null),m=(0,D.useRef)(null),h=Dr(i).isJSON,g=h&&i.trim()!==``,{isUploading:_,upload:v,cancel:y}=H({uploadPath:e,json:i,onSuccess:t,onError:e=>{s(e),r(e)}});(0,D.useEffect)(()=>{let e=f.current;if(e)return e.open||e.showModal(),p.current?.focus(),()=>{e.open&&e.close()}},[]);let b=(0,D.useCallback)(()=>{y(),n()},[y,n]),x=(0,D.useCallback)(e=>{e.target===f.current&&b()},[b]),S=(0,D.useCallback)(e=>{e.preventDefault(),b()},[b]),C=(0,D.useCallback)(()=>{s(null),v()},[v]),ee=(0,D.useCallback)(e=>{e.key===`Enter`&&(e.ctrlKey||e.metaKey)&&(e.preventDefault(),g&&!_&&C())},[g,_,C]),te=(0,D.useCallback)(()=>{h&&a(pr(i))},[h,i]),ne=(0,D.useCallback)(async e=>{l(null),s(null);let t=po(e);if(t){l(t);return}try{let t=await fo(e);if(!Dr(t).isJSON){l(`"${e.name}" contains invalid JSON.`);return}a(pr(t)),p.current?.focus()}catch(e){l(e.message)}},[]),w=(0,D.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),u||d(!0)},[u]),T=(0,D.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&d(!1)},[]),re=(0,D.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),d(!1);let t=e.dataTransfer.files[0];t&&ne(t)},[ne]),E=(0,D.useCallback)(e=>{let t=e.target.files?.[0];t&&(ne(t),e.target.value=``)},[ne]),ie=!h&&i.trim()!==``;return(0,P.jsx)(`dialog`,{ref:f,className:V.dialog,"aria-modal":`true`,"aria-labelledby":`mock-upload-modal-title`,onClick:x,onCancel:S,children:(0,P.jsxs)(`div`,{className:V.modalInner,onClick:e=>e.stopPropagation(),children:[(0,P.jsxs)(`div`,{className:V.modalHeader,children:[(0,P.jsxs)(`div`,{className:V.modalTitleGroup,children:[(0,P.jsx)(`span`,{id:`mock-upload-modal-title`,className:V.modalTitle,children:`⬆️ Upload Mock Data`}),(0,P.jsx)(`span`,{className:V.modalPath,children:e})]}),(0,P.jsx)(`button`,{className:V.closeButton,onClick:b,"aria-label":`Close upload modal`,title:`Close`,disabled:_,children:`✕`})]}),(0,P.jsxs)(`div`,{className:V.modalBody,children:[(0,P.jsxs)(`div`,{className:`${V.dropZone} ${u?V.dropZoneActive:``}`,onDragOver:w,onDragLeave:T,onDrop:re,"aria-label":`Drop a JSON file here`,children:[(0,P.jsx)(`span`,{className:V.dropZoneIcon,children:`📂`}),(0,P.jsxs)(`span`,{className:V.dropZoneText,children:[`Drop a `,(0,P.jsx)(`code`,{children:`.json`}),` file here`]}),(0,P.jsx)(`span`,{className:V.dropZoneOr,children:`— or —`}),(0,P.jsx)(`input`,{ref:m,type:`file`,accept:`.json,application/json`,className:V.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:E}),(0,P.jsx)(`button`,{type:`button`,className:V.browseButton,onClick:()=>m.current?.click(),disabled:_,"aria-label":`Browse for a JSON file`,children:`Browse file…`})]}),c&&(0,P.jsxs)(`span`,{className:V.fileError,role:`alert`,children:[`⚠️ `,c]}),(0,P.jsx)(`label`,{htmlFor:`mock-upload-textarea`,className:V.textareaLabel,children:`JSON Payload`}),(0,P.jsx)(`textarea`,{id:`mock-upload-textarea`,ref:p,className:V.textarea,value:i,onChange:e=>{a(e.target.value),l(null)},onKeyDown:ee,placeholder:`Paste JSON here, or drop / browse a .json file above`,rows:10,spellCheck:!1,"aria-describedby":ie?`mock-upload-validation`:void 0}),ie&&(0,P.jsx)(`span`,{id:`mock-upload-validation`,className:V.validationError,role:`status`,children:`⚠️ Invalid JSON — check syntax`}),(0,P.jsx)(`span`,{className:V.keyboardHint,children:U?`⌘+Enter to upload`:`Ctrl+Enter to upload`}),o&&(0,P.jsxs)(`div`,{className:V.errorBanner,role:`alert`,children:[`❌ Upload failed: `,o]})]}),(0,P.jsxs)(`div`,{className:V.modalFooter,children:[(0,P.jsx)(`button`,{className:V.formatButton,onClick:te,disabled:!h||_,title:`Format JSON`,"aria-label":`Format JSON`,children:`Format`}),(0,P.jsxs)(`div`,{className:V.footerActions,children:[(0,P.jsx)(`button`,{className:V.cancelButton,onClick:b,disabled:_,children:`Cancel`}),(0,P.jsx)(`button`,{className:V.uploadButton,onClick:C,disabled:!g||_,"aria-busy":_,children:_?(0,P.jsxs)(P.Fragment,{children:[(0,P.jsx)(`span`,{className:V.spinner,"aria-hidden":`true`}),` Uploading…`]}):`Upload ▶`})]})]})]})})}var ho=/^[A-Za-z0-9_-]+$/,go=/^[A-Za-z0-9_-]+(?:\[(?:0|[1-9]\d*)\])*$/,_o=new Set([`input`,`output`,`model`,`response`,`result`,`parameter`,`none`,`next`,`api`,`error`]);function vo(e,t){return`properties.${e}.${t}`}function yo(e){return e.split(`.`).every(e=>go.test(e))}function bo(e,t){return t===`edit`?yo(e):ho.test(e)}function xo(e,t={}){let n={},r=t.mode??`create`,i=e.alias.trim(),a=t.originalAlias?.trim()??``,o=e.nodeType.trim();r===`edit`?a?ho.test(a)||(n.alias=`Use only letters, numbers, underscore, and hyphen.`):n.alias=`Original alias is required.`:i?ho.test(i)?_o.has(i.toLowerCase())?n.alias=`"${i}" is reserved.`:t.graphData?.nodes.some(e=>e.alias.toLowerCase()===i.toLowerCase())&&(n.alias=`Node "${i}" already exists in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,o&&!ho.test(o)&&(n.nodeType=`Use only letters, numbers, underscore, and hyphen.`);for(let t of e.properties){let e=t.key.trim(),i=t.value.trim();!e&&!i||(!e&&i?n[vo(t.id,`key`)]=`Property key is required when value is present.`:bo(e,r)||(n[vo(t.id,`key`)]=r===`edit`?`Use a property name or dot/bracket path, for example mapping[0] or config.value.`:`Use only letters, numbers, underscore, and hyphen.`),r===`create`&&(i.includes(`\r`)||i.includes(`
`))?n[vo(t.id,`value`)]=`Property value must be a single line.`:i.includes(`'''`)&&(n[vo(t.id,`value`)]=`Property value cannot contain '''.`))}return{valid:Object.keys(n).length===0,errors:n}}function So(e,t={}){let n={},r=e.trim();return r?ho.test(r)?t.graphData&&!t.graphData.nodes.some(e=>e.alias.toLowerCase()===r.toLowerCase())&&(n.alias=`Node "${r}" is no longer available in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,{valid:Object.keys(n).length===0,errors:n}}function Co(e){return e.length<=63488?{valid:!0,errors:{}}:{valid:!1,errors:{command:`The node command is too large. Shorten property values before submitting.`}}}var wo=0;function To(e=``,t=``){return wo+=1,{id:`property-row-${wo}`,key:e,value:t}}function Eo(e){return{alias:e===`empty-graph`?`root`:``,nodeType:e===`empty-graph`?`Root`:``,properties:[To()],source:e}}var Do=`This node contains data that cannot be safely represented in the edit form. Use the console edit command for this node.`;function Oo(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function ko(e){return e===null?`null`:String(e)}function Ao(e,t,n){if(!yo(e))return!1;if(Array.isArray(t))return t.length!==0&&t.every((t,r)=>Ao(`${e}[${r}]`,t,n));if(Oo(t)){let r=Object.entries(t);return r.length!==0&&r.every(([t,r])=>Ao(`${e}.${t}`,r,n))}let r=ko(t);return r.includes(`'''`)?!1:(n.push(To(e,r)),!0)}function jo(e){if(!ho.test(e.alias)||e.types.length>1)return{valid:!1,formState:null,message:Do};let t=Object.entries(e.properties),n=[];for(let[e,r]of t)if(!Ao(e,r,n))return{valid:!1,formState:null,message:Do};return{valid:!0,formState:{alias:e.alias,nodeType:e.types[0]??``,properties:n.length>0?n:[To()],source:`edit-node`},message:null}}var Mo=e=>(0,P.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:16,height:16,stroke:`currentColor`,strokeWidth:1.8,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,P.jsx)(`line`,{x1:4.75,y1:4.75,x2:11.25,y2:11.25}),(0,P.jsx)(`line`,{x1:11.25,y1:4.75,x2:4.75,y2:11.25})]}),W={overlay:`_overlay_37wtf_1`,panel:`_panel_37wtf_21`,form:`_form_37wtf_34`,header:`_header_37wtf_41`,title:`_title_37wtf_50`,iconButton:`_iconButton_37wtf_57`,removeButton:`_removeButton_37wtf_58`,buttonIcon:`_buttonIcon_37wtf_93`,body:`_body_37wtf_151`,field:`_field_37wtf_161`,propertyField:`_propertyField_37wtf_162`,label:`_label_37wtf_169`,input:`_input_37wtf_176`,textarea:`_textarea_37wtf_189`,properties:`_properties_37wtf_213`,propertiesHeader:`_propertiesHeader_37wtf_229`,sectionTitle:`_sectionTitle_37wtf_236`,propertyRows:`_propertyRows_37wtf_242`,propertyActions:`_propertyActions_37wtf_248`,propertyRow:`_propertyRow_37wtf_242`,message:`_message_37wtf_266`,warningMessage:`_warningMessage_37wtf_267`,errorMessage:`_errorMessage_37wtf_268`,errorText:`_errorText_37wtf_293`,footer:`_footer_37wtf_298`,primaryButton:`_primaryButton_37wtf_308`,secondaryButton:`_secondaryButton_37wtf_309`,addPropertyButton:`_addPropertyButton_37wtf_341`},No=2,Po=8,Fo=42;function Io(e){let t=e.split(`
`).reduce((e,t)=>e+Math.max(1,Math.ceil(t.length/Fo)),0);return Math.min(Math.max(t,No),Po)}function Lo({open:e,mode:t,aliasReadOnly:n,formState:r,phase:i,lockReason:a,serverMessage:o,validationErrors:s,onFormStateChange:c,onSubmit:l,onClose:u}){let d=(0,D.useRef)(null),f=(0,D.useRef)(null),p=(0,D.useRef)(new Map),m=(0,D.useRef)(null),h=t===`edit`,g=i===`sending`,_=a===`disconnected`,v=g||_,y=h?`Edit Node`:`Create Node`,b=h?`Close edit node dialog`:`Close create node dialog`,x=h?`Save Changes`:`Create Node`,S=h?`Saving...`:`Creating...`,C=h?`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`:`Connection disconnected. Refresh the page and create the node again after the app reconnects.`;(0,D.useEffect)(()=>{if(!e)return;h?f.current?.focus():d.current?.focus();let t=e=>{e.key===`Escape`&&(e.preventDefault(),g||u())};return document.addEventListener(`keydown`,t),()=>{document.removeEventListener(`keydown`,t)}},[h,u,e,g]),(0,D.useEffect)(()=>{let e=m.current;if(!e)return;let t=p.current.get(e);t&&(t.focus(),m.current=null)},[r.properties]);let ee=(0,D.useCallback)(e=>{e.preventDefault(),e.stopPropagation()},[]),te=(0,D.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),g||u()},[u,g]),ne=(0,D.useCallback)(e=>{e.stopPropagation()},[]),w=(0,D.useCallback)(e=>{e.preventDefault(),!v&&l()},[v,l]),T=(0,D.useCallback)(e=>{c({...r,...e})},[r,c]),re=(0,D.useCallback)((e,t)=>{c({...r,properties:r.properties.map(n=>n.id===e?{...n,...t}:n)})},[r,c]),E=(0,D.useCallback)(()=>{let e=To();m.current=e.id,c({...r,properties:[...r.properties,e]})},[r,c]),ie=(0,D.useCallback)(e=>{let t=r.properties.filter(t=>t.id!==e);c({...r,properties:t.length>0?t:[To()]})},[r,c]);return e?(0,P.jsx)(`div`,{className:W.overlay,onPointerDown:ee,onClick:te,children:(0,P.jsxs)(`div`,{className:W.panel,role:`dialog`,"aria-modal":`true`,"aria-labelledby":`node-dialog-title`,onPointerDown:ne,onClick:e=>e.stopPropagation(),children:[(0,P.jsxs)(`header`,{className:W.header,children:[(0,P.jsx)(`div`,{children:(0,P.jsx)(`h2`,{id:`node-dialog-title`,className:W.title,children:y})}),(0,P.jsx)(`button`,{type:`button`,className:W.iconButton,"aria-label":b,onClick:u,disabled:g,children:(0,P.jsx)(Mo,{className:W.buttonIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,P.jsxs)(`form`,{className:W.form,onSubmit:w,children:[(0,P.jsxs)(`div`,{className:W.body,children:[o&&!_&&(0,P.jsx)(`div`,{className:W.message,role:`status`,children:o}),s.command&&(0,P.jsx)(`div`,{className:W.errorMessage,role:`alert`,children:s.command}),_&&(0,P.jsx)(`div`,{className:W.warningMessage,role:`status`,children:o??C}),(0,P.jsxs)(`label`,{className:W.field,children:[(0,P.jsx)(`span`,{className:W.label,children:`Alias`}),(0,P.jsx)(`input`,{ref:d,className:W.input,value:r.alias,disabled:v,readOnly:n,"aria-invalid":!!s.alias,"aria-describedby":s.alias?`node-alias-error`:void 0,onChange:e=>T({alias:e.target.value})}),s.alias&&(0,P.jsx)(`span`,{id:`node-alias-error`,className:W.errorText,children:s.alias})]}),(0,P.jsxs)(`label`,{className:W.field,children:[(0,P.jsx)(`span`,{className:W.label,children:`Node Type`}),(0,P.jsx)(`input`,{ref:f,className:W.input,value:r.nodeType,disabled:v,"aria-invalid":!!s.nodeType,"aria-describedby":s.nodeType?`node-type-error`:void 0,onChange:e=>T({nodeType:e.target.value})}),s.nodeType&&(0,P.jsx)(`span`,{id:`node-type-error`,className:W.errorText,children:s.nodeType})]}),(0,P.jsxs)(`section`,{className:W.properties,"aria-labelledby":`node-properties-title`,children:[(0,P.jsx)(`div`,{className:W.propertiesHeader,children:(0,P.jsx)(`h3`,{id:`node-properties-title`,className:W.sectionTitle,children:`Properties`})}),(0,P.jsx)(`div`,{className:W.propertyRows,children:r.properties.map(e=>{let t=s[vo(e.id,`key`)],n=s[vo(e.id,`value`)],r=Io(e.value);return(0,P.jsxs)(`div`,{className:W.propertyRow,children:[(0,P.jsxs)(`label`,{className:W.propertyField,children:[(0,P.jsx)(`span`,{className:W.label,children:`Key`}),(0,P.jsx)(`input`,{ref:t=>{t?p.current.set(e.id,t):p.current.delete(e.id)},className:W.input,value:e.key,disabled:v,"aria-invalid":!!t,onChange:t=>re(e.id,{key:t.target.value})}),t&&(0,P.jsx)(`span`,{className:W.errorText,children:t})]}),(0,P.jsxs)(`label`,{className:W.propertyField,children:[(0,P.jsx)(`span`,{className:W.label,children:`Value`}),h?(0,P.jsx)(`textarea`,{className:`${W.input} ${W.textarea}`,value:e.value,disabled:v,rows:r,"aria-invalid":!!n,onChange:t=>re(e.id,{value:t.target.value})}):(0,P.jsx)(`input`,{className:W.input,value:e.value,disabled:v,"aria-invalid":!!n,onChange:t=>re(e.id,{value:t.target.value})}),n&&(0,P.jsx)(`span`,{className:W.errorText,children:n})]}),(0,P.jsx)(`button`,{type:`button`,className:W.removeButton,"aria-label":`Remove property`,disabled:v,onClick:()=>ie(e.id),children:(0,P.jsx)(Mo,{className:W.buttonIcon,"aria-hidden":`true`,focusable:`false`})})]},e.id)})}),(0,P.jsx)(`div`,{className:W.propertyActions,children:(0,P.jsxs)(`button`,{type:`button`,className:`${W.secondaryButton} ${W.addPropertyButton}`,disabled:v,onClick:E,children:[(0,P.jsx)(`span`,{"aria-hidden":`true`,children:`+`}),(0,P.jsx)(`span`,{children:`Add Property`})]})})]})]}),(0,P.jsxs)(`footer`,{className:W.footer,children:[(0,P.jsx)(`button`,{type:`button`,className:W.secondaryButton,onClick:u,disabled:g,children:`Cancel`}),(0,P.jsx)(`button`,{type:`submit`,className:W.primaryButton,disabled:v,children:g?S:x})]})]})]})}):null}function Ro({state:e,validationErrors:t,onFormStateChange:n,onSubmit:r,onClose:i}){if(e.status===`closed`)return null;let a=e.phase===`sending`?`sending`:e.connectionLost?`disconnected`:null;return(0,P.jsx)(Lo,{open:!0,mode:e.action===`edit-node`?`edit`:`create`,aliasReadOnly:e.action===`edit-node`,formState:e.formState,phase:e.phase,lockReason:a,serverMessage:e.serverMessage,validationErrors:t,onFormStateChange:n,onSubmit:r,onClose:i})}function zo(e,t=!1){return e.properties.map(e=>({key:e.key.trim(),value:t?e.value.replace(/\r\n/g,`
`).replace(/\r/g,`
`):e.value.trim()})).filter(e=>e.key||e.value.trim())}function Bo(e){let t=Co(e);if(!t.valid)throw Error(t.errors.command)}function Vo(e,t,n){if(n.includes(`
`)){e.push(`${t}='''`),e.push(n),e.push(`'''`);return}e.push(`${t}=${n}`)}function Ho(e){let t=xo(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid node form state.`);let n=e.alias.trim(),r=e.nodeType.trim(),i=zo(e),a=[`create node ${n}`];if(r&&a.push(`with type ${r}`),i.length>0){a.push(`with properties`);for(let e of i)Vo(a,e.key,e.value)}let o=a.join(`
`);return Bo(o),o}function Uo(e,t){let n=t.trim(),r=xo(e,{mode:`edit`,originalAlias:n});if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node form state.`);let i=e.nodeType.trim(),a=zo(e,!0),o=[`update node ${n}`];if(i&&o.push(`with type ${i}`),a.length>0){o.push(`with properties`);for(let e of a)Vo(o,e.key,e.value)}let s=o.join(`
`);return Bo(s),s}function Wo(e,t={}){let n=e.trim(),r=So(n,t);if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node alias.`);let i=`delete node ${n}`;return Bo(i),i}var Go=1e4,Ko=`A node action is already pending. Wait for it to finish before starting another.`,qo=`Could not send the create-node command because the WebSocket is not open. The form values remain in this dialog.`,Jo=`Could not send the edit-node command because the WebSocket is not open. Your changes remain in this dialog.`,Yo=`Could not send the delete-node command because the WebSocket is not open.`,Xo=`This node is no longer available in the current graph.`,Zo=`Connection disconnected. Refresh the page and create the node again after the app reconnects.`,Qo=`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,$o=`Connection disconnected while the node action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.`,es={status:`closed`,pendingSubmit:null,serverMessage:null};function ts(e){return e.pendingSubmit}function ns(e){return e===`edit-node`?Jo:e===`delete-node`?Yo:qo}function rs(e){return`The ${e} command was sent, but no backend result was observed yet. The outcome is unknown.`}function is(e){return e===`edit-node`?Qo:Zo}function as(e,t){return e?.trim().toLowerCase()===t.trim().toLowerCase()}function os(e,t){return e?.nodes.find(e=>e.alias.toLowerCase()===t.toLowerCase())??null}function ss(e,t){return e.status===`error`?!0:as(e.alias,t.alias)?e.action===null||e.action===t.action:!1}function cs({bus:e,connected:t,graphData:n,executor:r,timeoutMs:i=Go,onAccepted:a,onUserMessage:o}){let[s,c]=(0,D.useState)(es),[l,u]=(0,D.useState)({}),d=(0,D.useRef)(s),f=(0,D.useRef)(null),p=(0,D.useRef)(t),m=(0,D.useRef)(n),h=(0,D.useRef)(a),g=(0,D.useRef)(o);(0,D.useEffect)(()=>{d.current=s},[s]),(0,D.useEffect)(()=>{m.current=n},[n]),(0,D.useEffect)(()=>{h.current=a},[a]),(0,D.useEffect)(()=>{g.current=o},[o]);let _=(0,D.useCallback)((e,t=`error`)=>{g.current?.(e,t)},[]),v=(0,D.useCallback)(e=>{d.current=e,c(e)},[]),y=(0,D.useCallback)(()=>{f.current!==null&&(clearTimeout(f.current),f.current=null)},[]),b=(0,D.useCallback)(()=>{y(),f.current=setTimeout(()=>{let e=d.current,t=ts(e);t&&(e.status===`open`?v({...e,phase:`editing`,pendingSubmit:null,serverMessage:rs(t.action)}):(v(es),_(rs(t.action),`error`)),f.current=null)},i)},[y,_,v,i]),x=(0,D.useCallback)(e=>{if(!t)return;if(ts(d.current)){_(Ko,`error`);return}let n=Eo(e);u({}),v({status:`open`,action:`create-node`,phase:`editing`,formState:n,originalAlias:null,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),S=(0,D.useCallback)(e=>{if(!t){_(Qo,`error`);return}if(ts(d.current)){_(Ko,`error`);return}let n=os(m.current,e.alias);if(!n){_(Xo,`error`);return}let r=jo(n);if(!r.valid||!r.formState){_(r.message??`This node cannot be edited in the UI.`,`error`);return}u({}),v({status:`open`,action:`edit-node`,phase:`editing`,formState:r.formState,originalAlias:n.alias,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),C=(0,D.useCallback)(e=>{if(!t){_(Yo,`error`);return}if(ts(d.current)){_(Ko,`error`);return}let n=So(e.alias,{graphData:m.current});if(!n.valid){_(Object.values(n.errors)[0]??`Invalid node alias.`,`error`);return}let i;try{i=Wo(e.alias,{graphData:m.current})}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}if(!r.execute(i)){_(Yo,`error`);return}let a={action:`delete-node`,alias:e.alias.trim(),command:i,sentAt:new Date().toISOString()};u({}),v({status:`closed`,pendingSubmit:a,serverMessage:null}),b()},[t,r,_,v,b]),ee=(0,D.useCallback)(e=>{let t=d.current;t.status===`open`&&(t.phase===`sending`||t.connectionLost||(u({}),v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1})))},[v]),te=(0,D.useCallback)(()=>{let e=d.current;if(e.status!==`open`||e.phase===`sending`||e.connectionLost)return;let n=e.action;if(!t){v({...e,serverMessage:ns(n)});return}let i=xo(e.formState,n===`edit-node`?{mode:`edit`,originalAlias:e.originalAlias}:{graphData:m.current});if(!i.valid){u(i.errors);return}let a,o;try{n===`edit-node`?(o=e.originalAlias?.trim()??``,a=Uo(e.formState,o)):(o=e.formState.alias.trim(),a=Ho(e.formState))}catch(e){u({command:e instanceof Error?e.message:String(e)});return}if(!r.execute(a)){v({...e,phase:`editing`,pendingSubmit:null,serverMessage:ns(n)});return}let s={action:n,alias:o,command:a,sentAt:new Date().toISOString()};u({}),v({...e,phase:`sending`,pendingSubmit:s,serverMessage:null,connectionLost:!1}),b()},[t,r,v,b]),ne=(0,D.useCallback)(()=>{let e=d.current;e.status===`open`&&e.phase!==`sending`&&(y(),u({}),v(es))},[y,v]);return(0,D.useEffect)(()=>e.on(`minigraph.nodeAction.textResult`,e=>{let t=d.current,n=ts(t);if(!(!n||!ss(e,n))){if(y(),e.status===`accepted`){u({}),v(es),h.current?.({status:e.status,action:e.action,alias:e.alias,message:e.message});return}t.status===`open`?v({...t,phase:`editing`,pendingSubmit:null,serverMessage:e.status===`error`?`Backend returned an error while this submit was pending: ${e.message}`:e.message}):(v(es),_(e.message,`error`))}}),[e,y,_,v]),(0,D.useEffect)(()=>{if(p.current&&!t){let e=d.current,t=ts(e);if(e.status===`open`){y();let n=t?$o:is(e.action);v({...e,phase:`editing`,pendingSubmit:null,serverMessage:n,connectionLost:!0})}else t&&(y(),v(es),_($o,`error`))}p.current=t},[y,t,_,v]),(0,D.useEffect)(()=>()=>{y()},[y]),{state:s,validationErrors:l,openCreateNode:x,openEditNode:S,deleteNode:C,updateFormState:ee,submit:te,close:ne}}var ls=(e,t)=>t.some(t=>e instanceof t),us,ds;function fs(){return us||=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction]}function ps(){return ds||=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey]}var ms=new WeakMap,hs=new WeakMap,gs=new WeakMap;function _s(e){let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`success`,i),e.removeEventListener(`error`,a)},i=()=>{t(Cs(e.result)),r()},a=()=>{n(e.error),r()};e.addEventListener(`success`,i),e.addEventListener(`error`,a)});return gs.set(t,e),t}function vs(e){if(ms.has(e))return;let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`complete`,i),e.removeEventListener(`error`,a),e.removeEventListener(`abort`,a)},i=()=>{t(),r()},a=()=>{n(e.error||new DOMException(`AbortError`,`AbortError`)),r()};e.addEventListener(`complete`,i),e.addEventListener(`error`,a),e.addEventListener(`abort`,a)});ms.set(e,t)}var ys={get(e,t,n){if(e instanceof IDBTransaction){if(t===`done`)return ms.get(e);if(t===`store`)return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return Cs(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){return e instanceof IDBTransaction&&(t===`done`||t===`store`)||t in e}};function bs(e){ys=e(ys)}function xs(e){return ps().includes(e)?function(...t){return e.apply(ws(this),t),Cs(this.request)}:function(...t){return Cs(e.apply(ws(this),t))}}function Ss(e){return typeof e==`function`?xs(e):(e instanceof IDBTransaction&&vs(e),ls(e,fs())?new Proxy(e,ys):e)}function Cs(e){if(e instanceof IDBRequest)return _s(e);if(hs.has(e))return hs.get(e);let t=Ss(e);return t!==e&&(hs.set(e,t),gs.set(t,e)),t}var ws=e=>gs.get(e);function Ts(e,t,{blocked:n,upgrade:r,blocking:i,terminated:a}={}){let o=indexedDB.open(e,t),s=Cs(o);return r&&o.addEventListener(`upgradeneeded`,e=>{r(Cs(o.result),e.oldVersion,e.newVersion,Cs(o.transaction),e)}),n&&o.addEventListener(`blocked`,e=>n(e.oldVersion,e.newVersion,e)),s.then(e=>{a&&e.addEventListener(`close`,()=>a()),i&&e.addEventListener(`versionchange`,e=>i(e.oldVersion,e.newVersion,e))}).catch(()=>{}),s}function Es(e,{blocked:t}={}){let n=indexedDB.deleteDatabase(e);return t&&n.addEventListener(`blocked`,e=>t(e.oldVersion,e)),Cs(n).then(()=>void 0)}var Ds=[`get`,`getKey`,`getAll`,`getAllKeys`,`count`],Os=[`put`,`add`,`delete`,`clear`],ks=new Map;function As(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t==`string`))return;if(ks.get(t))return ks.get(t);let n=t.replace(/FromIndex$/,``),r=t!==n,i=Os.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(i||Ds.includes(n)))return;let a=async function(e,...t){let a=this.transaction(e,i?`readwrite`:`readonly`),o=a.store;return r&&(o=o.index(t.shift())),(await Promise.all([o[n](...t),i&&a.done]))[0]};return ks.set(t,a),a}bs(e=>({...e,get:(t,n,r)=>As(t,n)||e.get(t,n,r),has:(t,n)=>!!As(t,n)||e.has(t,n)}));var js=[`continue`,`continuePrimaryKey`,`advance`],Ms={},Ns=new WeakMap,Ps=new WeakMap,Fs={get(e,t){if(!js.includes(t))return e[t];let n=Ms[t];return n||=Ms[t]=function(...e){Ns.set(this,Ps.get(this)[t](...e))},n}};async function*Is(...e){let t=this;if(t instanceof IDBCursor||(t=await t.openCursor(...e)),!t)return;t=t;let n=new Proxy(t,Fs);for(Ps.set(n,t),gs.set(n,ws(t));t;)yield n,t=await(Ns.get(n)||t.continue()),Ns.delete(n)}function Ls(e,t){return t===Symbol.asyncIterator&&ls(e,[IDBIndex,IDBObjectStore,IDBCursor])||t===`iterate`&&ls(e,[IDBIndex,IDBObjectStore])}bs(e=>({...e,get(t,n,r){return Ls(t,n)?Is:e.get(t,n,r)},has(t,n){return Ls(t,n)||e.has(t,n)}}));var Rs=`minigraph-clipboard`,zs=1,Bs=`items`,Vs=null;function Hs(){return Ts(Rs,zs,{upgrade(e){e.objectStoreNames.contains(Bs)&&e.deleteObjectStore(Bs);let t=e.createObjectStore(Bs,{keyPath:`id`});t.createIndex(`by-alias`,`node.alias`,{unique:!0}),t.createIndex(`by-clippedAt`,`clippedAt`)}})}function Us(){return Vs||=Hs().catch(async e=>(console.warn(`[clipboard/db] openDB failed, deleting and recreating:`,e),Vs=null,await Es(Rs),Hs())),Vs}async function Ws(){return(await(await Us()).getAllFromIndex(Bs,`by-clippedAt`)).reverse()}async function Gs(e){return(await Us()).getFromIndex(Bs,`by-alias`,e)}async function Ks(e){await(await Us()).add(Bs,e)}async function qs(e,t){let n=(await Us()).transaction(Bs,`readwrite`);await n.store.delete(e),await n.store.add(t),await n.done}async function Js(e){await(await Us()).delete(Bs,e)}async function Ys(){await(await Us()).clear(Bs)}var Xs=`minigraph-clipboard-sync`;function Zs(){return new BroadcastChannel(Xs)}function Qs(e,t){switch(t.type){case`HYDRATE`:return{items:t.items,isLoading:!1};case`ITEM_ADDED`:return{...e,items:[t.item,...e.items]};case`ITEM_REPLACED`:{let n=e.items.filter(e=>e.id!==t.previousId);return{...e,items:[t.item,...n]}}case`ITEM_REMOVED`:return{...e,items:e.items.filter(e=>e.id!==t.id)};case`ITEMS_CLEARED`:return{...e,items:[]};default:return e}}var $s=(0,D.createContext)(null);function ec({children:e}){let[t,n]=(0,D.useReducer)(Qs,{items:[],isLoading:!0}),r=(0,D.useRef)(null);(0,D.useEffect)(()=>{Ws().then(e=>n({type:`HYDRATE`,items:e}))},[]),(0,D.useEffect)(()=>{let e;try{e=Zs()}catch{return}return r.current=e,e.onmessage=e=>{let t=e.data;switch(t.type){case`item-added`:n({type:`ITEM_ADDED`,item:t.item});break;case`item-replaced`:n({type:`ITEM_REPLACED`,item:t.item,previousId:t.previousId});break;case`item-removed`:n({type:`ITEM_REMOVED`,id:t.id});break;case`items-cleared`:n({type:`ITEMS_CLEARED`});break}},()=>{e.close(),r.current=null}},[]);let i=(0,D.useCallback)(e=>{r.current?.postMessage(e)},[]),a=(0,D.useCallback)(async(e,t,r)=>{try{let a={id:crypto.randomUUID(),clippedAt:new Date().toISOString(),sourceWsPath:r.sourceWsPath,sourceLabel:r.sourceLabel,node:e,connections:t},o=await Gs(e.alias);if(o)return{status:`duplicate`,existingItem:o,pendingItem:a};try{await Ks(a)}catch(t){if(t instanceof DOMException&&t.name===`ConstraintError`){let t=await Gs(e.alias);if(t)return{status:`duplicate`,existingItem:t,pendingItem:a}}throw t}return n({type:`ITEM_ADDED`,item:a}),i({type:`item-added`,item:a}),{status:`added`}}catch(e){return{status:`error`,message:e instanceof Error?e.message:String(e)}}},[i]),o=(0,D.useCallback)(async(e,t)=>{await qs(t,e),n({type:`ITEM_REPLACED`,item:e,previousId:t}),i({type:`item-replaced`,item:e,previousId:t})},[i]),s=(0,D.useCallback)(async e=>{await Js(e),n({type:`ITEM_REMOVED`,id:e}),i({type:`item-removed`,id:e})},[i]),c=(0,D.useCallback)(async()=>{await Ys(),n({type:`ITEMS_CLEARED`}),i({type:`items-cleared`})},[i]);return(0,P.jsx)($s.Provider,{value:{items:t.items,isLoading:t.isLoading,clipNode:a,confirmReplace:o,removeItem:s,clearAll:c},children:e})}function tc(){let e=(0,D.useContext)($s);if(!e)throw Error(`useClipboardContext must be used inside <ClipboardProvider>`);return e}function nc(e){let t=Date.now()-new Date(e).getTime();if(t<0)return`just now`;let n=Math.floor(t/1e3);if(n<60)return`just now`;let r=Math.floor(n/60);if(r<60)return`${r} min ago`;let i=Math.floor(r/60);if(i<24)return`${i} hour${i>1?`s`:``} ago`;let a=Math.floor(i/24);return a===1?`yesterday`:a<30?`${a} days ago`:new Date(e).toLocaleDateString()}var G={item:`_item_1rbm8_1`,previewFrame:`_previewFrame_1rbm8_13`,preview:`_preview_1rbm8_13`,previewShell:`_previewShell_1rbm8_25`,metaBlock:`_metaBlock_1rbm8_29`,timestamp:`_timestamp_1rbm8_35`,removeChrome:`_removeChrome_1rbm8_40`,removeIcon:`_removeIcon_1rbm8_68`};function rc({item:e,onRemove:t,onOpenMenu:n,onCloseMenu:r}){let{node:i,clippedAt:a,sourceLabel:o}=e;return(0,P.jsxs)(`div`,{className:G.item,children:[(0,P.jsxs)(`div`,{className:G.previewFrame,children:[(0,P.jsx)(`button`,{type:`button`,className:G.removeChrome,draggable:!1,"aria-label":`Remove node ${i.alias} from clipboard`,onClick:n=>{n.stopPropagation(),r(),t(e.id)},children:(0,P.jsx)(Mo,{className:G.removeIcon,"aria-hidden":`true`,focusable:`false`})}),(0,P.jsx)(`div`,{className:G.preview,role:`group`,draggable:!0,onDragStart:t=>{r(),Aa(t.dataTransfer,e.id)},onContextMenu:t=>{t.preventDefault(),n(e.id,t.clientX,t.clientY)},onKeyDown:t=>{if(t.key===`ContextMenu`||t.key===`F10`&&t.shiftKey){t.preventDefault();let r=t.currentTarget.getBoundingClientRect();n(e.id,Math.round(r.left+8),Math.round(r.top+8))}},tabIndex:0,"aria-label":`Drag node ${i.alias} into the graph to paste`,children:(0,P.jsx)(`div`,{className:G.previewShell,style:Ui(i.types[0]??`unknown`),children:(0,P.jsx)(qi,{alias:i.alias,nodeType:i.types[0]??`unknown`,properties:i.properties})})})]}),(0,P.jsx)(`div`,{className:G.metaBlock,children:(0,P.jsxs)(`div`,{className:G.timestamp,children:[`Clipped `,nc(a),` from `,o]})})]})}var ic={menu:`_menu_164vh_1`,menuItem:`_menuItem_164vh_12`},ac=16;function oc(e,t,n){let r=ac,i=Math.max(ac,n-t-ac);return Math.min(Math.max(e,r),i)}function sc({open:e,x:t,y:n,canPasteToInput:r,onPasteToInput:i,onInspect:a,onClose:o}){let s=(0,D.useRef)(null),c=(0,D.useRef)(null),l=(0,D.useRef)(null),[u,d]=(0,D.useState)({left:t,top:n});return(0,D.useLayoutEffect)(()=>{if(!e||!s.current)return;let r=s.current.getBoundingClientRect();d({left:oc(t,r.width,window.innerWidth),top:oc(n,r.height,window.innerHeight)})},[e,t,n]),(0,D.useEffect)(()=>{if(!e)return;r?c.current?.focus():l.current?.focus();let t=e=>{s.current&&!s.current.contains(e.target)&&o()},n=e=>{e.key===`Escape`&&(e.preventDefault(),o())},i=()=>o();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,i,!0),window.addEventListener(`resize`,i),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,i,!0),window.removeEventListener(`resize`,i)}},[e,r,o]),e?(0,P.jsxs)(`div`,{ref:s,className:ic.menu,style:{left:u.left,top:u.top},role:`menu`,"aria-label":`Clipboard item actions`,children:[(0,P.jsx)(`button`,{ref:c,role:`menuitem`,type:`button`,className:ic.menuItem,disabled:!r,onClick:()=>{r&&i()},children:`Paste to Input`}),(0,P.jsx)(`button`,{ref:l,role:`menuitem`,type:`button`,className:ic.menuItem,onClick:a,children:`Inspect`})]}):null}var cc={sidebar:`_sidebar_nf394_2`,header:`_header_nf394_12`,headerTitle:`_headerTitle_nf394_22`,clearBtn:`_clearBtn_nf394_29`,itemList:`_itemList_nf394_45`,loading:`_loading_nf394_55`,emptyState:`_emptyState_nf394_65`,emptyIcon:`_emptyIcon_nf394_78`,emptyTitle:`_emptyTitle_nf394_83`,emptyHint:`_emptyHint_nf394_87`,inspectPanel:`_inspectPanel_nf394_93`,inspectHeader:`_inspectHeader_nf394_101`,inspectClose:`_inspectClose_nf394_115`,inspectBody:`_inspectBody_nf394_129`,dialog:`_dialog_nf394_135`,dialogTitle:`_dialogTitle_nf394_150`,dialogBody:`_dialogBody_nf394_157`,dialogActions:`_dialogActions_nf394_164`,cancelBtn:`_cancelBtn_nf394_171`,replaceBtn:`_replaceBtn_nf394_185`};function lc(){return(0,P.jsxs)(`div`,{className:cc.emptyState,children:[(0,P.jsx)(`span`,{className:cc.emptyIcon,children:`📋`}),(0,P.jsx)(`span`,{className:cc.emptyTitle,children:`No items clipped yet.`}),(0,P.jsx)(`span`,{className:cc.emptyHint,children:`Right-click a node in the Graph view to get started.`})]})}function uc({connected:e,onPasteToInput:t}){let n=tc(),[i,a]=(0,D.useState)(null),[s,c]=(0,D.useState)(null),l=(e,t,n)=>{c({itemId:e,x:t,y:n})},u=()=>{c(null)},d=e=>{u(),t(e)},f=e=>{u(),a(t=>t?.id===e.id?null:e)},p=e=>{u(),a(t=>t?.id===e?null:t),n.removeItem(e)},m=()=>{u(),a(null),n.clearAll()};(0,D.useEffect)(()=>{let e=new Set(n.items.map(e=>e.id));s&&!e.has(s.itemId)&&c(null),i&&!e.has(i.id)&&a(null)},[n.items,s,i]);let h=(0,D.useMemo)(()=>s?n.items.find(e=>e.id===s.itemId)??null:null,[s,n.items]);return(0,P.jsxs)(`div`,{className:cc.sidebar,children:[(0,P.jsxs)(`div`,{className:cc.header,children:[(0,P.jsx)(`span`,{className:cc.headerTitle,children:`Workspace`}),n.items.length>0&&(0,P.jsx)(`button`,{className:cc.clearBtn,onClick:m,"aria-label":`Clear all workspace items`,children:`Clear`})]}),(0,P.jsx)(`div`,{className:cc.itemList,children:n.isLoading?(0,P.jsx)(`div`,{className:cc.loading,children:`Loading…`}):n.items.length===0?(0,P.jsx)(lc,{}):n.items.map(e=>(0,P.jsx)(rc,{item:e,onRemove:p,onOpenMenu:l,onCloseMenu:u},e.id))}),i&&(0,P.jsxs)(`div`,{className:cc.inspectPanel,children:[(0,P.jsxs)(`div`,{className:cc.inspectHeader,children:[(0,P.jsxs)(`span`,{children:[`Inspect node `,i.node.alias]}),(0,P.jsx)(`button`,{className:cc.inspectClose,onClick:()=>a(null),"aria-label":`Close inspect panel`,children:`✕`})]}),(0,P.jsx)(`div`,{className:cc.inspectBody,children:(0,P.jsx)(o,{data:{node:i.node,connections:i.connections},style:r})})]}),s&&h&&(0,P.jsx)(sc,{open:!0,x:s.x,y:s.y,canPasteToInput:e,onPasteToInput:()=>d(h),onInspect:()=>f(h),onClose:u})]})}function dc(e){let{wheelTargetRef:t,scrollRef:n,contentWrapperRef:r,currentIndex:i,totalPages:a,onNavigatePrev:o,onNavigateNext:s}=e,c=(0,D.useRef)(0),l=(0,D.useRef)(null),u=(0,D.useRef)(!1),d=(0,D.useRef)(null),f=(0,D.useRef)(o),p=(0,D.useRef)(s),m=(0,D.useRef)(i),h=(0,D.useRef)(a);(0,D.useEffect)(()=>{f.current=o}),(0,D.useEffect)(()=>{p.current=s}),(0,D.useEffect)(()=>{m.current=i}),(0,D.useEffect)(()=>{h.current=a}),(0,D.useEffect)(()=>{d.current!==null&&(clearTimeout(d.current),d.current=null),r.current&&(r.current.style.transition=`none`,r.current.style.transform=`translateY(0)`),c.current=0,l.current=null},[i]),(0,D.useEffect)(()=>{let e=t.current;if(!e)return;function i(){c.current=0,l.current=null,r.current&&(r.current.style.transition=`transform 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94)`,r.current.style.transform=`translateY(0)`)}function a(e){if(e.deltaY===0)return;let t=n.current;if(!t)return;let a=t.scrollTop<=0,o=t.scrollTop+t.clientHeight>=t.scrollHeight-1,s=e.deltaY<0,g=e.deltaY>0,_=a&&s,v=o&&g;if(!_&&!v){i();return}if(u.current)return;let y=m.current,b=h.current;if(_&&y===0||v&&y===b-1)return;let x=_?`prev`:`next`;if(l.current!==null&&l.current!==x&&i(),l.current=x,c.current+=Math.abs(e.deltaY),r.current){let e=x===`prev`?-1:1,t=c.current*(18/120),n=Math.min(t,18)*e;r.current.style.transition=`none`,r.current.style.transform=`translateY(${n}px)`}if(d.current!==null&&clearTimeout(d.current),d.current=setTimeout(i,180),c.current>=120){d.current!==null&&clearTimeout(d.current);let e=l.current;i(),u.current=!0,e===`prev`?f.current():p.current(),setTimeout(()=>{u.current=!1},650)}}return e.addEventListener(`wheel`,a,{passive:!0}),()=>{d.current!==null&&clearTimeout(d.current),e.removeEventListener(`wheel`,a)}},[])}var fc={helpRoot:`_helpRoot_18tja_2`,categoryNav:`_categoryNav_18tja_11`,categoryTabScroller:`_categoryTabScroller_18tja_21`,categoryTab:`_categoryTab_18tja_21`,categoryTabActive:`_categoryTabActive_18tja_71`,maximizeButton:`_maximizeButton_18tja_78`,closeButton:`_closeButton_18tja_100`,helpBody:`_helpBody_18tja_122`,emptyFallback:`_emptyFallback_18tja_130`,helpContent:`_helpContent_18tja_147`,topicLink:`_topicLink_18tja_226`,helpBodyContent:`_helpBodyContent_18tja_271`,chipStrip:`_chipStrip_18tja_276`,chipStripLabel:`_chipStripLabel_18tja_294`,topicChip:`_topicChip_18tja_310`,topicChipActive:`_topicChipActive_18tja_338`};function pc(e){return typeof e==`string`?e:typeof e==`number`?String(e):Array.isArray(e)?e.map(pc).join(``):D.isValidElement(e)?pc(e.props.children):``}function mc(e){if(!e.trim().toLowerCase().startsWith(`help `))return null;let t=e.trim().slice(5).replace(/\s*\(.*\)\s*$/,``).trim().toLowerCase();return t.length>0?t:null}function hc({activeTopic:e,onNavigate:t,onClose:n,onToggleMaximize:r,isMaximized:i}){let a=(0,D.useRef)(null),o=(0,D.useRef)(null),s=(0,D.useRef)(null),c=(0,D.useRef)(null);(0,D.useEffect)(()=>{a.current&&(a.current.scrollTop=0)},[e]),(0,D.useEffect)(()=>{let e=c.current;if(!e)return;let t=e.querySelector(`[aria-current="step"]`);t&&t.scrollIntoView({block:`nearest`,inline:`nearest`,behavior:`smooth`})},[e]);let l=si(e),u=(0,D.useMemo)(()=>ci(l),[l]),d=u.length,f=(0,D.useMemo)(()=>ai.find(e=>e.id===l)?.chipStripLabel??null,[l]),p=ui.indexOf(e),m=p<0?0:p,h=ui.length;dc({wheelTargetRef:o,scrollRef:a,contentWrapperRef:s,currentIndex:m,totalPages:h,onNavigatePrev:()=>t(ui[m-1]??``),onNavigateNext:()=>t(ui[m+1]??ui[ui.length-1])});let g=ri(e);return(0,P.jsxs)(`div`,{className:fc.helpRoot,role:`region`,"aria-label":`Help browser`,ref:o,children:[(0,P.jsxs)(`nav`,{className:fc.categoryNav,"aria-label":`Help categories`,children:[(0,P.jsx)(`div`,{className:fc.categoryTabScroller,children:ai.map(e=>(0,P.jsx)(`button`,{className:[fc.categoryTab,e.id===l?fc.categoryTabActive:``].join(` `).trim(),"aria-current":e.id===l?`true`:void 0,onClick:()=>{t(ci(e.id)[0]??``)},children:e.label},e.id))}),r&&(0,P.jsx)(`button`,{className:fc.maximizeButton,onClick:r,"aria-label":i?`Restore help panel`:`Maximize help panel`,children:i?`⊞`:`⛶`}),n&&(0,P.jsx)(`button`,{className:fc.closeButton,onClick:n,"aria-label":`Close help panel`,children:`×`})]}),d>1&&(0,P.jsxs)(`div`,{className:fc.chipStrip,ref:c,children:[f!==null&&(0,P.jsx)(`span`,{className:fc.chipStripLabel,children:f}),u.map(n=>{let r=n===e,i=li(n,l);return(0,P.jsx)(`button`,{className:[fc.topicChip,r?fc.topicChipActive:``].join(` `).trim(),"aria-current":r?`step`:void 0,onClick:()=>t(n),children:i},n)})]}),(0,P.jsx)(`div`,{className:fc.helpBody,ref:a,children:(0,P.jsx)(`div`,{className:fc.helpBodyContent,ref:s,children:g===null?(0,P.jsxs)(`div`,{className:fc.emptyFallback,children:[(0,P.jsxs)(`code`,{children:[`help `,e||``]}),`\xA0 not found in the local bundle.`]}):(0,P.jsx)(`div`,{className:fc.helpContent,children:(0,P.jsx)(y,{remarkPlugins:[x],components:e===``?{li:({children:e,...n})=>{let r=mc(pc(e).trim());return r!==null&&ri(r)!==null?(0,P.jsx)(`li`,{...n,children:(0,P.jsx)(`button`,{className:fc.topicLink,"aria-label":`Open help topic: ${r}`,onClick:()=>t(r),children:e})}):(0,P.jsx)(`li`,{...n,children:e})}}:void 0,children:g})})})})]})}function gc({existingItem:e,pendingItem:t,onReplace:n,onCancel:r}){let i=(0,D.useRef)(null);return(0,D.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal()},[]),(0,P.jsxs)(`dialog`,{ref:i,className:cc.dialog,onClose:r,"aria-labelledby":`duplicate-dialog-title`,children:[(0,P.jsx)(`h2`,{id:`duplicate-dialog-title`,className:cc.dialogTitle,children:`Duplicate Node`}),(0,P.jsxs)(`p`,{className:cc.dialogBody,children:[`A clipboard item with alias `,(0,P.jsxs)(`strong`,{children:[`"`,t.node.alias,`"`]}),` already exists (clipped `,nc(e.clippedAt),`).`]}),(0,P.jsx)(`p`,{className:cc.dialogBody,children:`Replace it with the new snapshot?`}),(0,P.jsxs)(`div`,{className:cc.dialogActions,children:[(0,P.jsx)(`button`,{className:cc.cancelBtn,onClick:r,children:`Cancel`}),(0,P.jsx)(`button`,{className:cc.replaceBtn,onClick:n,children:`Replace`})]})]})}function _c(e,t){if(!t)return null;let n=e.trim().toLowerCase();if(n!==`help`&&!n.startsWith(`help `))return null;let r=di(e);return ri(r)===null?null:r}var vc=class{constructor(){this.listeners=new Map}on(e,t){let n=e;return this.listeners.has(n)||this.listeners.set(n,new Set),this.listeners.get(n).add(t),()=>{this.listeners.get(n)?.delete(t)}}emit(e){let t=this.listeners.get(e.kind);t&&t.forEach(t=>{try{t(e)}catch(t){console.error(`[ProtocolBus] listener for '${e.kind}' threw:`,t)}})}clear(){this.listeners.clear()}},yc=new Set([`info`,`error`,`ping`,`welcome`]);function bc(e,t){let n=[],r={msgId:e,raw:t},i=!1,a=!1,o=!1,s=!1,c=!1,l=Dr(t);if(l.isJSON){let e=l.data;if(typeof e.type==`string`){let i=e.type;return n.push({...r,kind:`lifecycle`,type:i,knownType:yc.has(i),message:typeof e.message==`string`?e.message:t,time:e.time??null}),n.length>0?n:[{...r,kind:`unclassified`}]}return n.push({...r,kind:`json.response`,data:l.data}),n.length>0?n:[{...r,kind:`unclassified`}]}let u=Pr(t);u&&(c=!0,n.push({...r,kind:`payload.large`,apiPath:u.apiPath,byteSize:u.byteSize,filename:u.filename}));let d=Fr(t);d&&(o=!0,n.push({...r,kind:`upload.invitation`,uploadPath:d}));let f=Nr(t);if(f&&(s=!0,n.push({...r,kind:`upload.contentPath`,uploadPath:f})),Mr(t)){a=!0;let e=jr(t);e&&n.push({...r,kind:`graph.link`,apiPath:e})}if(a){let e=Or(t);e&&n.push({...r,kind:`graph.exported`,graphName:e.graphName,apiPath:e.apiPath})}let p=Gr(t);p&&n.push({...r,kind:`graph.mutation`,mutationType:p});let m=Wr(t);m&&n.push({...r,kind:`minigraph.nodeAction.textResult`,status:m.status,action:m.action,alias:m.alias,message:m.message}),m&&(m.action===`create-node`||m.status===`error`)&&n.push({...r,kind:`minigraph.createNode.textResult`,status:m.status,alias:m.alias,message:m.message}),t===`Session restarted`&&n.push({...r,kind:`session.reset`}),t.startsWith(`> `)&&(i=!0,n.push({...r,kind:`command.echo`,commandText:t.slice(2)})),Ir(t)&&n.push({...r,kind:`command.helpOrDescribe`,commandText:t.slice(2)});let h=Lr(t);h&&n.push({...r,kind:`command.importGraph`,graphName:h});let g=kr(t);return g&&n.push({...r,kind:`graph.export.failed`,reason:g.reason}),!i&&!a&&!o&&!s&&!c&&Ar(t)&&n.push({...r,kind:`docs.response`,isMarkdown:!0}),n.length===0&&n.push({...r,kind:`unclassified`}),n}function xc({messages:e,bus:t}){let n=(0,D.useRef)(-1);(0,D.useEffect)(()=>{e.length>0&&(n.current=e[e.length-1].id)},[]);let r=(0,D.useMemo)(()=>{let t=new Map;for(let n of e)t.set(n.id,bc(n.id,n.raw));return t},[e]);return(0,D.useEffect)(()=>{if(e.length===0)return;let i=e.filter(e=>e.id>n.current);if(i.length!==0){n.current=e[e.length-1].id;for(let e of i){let n=r.get(e.id);if(n)for(let e of n)t.emit(e)}}},[e,t,r]),{classificationMap:r}}function Sc({config:e}){let{title:t,wsPath:n,storageKeyPayload:r,storageKeyHistory:i,storageKeyTab:a,storageKeySavedGraphs:o,supportsUpload:s,supportsClipboard:c,supportsHelp:l,supportsAuthoring:u,tabs:d}=e,f=Et(),[p,m]=hr(r,``),h=wr(),[g,_]=(0,D.useState)(()=>h.peekPendingPayload(n)),{takePendingPayload:v}=h;(0,D.useEffect)(()=>{let e=v(n);e!==null&&_(e)},[v,n]);let y=g??p,b=(0,D.useCallback)(e=>{_(null),m(e)},[m]),x=(0,D.useMemo)(()=>y?fr(y):{valid:!0,error:null,type:null},[y]),{toasts:ne,addToast:w,removeToast:T}=mr(),re=(0,D.useRef)(new vc).current,E=Jr({wsPath:n,storageKeyHistory:i,payload:y,addToast:w,bus:re,handleLocalCommand:(0,D.useCallback)(e=>_c(e,l===!0)!==null,[l])}),{classificationMap:ie}=xc({messages:E.messages,bus:re}),[ae,oe]=xi(n),{graphData:se,setGraphData:O,rightTab:k,setRightTab:ce,isRefreshing:le}=Qr(ae,w,d[0],d,a),{modalUploadPath:ue,successfulUploadPaths:A,handleOpenUploadModal:j,handleCloseUploadModal:M,handleUploadSuccess:de,handleUploadError:fe,resetSuccessfulPaths:pe}=hi({bus:re,addToast:w});$r({bus:re,pinnedGraphPath:ae,setPinnedGraphPath:oe,connected:E.connected,sendRawText:E.sendRawText,addToast:w});let me=(0,D.useRef)(!1);(0,D.useEffect)(()=>{me.current&&!E.connected&&(oe(null),O(null)),me.current=E.connected},[E.connected,oe,O]);let[he,ge]=hr(e.storageKeyHelpTopic??`help-topic-fallback`,``),[_e,ve]=hr(`help-panel-open`,!1),[ye,be]=(0,D.useState)(()=>!!l&&!_e),[xe,Se]=(0,D.useState)(!1),Ce=(0,D.useRef)(null),we=(0,D.useCallback)(()=>{ye&&(Se(!0),Ce.current=setTimeout(()=>be(!1),400))},[ye]);(0,D.useEffect)(()=>{if(!ye||xe)return;let e=setTimeout(we,3e3);return()=>clearTimeout(e)},[ye,xe,we]),(0,D.useEffect)(()=>{_e&&ye&&we()},[_e,ye,we]),(0,D.useEffect)(()=>()=>{Ce.current&&clearTimeout(Ce.current)},[]),(0,D.useEffect)(()=>{if(!l)return;let e=e=>{e.ctrlKey&&e.key==="`"&&(e.preventDefault(),ve(e=>!e))};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[l,ve]),fi({bus:re,setHelpTopic:ge,onTabSwitch:l?()=>ve(!0):()=>{}}),gi({bus:re,connected:E.connected,appendMessage:E.appendMessage,addToast:w});let Te=tc(),[Ee,De]=hr(`clipboard-sidebar-open`,!1),[Oe,ke]=(0,D.useState)(null),Ae=(0,D.useCallback)(e=>{let t=wi(e,se);E.setCommand(t.command),w(`${t.verb===`create`?`Create`:`Update`} command for "${e.node.alias}" pasted to input`,`info`)},[se,E.setCommand,w]),je=(0,D.useCallback)(e=>{let t=Te.items.find(t=>t.id===e);if(!t){w(`Clipboard item is no longer available. It may have been removed in another tab.`,`error`);return}let n=wi(t,se);if(!E.sendRawText(n.command)){w(`Could not send clipboard paste command because the WebSocket is not open.`,`error`);return}w(`Clipboard node "${t.node.alias}" sent as ${n.verb}. Waiting for backend response.`,`info`)},[Te.items,se,E.sendRawText,w]),Me=(0,D.useCallback)(async(t,r)=>{try{let i=await Te.clipNode(t,r,{sourceWsPath:n,sourceLabel:e.label});switch(i.status){case`added`:w(`Node "${t.alias}" clipped to workspace`,`success`);break;case`duplicate`:ke({pendingItem:i.pendingItem,existingItem:i.existingItem});break;case`error`:w(`Clip failed: ${i.message}`,`error`);break}}catch(e){w(`Clip failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[Te,n,e.label,w]),Ne=_i(o??``),{defaultName:Pe,setLastSavedName:Fe,resetName:Ie}=vi(o?`${o}-untitled-counter`:`untitled-counter`,re),Le=(0,D.useMemo)(()=>{let e=se?.nodes.find(e=>e.types.includes(`Root`)),t=typeof e?.properties?.name==`string`?e.properties.name:void 0;return t?.trim()?t:null},[se])??Pe,Re=(0,D.useMemo)(()=>Ti(E.sendRawText),[E.sendRawText]),ze=cs({bus:re,connected:E.connected,graphData:se,executor:Re,onUserMessage:w}),{handleSaveGraph:Be,handleLoadGraph:Ve}=yi({bus:re,connected:E.connected,sendRawText:E.sendRawText,saveGraph:Ne.saveGraph,setLastSavedName:Fe,addToast:w}),He=(0,D.useCallback)(e=>{let t=ie.get(e.id)?.find(e=>e.kind===`graph.link`);t&&oe(t.apiPath)},[ie]),{handleSendToJsonPath:Ue}=pi({ctx:h,navigate:f,addToast:w,wsPath:n}),We=Yr(`(max-width: 768px)`),{defaultLayout:Ge,onLayoutChanged:Ke}=ee({id:e.path+`-panel-split`,storage:localStorage}),qe=(0,D.useCallback)(()=>b(pr(y)),[y]),Je=(0,D.useCallback)(()=>{E.clearMessages(),oe(null),O(null),pe(),Ie()},[E.clearMessages,O,pe,Ie]);return(0,P.jsxs)(`div`,{className:lr.wrapper,children:[(0,P.jsx)(Di,{toasts:ne,onRemove:T}),ue&&(0,P.jsx)(mo,{uploadPath:ue,onSuccess:de,onClose:M,onError:fe}),u&&(0,P.jsx)(Ro,{state:ze.state,validationErrors:ze.validationErrors,onFormStateChange:ze.updateFormState,onSubmit:ze.submit,onClose:ze.close}),(0,P.jsxs)(`header`,{className:lr.header,children:[(0,P.jsx)(`h1`,{className:lr.title,children:t}),(0,P.jsxs)(`div`,{className:lr.headerActions,children:[o&&(0,P.jsx)(Fi,{disabled:!se,defaultName:Pe,onSave:Be,nameExists:Ne.hasGraph,connected:E.connected}),o&&Ne.savedGraphs.length>0&&(0,P.jsx)(L,{savedGraphs:Ne.savedGraphs,onLoad:Ve,onDelete:Ne.deleteGraph,connected:E.connected}),c&&(0,P.jsxs)(`button`,{className:lr.clipboardToggle,onClick:()=>De(e=>!e),"aria-label":Ee?`Close workspace sidebar`:`Open workspace sidebar`,"aria-pressed":Ee,children:[`Workspace`,Te.items.length>0?` (${Te.items.length})`:``]}),(0,P.jsx)(Ni,{addToast:w}),l&&(0,P.jsxs)(`div`,{className:lr.helpButtonWrapper,children:[(0,P.jsx)(`button`,{className:`${lr.helpToggle}${ye&&!xe?` ${lr.helpTogglePulsing}`:``}`,onClick:()=>ve(e=>!e),"aria-label":_e?`Close help panel`:`Open help panel`,"aria-pressed":_e,children:`?`}),ye&&(0,P.jsxs)(`div`,{className:`${lr.helpHint}${xe?` ${lr.helpHintFading}`:``}`,onClick:we,role:`status`,children:[(0,P.jsx)(`kbd`,{className:lr.helpHintKbd,children:"Ctrl + `"}),` to toggle help`]})]})]})]}),Oe&&(0,P.jsx)(gc,{existingItem:Oe.existingItem,pendingItem:Oe.pendingItem,onReplace:async()=>{try{await Te.confirmReplace(Oe.pendingItem,Oe.existingItem.id),ke(null),w(`Clipboard item "${Oe.pendingItem.node.alias}" replaced`,`success`)}catch(e){w(`Replace failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},onCancel:()=>{ke(null),w(`Clip cancelled`,`info`)}}),(0,P.jsxs)(te,{className:lr.panelGroup,orientation:We?`vertical`:`horizontal`,defaultLayout:Ge,onLayoutChanged:Ke,children:[(0,P.jsx)(C,{defaultSize:_e||Ee?`50%`:`60%`,minSize:`25%`,children:(0,P.jsx)(uo,{messages:E.messages,classificationMap:ie,onCopy:E.copyMessages,onClear:Je,consoleRef:E.consoleRef,command:E.command,onCommandChange:E.setCommand,onCommandKeyDown:E.handleKeyDown,onSend:E.sendCommand,sendDisabled:!E.connected||!E.command.trim(),inputDisabled:!E.connected,commandHistory:E.history,onGraphLinkMessage:He,onCopyMessage:()=>w(`Copied to clipboard`,`success`),onSendToJsonPath:Ue,onUploadMockData:j,successfulUploadPaths:A})}),(0,P.jsx)(S,{className:lr.resizeHandle,"aria-label":`Resize panels`}),(0,P.jsx)(C,{defaultSize:_e?`50%`:Ee?`30%`:`40%`,minSize:`20%`,children:(0,P.jsx)(Qa,{tabs:d,payload:y,onChange:b,validation:x,onFormat:qe,onUpload:s?E.uploadPayload:void 0,graphData:se,graphName:Le,activeTab:k,onTabChange:ce,onGraphRenderError:e=>w(e,`error`),onGraphDataCopySuccess:()=>w(`Graph JSON copied to clipboard!`,`success`),onGraphDataCopyError:()=>w(`Copy failed`,`error`),isGraphRefreshing:le,onClipNode:c?Me:void 0,onClipboardDrop:c?je:void 0,isConnected:E.connected,supportsAuthoring:u,onCreateNode:u?ze.openCreateNode:void 0,onEditNode:u?ze.openEditNode:void 0,onDeleteNode:u?ze.deleteNode:void 0,helpPanel:l&&_e?((e,t)=>(0,P.jsx)(hc,{activeTopic:he,onNavigate:ge,onClose:()=>ve(!1),onToggleMaximize:e,isMaximized:t})):void 0})}),c&&Ee&&(0,P.jsxs)(P.Fragment,{children:[(0,P.jsx)(S,{className:lr.resizeHandle,"aria-label":`Resize clipboard`}),(0,P.jsx)(C,{defaultSize:`20%`,minSize:`10%`,maxSize:`40%`,children:(0,P.jsx)(uc,{connected:E.connected,onPasteToInput:Ae})})]})]})]})}function Cc(){let e=_r[0].path;return(0,P.jsx)(Cr,{children:(0,P.jsx)(ec,{children:(0,P.jsx)(Hn,{children:(0,P.jsxs)(en,{children:[_r.map(e=>(0,P.jsx)(Qt,{path:e.path,element:(0,P.jsx)(Sc,{config:e},e.path)},e.path)),(0,P.jsx)(Qt,{path:`*`,element:(0,P.jsx)(Zt,{to:e,replace:!0})})]})})})})}(0,cr.createRoot)(document.getElementById(`root`)).render((0,P.jsx)(D.StrictMode,{children:(0,P.jsx)(Cc,{})}));
//# sourceMappingURL=index-CkYdMugy.js.map
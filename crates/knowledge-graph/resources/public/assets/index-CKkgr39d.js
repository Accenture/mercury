import{r as e,t}from"./rolldown-runtime-QTnfLwEv.js";import{a as n,i as r,n as i,r as a,t as o}from"./vendor-json-view-XsbV5yWD.js";import{a as s,c,d as l,f as u,h as d,i as f,l as p,m,n as h,o as g,p as _,r as v,s as y,t as b,u as x}from"./vendor-xyflow-Dg2A4cNM.js";import{a as S,i as C,n as w,o as ee,r as te,t as T}from"./vendor-router-DrFk6dZO.js";import{n as E,r as ne,t as D}from"./vendor-markdown-Di8s8vbi.js";import{i as re,n as ie,r as ae,t as oe}from"./vendor-panels-CXf4xYpQ.js";(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();var O=t((e=>{function t(e,t){var n=e.length;e.push(t);a:for(;0<n;){var r=n-1>>>1,a=e[r];if(0<i(a,t))e[r]=t,e[n]=a,n=r;else break a}}function n(e){return e.length===0?null:e[0]}function r(e){if(e.length===0)return null;var t=e[0],n=e.pop();if(n!==t){e[0]=n;a:for(var r=0,a=e.length,o=a>>>1;r<o;){var s=2*(r+1)-1,c=e[s],l=s+1,u=e[l];if(0>i(c,n))l<a&&0>i(u,c)?(e[r]=u,e[l]=n,r=l):(e[r]=c,e[s]=n,r=s);else if(l<a&&0>i(u,n))e[r]=u,e[l]=n,r=l;else break a}}return t}function i(e,t){var n=e.sortIndex-t.sortIndex;return n===0?e.id-t.id:n}if(e.unstable_now=void 0,typeof performance==`object`&&typeof performance.now==`function`){var a=performance;e.unstable_now=function(){return a.now()}}else{var o=Date,s=o.now();e.unstable_now=function(){return o.now()-s}}var c=[],l=[],u=1,d=null,f=3,p=!1,m=!1,h=!1,g=!1,_=typeof setTimeout==`function`?setTimeout:null,v=typeof clearTimeout==`function`?clearTimeout:null,y=typeof setImmediate<`u`?setImmediate:null;function b(e){for(var i=n(l);i!==null;){if(i.callback===null)r(l);else if(i.startTime<=e)r(l),i.sortIndex=i.expirationTime,t(c,i);else break;i=n(l)}}function x(e){if(h=!1,b(e),!m)if(n(c)!==null)m=!0,S||(S=!0,E());else{var t=n(l);t!==null&&re(x,t.startTime-e)}}var S=!1,C=-1,w=5,ee=-1;function te(){return g?!0:!(e.unstable_now()-ee<w)}function T(){if(g=!1,S){var t=e.unstable_now();ee=t;var i=!0;try{a:{m=!1,h&&(h=!1,v(C),C=-1),p=!0;var a=f;try{b:{for(b(t),d=n(c);d!==null&&!(d.expirationTime>t&&te());){var o=d.callback;if(typeof o==`function`){d.callback=null,f=d.priorityLevel;var s=o(d.expirationTime<=t);if(t=e.unstable_now(),typeof s==`function`){d.callback=s,b(t),i=!0;break b}d===n(c)&&r(c),b(t)}else r(c);d=n(c)}if(d!==null)i=!0;else{var u=n(l);u!==null&&re(x,u.startTime-t),i=!1}}break a}finally{d=null,f=a,p=!1}i=void 0}}finally{i?E():S=!1}}}var E;if(typeof y==`function`)E=function(){y(T)};else if(typeof MessageChannel<`u`){var ne=new MessageChannel,D=ne.port2;ne.port1.onmessage=T,E=function(){D.postMessage(null)}}else E=function(){_(T,0)};function re(t,n){C=_(function(){t(e.unstable_now())},n)}e.unstable_IdlePriority=5,e.unstable_ImmediatePriority=1,e.unstable_LowPriority=4,e.unstable_NormalPriority=3,e.unstable_Profiling=null,e.unstable_UserBlockingPriority=2,e.unstable_cancelCallback=function(e){e.callback=null},e.unstable_forceFrameRate=function(e){0>e||125<e?console.error(`forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported`):w=0<e?Math.floor(1e3/e):5},e.unstable_getCurrentPriorityLevel=function(){return f},e.unstable_next=function(e){switch(f){case 1:case 2:case 3:var t=3;break;default:t=f}var n=f;f=t;try{return e()}finally{f=n}},e.unstable_requestPaint=function(){g=!0},e.unstable_runWithPriority=function(e,t){switch(e){case 1:case 2:case 3:case 4:case 5:break;default:e=3}var n=f;f=e;try{return t()}finally{f=n}},e.unstable_scheduleCallback=function(r,i,a){var o=e.unstable_now();switch(typeof a==`object`&&a?(a=a.delay,a=typeof a==`number`&&0<a?o+a:o):a=o,r){case 1:var s=-1;break;case 2:s=250;break;case 5:s=1073741823;break;case 4:s=1e4;break;default:s=5e3}return s=a+s,r={id:u++,callback:i,priorityLevel:r,startTime:a,expirationTime:s,sortIndex:-1},a>o?(r.sortIndex=a,t(l,r),n(c)===null&&r===n(l)&&(h?(v(C),C=-1):h=!0,re(x,a-o))):(r.sortIndex=s,t(c,r),m||p||(m=!0,S||(S=!0,E()))),r},e.unstable_shouldYield=te,e.unstable_wrapCallback=function(e){var t=f;return function(){var n=f;f=t;try{return e.apply(this,arguments)}finally{f=n}}}})),se=t(((e,t)=>{t.exports=O()})),k=t((e=>{var t=se(),r=n(),i=d();function a(e){var t=`https://react.dev/errors/`+e;if(1<arguments.length){t+=`?args[]=`+encodeURIComponent(arguments[1]);for(var n=2;n<arguments.length;n++)t+=`&args[]=`+encodeURIComponent(arguments[n])}return`Minified React error #`+e+`; visit `+t+` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`}function o(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function s(e){var t=e,n=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,t.flags&4098&&(n=t.return),e=t.return;while(e)}return t.tag===3?n:null}function c(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function l(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function u(e){if(s(e)!==e)throw Error(a(188))}function f(e){var t=e.alternate;if(!t){if(t=s(e),t===null)throw Error(a(188));return t===e?e:null}for(var n=e,r=t;;){var i=n.return;if(i===null)break;var o=i.alternate;if(o===null){if(r=i.return,r!==null){n=r;continue}break}if(i.child===o.child){for(o=i.child;o;){if(o===n)return u(i),e;if(o===r)return u(i),t;o=o.sibling}throw Error(a(188))}if(n.return!==r.return)n=i,r=o;else{for(var c=!1,l=i.child;l;){if(l===n){c=!0,n=i,r=o;break}if(l===r){c=!0,r=i,n=o;break}l=l.sibling}if(!c){for(l=o.child;l;){if(l===n){c=!0,n=o,r=i;break}if(l===r){c=!0,r=o,n=i;break}l=l.sibling}if(!c)throw Error(a(189))}}if(n.alternate!==r)throw Error(a(190))}if(n.tag!==3)throw Error(a(188));return n.stateNode.current===n?e:t}function p(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=p(e),t!==null)return t;e=e.sibling}return null}var m=Object.assign,h=Symbol.for(`react.element`),g=Symbol.for(`react.transitional.element`),_=Symbol.for(`react.portal`),v=Symbol.for(`react.fragment`),y=Symbol.for(`react.strict_mode`),b=Symbol.for(`react.profiler`),x=Symbol.for(`react.consumer`),S=Symbol.for(`react.context`),C=Symbol.for(`react.forward_ref`),w=Symbol.for(`react.suspense`),ee=Symbol.for(`react.suspense_list`),te=Symbol.for(`react.memo`),T=Symbol.for(`react.lazy`),E=Symbol.for(`react.activity`),ne=Symbol.for(`react.memo_cache_sentinel`),D=Symbol.iterator;function re(e){return typeof e!=`object`||!e?null:(e=D&&e[D]||e[`@@iterator`],typeof e==`function`?e:null)}var ie=Symbol.for(`react.client.reference`);function ae(e){if(e==null)return null;if(typeof e==`function`)return e.$$typeof===ie?null:e.displayName||e.name||null;if(typeof e==`string`)return e;switch(e){case v:return`Fragment`;case b:return`Profiler`;case y:return`StrictMode`;case w:return`Suspense`;case ee:return`SuspenseList`;case E:return`Activity`}if(typeof e==`object`)switch(e.$$typeof){case _:return`Portal`;case S:return e.displayName||`Context`;case x:return(e._context.displayName||`Context`)+`.Consumer`;case C:var t=e.render;return e=e.displayName,e||=(e=t.displayName||t.name||``,e===``?`ForwardRef`:`ForwardRef(`+e+`)`),e;case te:return t=e.displayName||null,t===null?ae(e.type)||`Memo`:t;case T:t=e._payload,e=e._init;try{return ae(e(t))}catch{}}return null}var oe=Array.isArray,O=r.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,k=i.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,ce={pending:!1,data:null,method:null,action:null},A=[],j=-1;function M(e){return{current:e}}function N(e){0>j||(e.current=A[j],A[j]=null,j--)}function P(e,t){j++,A[j]=e.current,e.current=t}var le=M(null),ue=M(null),de=M(null),fe=M(null);function pe(e,t){switch(P(de,t),P(ue,e),P(le,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?Vd(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=Vd(t),e=Hd(t,e);else switch(e){case`svg`:e=1;break;case`math`:e=2;break;default:e=0}}N(le),P(le,e)}function me(){N(le),N(ue),N(de)}function he(e){e.memoizedState!==null&&P(fe,e);var t=le.current,n=Hd(t,e.type);t!==n&&(P(ue,e),P(le,n))}function ge(e){ue.current===e&&(N(le),N(ue)),fe.current===e&&(N(fe),Qf._currentValue=ce)}var F,_e;function ve(e){if(F===void 0)try{throw Error()}catch(e){var t=e.stack.trim().match(/\n( *(at )?)/);F=t&&t[1]||``,_e=-1<e.stack.indexOf(`
    at`)?` (<anonymous>)`:-1<e.stack.indexOf(`@`)?`@unknown:0:0`:``}return`
`+F+e+_e}var ye=!1;function be(e,t){if(!e||ye)return``;ye=!0;var n=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var r={DetermineComponentFrameRoot:function(){try{if(t){var n=function(){throw Error()};if(Object.defineProperty(n.prototype,"props",{set:function(){throw Error()}}),typeof Reflect==`object`&&Reflect.construct){try{Reflect.construct(n,[])}catch(e){var r=e}Reflect.construct(e,[],n)}else{try{n.call()}catch(e){r=e}e.call(n.prototype)}}else{try{throw Error()}catch(e){r=e}(n=e())&&typeof n.catch==`function`&&n.catch(function(){})}}catch(e){if(e&&r&&typeof e.stack==`string`)return[e.stack,r.stack]}return[null,null]}};r.DetermineComponentFrameRoot.displayName=`DetermineComponentFrameRoot`;var i=Object.getOwnPropertyDescriptor(r.DetermineComponentFrameRoot,`name`);i&&i.configurable&&Object.defineProperty(r.DetermineComponentFrameRoot,"name",{value:`DetermineComponentFrameRoot`});var a=r.DetermineComponentFrameRoot(),o=a[0],s=a[1];if(o&&s){var c=o.split(`
`),l=s.split(`
`);for(i=r=0;r<c.length&&!c[r].includes(`DetermineComponentFrameRoot`);)r++;for(;i<l.length&&!l[i].includes(`DetermineComponentFrameRoot`);)i++;if(r===c.length||i===l.length)for(r=c.length-1,i=l.length-1;1<=r&&0<=i&&c[r]!==l[i];)i--;for(;1<=r&&0<=i;r--,i--)if(c[r]!==l[i]){if(r!==1||i!==1)do if(r--,i--,0>i||c[r]!==l[i]){var u=`
`+c[r].replace(` at new `,` at `);return e.displayName&&u.includes(`<anonymous>`)&&(u=u.replace(`<anonymous>`,e.displayName)),u}while(1<=r&&0<=i);break}}}finally{ye=!1,Error.prepareStackTrace=n}return(n=e?e.displayName||e.name:``)?ve(n):``}function xe(e,t){switch(e.tag){case 26:case 27:case 5:return ve(e.type);case 16:return ve(`Lazy`);case 13:return e.child!==t&&t!==null?ve(`Suspense Fallback`):ve(`Suspense`);case 19:return ve(`SuspenseList`);case 0:case 15:return be(e.type,!1);case 11:return be(e.type.render,!1);case 1:return be(e.type,!0);case 31:return ve(`Activity`);default:return``}}function Se(e){try{var t=``,n=null;do t+=xe(e,n),n=e,e=e.return;while(e);return t}catch(e){return`
Error generating stack: `+e.message+`
`+e.stack}}var Ce=Object.prototype.hasOwnProperty,we=t.unstable_scheduleCallback,Te=t.unstable_cancelCallback,Ee=t.unstable_shouldYield,De=t.unstable_requestPaint,Oe=t.unstable_now,ke=t.unstable_getCurrentPriorityLevel,Ae=t.unstable_ImmediatePriority,je=t.unstable_UserBlockingPriority,Me=t.unstable_NormalPriority,Ne=t.unstable_LowPriority,Pe=t.unstable_IdlePriority,Fe=t.log,Ie=t.unstable_setDisableYieldValue,Le=null,Re=null;function ze(e){if(typeof Fe==`function`&&Ie(e),Re&&typeof Re.setStrictMode==`function`)try{Re.setStrictMode(Le,e)}catch{}}var Be=Math.clz32?Math.clz32:Ue,Ve=Math.log,He=Math.LN2;function Ue(e){return e>>>=0,e===0?32:31-(Ve(e)/He|0)|0}var We=256,Ge=262144,Ke=4194304;function qe(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function Je(e,t,n){var r=e.pendingLanes;if(r===0)return 0;var i=0,a=e.suspendedLanes,o=e.pingedLanes;e=e.warmLanes;var s=r&134217727;return s===0?(s=r&~a,s===0?o===0?n||(n=r&~e,n!==0&&(i=qe(n))):i=qe(o):i=qe(s)):(r=s&~a,r===0?(o&=s,o===0?n||(n=s&~e,n!==0&&(i=qe(n))):i=qe(o)):i=qe(r)),i===0?0:t!==0&&t!==i&&(t&a)===0&&(a=i&-i,n=t&-t,a>=n||a===32&&n&4194048)?t:i}function Ye(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function Xe(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function Ze(){var e=Ke;return Ke<<=1,!(Ke&62914560)&&(Ke=4194304),e}function Qe(e){for(var t=[],n=0;31>n;n++)t.push(e);return t}function $e(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function et(e,t,n,r,i,a){var o=e.pendingLanes;e.pendingLanes=n,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=n,e.entangledLanes&=n,e.errorRecoveryDisabledLanes&=n,e.shellSuspendCounter=0;var s=e.entanglements,c=e.expirationTimes,l=e.hiddenUpdates;for(n=o&~n;0<n;){var u=31-Be(n),d=1<<u;s[u]=0,c[u]=-1;var f=l[u];if(f!==null)for(l[u]=null,u=0;u<f.length;u++){var p=f[u];p!==null&&(p.lane&=-536870913)}n&=~d}r!==0&&tt(e,r,0),a!==0&&i===0&&e.tag!==0&&(e.suspendedLanes|=a&~(o&~t))}function tt(e,t,n){e.pendingLanes|=t,e.suspendedLanes&=~t;var r=31-Be(t);e.entangledLanes|=t,e.entanglements[r]=e.entanglements[r]|1073741824|n&261930}function nt(e,t){var n=e.entangledLanes|=t;for(e=e.entanglements;n;){var r=31-Be(n),i=1<<r;i&t|e[r]&t&&(e[r]|=t),n&=~i}}function rt(e,t){var n=t&-t;return n=n&42?1:it(n),(n&(e.suspendedLanes|t))===0?n:0}function it(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function at(e){return e&=-e,2<e?8<e?e&134217727?32:268435456:8:2}function ot(){var e=k.p;return e===0?(e=window.event,e===void 0?32:mp(e.type)):e}function st(e,t){var n=k.p;try{return k.p=e,t()}finally{k.p=n}}var ct=Math.random().toString(36).slice(2),lt=`__reactFiber$`+ct,I=`__reactProps$`+ct,ut=`__reactContainer$`+ct,dt=`__reactEvents$`+ct,ft=`__reactListeners$`+ct,pt=`__reactHandles$`+ct,mt=`__reactResources$`+ct,ht=`__reactMarker$`+ct;function gt(e){delete e[lt],delete e[I],delete e[dt],delete e[ft],delete e[pt]}function _t(e){var t=e[lt];if(t)return t;for(var n=e.parentNode;n;){if(t=n[ut]||n[lt]){if(n=t.alternate,t.child!==null||n!==null&&n.child!==null)for(e=df(e);e!==null;){if(n=e[lt])return n;e=df(e)}return t}e=n,n=e.parentNode}return null}function vt(e){if(e=e[lt]||e[ut]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function yt(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(a(33))}function bt(e){var t=e[mt];return t||=e[mt]={hoistableStyles:new Map,hoistableScripts:new Map},t}function xt(e){e[ht]=!0}var St=new Set,Ct={};function wt(e,t){Tt(e,t),Tt(e+`Capture`,t)}function Tt(e,t){for(Ct[e]=t,e=0;e<t.length;e++)St.add(t[e])}var Et=RegExp(`^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`),Dt={},Ot={};function kt(e){return Ce.call(Ot,e)?!0:Ce.call(Dt,e)?!1:Et.test(e)?Ot[e]=!0:(Dt[e]=!0,!1)}function At(e,t,n){if(kt(t))if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:e.removeAttribute(t);return;case`boolean`:var r=t.toLowerCase().slice(0,5);if(r!==`data-`&&r!==`aria-`){e.removeAttribute(t);return}}e.setAttribute(t,``+n)}}function jt(e,t,n){if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(t);return}e.setAttribute(t,``+n)}}function Mt(e,t,n,r){if(r===null)e.removeAttribute(n);else{switch(typeof r){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(n);return}e.setAttributeNS(t,n,``+r)}}function Nt(e){switch(typeof e){case`bigint`:case`boolean`:case`number`:case`string`:case`undefined`:return e;case`object`:return e;default:return``}}function Pt(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()===`input`&&(t===`checkbox`||t===`radio`)}function Ft(e,t,n){var r=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&r!==void 0&&typeof r.get==`function`&&typeof r.set==`function`){var i=r.get,a=r.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return i.call(this)},set:function(e){n=``+e,a.call(this,e)}}),Object.defineProperty(e,t,{enumerable:r.enumerable}),{getValue:function(){return n},setValue:function(e){n=``+e},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function It(e){if(!e._valueTracker){var t=Pt(e)?`checked`:`value`;e._valueTracker=Ft(e,t,``+e[t])}}function Lt(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var n=t.getValue(),r=``;return e&&(r=Pt(e)?e.checked?`true`:`false`:e.value),e=r,e===n?!1:(t.setValue(e),!0)}function Rt(e){if(e||=typeof document<`u`?document:void 0,e===void 0)return null;try{return e.activeElement||e.body}catch{return e.body}}var zt=/[\n"\\]/g;function Bt(e){return e.replace(zt,function(e){return`\\`+e.charCodeAt(0).toString(16)+` `})}function Vt(e,t,n,r,i,a,o,s){e.name=``,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`?e.type=o:e.removeAttribute(`type`),t==null?o!==`submit`&&o!==`reset`||e.removeAttribute(`value`):o===`number`?(t===0&&e.value===``||e.value!=t)&&(e.value=``+Nt(t)):e.value!==``+Nt(t)&&(e.value=``+Nt(t)),t==null?n==null?r!=null&&e.removeAttribute(`value`):Ut(e,o,Nt(n)):Ut(e,o,Nt(t)),i==null&&a!=null&&(e.defaultChecked=!!a),i!=null&&(e.checked=i&&typeof i!=`function`&&typeof i!=`symbol`),s!=null&&typeof s!=`function`&&typeof s!=`symbol`&&typeof s!=`boolean`?e.name=``+Nt(s):e.removeAttribute(`name`)}function Ht(e,t,n,r,i,a,o,s){if(a!=null&&typeof a!=`function`&&typeof a!=`symbol`&&typeof a!=`boolean`&&(e.type=a),t!=null||n!=null){if(!(a!==`submit`&&a!==`reset`||t!=null)){It(e);return}n=n==null?``:``+Nt(n),t=t==null?n:``+Nt(t),s||t===e.value||(e.value=t),e.defaultValue=t}r??=i,r=typeof r!=`function`&&typeof r!=`symbol`&&!!r,e.checked=s?e.checked:!!r,e.defaultChecked=!!r,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`&&(e.name=o),It(e)}function Ut(e,t,n){t===`number`&&Rt(e.ownerDocument)===e||e.defaultValue===``+n||(e.defaultValue=``+n)}function Wt(e,t,n,r){if(e=e.options,t){t={};for(var i=0;i<n.length;i++)t[`$`+n[i]]=!0;for(n=0;n<e.length;n++)i=t.hasOwnProperty(`$`+e[n].value),e[n].selected!==i&&(e[n].selected=i),i&&r&&(e[n].defaultSelected=!0)}else{for(n=``+Nt(n),t=null,i=0;i<e.length;i++){if(e[i].value===n){e[i].selected=!0,r&&(e[i].defaultSelected=!0);return}t!==null||e[i].disabled||(t=e[i])}t!==null&&(t.selected=!0)}}function Gt(e,t,n){if(t!=null&&(t=``+Nt(t),t!==e.value&&(e.value=t),n==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=n==null?``:``+Nt(n)}function Kt(e,t,n,r){if(t==null){if(r!=null){if(n!=null)throw Error(a(92));if(oe(r)){if(1<r.length)throw Error(a(93));r=r[0]}n=r}n??=``,t=n}n=Nt(t),e.defaultValue=n,r=e.textContent,r===n&&r!==``&&r!==null&&(e.value=r),It(e)}function qt(e,t){if(t){var n=e.firstChild;if(n&&n===e.lastChild&&n.nodeType===3){n.nodeValue=t;return}}e.textContent=t}var Jt=new Set(`animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(` `));function Yt(e,t,n){var r=t.indexOf(`--`)===0;n==null||typeof n==`boolean`||n===``?r?e.setProperty(t,``):t===`float`?e.cssFloat=``:e[t]=``:r?e.setProperty(t,n):typeof n!=`number`||n===0||Jt.has(t)?t===`float`?e.cssFloat=n:e[t]=(``+n).trim():e[t]=n+`px`}function Xt(e,t,n){if(t!=null&&typeof t!=`object`)throw Error(a(62));if(e=e.style,n!=null){for(var r in n)!n.hasOwnProperty(r)||t!=null&&t.hasOwnProperty(r)||(r.indexOf(`--`)===0?e.setProperty(r,``):r===`float`?e.cssFloat=``:e[r]=``);for(var i in t)r=t[i],t.hasOwnProperty(i)&&n[i]!==r&&Yt(e,i,r)}else for(var o in t)t.hasOwnProperty(o)&&Yt(e,o,t[o])}function Zt(e){if(e.indexOf(`-`)===-1)return!1;switch(e){case`annotation-xml`:case`color-profile`:case`font-face`:case`font-face-src`:case`font-face-uri`:case`font-face-format`:case`font-face-name`:case`missing-glyph`:return!1;default:return!0}}var Qt=new Map([[`acceptCharset`,`accept-charset`],[`htmlFor`,`for`],[`httpEquiv`,`http-equiv`],[`crossOrigin`,`crossorigin`],[`accentHeight`,`accent-height`],[`alignmentBaseline`,`alignment-baseline`],[`arabicForm`,`arabic-form`],[`baselineShift`,`baseline-shift`],[`capHeight`,`cap-height`],[`clipPath`,`clip-path`],[`clipRule`,`clip-rule`],[`colorInterpolation`,`color-interpolation`],[`colorInterpolationFilters`,`color-interpolation-filters`],[`colorProfile`,`color-profile`],[`colorRendering`,`color-rendering`],[`dominantBaseline`,`dominant-baseline`],[`enableBackground`,`enable-background`],[`fillOpacity`,`fill-opacity`],[`fillRule`,`fill-rule`],[`floodColor`,`flood-color`],[`floodOpacity`,`flood-opacity`],[`fontFamily`,`font-family`],[`fontSize`,`font-size`],[`fontSizeAdjust`,`font-size-adjust`],[`fontStretch`,`font-stretch`],[`fontStyle`,`font-style`],[`fontVariant`,`font-variant`],[`fontWeight`,`font-weight`],[`glyphName`,`glyph-name`],[`glyphOrientationHorizontal`,`glyph-orientation-horizontal`],[`glyphOrientationVertical`,`glyph-orientation-vertical`],[`horizAdvX`,`horiz-adv-x`],[`horizOriginX`,`horiz-origin-x`],[`imageRendering`,`image-rendering`],[`letterSpacing`,`letter-spacing`],[`lightingColor`,`lighting-color`],[`markerEnd`,`marker-end`],[`markerMid`,`marker-mid`],[`markerStart`,`marker-start`],[`overlinePosition`,`overline-position`],[`overlineThickness`,`overline-thickness`],[`paintOrder`,`paint-order`],[`panose-1`,`panose-1`],[`pointerEvents`,`pointer-events`],[`renderingIntent`,`rendering-intent`],[`shapeRendering`,`shape-rendering`],[`stopColor`,`stop-color`],[`stopOpacity`,`stop-opacity`],[`strikethroughPosition`,`strikethrough-position`],[`strikethroughThickness`,`strikethrough-thickness`],[`strokeDasharray`,`stroke-dasharray`],[`strokeDashoffset`,`stroke-dashoffset`],[`strokeLinecap`,`stroke-linecap`],[`strokeLinejoin`,`stroke-linejoin`],[`strokeMiterlimit`,`stroke-miterlimit`],[`strokeOpacity`,`stroke-opacity`],[`strokeWidth`,`stroke-width`],[`textAnchor`,`text-anchor`],[`textDecoration`,`text-decoration`],[`textRendering`,`text-rendering`],[`transformOrigin`,`transform-origin`],[`underlinePosition`,`underline-position`],[`underlineThickness`,`underline-thickness`],[`unicodeBidi`,`unicode-bidi`],[`unicodeRange`,`unicode-range`],[`unitsPerEm`,`units-per-em`],[`vAlphabetic`,`v-alphabetic`],[`vHanging`,`v-hanging`],[`vIdeographic`,`v-ideographic`],[`vMathematical`,`v-mathematical`],[`vectorEffect`,`vector-effect`],[`vertAdvY`,`vert-adv-y`],[`vertOriginX`,`vert-origin-x`],[`vertOriginY`,`vert-origin-y`],[`wordSpacing`,`word-spacing`],[`writingMode`,`writing-mode`],[`xmlnsXlink`,`xmlns:xlink`],[`xHeight`,`x-height`]]),$t=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function en(e){return $t.test(``+e)?`javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`:e}function tn(){}var nn=null;function rn(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var an=null,on=null;function sn(e){var t=vt(e);if(t&&(e=t.stateNode)){var n=e[I]||null;a:switch(e=t.stateNode,t.type){case`input`:if(Vt(e,n.value,n.defaultValue,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name),t=n.name,n.type===`radio`&&t!=null){for(n=e;n.parentNode;)n=n.parentNode;for(n=n.querySelectorAll(`input[name="`+Bt(``+t)+`"][type="radio"]`),t=0;t<n.length;t++){var r=n[t];if(r!==e&&r.form===e.form){var i=r[I]||null;if(!i)throw Error(a(90));Vt(r,i.value,i.defaultValue,i.defaultValue,i.checked,i.defaultChecked,i.type,i.name)}}for(t=0;t<n.length;t++)r=n[t],r.form===e.form&&Lt(r)}break a;case`textarea`:Gt(e,n.value,n.defaultValue);break a;case`select`:t=n.value,t!=null&&Wt(e,!!n.multiple,t,!1)}}}var cn=!1;function ln(e,t,n){if(cn)return e(t,n);cn=!0;try{return e(t)}finally{if(cn=!1,(an!==null||on!==null)&&(bu(),an&&(t=an,e=on,on=an=null,sn(t),e)))for(t=0;t<e.length;t++)sn(e[t])}}function un(e,t){var n=e.stateNode;if(n===null)return null;var r=n[I]||null;if(r===null)return null;n=r[t];a:switch(t){case`onClick`:case`onClickCapture`:case`onDoubleClick`:case`onDoubleClickCapture`:case`onMouseDown`:case`onMouseDownCapture`:case`onMouseMove`:case`onMouseMoveCapture`:case`onMouseUp`:case`onMouseUpCapture`:case`onMouseEnter`:(r=!r.disabled)||(e=e.type,r=!(e===`button`||e===`input`||e===`select`||e===`textarea`)),e=!r;break a;default:e=!1}if(e)return null;if(n&&typeof n!=`function`)throw Error(a(231,t,typeof n));return n}var dn=!(typeof window>`u`||window.document===void 0||window.document.createElement===void 0),fn=!1;if(dn)try{var pn={};Object.defineProperty(pn,"passive",{get:function(){fn=!0}}),window.addEventListener(`test`,pn,pn),window.removeEventListener(`test`,pn,pn)}catch{fn=!1}var mn=null,hn=null,gn=null;function _n(){if(gn)return gn;var e,t=hn,n=t.length,r,i=`value`in mn?mn.value:mn.textContent,a=i.length;for(e=0;e<n&&t[e]===i[e];e++);var o=n-e;for(r=1;r<=o&&t[n-r]===i[a-r];r++);return gn=i.slice(e,1<r?1-r:void 0)}function vn(e){var t=e.keyCode;return`charCode`in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function yn(){return!0}function bn(){return!1}function xn(e){function t(t,n,r,i,a){for(var o in this._reactName=t,this._targetInst=r,this.type=n,this.nativeEvent=i,this.target=a,this.currentTarget=null,e)e.hasOwnProperty(o)&&(t=e[o],this[o]=t?t(i):i[o]);return this.isDefaultPrevented=(i.defaultPrevented==null?!1===i.returnValue:i.defaultPrevented)?yn:bn,this.isPropagationStopped=bn,this}return m(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var e=this.nativeEvent;e&&(e.preventDefault?e.preventDefault():typeof e.returnValue!=`unknown`&&(e.returnValue=!1),this.isDefaultPrevented=yn)},stopPropagation:function(){var e=this.nativeEvent;e&&(e.stopPropagation?e.stopPropagation():typeof e.cancelBubble!=`unknown`&&(e.cancelBubble=!0),this.isPropagationStopped=yn)},persist:function(){},isPersistent:yn}),t}var Sn={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},Cn=xn(Sn),wn=m({},Sn,{view:0,detail:0}),Tn=xn(wn),En,Dn,On,kn=m({},wn,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Bn,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return`movementX`in e?e.movementX:(e!==On&&(On&&e.type===`mousemove`?(En=e.screenX-On.screenX,Dn=e.screenY-On.screenY):Dn=En=0,On=e),En)},movementY:function(e){return`movementY`in e?e.movementY:Dn}}),An=xn(kn),jn=xn(m({},kn,{dataTransfer:0})),Mn=xn(m({},wn,{relatedTarget:0})),Nn=xn(m({},Sn,{animationName:0,elapsedTime:0,pseudoElement:0})),Pn=xn(m({},Sn,{clipboardData:function(e){return`clipboardData`in e?e.clipboardData:window.clipboardData}})),Fn=xn(m({},Sn,{data:0})),In={Esc:`Escape`,Spacebar:` `,Left:`ArrowLeft`,Up:`ArrowUp`,Right:`ArrowRight`,Down:`ArrowDown`,Del:`Delete`,Win:`OS`,Menu:`ContextMenu`,Apps:`ContextMenu`,Scroll:`ScrollLock`,MozPrintableKey:`Unidentified`},Ln={8:`Backspace`,9:`Tab`,12:`Clear`,13:`Enter`,16:`Shift`,17:`Control`,18:`Alt`,19:`Pause`,20:`CapsLock`,27:`Escape`,32:` `,33:`PageUp`,34:`PageDown`,35:`End`,36:`Home`,37:`ArrowLeft`,38:`ArrowUp`,39:`ArrowRight`,40:`ArrowDown`,45:`Insert`,46:`Delete`,112:`F1`,113:`F2`,114:`F3`,115:`F4`,116:`F5`,117:`F6`,118:`F7`,119:`F8`,120:`F9`,121:`F10`,122:`F11`,123:`F12`,144:`NumLock`,145:`ScrollLock`,224:`Meta`},Rn={Alt:`altKey`,Control:`ctrlKey`,Meta:`metaKey`,Shift:`shiftKey`};function zn(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=Rn[e])?!!t[e]:!1}function Bn(){return zn}var Vn=xn(m({},wn,{key:function(e){if(e.key){var t=In[e.key]||e.key;if(t!==`Unidentified`)return t}return e.type===`keypress`?(e=vn(e),e===13?`Enter`:String.fromCharCode(e)):e.type===`keydown`||e.type===`keyup`?Ln[e.keyCode]||`Unidentified`:``},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Bn,charCode:function(e){return e.type===`keypress`?vn(e):0},keyCode:function(e){return e.type===`keydown`||e.type===`keyup`?e.keyCode:0},which:function(e){return e.type===`keypress`?vn(e):e.type===`keydown`||e.type===`keyup`?e.keyCode:0}})),Hn=xn(m({},kn,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0})),Un=xn(m({},wn,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Bn})),Wn=xn(m({},Sn,{propertyName:0,elapsedTime:0,pseudoElement:0})),Gn=xn(m({},kn,{deltaX:function(e){return`deltaX`in e?e.deltaX:`wheelDeltaX`in e?-e.wheelDeltaX:0},deltaY:function(e){return`deltaY`in e?e.deltaY:`wheelDeltaY`in e?-e.wheelDeltaY:`wheelDelta`in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0})),Kn=xn(m({},Sn,{newState:0,oldState:0})),qn=[9,13,27,32],Jn=dn&&`CompositionEvent`in window,Yn=null;dn&&`documentMode`in document&&(Yn=document.documentMode);var Xn=dn&&`TextEvent`in window&&!Yn,Zn=dn&&(!Jn||Yn&&8<Yn&&11>=Yn),Qn=` `,$n=!1;function er(e,t){switch(e){case`keyup`:return qn.indexOf(t.keyCode)!==-1;case`keydown`:return t.keyCode!==229;case`keypress`:case`mousedown`:case`focusout`:return!0;default:return!1}}function tr(e){return e=e.detail,typeof e==`object`&&`data`in e?e.data:null}var nr=!1;function rr(e,t){switch(e){case`compositionend`:return tr(t);case`keypress`:return t.which===32?($n=!0,Qn):null;case`textInput`:return e=t.data,e===Qn&&$n?null:e;default:return null}}function ir(e,t){if(nr)return e===`compositionend`||!Jn&&er(e,t)?(e=_n(),gn=hn=mn=null,nr=!1,e):null;switch(e){case`paste`:return null;case`keypress`:if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case`compositionend`:return Zn&&t.locale!==`ko`?null:t.data;default:return null}}var ar={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function or(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t===`input`?!!ar[e.type]:t===`textarea`}function sr(e,t,n,r){an?on?on.push(r):on=[r]:an=r,t=Ed(t,`onChange`),0<t.length&&(n=new Cn(`onChange`,`change`,null,n,r),e.push({event:n,listeners:t}))}var cr=null,lr=null;function ur(e){yd(e,0)}function dr(e){if(Lt(yt(e)))return e}function fr(e,t){if(e===`change`)return t}var pr=!1;if(dn){var mr;if(dn){var hr=`oninput`in document;if(!hr){var gr=document.createElement(`div`);gr.setAttribute(`oninput`,`return;`),hr=typeof gr.oninput==`function`}mr=hr}else mr=!1;pr=mr&&(!document.documentMode||9<document.documentMode)}function _r(){cr&&(cr.detachEvent(`onpropertychange`,vr),lr=cr=null)}function vr(e){if(e.propertyName===`value`&&dr(lr)){var t=[];sr(t,lr,e,rn(e)),ln(ur,t)}}function yr(e,t,n){e===`focusin`?(_r(),cr=t,lr=n,cr.attachEvent(`onpropertychange`,vr)):e===`focusout`&&_r()}function br(e){if(e===`selectionchange`||e===`keyup`||e===`keydown`)return dr(lr)}function xr(e,t){if(e===`click`)return dr(t)}function Sr(e,t){if(e===`input`||e===`change`)return dr(t)}function Cr(e,t){return e===t&&(e!==0||1/e==1/t)||e!==e&&t!==t}var wr=typeof Object.is==`function`?Object.is:Cr;function Tr(e,t){if(wr(e,t))return!0;if(typeof e!=`object`||!e||typeof t!=`object`||!t)return!1;var n=Object.keys(e),r=Object.keys(t);if(n.length!==r.length)return!1;for(r=0;r<n.length;r++){var i=n[r];if(!Ce.call(t,i)||!wr(e[i],t[i]))return!1}return!0}function Er(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function Dr(e,t){var n=Er(e);e=0;for(var r;n;){if(n.nodeType===3){if(r=e+n.textContent.length,e<=t&&r>=t)return{node:n,offset:t-e};e=r}a:{for(;n;){if(n.nextSibling){n=n.nextSibling;break a}n=n.parentNode}n=void 0}n=Er(n)}}function Or(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Or(e,t.parentNode):`contains`in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function kr(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=Rt(e.document);t instanceof e.HTMLIFrameElement;){try{var n=typeof t.contentWindow.location.href==`string`}catch{n=!1}if(n)e=t.contentWindow;else break;t=Rt(e.document)}return t}function Ar(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t===`input`&&(e.type===`text`||e.type===`search`||e.type===`tel`||e.type===`url`||e.type===`password`)||t===`textarea`||e.contentEditable===`true`)}var jr=dn&&`documentMode`in document&&11>=document.documentMode,Mr=null,Nr=null,Pr=null,Fr=!1;function Ir(e,t,n){var r=n.window===n?n.document:n.nodeType===9?n:n.ownerDocument;Fr||Mr==null||Mr!==Rt(r)||(r=Mr,`selectionStart`in r&&Ar(r)?r={start:r.selectionStart,end:r.selectionEnd}:(r=(r.ownerDocument&&r.ownerDocument.defaultView||window).getSelection(),r={anchorNode:r.anchorNode,anchorOffset:r.anchorOffset,focusNode:r.focusNode,focusOffset:r.focusOffset}),Pr&&Tr(Pr,r)||(Pr=r,r=Ed(Nr,`onSelect`),0<r.length&&(t=new Cn(`onSelect`,`select`,null,t,n),e.push({event:t,listeners:r}),t.target=Mr)))}function Lr(e,t){var n={};return n[e.toLowerCase()]=t.toLowerCase(),n[`Webkit`+e]=`webkit`+t,n[`Moz`+e]=`moz`+t,n}var Rr={animationend:Lr(`Animation`,`AnimationEnd`),animationiteration:Lr(`Animation`,`AnimationIteration`),animationstart:Lr(`Animation`,`AnimationStart`),transitionrun:Lr(`Transition`,`TransitionRun`),transitionstart:Lr(`Transition`,`TransitionStart`),transitioncancel:Lr(`Transition`,`TransitionCancel`),transitionend:Lr(`Transition`,`TransitionEnd`)},zr={},Br={};dn&&(Br=document.createElement(`div`).style,`AnimationEvent`in window||(delete Rr.animationend.animation,delete Rr.animationiteration.animation,delete Rr.animationstart.animation),`TransitionEvent`in window||delete Rr.transitionend.transition);function Vr(e){if(zr[e])return zr[e];if(!Rr[e])return e;var t=Rr[e],n;for(n in t)if(t.hasOwnProperty(n)&&n in Br)return zr[e]=t[n];return e}var Hr=Vr(`animationend`),Ur=Vr(`animationiteration`),Wr=Vr(`animationstart`),L=Vr(`transitionrun`),Gr=Vr(`transitionstart`),Kr=Vr(`transitioncancel`),qr=Vr(`transitionend`),Jr=new Map,Yr=`abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(` `);Yr.push(`scrollEnd`);function Xr(e,t){Jr.set(e,t),wt(t,[e])}var Zr=typeof reportError==`function`?reportError:function(e){if(typeof window==`object`&&typeof window.ErrorEvent==`function`){var t=new window.ErrorEvent(`error`,{bubbles:!0,cancelable:!0,message:typeof e==`object`&&e&&typeof e.message==`string`?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process==`object`&&typeof process.emit==`function`){process.emit(`uncaughtException`,e);return}console.error(e)},Qr=[],$r=0,ei=0;function ti(){for(var e=$r,t=ei=$r=0;t<e;){var n=Qr[t];Qr[t++]=null;var r=Qr[t];Qr[t++]=null;var i=Qr[t];Qr[t++]=null;var a=Qr[t];if(Qr[t++]=null,r!==null&&i!==null){var o=r.pending;o===null?i.next=i:(i.next=o.next,o.next=i),r.pending=i}a!==0&&ai(n,i,a)}}function ni(e,t,n,r){Qr[$r++]=e,Qr[$r++]=t,Qr[$r++]=n,Qr[$r++]=r,ei|=r,e.lanes|=r,e=e.alternate,e!==null&&(e.lanes|=r)}function ri(e,t,n,r){return ni(e,t,n,r),oi(e)}function ii(e,t){return ni(e,null,null,t),oi(e)}function ai(e,t,n){e.lanes|=n;var r=e.alternate;r!==null&&(r.lanes|=n);for(var i=!1,a=e.return;a!==null;)a.childLanes|=n,r=a.alternate,r!==null&&(r.childLanes|=n),a.tag===22&&(e=a.stateNode,e===null||e._visibility&1||(i=!0)),e=a,a=a.return;return e.tag===3?(a=e.stateNode,i&&t!==null&&(i=31-Be(n),e=a.hiddenUpdates,r=e[i],r===null?e[i]=[t]:r.push(t),t.lane=n|536870912),a):null}function oi(e){if(50<du)throw du=0,fu=null,Error(a(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var si={};function ci(e,t,n,r){this.tag=e,this.key=n,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=r,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function li(e,t,n,r){return new ci(e,t,n,r)}function ui(e){return e=e.prototype,!(!e||!e.isReactComponent)}function di(e,t){var n=e.alternate;return n===null?(n=li(e.tag,t,e.key,e.mode),n.elementType=e.elementType,n.type=e.type,n.stateNode=e.stateNode,n.alternate=e,e.alternate=n):(n.pendingProps=t,n.type=e.type,n.flags=0,n.subtreeFlags=0,n.deletions=null),n.flags=e.flags&65011712,n.childLanes=e.childLanes,n.lanes=e.lanes,n.child=e.child,n.memoizedProps=e.memoizedProps,n.memoizedState=e.memoizedState,n.updateQueue=e.updateQueue,t=e.dependencies,n.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},n.sibling=e.sibling,n.index=e.index,n.ref=e.ref,n.refCleanup=e.refCleanup,n}function fi(e,t){e.flags&=65011714;var n=e.alternate;return n===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=n.childLanes,e.lanes=n.lanes,e.child=n.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=n.memoizedProps,e.memoizedState=n.memoizedState,e.updateQueue=n.updateQueue,e.type=n.type,t=n.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function pi(e,t,n,r,i,o){var s=0;if(r=e,typeof e==`function`)ui(e)&&(s=1);else if(typeof e==`string`)s=Uf(e,n,le.current)?26:e===`html`||e===`head`||e===`body`?27:5;else a:switch(e){case E:return e=li(31,n,t,i),e.elementType=E,e.lanes=o,e;case v:return mi(n.children,i,o,t);case y:s=8,i|=24;break;case b:return e=li(12,n,t,i|2),e.elementType=b,e.lanes=o,e;case w:return e=li(13,n,t,i),e.elementType=w,e.lanes=o,e;case ee:return e=li(19,n,t,i),e.elementType=ee,e.lanes=o,e;default:if(typeof e==`object`&&e)switch(e.$$typeof){case S:s=10;break a;case x:s=9;break a;case C:s=11;break a;case te:s=14;break a;case T:s=16,r=null;break a}s=29,n=Error(a(130,e===null?`null`:typeof e,``)),r=null}return t=li(s,n,t,i),t.elementType=e,t.type=r,t.lanes=o,t}function mi(e,t,n,r){return e=li(7,e,r,t),e.lanes=n,e}function hi(e,t,n){return e=li(6,e,null,t),e.lanes=n,e}function gi(e){var t=li(18,null,null,0);return t.stateNode=e,t}function _i(e,t,n){return t=li(4,e.children===null?[]:e.children,e.key,t),t.lanes=n,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var vi=new WeakMap;function yi(e,t){if(typeof e==`object`&&e){var n=vi.get(e);return n===void 0?(t={value:e,source:t,stack:Se(t)},vi.set(e,t),t):n}return{value:e,source:t,stack:Se(t)}}var bi=[],xi=0,Si=null,Ci=0,wi=[],Ti=0,Ei=null,Di=1,Oi=``;function ki(e,t){bi[xi++]=Ci,bi[xi++]=Si,Si=e,Ci=t}function Ai(e,t,n){wi[Ti++]=Di,wi[Ti++]=Oi,wi[Ti++]=Ei,Ei=e;var r=Di;e=Oi;var i=32-Be(r)-1;r&=~(1<<i),n+=1;var a=32-Be(t)+i;if(30<a){var o=i-i%5;a=(r&(1<<o)-1).toString(32),r>>=o,i-=o,Di=1<<32-Be(t)+i|n<<i|r,Oi=a+e}else Di=1<<a|n<<i|r,Oi=e}function ji(e){e.return!==null&&(ki(e,1),Ai(e,1,0))}function Mi(e){for(;e===Si;)Si=bi[--xi],bi[xi]=null,Ci=bi[--xi],bi[xi]=null;for(;e===Ei;)Ei=wi[--Ti],wi[Ti]=null,Oi=wi[--Ti],wi[Ti]=null,Di=wi[--Ti],wi[Ti]=null}function Ni(e,t){wi[Ti++]=Di,wi[Ti++]=Oi,wi[Ti++]=Ei,Di=t.id,Oi=t.overflow,Ei=e}var Pi=null,Fi=null,R=!1,Ii=null,Li=!1,Ri=Error(a(519));function zi(e){throw Gi(yi(Error(a(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?`text`:`HTML`,``)),e)),Ri}function Bi(e){var t=e.stateNode,n=e.type,r=e.memoizedProps;switch(t[lt]=e,t[I]=r,n){case`dialog`:Q(`cancel`,t),Q(`close`,t);break;case`iframe`:case`object`:case`embed`:Q(`load`,t);break;case`video`:case`audio`:for(n=0;n<_d.length;n++)Q(_d[n],t);break;case`source`:Q(`error`,t);break;case`img`:case`image`:case`link`:Q(`error`,t),Q(`load`,t);break;case`details`:Q(`toggle`,t);break;case`input`:Q(`invalid`,t),Ht(t,r.value,r.defaultValue,r.checked,r.defaultChecked,r.type,r.name,!0);break;case`select`:Q(`invalid`,t);break;case`textarea`:Q(`invalid`,t),Kt(t,r.value,r.defaultValue,r.children)}n=r.children,typeof n!=`string`&&typeof n!=`number`&&typeof n!=`bigint`||t.textContent===``+n||!0===r.suppressHydrationWarning||Md(t.textContent,n)?(r.popover!=null&&(Q(`beforetoggle`,t),Q(`toggle`,t)),r.onScroll!=null&&Q(`scroll`,t),r.onScrollEnd!=null&&Q(`scrollend`,t),r.onClick!=null&&(t.onclick=tn),t=!0):t=!1,t||zi(e,!0)}function Vi(e){for(Pi=e.return;Pi;)switch(Pi.tag){case 5:case 31:case 13:Li=!1;return;case 27:case 3:Li=!0;return;default:Pi=Pi.return}}function Hi(e){if(e!==Pi)return!1;if(!R)return Vi(e),R=!0,!1;var t=e.tag,n;if((n=t!==3&&t!==27)&&((n=t===5)&&(n=e.type,n=!(n!==`form`&&n!==`button`)||Ud(e.type,e.memoizedProps)),n=!n),n&&Fi&&zi(e),Vi(e),t===13){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));Fi=uf(e)}else if(t===31){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));Fi=uf(e)}else t===27?(t=Fi,Zd(e.type)?(e=lf,lf=null,Fi=e):Fi=t):Fi=Pi?cf(e.stateNode.nextSibling):null;return!0}function Ui(){Fi=Pi=null,R=!1}function Wi(){var e=Ii;return e!==null&&(Zl===null?Zl=e:Zl.push.apply(Zl,e),Ii=null),e}function Gi(e){Ii===null?Ii=[e]:Ii.push(e)}var Ki=M(null),qi=null,Ji=null;function Yi(e,t,n){P(Ki,t._currentValue),t._currentValue=n}function Xi(e){e._currentValue=Ki.current,N(Ki)}function Zi(e,t,n){for(;e!==null;){var r=e.alternate;if((e.childLanes&t)===t?r!==null&&(r.childLanes&t)!==t&&(r.childLanes|=t):(e.childLanes|=t,r!==null&&(r.childLanes|=t)),e===n)break;e=e.return}}function Qi(e,t,n,r){var i=e.child;for(i!==null&&(i.return=e);i!==null;){var o=i.dependencies;if(o!==null){var s=i.child;o=o.firstContext;a:for(;o!==null;){var c=o;o=i;for(var l=0;l<t.length;l++)if(c.context===t[l]){o.lanes|=n,c=o.alternate,c!==null&&(c.lanes|=n),Zi(o.return,n,e),r||(s=null);break a}o=c.next}}else if(i.tag===18){if(s=i.return,s===null)throw Error(a(341));s.lanes|=n,o=s.alternate,o!==null&&(o.lanes|=n),Zi(s,n,e),s=null}else s=i.child;if(s!==null)s.return=i;else for(s=i;s!==null;){if(s===e){s=null;break}if(i=s.sibling,i!==null){i.return=s.return,s=i;break}s=s.return}i=s}}function $i(e,t,n,r){e=null;for(var i=t,o=!1;i!==null;){if(!o){if(i.flags&524288)o=!0;else if(i.flags&262144)break}if(i.tag===10){var s=i.alternate;if(s===null)throw Error(a(387));if(s=s.memoizedProps,s!==null){var c=i.type;wr(i.pendingProps.value,s.value)||(e===null?e=[c]:e.push(c))}}else if(i===fe.current){if(s=i.alternate,s===null)throw Error(a(387));s.memoizedState.memoizedState!==i.memoizedState.memoizedState&&(e===null?e=[Qf]:e.push(Qf))}i=i.return}e!==null&&Qi(t,e,n,r),t.flags|=262144}function ea(e){for(e=e.firstContext;e!==null;){if(!wr(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function ta(e){qi=e,Ji=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function na(e){return ia(qi,e)}function ra(e,t){return qi===null&&ta(e),ia(e,t)}function ia(e,t){var n=t._currentValue;if(t={context:t,memoizedValue:n,next:null},Ji===null){if(e===null)throw Error(a(308));Ji=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else Ji=Ji.next=t;return n}var aa=typeof AbortController<`u`?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(t,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(e){return e()})}},oa=t.unstable_scheduleCallback,sa=t.unstable_NormalPriority,ca={$$typeof:S,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function la(){return{controller:new aa,data:new Map,refCount:0}}function ua(e){e.refCount--,e.refCount===0&&oa(sa,function(){e.controller.abort()})}var da=null,fa=0,pa=0,ma=null;function ha(e,t){if(da===null){var n=da=[];fa=0,pa=dd(),ma={status:`pending`,value:void 0,then:function(e){n.push(e)}}}return fa++,t.then(ga,ga),t}function ga(){if(--fa===0&&da!==null){ma!==null&&(ma.status=`fulfilled`);var e=da;da=null,pa=0,ma=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function _a(e,t){var n=[],r={status:`pending`,value:null,reason:null,then:function(e){n.push(e)}};return e.then(function(){r.status=`fulfilled`,r.value=t;for(var e=0;e<n.length;e++)(0,n[e])(t)},function(e){for(r.status=`rejected`,r.reason=e,e=0;e<n.length;e++)(0,n[e])(void 0)}),r}var va=O.S;O.S=function(e,t){eu=Oe(),typeof t==`object`&&t&&typeof t.then==`function`&&ha(e,t),va!==null&&va(e,t)};var ya=M(null);function ba(){var e=ya.current;return e===null?q.pooledCache:e}function xa(e,t){t===null?P(ya,ya.current):P(ya,t.pool)}function Sa(){var e=ba();return e===null?null:{parent:ca._currentValue,pool:e}}var Ca=Error(a(460)),wa=Error(a(474)),Ta=Error(a(542)),Ea={then:function(){}};function Da(e){return e=e.status,e===`fulfilled`||e===`rejected`}function Oa(e,t,n){switch(n=e[n],n===void 0?e.push(t):n!==t&&(t.then(tn,tn),t=n),t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Ma(e),e;default:if(typeof t.status==`string`)t.then(tn,tn);else{if(e=q,e!==null&&100<e.shellSuspendCounter)throw Error(a(482));e=t,e.status=`pending`,e.then(function(e){if(t.status===`pending`){var n=t;n.status=`fulfilled`,n.value=e}},function(e){if(t.status===`pending`){var n=t;n.status=`rejected`,n.reason=e}})}switch(t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Ma(e),e}throw Aa=t,Ca}}function ka(e){try{var t=e._init;return t(e._payload)}catch(e){throw typeof e==`object`&&e&&typeof e.then==`function`?(Aa=e,Ca):e}}var Aa=null;function ja(){if(Aa===null)throw Error(a(459));var e=Aa;return Aa=null,e}function Ma(e){if(e===Ca||e===Ta)throw Error(a(483))}var Na=null,Pa=0;function Fa(e){var t=Pa;return Pa+=1,Na===null&&(Na=[]),Oa(Na,e,t)}function Ia(e,t){t=t.props.ref,e.ref=t===void 0?null:t}function La(e,t){throw t.$$typeof===h?Error(a(525)):(e=Object.prototype.toString.call(t),Error(a(31,e===`[object Object]`?`object with keys {`+Object.keys(t).join(`, `)+`}`:e)))}function Ra(e){function t(t,n){if(e){var r=t.deletions;r===null?(t.deletions=[n],t.flags|=16):r.push(n)}}function n(n,r){if(!e)return null;for(;r!==null;)t(n,r),r=r.sibling;return null}function r(e){for(var t=new Map;e!==null;)e.key===null?t.set(e.index,e):t.set(e.key,e),e=e.sibling;return t}function i(e,t){return e=di(e,t),e.index=0,e.sibling=null,e}function o(t,n,r){return t.index=r,e?(r=t.alternate,r===null?(t.flags|=67108866,n):(r=r.index,r<n?(t.flags|=67108866,n):r)):(t.flags|=1048576,n)}function s(t){return e&&t.alternate===null&&(t.flags|=67108866),t}function c(e,t,n,r){return t===null||t.tag!==6?(t=hi(n,e.mode,r),t.return=e,t):(t=i(t,n),t.return=e,t)}function l(e,t,n,r){var a=n.type;return a===v?d(e,t,n.props.children,r,n.key):t!==null&&(t.elementType===a||typeof a==`object`&&a&&a.$$typeof===T&&ka(a)===t.type)?(t=i(t,n.props),Ia(t,n),t.return=e,t):(t=pi(n.type,n.key,n.props,null,e.mode,r),Ia(t,n),t.return=e,t)}function u(e,t,n,r){return t===null||t.tag!==4||t.stateNode.containerInfo!==n.containerInfo||t.stateNode.implementation!==n.implementation?(t=_i(n,e.mode,r),t.return=e,t):(t=i(t,n.children||[]),t.return=e,t)}function d(e,t,n,r,a){return t===null||t.tag!==7?(t=mi(n,e.mode,r,a),t.return=e,t):(t=i(t,n),t.return=e,t)}function f(e,t,n){if(typeof t==`string`&&t!==``||typeof t==`number`||typeof t==`bigint`)return t=hi(``+t,e.mode,n),t.return=e,t;if(typeof t==`object`&&t){switch(t.$$typeof){case g:return n=pi(t.type,t.key,t.props,null,e.mode,n),Ia(n,t),n.return=e,n;case _:return t=_i(t,e.mode,n),t.return=e,t;case T:return t=ka(t),f(e,t,n)}if(oe(t)||re(t))return t=mi(t,e.mode,n,null),t.return=e,t;if(typeof t.then==`function`)return f(e,Fa(t),n);if(t.$$typeof===S)return f(e,ra(e,t),n);La(e,t)}return null}function p(e,t,n,r){var i=t===null?null:t.key;if(typeof n==`string`&&n!==``||typeof n==`number`||typeof n==`bigint`)return i===null?c(e,t,``+n,r):null;if(typeof n==`object`&&n){switch(n.$$typeof){case g:return n.key===i?l(e,t,n,r):null;case _:return n.key===i?u(e,t,n,r):null;case T:return n=ka(n),p(e,t,n,r)}if(oe(n)||re(n))return i===null?d(e,t,n,r,null):null;if(typeof n.then==`function`)return p(e,t,Fa(n),r);if(n.$$typeof===S)return p(e,t,ra(e,n),r);La(e,n)}return null}function m(e,t,n,r,i){if(typeof r==`string`&&r!==``||typeof r==`number`||typeof r==`bigint`)return e=e.get(n)||null,c(t,e,``+r,i);if(typeof r==`object`&&r){switch(r.$$typeof){case g:return e=e.get(r.key===null?n:r.key)||null,l(t,e,r,i);case _:return e=e.get(r.key===null?n:r.key)||null,u(t,e,r,i);case T:return r=ka(r),m(e,t,n,r,i)}if(oe(r)||re(r))return e=e.get(n)||null,d(t,e,r,i,null);if(typeof r.then==`function`)return m(e,t,n,Fa(r),i);if(r.$$typeof===S)return m(e,t,n,ra(t,r),i);La(t,r)}return null}function h(i,a,s,c){for(var l=null,u=null,d=a,h=a=0,g=null;d!==null&&h<s.length;h++){d.index>h?(g=d,d=null):g=d.sibling;var _=p(i,d,s[h],c);if(_===null){d===null&&(d=g);break}e&&d&&_.alternate===null&&t(i,d),a=o(_,a,h),u===null?l=_:u.sibling=_,u=_,d=g}if(h===s.length)return n(i,d),R&&ki(i,h),l;if(d===null){for(;h<s.length;h++)d=f(i,s[h],c),d!==null&&(a=o(d,a,h),u===null?l=d:u.sibling=d,u=d);return R&&ki(i,h),l}for(d=r(d);h<s.length;h++)g=m(d,i,h,s[h],c),g!==null&&(e&&g.alternate!==null&&d.delete(g.key===null?h:g.key),a=o(g,a,h),u===null?l=g:u.sibling=g,u=g);return e&&d.forEach(function(e){return t(i,e)}),R&&ki(i,h),l}function y(i,s,c,l){if(c==null)throw Error(a(151));for(var u=null,d=null,h=s,g=s=0,_=null,v=c.next();h!==null&&!v.done;g++,v=c.next()){h.index>g?(_=h,h=null):_=h.sibling;var y=p(i,h,v.value,l);if(y===null){h===null&&(h=_);break}e&&h&&y.alternate===null&&t(i,h),s=o(y,s,g),d===null?u=y:d.sibling=y,d=y,h=_}if(v.done)return n(i,h),R&&ki(i,g),u;if(h===null){for(;!v.done;g++,v=c.next())v=f(i,v.value,l),v!==null&&(s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return R&&ki(i,g),u}for(h=r(h);!v.done;g++,v=c.next())v=m(h,i,g,v.value,l),v!==null&&(e&&v.alternate!==null&&h.delete(v.key===null?g:v.key),s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return e&&h.forEach(function(e){return t(i,e)}),R&&ki(i,g),u}function b(e,r,o,c){if(typeof o==`object`&&o&&o.type===v&&o.key===null&&(o=o.props.children),typeof o==`object`&&o){switch(o.$$typeof){case g:a:{for(var l=o.key;r!==null;){if(r.key===l){if(l=o.type,l===v){if(r.tag===7){n(e,r.sibling),c=i(r,o.props.children),c.return=e,e=c;break a}}else if(r.elementType===l||typeof l==`object`&&l&&l.$$typeof===T&&ka(l)===r.type){n(e,r.sibling),c=i(r,o.props),Ia(c,o),c.return=e,e=c;break a}n(e,r);break}else t(e,r);r=r.sibling}o.type===v?(c=mi(o.props.children,e.mode,c,o.key),c.return=e,e=c):(c=pi(o.type,o.key,o.props,null,e.mode,c),Ia(c,o),c.return=e,e=c)}return s(e);case _:a:{for(l=o.key;r!==null;){if(r.key===l)if(r.tag===4&&r.stateNode.containerInfo===o.containerInfo&&r.stateNode.implementation===o.implementation){n(e,r.sibling),c=i(r,o.children||[]),c.return=e,e=c;break a}else{n(e,r);break}else t(e,r);r=r.sibling}c=_i(o,e.mode,c),c.return=e,e=c}return s(e);case T:return o=ka(o),b(e,r,o,c)}if(oe(o))return h(e,r,o,c);if(re(o)){if(l=re(o),typeof l!=`function`)throw Error(a(150));return o=l.call(o),y(e,r,o,c)}if(typeof o.then==`function`)return b(e,r,Fa(o),c);if(o.$$typeof===S)return b(e,r,ra(e,o),c);La(e,o)}return typeof o==`string`&&o!==``||typeof o==`number`||typeof o==`bigint`?(o=``+o,r!==null&&r.tag===6?(n(e,r.sibling),c=i(r,o),c.return=e,e=c):(n(e,r),c=hi(o,e.mode,c),c.return=e,e=c),s(e)):n(e,r)}return function(e,t,n,r){try{Pa=0;var i=b(e,t,n,r);return Na=null,i}catch(t){if(t===Ca||t===Ta)throw t;var a=li(29,t,null,e.mode);return a.lanes=r,a.return=e,a}}}var za=Ra(!0),Ba=Ra(!1),Va=!1;function Ha(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function Ua(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Wa(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function Ga(e,t,n){var r=e.updateQueue;if(r===null)return null;if(r=r.shared,K&2){var i=r.pending;return i===null?t.next=t:(t.next=i.next,i.next=t),r.pending=t,t=oi(e),ai(e,null,n),t}return ni(e,r,t,n),oi(e)}function Ka(e,t,n){if(t=t.updateQueue,t!==null&&(t=t.shared,n&4194048)){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,nt(e,n)}}function qa(e,t){var n=e.updateQueue,r=e.alternate;if(r!==null&&(r=r.updateQueue,n===r)){var i=null,a=null;if(n=n.firstBaseUpdate,n!==null){do{var o={lane:n.lane,tag:n.tag,payload:n.payload,callback:null,next:null};a===null?i=a=o:a=a.next=o,n=n.next}while(n!==null);a===null?i=a=t:a=a.next=t}else i=a=t;n={baseState:r.baseState,firstBaseUpdate:i,lastBaseUpdate:a,shared:r.shared,callbacks:r.callbacks},e.updateQueue=n;return}e=n.lastBaseUpdate,e===null?n.firstBaseUpdate=t:e.next=t,n.lastBaseUpdate=t}var Ja=!1;function Ya(){if(Ja){var e=ma;if(e!==null)throw e}}function Xa(e,t,n,r){Ja=!1;var i=e.updateQueue;Va=!1;var a=i.firstBaseUpdate,o=i.lastBaseUpdate,s=i.shared.pending;if(s!==null){i.shared.pending=null;var c=s,l=c.next;c.next=null,o===null?a=l:o.next=l,o=c;var u=e.alternate;u!==null&&(u=u.updateQueue,s=u.lastBaseUpdate,s!==o&&(s===null?u.firstBaseUpdate=l:s.next=l,u.lastBaseUpdate=c))}if(a!==null){var d=i.baseState;o=0,u=l=c=null,s=a;do{var f=s.lane&-536870913,p=f!==s.lane;if(p?(Y&f)===f:(r&f)===f){f!==0&&f===pa&&(Ja=!0),u!==null&&(u=u.next={lane:0,tag:s.tag,payload:s.payload,callback:null,next:null});a:{var h=e,g=s;f=t;var _=n;switch(g.tag){case 1:if(h=g.payload,typeof h==`function`){d=h.call(_,d,f);break a}d=h;break a;case 3:h.flags=h.flags&-65537|128;case 0:if(h=g.payload,f=typeof h==`function`?h.call(_,d,f):h,f==null)break a;d=m({},d,f);break a;case 2:Va=!0}}f=s.callback,f!==null&&(e.flags|=64,p&&(e.flags|=8192),p=i.callbacks,p===null?i.callbacks=[f]:p.push(f))}else p={lane:f,tag:s.tag,payload:s.payload,callback:s.callback,next:null},u===null?(l=u=p,c=d):u=u.next=p,o|=f;if(s=s.next,s===null){if(s=i.shared.pending,s===null)break;p=s,s=p.next,p.next=null,i.lastBaseUpdate=p,i.shared.pending=null}}while(1);u===null&&(c=d),i.baseState=c,i.firstBaseUpdate=l,i.lastBaseUpdate=u,a===null&&(i.shared.lanes=0),Gl|=o,e.lanes=o,e.memoizedState=d}}function Za(e,t){if(typeof e!=`function`)throw Error(a(191,e));e.call(t)}function Qa(e,t){var n=e.callbacks;if(n!==null)for(e.callbacks=null,e=0;e<n.length;e++)Za(n[e],t)}var $a=M(null),eo=M(0);function to(e,t){e=Ul,P(eo,e),P($a,t),Ul=e|t.baseLanes}function no(){P(eo,Ul),P($a,$a.current)}function ro(){Ul=eo.current,N($a),N(eo)}var io=M(null),ao=null;function oo(e){var t=e.alternate;P(fo,fo.current&1),P(io,e),ao===null&&(t===null||$a.current!==null||t.memoizedState!==null)&&(ao=e)}function so(e){P(fo,fo.current),P(io,e),ao===null&&(ao=e)}function co(e){e.tag===22?(P(fo,fo.current),P(io,e),ao===null&&(ao=e)):lo(e)}function lo(){P(fo,fo.current),P(io,io.current)}function uo(e){N(io),ao===e&&(ao=null),N(fo)}var fo=M(0);function po(e){for(var t=e;t!==null;){if(t.tag===13){var n=t.memoizedState;if(n!==null&&(n=n.dehydrated,n===null||af(n)||of(n)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder===`forwards`||t.memoizedProps.revealOrder===`backwards`||t.memoizedProps.revealOrder===`unstable_legacy-backwards`||t.memoizedProps.revealOrder===`together`)){if(t.flags&128)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var mo=0,z=null,B=null,ho=null,go=!1,_o=!1,vo=!1,yo=0,bo=0,xo=null,So=0;function Co(){throw Error(a(321))}function wo(e,t){if(t===null)return!1;for(var n=0;n<t.length&&n<e.length;n++)if(!wr(e[n],t[n]))return!1;return!0}function To(e,t,n,r,i,a){return mo=a,z=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,O.H=e===null||e.memoizedState===null?Rs:zs,vo=!1,a=n(r,i),vo=!1,_o&&(a=Do(t,n,r,i)),Eo(e),a}function Eo(e){O.H=Ls;var t=B!==null&&B.next!==null;if(mo=0,ho=B=z=null,go=!1,bo=0,xo=null,t)throw Error(a(300));e===null||nc||(e=e.dependencies,e!==null&&ea(e)&&(nc=!0))}function Do(e,t,n,r){z=e;var i=0;do{if(_o&&(xo=null),bo=0,_o=!1,25<=i)throw Error(a(301));if(i+=1,ho=B=null,e.updateQueue!=null){var o=e.updateQueue;o.lastEffect=null,o.events=null,o.stores=null,o.memoCache!=null&&(o.memoCache.index=0)}O.H=Bs,o=t(n,r)}while(_o);return o}function Oo(){var e=O.H,t=e.useState()[0];return t=typeof t.then==`function`?Po(t):t,e=e.useState()[0],(B===null?null:B.memoizedState)!==e&&(z.flags|=1024),t}function ko(){var e=yo!==0;return yo=0,e}function Ao(e,t,n){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~n}function jo(e){if(go){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}go=!1}mo=0,ho=B=z=null,_o=!1,bo=yo=0,xo=null}function V(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return ho===null?z.memoizedState=ho=e:ho=ho.next=e,ho}function Mo(){if(B===null){var e=z.alternate;e=e===null?null:e.memoizedState}else e=B.next;var t=ho===null?z.memoizedState:ho.next;if(t!==null)ho=t,B=e;else{if(e===null)throw z.alternate===null?Error(a(467)):Error(a(310));B=e,e={memoizedState:B.memoizedState,baseState:B.baseState,baseQueue:B.baseQueue,queue:B.queue,next:null},ho===null?z.memoizedState=ho=e:ho=ho.next=e}return ho}function No(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Po(e){var t=bo;return bo+=1,xo===null&&(xo=[]),e=Oa(xo,e,t),t=z,(ho===null?t.memoizedState:ho.next)===null&&(t=t.alternate,O.H=t===null||t.memoizedState===null?Rs:zs),e}function Fo(e){if(typeof e==`object`&&e){if(typeof e.then==`function`)return Po(e);if(e.$$typeof===S)return na(e)}throw Error(a(438,String(e)))}function Io(e){var t=null,n=z.updateQueue;if(n!==null&&(t=n.memoCache),t==null){var r=z.alternate;r!==null&&(r=r.updateQueue,r!==null&&(r=r.memoCache,r!=null&&(t={data:r.data.map(function(e){return e.slice()}),index:0})))}if(t??={data:[],index:0},n===null&&(n=No(),z.updateQueue=n),n.memoCache=t,n=t.data[t.index],n===void 0)for(n=t.data[t.index]=Array(e),r=0;r<e;r++)n[r]=ne;return t.index++,n}function Lo(e,t){return typeof t==`function`?t(e):t}function Ro(e){return zo(Mo(),B,e)}function zo(e,t,n){var r=e.queue;if(r===null)throw Error(a(311));r.lastRenderedReducer=n;var i=e.baseQueue,o=r.pending;if(o!==null){if(i!==null){var s=i.next;i.next=o.next,o.next=s}t.baseQueue=i=o,r.pending=null}if(o=e.baseState,i===null)e.memoizedState=o;else{t=i.next;var c=s=null,l=null,u=t,d=!1;do{var f=u.lane&-536870913;if(f===u.lane?(mo&f)===f:(Y&f)===f){var p=u.revertLane;if(p===0)l!==null&&(l=l.next={lane:0,revertLane:0,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null}),f===pa&&(d=!0);else if((mo&p)===p){u=u.next,p===pa&&(d=!0);continue}else f={lane:0,revertLane:u.revertLane,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=f,s=o):l=l.next=f,z.lanes|=p,Gl|=p;f=u.action,vo&&n(o,f),o=u.hasEagerState?u.eagerState:n(o,f)}else p={lane:f,revertLane:u.revertLane,gesture:u.gesture,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=p,s=o):l=l.next=p,z.lanes|=f,Gl|=f;u=u.next}while(u!==null&&u!==t);if(l===null?s=o:l.next=c,!wr(o,e.memoizedState)&&(nc=!0,d&&(n=ma,n!==null)))throw n;e.memoizedState=o,e.baseState=s,e.baseQueue=l,r.lastRenderedState=o}return i===null&&(r.lanes=0),[e.memoizedState,r.dispatch]}function Bo(e){var t=Mo(),n=t.queue;if(n===null)throw Error(a(311));n.lastRenderedReducer=e;var r=n.dispatch,i=n.pending,o=t.memoizedState;if(i!==null){n.pending=null;var s=i=i.next;do o=e(o,s.action),s=s.next;while(s!==i);wr(o,t.memoizedState)||(nc=!0),t.memoizedState=o,t.baseQueue===null&&(t.baseState=o),n.lastRenderedState=o}return[o,r]}function Vo(e,t,n){var r=z,i=Mo(),o=R;if(o){if(n===void 0)throw Error(a(407));n=n()}else n=t();var s=!wr((B||i).memoizedState,n);if(s&&(i.memoizedState=n,nc=!0),i=i.queue,ls(Uo.bind(null,r,i,e),[e]),i.getSnapshot!==t||s||ho!==null&&ho.memoizedState.tag&1){if(r.flags|=2048,W(9,{destroy:void 0},H.bind(null,r,i,n,t),null),q===null)throw Error(a(349));o||mo&127||Ho(r,t,n)}return n}function Ho(e,t,n){e.flags|=16384,e={getSnapshot:t,value:n},t=z.updateQueue,t===null?(t=No(),z.updateQueue=t,t.stores=[e]):(n=t.stores,n===null?t.stores=[e]:n.push(e))}function H(e,t,n,r){t.value=n,t.getSnapshot=r,Wo(t)&&Go(e)}function Uo(e,t,n){return n(function(){Wo(t)&&Go(e)})}function Wo(e){var t=e.getSnapshot;e=e.value;try{var n=t();return!wr(e,n)}catch{return!0}}function Go(e){var t=ii(e,2);t!==null&&hu(t,e,2)}function Ko(e){var t=V();if(typeof e==`function`){var n=e;if(e=n(),vo){ze(!0);try{n()}finally{ze(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Lo,lastRenderedState:e},t}function qo(e,t,n,r){return e.baseState=n,zo(e,B,typeof r==`function`?r:Lo)}function Jo(e,t,n,r,i){if(Ps(e))throw Error(a(485));if(e=t.action,e!==null){var o={payload:i,action:e,next:null,isTransition:!0,status:`pending`,value:null,reason:null,listeners:[],then:function(e){o.listeners.push(e)}};O.T===null?o.isTransition=!1:n(!0),r(o),n=t.pending,n===null?(o.next=t.pending=o,Yo(t,o)):(o.next=n.next,t.pending=n.next=o)}}function Yo(e,t){var n=t.action,r=t.payload,i=e.state;if(t.isTransition){var a=O.T,o={};O.T=o;try{var s=n(i,r),c=O.S;c!==null&&c(o,s),Xo(e,t,s)}catch(n){Qo(e,t,n)}finally{a!==null&&o.types!==null&&(a.types=o.types),O.T=a}}else try{a=n(i,r),Xo(e,t,a)}catch(n){Qo(e,t,n)}}function Xo(e,t,n){typeof n==`object`&&n&&typeof n.then==`function`?n.then(function(n){Zo(e,t,n)},function(n){return Qo(e,t,n)}):Zo(e,t,n)}function Zo(e,t,n){t.status=`fulfilled`,t.value=n,$o(t),e.state=n,t=e.pending,t!==null&&(n=t.next,n===t?e.pending=null:(n=n.next,t.next=n,Yo(e,n)))}function Qo(e,t,n){var r=e.pending;if(e.pending=null,r!==null){r=r.next;do t.status=`rejected`,t.reason=n,$o(t),t=t.next;while(t!==r)}e.action=null}function $o(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function U(e,t){return t}function es(e,t){if(R){var n=q.formState;if(n!==null){a:{var r=z;if(R){if(Fi){b:{for(var i=Fi,a=Li;i.nodeType!==8;){if(!a){i=null;break b}if(i=cf(i.nextSibling),i===null){i=null;break b}}a=i.data,i=a===`F!`||a===`F`?i:null}if(i){Fi=cf(i.nextSibling),r=i.data===`F!`;break a}}zi(r)}r=!1}r&&(t=n[0])}}return n=V(),n.memoizedState=n.baseState=t,r={pending:null,lanes:0,dispatch:null,lastRenderedReducer:U,lastRenderedState:t},n.queue=r,n=js.bind(null,z,r),r.dispatch=n,r=Ko(!1),a=Ns.bind(null,z,!1,r.queue),r=V(),i={state:t,dispatch:null,action:e,pending:null},r.queue=i,n=Jo.bind(null,z,i,a,n),i.dispatch=n,r.memoizedState=e,[t,n,!1]}function ts(e){return ns(Mo(),B,e)}function ns(e,t,n){if(t=zo(e,t,U)[0],e=Ro(Lo)[0],typeof t==`object`&&t&&typeof t.then==`function`)try{var r=Po(t)}catch(e){throw e===Ca?Ta:e}else r=t;t=Mo();var i=t.queue,a=i.dispatch;return n!==t.memoizedState&&(z.flags|=2048,W(9,{destroy:void 0},rs.bind(null,i,n),null)),[r,a,e]}function rs(e,t){e.action=t}function is(e){var t=Mo(),n=B;if(n!==null)return ns(t,n,e);Mo(),t=t.memoizedState,n=Mo();var r=n.queue.dispatch;return n.memoizedState=e,[t,r,!1]}function W(e,t,n,r){return e={tag:e,create:n,deps:r,inst:t,next:null},t=z.updateQueue,t===null&&(t=No(),z.updateQueue=t),n=t.lastEffect,n===null?t.lastEffect=e.next=e:(r=n.next,n.next=e,e.next=r,t.lastEffect=e),e}function as(){return Mo().memoizedState}function os(e,t,n,r){var i=V();z.flags|=e,i.memoizedState=W(1|t,{destroy:void 0},n,r===void 0?null:r)}function ss(e,t,n,r){var i=Mo();r=r===void 0?null:r;var a=i.memoizedState.inst;B!==null&&r!==null&&wo(r,B.memoizedState.deps)?i.memoizedState=W(t,a,n,r):(z.flags|=e,i.memoizedState=W(1|t,a,n,r))}function cs(e,t){os(8390656,8,e,t)}function ls(e,t){ss(2048,8,e,t)}function us(e){z.flags|=4;var t=z.updateQueue;if(t===null)t=No(),z.updateQueue=t,t.events=[e];else{var n=t.events;n===null?t.events=[e]:n.push(e)}}function ds(e){var t=Mo().memoizedState;return us({ref:t,nextImpl:e}),function(){if(K&2)throw Error(a(440));return t.impl.apply(void 0,arguments)}}function fs(e,t){return ss(4,2,e,t)}function ps(e,t){return ss(4,4,e,t)}function ms(e,t){if(typeof t==`function`){e=e();var n=t(e);return function(){typeof n==`function`?n():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function hs(e,t,n){n=n==null?null:n.concat([e]),ss(4,4,ms.bind(null,t,e),n)}function gs(){}function _s(e,t){var n=Mo();t=t===void 0?null:t;var r=n.memoizedState;return t!==null&&wo(t,r[1])?r[0]:(n.memoizedState=[e,t],e)}function vs(e,t){var n=Mo();t=t===void 0?null:t;var r=n.memoizedState;if(t!==null&&wo(t,r[1]))return r[0];if(r=e(),vo){ze(!0);try{e()}finally{ze(!1)}}return n.memoizedState=[r,t],r}function ys(e,t,n){return n===void 0||mo&1073741824&&!(Y&261930)?e.memoizedState=t:(e.memoizedState=n,e=mu(),z.lanes|=e,Gl|=e,n)}function bs(e,t,n,r){return wr(n,t)?n:$a.current===null?!(mo&42)||mo&1073741824&&!(Y&261930)?(nc=!0,e.memoizedState=n):(e=mu(),z.lanes|=e,Gl|=e,t):(e=ys(e,n,r),wr(e,t)||(nc=!0),e)}function xs(e,t,n,r,i){var a=k.p;k.p=a!==0&&8>a?a:8;var o=O.T,s={};O.T=s,Ns(e,!1,t,n);try{var c=i(),l=O.S;l!==null&&l(s,c),typeof c==`object`&&c&&typeof c.then==`function`?Ms(e,t,_a(c,r),pu(e)):Ms(e,t,r,pu(e))}catch(n){Ms(e,t,{then:function(){},status:`rejected`,reason:n},pu())}finally{k.p=a,o!==null&&s.types!==null&&(o.types=s.types),O.T=o}}function Ss(){}function Cs(e,t,n,r){if(e.tag!==5)throw Error(a(476));var i=ws(e).queue;xs(e,i,t,ce,n===null?Ss:function(){return Ts(e),n(r)})}function ws(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:ce,baseState:ce,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Lo,lastRenderedState:ce},next:null};var n={};return t.next={memoizedState:n,baseState:n,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Lo,lastRenderedState:n},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function Ts(e){var t=ws(e);t.next===null&&(t=e.alternate.memoizedState),Ms(e,t.next.queue,{},pu())}function Es(){return na(Qf)}function Ds(){return Mo().memoizedState}function Os(){return Mo().memoizedState}function ks(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var n=pu();e=Wa(n);var r=Ga(t,e,n);r!==null&&(hu(r,t,n),Ka(r,t,n)),t={cache:la()},e.payload=t;return}t=t.return}}function As(e,t,n){var r=pu();n={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},Ps(e)?Fs(t,n):(n=ri(e,t,n,r),n!==null&&(hu(n,e,r),Is(n,t,r)))}function js(e,t,n){Ms(e,t,n,pu())}function Ms(e,t,n,r){var i={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null};if(Ps(e))Fs(t,i);else{var a=e.alternate;if(e.lanes===0&&(a===null||a.lanes===0)&&(a=t.lastRenderedReducer,a!==null))try{var o=t.lastRenderedState,s=a(o,n);if(i.hasEagerState=!0,i.eagerState=s,wr(s,o))return ni(e,t,i,0),q===null&&ti(),!1}catch{}if(n=ri(e,t,i,r),n!==null)return hu(n,e,r),Is(n,t,r),!0}return!1}function Ns(e,t,n,r){if(r={lane:2,revertLane:dd(),gesture:null,action:r,hasEagerState:!1,eagerState:null,next:null},Ps(e)){if(t)throw Error(a(479))}else t=ri(e,n,r,2),t!==null&&hu(t,e,2)}function Ps(e){var t=e.alternate;return e===z||t!==null&&t===z}function Fs(e,t){_o=go=!0;var n=e.pending;n===null?t.next=t:(t.next=n.next,n.next=t),e.pending=t}function Is(e,t,n){if(n&4194048){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,nt(e,n)}}var Ls={readContext:na,use:Fo,useCallback:Co,useContext:Co,useEffect:Co,useImperativeHandle:Co,useLayoutEffect:Co,useInsertionEffect:Co,useMemo:Co,useReducer:Co,useRef:Co,useState:Co,useDebugValue:Co,useDeferredValue:Co,useTransition:Co,useSyncExternalStore:Co,useId:Co,useHostTransitionStatus:Co,useFormState:Co,useActionState:Co,useOptimistic:Co,useMemoCache:Co,useCacheRefresh:Co};Ls.useEffectEvent=Co;var Rs={readContext:na,use:Fo,useCallback:function(e,t){return V().memoizedState=[e,t===void 0?null:t],e},useContext:na,useEffect:cs,useImperativeHandle:function(e,t,n){n=n==null?null:n.concat([e]),os(4194308,4,ms.bind(null,t,e),n)},useLayoutEffect:function(e,t){return os(4194308,4,e,t)},useInsertionEffect:function(e,t){os(4,2,e,t)},useMemo:function(e,t){var n=V();t=t===void 0?null:t;var r=e();if(vo){ze(!0);try{e()}finally{ze(!1)}}return n.memoizedState=[r,t],r},useReducer:function(e,t,n){var r=V();if(n!==void 0){var i=n(t);if(vo){ze(!0);try{n(t)}finally{ze(!1)}}}else i=t;return r.memoizedState=r.baseState=i,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:i},r.queue=e,e=e.dispatch=As.bind(null,z,e),[r.memoizedState,e]},useRef:function(e){var t=V();return e={current:e},t.memoizedState=e},useState:function(e){e=Ko(e);var t=e.queue,n=js.bind(null,z,t);return t.dispatch=n,[e.memoizedState,n]},useDebugValue:gs,useDeferredValue:function(e,t){return ys(V(),e,t)},useTransition:function(){var e=Ko(!1);return e=xs.bind(null,z,e.queue,!0,!1),V().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,n){var r=z,i=V();if(R){if(n===void 0)throw Error(a(407));n=n()}else{if(n=t(),q===null)throw Error(a(349));Y&127||Ho(r,t,n)}i.memoizedState=n;var o={value:n,getSnapshot:t};return i.queue=o,cs(Uo.bind(null,r,o,e),[e]),r.flags|=2048,W(9,{destroy:void 0},H.bind(null,r,o,n,t),null),n},useId:function(){var e=V(),t=q.identifierPrefix;if(R){var n=Oi,r=Di;n=(r&~(1<<32-Be(r)-1)).toString(32)+n,t=`_`+t+`R_`+n,n=yo++,0<n&&(t+=`H`+n.toString(32)),t+=`_`}else n=So++,t=`_`+t+`r_`+n.toString(32)+`_`;return e.memoizedState=t},useHostTransitionStatus:Es,useFormState:es,useActionState:es,useOptimistic:function(e){var t=V();t.memoizedState=t.baseState=e;var n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=n,t=Ns.bind(null,z,!0,n),n.dispatch=t,[e,t]},useMemoCache:Io,useCacheRefresh:function(){return V().memoizedState=ks.bind(null,z)},useEffectEvent:function(e){var t=V(),n={impl:e};return t.memoizedState=n,function(){if(K&2)throw Error(a(440));return n.impl.apply(void 0,arguments)}}},zs={readContext:na,use:Fo,useCallback:_s,useContext:na,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:vs,useReducer:Ro,useRef:as,useState:function(){return Ro(Lo)},useDebugValue:gs,useDeferredValue:function(e,t){return bs(Mo(),B.memoizedState,e,t)},useTransition:function(){var e=Ro(Lo)[0],t=Mo().memoizedState;return[typeof e==`boolean`?e:Po(e),t]},useSyncExternalStore:Vo,useId:Ds,useHostTransitionStatus:Es,useFormState:ts,useActionState:ts,useOptimistic:function(e,t){return qo(Mo(),B,e,t)},useMemoCache:Io,useCacheRefresh:Os};zs.useEffectEvent=ds;var Bs={readContext:na,use:Fo,useCallback:_s,useContext:na,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:vs,useReducer:Bo,useRef:as,useState:function(){return Bo(Lo)},useDebugValue:gs,useDeferredValue:function(e,t){var n=Mo();return B===null?ys(n,e,t):bs(n,B.memoizedState,e,t)},useTransition:function(){var e=Bo(Lo)[0],t=Mo().memoizedState;return[typeof e==`boolean`?e:Po(e),t]},useSyncExternalStore:Vo,useId:Ds,useHostTransitionStatus:Es,useFormState:is,useActionState:is,useOptimistic:function(e,t){var n=Mo();return B===null?(n.baseState=e,[e,n.queue.dispatch]):qo(n,B,e,t)},useMemoCache:Io,useCacheRefresh:Os};Bs.useEffectEvent=ds;function Vs(e,t,n,r){t=e.memoizedState,n=n(r,t),n=n==null?t:m({},t,n),e.memoizedState=n,e.lanes===0&&(e.updateQueue.baseState=n)}var Hs={enqueueSetState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Wa(r);i.payload=t,n!=null&&(i.callback=n),t=Ga(e,i,r),t!==null&&(hu(t,e,r),Ka(t,e,r))},enqueueReplaceState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Wa(r);i.tag=1,i.payload=t,n!=null&&(i.callback=n),t=Ga(e,i,r),t!==null&&(hu(t,e,r),Ka(t,e,r))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var n=pu(),r=Wa(n);r.tag=2,t!=null&&(r.callback=t),t=Ga(e,r,n),t!==null&&(hu(t,e,n),Ka(t,e,n))}};function Us(e,t,n,r,i,a,o){return e=e.stateNode,typeof e.shouldComponentUpdate==`function`?e.shouldComponentUpdate(r,a,o):t.prototype&&t.prototype.isPureReactComponent?!Tr(n,r)||!Tr(i,a):!0}function Ws(e,t,n,r){e=t.state,typeof t.componentWillReceiveProps==`function`&&t.componentWillReceiveProps(n,r),typeof t.UNSAFE_componentWillReceiveProps==`function`&&t.UNSAFE_componentWillReceiveProps(n,r),t.state!==e&&Hs.enqueueReplaceState(t,t.state,null)}function Gs(e,t){var n=t;if(`ref`in t)for(var r in n={},t)r!==`ref`&&(n[r]=t[r]);if(e=e.defaultProps)for(var i in n===t&&(n=m({},n)),e)n[i]===void 0&&(n[i]=e[i]);return n}function Ks(e){Zr(e)}function qs(e){console.error(e)}function Js(e){Zr(e)}function Ys(e,t){try{var n=e.onUncaughtError;n(t.value,{componentStack:t.stack})}catch(e){setTimeout(function(){throw e})}}function Xs(e,t,n){try{var r=e.onCaughtError;r(n.value,{componentStack:n.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(e){setTimeout(function(){throw e})}}function Zs(e,t,n){return n=Wa(n),n.tag=3,n.payload={element:null},n.callback=function(){Ys(e,t)},n}function Qs(e){return e=Wa(e),e.tag=3,e}function $s(e,t,n,r){var i=n.type.getDerivedStateFromError;if(typeof i==`function`){var a=r.value;e.payload=function(){return i(a)},e.callback=function(){Xs(t,n,r)}}var o=n.stateNode;o!==null&&typeof o.componentDidCatch==`function`&&(e.callback=function(){Xs(t,n,r),typeof i!=`function`&&(ru===null?ru=new Set([this]):ru.add(this));var e=r.stack;this.componentDidCatch(r.value,{componentStack:e===null?``:e})})}function ec(e,t,n,r,i){if(n.flags|=32768,typeof r==`object`&&r&&typeof r.then==`function`){if(t=n.alternate,t!==null&&$i(t,n,i,!0),n=io.current,n!==null){switch(n.tag){case 31:case 13:return ao===null?Du():n.alternate===null&&Wl===0&&(Wl=3),n.flags&=-257,n.flags|=65536,n.lanes=i,r===Ea?n.flags|=16384:(t=n.updateQueue,t===null?n.updateQueue=new Set([r]):t.add(r),Gu(e,r,i)),!1;case 22:return n.flags|=65536,r===Ea?n.flags|=16384:(t=n.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([r])},n.updateQueue=t):(n=t.retryQueue,n===null?t.retryQueue=new Set([r]):n.add(r)),Gu(e,r,i)),!1}throw Error(a(435,n.tag))}return Gu(e,r,i),Du(),!1}if(R)return t=io.current,t===null?(r!==Ri&&(t=Error(a(423),{cause:r}),Gi(yi(t,n))),e=e.current.alternate,e.flags|=65536,i&=-i,e.lanes|=i,r=yi(r,n),i=Zs(e.stateNode,r,i),qa(e,i),Wl!==4&&(Wl=2)):(!(t.flags&65536)&&(t.flags|=256),t.flags|=65536,t.lanes=i,r!==Ri&&(e=Error(a(422),{cause:r}),Gi(yi(e,n)))),!1;var o=Error(a(520),{cause:r});if(o=yi(o,n),Xl===null?Xl=[o]:Xl.push(o),Wl!==4&&(Wl=2),t===null)return!0;r=yi(r,n),n=t;do{switch(n.tag){case 3:return n.flags|=65536,e=i&-i,n.lanes|=e,e=Zs(n.stateNode,r,e),qa(n,e),!1;case 1:if(t=n.type,o=n.stateNode,!(n.flags&128)&&(typeof t.getDerivedStateFromError==`function`||o!==null&&typeof o.componentDidCatch==`function`&&(ru===null||!ru.has(o))))return n.flags|=65536,i&=-i,n.lanes|=i,i=Qs(i),$s(i,e,n,r),qa(n,i),!1}n=n.return}while(n!==null);return!1}var tc=Error(a(461)),nc=!1;function rc(e,t,n,r){t.child=e===null?Ba(t,null,n,r):za(t,e.child,n,r)}function ic(e,t,n,r,i){n=n.render;var a=t.ref;if(`ref`in r){var o={};for(var s in r)s!==`ref`&&(o[s]=r[s])}else o=r;return ta(t),r=To(e,t,n,o,a,i),s=ko(),e!==null&&!nc?(Ao(e,t,i),Oc(e,t,i)):(R&&s&&ji(t),t.flags|=1,rc(e,t,r,i),t.child)}function ac(e,t,n,r,i){if(e===null){var a=n.type;return typeof a==`function`&&!ui(a)&&a.defaultProps===void 0&&n.compare===null?(t.tag=15,t.type=a,oc(e,t,a,r,i)):(e=pi(n.type,null,r,t,t.mode,i),e.ref=t.ref,e.return=t,t.child=e)}if(a=e.child,!kc(e,i)){var o=a.memoizedProps;if(n=n.compare,n=n===null?Tr:n,n(o,r)&&e.ref===t.ref)return Oc(e,t,i)}return t.flags|=1,e=di(a,r),e.ref=t.ref,e.return=t,t.child=e}function oc(e,t,n,r,i){if(e!==null){var a=e.memoizedProps;if(Tr(a,r)&&e.ref===t.ref)if(nc=!1,t.pendingProps=r=a,kc(e,i))e.flags&131072&&(nc=!0);else return t.lanes=e.lanes,Oc(e,t,i)}return mc(e,t,n,r,i)}function sc(e,t,n,r){var i=r.children,a=e===null?null:e.memoizedState;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),r.mode===`hidden`){if(t.flags&128){if(a=a===null?n:a.baseLanes|n,e!==null){for(r=t.child=e.child,i=0;r!==null;)i=i|r.lanes|r.childLanes,r=r.sibling;r=i&~a}else r=0,t.child=null;return lc(e,t,a,n,r)}if(n&536870912)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&xa(t,a===null?null:a.cachePool),a===null?no():to(t,a),co(t);else return r=t.lanes=536870912,lc(e,t,a===null?n:a.baseLanes|n,n,r)}else a===null?(e!==null&&xa(t,null),no(),lo(t)):(xa(t,a.cachePool),to(t,a),lo(t),t.memoizedState=null);return rc(e,t,i,n),t.child}function cc(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function lc(e,t,n,r,i){var a=ba();return a=a===null?null:{parent:ca._currentValue,pool:a},t.memoizedState={baseLanes:n,cachePool:a},e!==null&&xa(t,null),no(),co(t),e!==null&&$i(e,t,r,!0),t.childLanes=i,null}function uc(e,t){return t=Cc({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function dc(e,t,n){return za(t,e.child,null,n),e=uc(t,t.pendingProps),e.flags|=2,uo(t),t.memoizedState=null,e}function fc(e,t,n){var r=t.pendingProps,i=(t.flags&128)!=0;if(t.flags&=-129,e===null){if(R){if(r.mode===`hidden`)return e=uc(t,r),t.lanes=536870912,cc(null,e);if(so(t),(e=Fi)?(e=rf(e,Li),e=e!==null&&e.data===`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Ei===null?null:{id:Di,overflow:Oi},retryLane:536870912,hydrationErrors:null},n=gi(e),n.return=t,t.child=n,Pi=t,Fi=null)):e=null,e===null)throw zi(t);return t.lanes=536870912,null}return uc(t,r)}var o=e.memoizedState;if(o!==null){var s=o.dehydrated;if(so(t),i)if(t.flags&256)t.flags&=-257,t=dc(e,t,n);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(a(558));else if(nc||$i(e,t,n,!1),i=(n&e.childLanes)!==0,nc||i){if(r=q,r!==null&&(s=rt(r,n),s!==0&&s!==o.retryLane))throw o.retryLane=s,ii(e,s),hu(r,e,s),tc;Du(),t=dc(e,t,n)}else e=o.treeContext,Fi=cf(s.nextSibling),Pi=t,R=!0,Ii=null,Li=!1,e!==null&&Ni(t,e),t=uc(t,r),t.flags|=4096;return t}return e=di(e.child,{mode:r.mode,children:r.children}),e.ref=t.ref,t.child=e,e.return=t,e}function pc(e,t){var n=t.ref;if(n===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof n!=`function`&&typeof n!=`object`)throw Error(a(284));(e===null||e.ref!==n)&&(t.flags|=4194816)}}function mc(e,t,n,r,i){return ta(t),n=To(e,t,n,r,void 0,i),r=ko(),e!==null&&!nc?(Ao(e,t,i),Oc(e,t,i)):(R&&r&&ji(t),t.flags|=1,rc(e,t,n,i),t.child)}function hc(e,t,n,r,i,a){return ta(t),t.updateQueue=null,n=Do(t,r,n,i),Eo(e),r=ko(),e!==null&&!nc?(Ao(e,t,a),Oc(e,t,a)):(R&&r&&ji(t),t.flags|=1,rc(e,t,n,a),t.child)}function gc(e,t,n,r,i){if(ta(t),t.stateNode===null){var a=si,o=n.contextType;typeof o==`object`&&o&&(a=na(o)),a=new n(r,a),t.memoizedState=a.state!==null&&a.state!==void 0?a.state:null,a.updater=Hs,t.stateNode=a,a._reactInternals=t,a=t.stateNode,a.props=r,a.state=t.memoizedState,a.refs={},Ha(t),o=n.contextType,a.context=typeof o==`object`&&o?na(o):si,a.state=t.memoizedState,o=n.getDerivedStateFromProps,typeof o==`function`&&(Vs(t,n,o,r),a.state=t.memoizedState),typeof n.getDerivedStateFromProps==`function`||typeof a.getSnapshotBeforeUpdate==`function`||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(o=a.state,typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount(),o!==a.state&&Hs.enqueueReplaceState(a,a.state,null),Xa(t,r,a,i),Ya(),a.state=t.memoizedState),typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!0}else if(e===null){a=t.stateNode;var s=t.memoizedProps,c=Gs(n,s);a.props=c;var l=a.context,u=n.contextType;o=si,typeof u==`object`&&u&&(o=na(u));var d=n.getDerivedStateFromProps;u=typeof d==`function`||typeof a.getSnapshotBeforeUpdate==`function`,s=t.pendingProps!==s,u||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(s||l!==o)&&Ws(t,a,r,o),Va=!1;var f=t.memoizedState;a.state=f,Xa(t,r,a,i),Ya(),l=t.memoizedState,s||f!==l||Va?(typeof d==`function`&&(Vs(t,n,d,r),l=t.memoizedState),(c=Va||Us(t,n,c,r,f,l,o))?(u||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount()),typeof a.componentDidMount==`function`&&(t.flags|=4194308)):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),t.memoizedProps=r,t.memoizedState=l),a.props=r,a.state=l,a.context=o,r=c):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!1)}else{a=t.stateNode,Ua(e,t),o=t.memoizedProps,u=Gs(n,o),a.props=u,d=t.pendingProps,f=a.context,l=n.contextType,c=si,typeof l==`object`&&l&&(c=na(l)),s=n.getDerivedStateFromProps,(l=typeof s==`function`||typeof a.getSnapshotBeforeUpdate==`function`)||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(o!==d||f!==c)&&Ws(t,a,r,c),Va=!1,f=t.memoizedState,a.state=f,Xa(t,r,a,i),Ya();var p=t.memoizedState;o!==d||f!==p||Va||e!==null&&e.dependencies!==null&&ea(e.dependencies)?(typeof s==`function`&&(Vs(t,n,s,r),p=t.memoizedState),(u=Va||Us(t,n,u,r,f,p,c)||e!==null&&e.dependencies!==null&&ea(e.dependencies))?(l||typeof a.UNSAFE_componentWillUpdate!=`function`&&typeof a.componentWillUpdate!=`function`||(typeof a.componentWillUpdate==`function`&&a.componentWillUpdate(r,p,c),typeof a.UNSAFE_componentWillUpdate==`function`&&a.UNSAFE_componentWillUpdate(r,p,c)),typeof a.componentDidUpdate==`function`&&(t.flags|=4),typeof a.getSnapshotBeforeUpdate==`function`&&(t.flags|=1024)):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),t.memoizedProps=r,t.memoizedState=p),a.props=r,a.state=p,a.context=c,r=u):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),r=!1)}return a=r,pc(e,t),r=(t.flags&128)!=0,a||r?(a=t.stateNode,n=r&&typeof n.getDerivedStateFromError!=`function`?null:a.render(),t.flags|=1,e!==null&&r?(t.child=za(t,e.child,null,i),t.child=za(t,null,n,i)):rc(e,t,n,i),t.memoizedState=a.state,e=t.child):e=Oc(e,t,i),e}function _c(e,t,n,r){return Ui(),t.flags|=256,rc(e,t,n,r),t.child}var vc={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function yc(e){return{baseLanes:e,cachePool:Sa()}}function bc(e,t,n){return e=e===null?0:e.childLanes&~n,t&&(e|=Jl),e}function xc(e,t,n){var r=t.pendingProps,i=!1,o=(t.flags&128)!=0,s;if((s=o)||(s=e!==null&&e.memoizedState===null?!1:(fo.current&2)!=0),s&&(i=!0,t.flags&=-129),s=(t.flags&32)!=0,t.flags&=-33,e===null){if(R){if(i?oo(t):lo(t),(e=Fi)?(e=rf(e,Li),e=e!==null&&e.data!==`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Ei===null?null:{id:Di,overflow:Oi},retryLane:536870912,hydrationErrors:null},n=gi(e),n.return=t,t.child=n,Pi=t,Fi=null)):e=null,e===null)throw zi(t);return of(e)?t.lanes=32:t.lanes=536870912,null}var c=r.children;return r=r.fallback,i?(lo(t),i=t.mode,c=Cc({mode:`hidden`,children:c},i),r=mi(r,i,n,null),c.return=t,r.return=t,c.sibling=r,t.child=c,r=t.child,r.memoizedState=yc(n),r.childLanes=bc(e,s,n),t.memoizedState=vc,cc(null,r)):(oo(t),Sc(t,c))}var l=e.memoizedState;if(l!==null&&(c=l.dehydrated,c!==null)){if(o)t.flags&256?(oo(t),t.flags&=-257,t=wc(e,t,n)):t.memoizedState===null?(lo(t),c=r.fallback,i=t.mode,r=Cc({mode:`visible`,children:r.children},i),c=mi(c,i,n,null),c.flags|=2,r.return=t,c.return=t,r.sibling=c,t.child=r,za(t,e.child,null,n),r=t.child,r.memoizedState=yc(n),r.childLanes=bc(e,s,n),t.memoizedState=vc,t=cc(null,r)):(lo(t),t.child=e.child,t.flags|=128,t=null);else if(oo(t),of(c)){if(s=c.nextSibling&&c.nextSibling.dataset,s)var u=s.dgst;s=u,r=Error(a(419)),r.stack=``,r.digest=s,Gi({value:r,source:null,stack:null}),t=wc(e,t,n)}else if(nc||$i(e,t,n,!1),s=(n&e.childLanes)!==0,nc||s){if(s=q,s!==null&&(r=rt(s,n),r!==0&&r!==l.retryLane))throw l.retryLane=r,ii(e,r),hu(s,e,r),tc;af(c)||Du(),t=wc(e,t,n)}else af(c)?(t.flags|=192,t.child=e.child,t=null):(e=l.treeContext,Fi=cf(c.nextSibling),Pi=t,R=!0,Ii=null,Li=!1,e!==null&&Ni(t,e),t=Sc(t,r.children),t.flags|=4096);return t}return i?(lo(t),c=r.fallback,i=t.mode,l=e.child,u=l.sibling,r=di(l,{mode:`hidden`,children:r.children}),r.subtreeFlags=l.subtreeFlags&65011712,u===null?(c=mi(c,i,n,null),c.flags|=2):c=di(u,c),c.return=t,r.return=t,r.sibling=c,t.child=r,cc(null,r),r=t.child,c=e.child.memoizedState,c===null?c=yc(n):(i=c.cachePool,i===null?i=Sa():(l=ca._currentValue,i=i.parent===l?i:{parent:l,pool:l}),c={baseLanes:c.baseLanes|n,cachePool:i}),r.memoizedState=c,r.childLanes=bc(e,s,n),t.memoizedState=vc,cc(e.child,r)):(oo(t),n=e.child,e=n.sibling,n=di(n,{mode:`visible`,children:r.children}),n.return=t,n.sibling=null,e!==null&&(s=t.deletions,s===null?(t.deletions=[e],t.flags|=16):s.push(e)),t.child=n,t.memoizedState=null,n)}function Sc(e,t){return t=Cc({mode:`visible`,children:t},e.mode),t.return=e,e.child=t}function Cc(e,t){return e=li(22,e,null,t),e.lanes=0,e}function wc(e,t,n){return za(t,e.child,null,n),e=Sc(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function Tc(e,t,n){e.lanes|=t;var r=e.alternate;r!==null&&(r.lanes|=t),Zi(e.return,t,n)}function Ec(e,t,n,r,i,a){var o=e.memoizedState;o===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:r,tail:n,tailMode:i,treeForkCount:a}:(o.isBackwards=t,o.rendering=null,o.renderingStartTime=0,o.last=r,o.tail=n,o.tailMode=i,o.treeForkCount=a)}function Dc(e,t,n){var r=t.pendingProps,i=r.revealOrder,a=r.tail;r=r.children;var o=fo.current,s=(o&2)!=0;if(s?(o=o&1|2,t.flags|=128):o&=1,P(fo,o),rc(e,t,r,n),r=R?Ci:0,!s&&e!==null&&e.flags&128)a:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&Tc(e,n,t);else if(e.tag===19)Tc(e,n,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break a;for(;e.sibling===null;){if(e.return===null||e.return===t)break a;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(i){case`forwards`:for(n=t.child,i=null;n!==null;)e=n.alternate,e!==null&&po(e)===null&&(i=n),n=n.sibling;n=i,n===null?(i=t.child,t.child=null):(i=n.sibling,n.sibling=null),Ec(t,!1,i,n,a,r);break;case`backwards`:case`unstable_legacy-backwards`:for(n=null,i=t.child,t.child=null;i!==null;){if(e=i.alternate,e!==null&&po(e)===null){t.child=i;break}e=i.sibling,i.sibling=n,n=i,i=e}Ec(t,!0,n,null,a,r);break;case`together`:Ec(t,!1,null,null,void 0,r);break;default:t.memoizedState=null}return t.child}function Oc(e,t,n){if(e!==null&&(t.dependencies=e.dependencies),Gl|=t.lanes,(n&t.childLanes)===0)if(e!==null){if($i(e,t,n,!1),(n&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(a(153));if(t.child!==null){for(e=t.child,n=di(e,e.pendingProps),t.child=n,n.return=t;e.sibling!==null;)e=e.sibling,n=n.sibling=di(e,e.pendingProps),n.return=t;n.sibling=null}return t.child}function kc(e,t){return(e.lanes&t)===0?(e=e.dependencies,!!(e!==null&&ea(e))):!0}function Ac(e,t,n){switch(t.tag){case 3:pe(t,t.stateNode.containerInfo),Yi(t,ca,e.memoizedState.cache),Ui();break;case 27:case 5:he(t);break;case 4:pe(t,t.stateNode.containerInfo);break;case 10:Yi(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,so(t),null;break;case 13:var r=t.memoizedState;if(r!==null)return r.dehydrated===null?(n&t.child.childLanes)===0?(oo(t),e=Oc(e,t,n),e===null?null:e.sibling):xc(e,t,n):(oo(t),t.flags|=128,null);oo(t);break;case 19:var i=(e.flags&128)!=0;if(r=(n&t.childLanes)!==0,r||=($i(e,t,n,!1),(n&t.childLanes)!==0),i){if(r)return Dc(e,t,n);t.flags|=128}if(i=t.memoizedState,i!==null&&(i.rendering=null,i.tail=null,i.lastEffect=null),P(fo,fo.current),r)break;return null;case 22:return t.lanes=0,sc(e,t,n,t.pendingProps);case 24:Yi(t,ca,e.memoizedState.cache)}return Oc(e,t,n)}function jc(e,t,n){if(e!==null)if(e.memoizedProps!==t.pendingProps)nc=!0;else{if(!kc(e,n)&&!(t.flags&128))return nc=!1,Ac(e,t,n);nc=!!(e.flags&131072)}else nc=!1,R&&t.flags&1048576&&Ai(t,Ci,t.index);switch(t.lanes=0,t.tag){case 16:a:{var r=t.pendingProps;if(e=ka(t.elementType),t.type=e,typeof e==`function`)ui(e)?(r=Gs(e,r),t.tag=1,t=gc(null,t,e,r,n)):(t.tag=0,t=mc(null,t,e,r,n));else{if(e!=null){var i=e.$$typeof;if(i===C){t.tag=11,t=ic(null,t,e,r,n);break a}else if(i===te){t.tag=14,t=ac(null,t,e,r,n);break a}}throw t=ae(e)||e,Error(a(306,t,``))}}return t;case 0:return mc(e,t,t.type,t.pendingProps,n);case 1:return r=t.type,i=Gs(r,t.pendingProps),gc(e,t,r,i,n);case 3:a:{if(pe(t,t.stateNode.containerInfo),e===null)throw Error(a(387));r=t.pendingProps;var o=t.memoizedState;i=o.element,Ua(e,t),Xa(t,r,null,n);var s=t.memoizedState;if(r=s.cache,Yi(t,ca,r),r!==o.cache&&Qi(t,[ca],n,!0),Ya(),r=s.element,o.isDehydrated)if(o={element:r,isDehydrated:!1,cache:s.cache},t.updateQueue.baseState=o,t.memoizedState=o,t.flags&256){t=_c(e,t,r,n);break a}else if(r!==i){i=yi(Error(a(424)),t),Gi(i),t=_c(e,t,r,n);break a}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName===`HTML`?e.ownerDocument.body:e}for(Fi=cf(e.firstChild),Pi=t,R=!0,Ii=null,Li=!0,n=Ba(t,null,r,n),t.child=n;n;)n.flags=n.flags&-3|4096,n=n.sibling}else{if(Ui(),r===i){t=Oc(e,t,n);break a}rc(e,t,r,n)}t=t.child}return t;case 26:return pc(e,t),e===null?(n=kf(t.type,null,t.pendingProps,null))?t.memoizedState=n:R||(n=t.type,e=t.pendingProps,r=Bd(de.current).createElement(n),r[lt]=t,r[I]=e,Pd(r,n,e),xt(r),t.stateNode=r):t.memoizedState=kf(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return he(t),e===null&&R&&(r=t.stateNode=ff(t.type,t.pendingProps,de.current),Pi=t,Li=!0,i=Fi,Zd(t.type)?(lf=i,Fi=cf(r.firstChild)):Fi=i),rc(e,t,t.pendingProps.children,n),pc(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&R&&((i=r=Fi)&&(r=tf(r,t.type,t.pendingProps,Li),r===null?i=!1:(t.stateNode=r,Pi=t,Fi=cf(r.firstChild),Li=!1,i=!0)),i||zi(t)),he(t),i=t.type,o=t.pendingProps,s=e===null?null:e.memoizedProps,r=o.children,Ud(i,o)?r=null:s!==null&&Ud(i,s)&&(t.flags|=32),t.memoizedState!==null&&(i=To(e,t,Oo,null,null,n),Qf._currentValue=i),pc(e,t),rc(e,t,r,n),t.child;case 6:return e===null&&R&&((e=n=Fi)&&(n=nf(n,t.pendingProps,Li),n===null?e=!1:(t.stateNode=n,Pi=t,Fi=null,e=!0)),e||zi(t)),null;case 13:return xc(e,t,n);case 4:return pe(t,t.stateNode.containerInfo),r=t.pendingProps,e===null?t.child=za(t,null,r,n):rc(e,t,r,n),t.child;case 11:return ic(e,t,t.type,t.pendingProps,n);case 7:return rc(e,t,t.pendingProps,n),t.child;case 8:return rc(e,t,t.pendingProps.children,n),t.child;case 12:return rc(e,t,t.pendingProps.children,n),t.child;case 10:return r=t.pendingProps,Yi(t,t.type,r.value),rc(e,t,r.children,n),t.child;case 9:return i=t.type._context,r=t.pendingProps.children,ta(t),i=na(i),r=r(i),t.flags|=1,rc(e,t,r,n),t.child;case 14:return ac(e,t,t.type,t.pendingProps,n);case 15:return oc(e,t,t.type,t.pendingProps,n);case 19:return Dc(e,t,n);case 31:return fc(e,t,n);case 22:return sc(e,t,n,t.pendingProps);case 24:return ta(t),r=na(ca),e===null?(i=ba(),i===null&&(i=q,o=la(),i.pooledCache=o,o.refCount++,o!==null&&(i.pooledCacheLanes|=n),i=o),t.memoizedState={parent:r,cache:i},Ha(t),Yi(t,ca,i)):((e.lanes&n)!==0&&(Ua(e,t),Xa(t,null,null,n),Ya()),i=e.memoizedState,o=t.memoizedState,i.parent===r?(r=o.cache,Yi(t,ca,r),r!==i.cache&&Qi(t,[ca],n,!0)):(i={parent:r,cache:r},t.memoizedState=i,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=i),Yi(t,ca,r))),rc(e,t,t.pendingProps.children,n),t.child;case 29:throw t.pendingProps}throw Error(a(156,t.tag))}function Mc(e){e.flags|=4}function Nc(e,t,n,r,i){if((t=(e.mode&32)!=0)&&(t=!1),t){if(e.flags|=16777216,(i&335544128)===i)if(e.stateNode.complete)e.flags|=8192;else if(wu())e.flags|=8192;else throw Aa=Ea,wa}else e.flags&=-16777217}function Pc(e,t){if(t.type!==`stylesheet`||t.state.loading&4)e.flags&=-16777217;else if(e.flags|=16777216,!Wf(t))if(wu())e.flags|=8192;else throw Aa=Ea,wa}function Fc(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag===22?536870912:Ze(),e.lanes|=t,Yl|=t)}function Ic(e,t){if(!R)switch(e.tailMode){case`hidden`:t=e.tail;for(var n=null;t!==null;)t.alternate!==null&&(n=t),t=t.sibling;n===null?e.tail=null:n.sibling=null;break;case`collapsed`:n=e.tail;for(var r=null;n!==null;)n.alternate!==null&&(r=n),n=n.sibling;r===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:r.sibling=null}}function Lc(e){var t=e.alternate!==null&&e.alternate.child===e.child,n=0,r=0;if(t)for(var i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags&65011712,r|=i.flags&65011712,i.return=e,i=i.sibling;else for(i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags,r|=i.flags,i.return=e,i=i.sibling;return e.subtreeFlags|=r,e.childLanes=n,t}function Rc(e,t,n){var r=t.pendingProps;switch(Mi(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return Lc(t),null;case 1:return Lc(t),null;case 3:return n=t.stateNode,r=null,e!==null&&(r=e.memoizedState.cache),t.memoizedState.cache!==r&&(t.flags|=2048),Xi(ca),me(),n.pendingContext&&(n.context=n.pendingContext,n.pendingContext=null),(e===null||e.child===null)&&(Hi(t)?Mc(t):e===null||e.memoizedState.isDehydrated&&!(t.flags&256)||(t.flags|=1024,Wi())),Lc(t),null;case 26:var i=t.type,o=t.memoizedState;return e===null?(Mc(t),o===null?(Lc(t),Nc(t,i,null,r,n)):(Lc(t),Pc(t,o))):o?o===e.memoizedState?(Lc(t),t.flags&=-16777217):(Mc(t),Lc(t),Pc(t,o)):(e=e.memoizedProps,e!==r&&Mc(t),Lc(t),Nc(t,i,e,r,n)),null;case 27:if(ge(t),n=de.current,i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Lc(t),null}e=le.current,Hi(t)?Bi(t,e):(e=ff(i,r,n),t.stateNode=e,Mc(t))}return Lc(t),null;case 5:if(ge(t),i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Lc(t),null}if(o=le.current,Hi(t))Bi(t,o);else{var s=Bd(de.current);switch(o){case 1:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case 2:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;default:switch(i){case`svg`:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case`math`:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;case`script`:o=s.createElement(`div`),o.innerHTML=`<script><\/script>`,o=o.removeChild(o.firstChild);break;case`select`:o=typeof r.is==`string`?s.createElement(`select`,{is:r.is}):s.createElement(`select`),r.multiple?o.multiple=!0:r.size&&(o.size=r.size);break;default:o=typeof r.is==`string`?s.createElement(i,{is:r.is}):s.createElement(i)}}o[lt]=t,o[I]=r;a:for(s=t.child;s!==null;){if(s.tag===5||s.tag===6)o.appendChild(s.stateNode);else if(s.tag!==4&&s.tag!==27&&s.child!==null){s.child.return=s,s=s.child;continue}if(s===t)break a;for(;s.sibling===null;){if(s.return===null||s.return===t)break a;s=s.return}s.sibling.return=s.return,s=s.sibling}t.stateNode=o;a:switch(Pd(o,i,r),i){case`button`:case`input`:case`select`:case`textarea`:r=!!r.autoFocus;break a;case`img`:r=!0;break a;default:r=!1}r&&Mc(t)}}return Lc(t),Nc(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,n),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(typeof r!=`string`&&t.stateNode===null)throw Error(a(166));if(e=de.current,Hi(t)){if(e=t.stateNode,n=t.memoizedProps,r=null,i=Pi,i!==null)switch(i.tag){case 27:case 5:r=i.memoizedProps}e[lt]=t,e=!!(e.nodeValue===n||r!==null&&!0===r.suppressHydrationWarning||Md(e.nodeValue,n)),e||zi(t,!0)}else e=Bd(e).createTextNode(r),e[lt]=t,t.stateNode=e}return Lc(t),null;case 31:if(n=t.memoizedState,e===null||e.memoizedState!==null){if(r=Hi(t),n!==null){if(e===null){if(!r)throw Error(a(318));if(e=t.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(557));e[lt]=t}else Ui(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Lc(t),e=!1}else n=Wi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=n),e=!0;if(!e)return t.flags&256?(uo(t),t):(uo(t),null);if(t.flags&128)throw Error(a(558))}return Lc(t),null;case 13:if(r=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(i=Hi(t),r!==null&&r.dehydrated!==null){if(e===null){if(!i)throw Error(a(318));if(i=t.memoizedState,i=i===null?null:i.dehydrated,!i)throw Error(a(317));i[lt]=t}else Ui(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Lc(t),i=!1}else i=Wi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=i),i=!0;if(!i)return t.flags&256?(uo(t),t):(uo(t),null)}return uo(t),t.flags&128?(t.lanes=n,t):(n=r!==null,e=e!==null&&e.memoizedState!==null,n&&(r=t.child,i=null,r.alternate!==null&&r.alternate.memoizedState!==null&&r.alternate.memoizedState.cachePool!==null&&(i=r.alternate.memoizedState.cachePool.pool),o=null,r.memoizedState!==null&&r.memoizedState.cachePool!==null&&(o=r.memoizedState.cachePool.pool),o!==i&&(r.flags|=2048)),n!==e&&n&&(t.child.flags|=8192),Fc(t,t.updateQueue),Lc(t),null);case 4:return me(),e===null&&Sd(t.stateNode.containerInfo),Lc(t),null;case 10:return Xi(t.type),Lc(t),null;case 19:if(N(fo),r=t.memoizedState,r===null)return Lc(t),null;if(i=(t.flags&128)!=0,o=r.rendering,o===null)if(i)Ic(r,!1);else{if(Wl!==0||e!==null&&e.flags&128)for(e=t.child;e!==null;){if(o=po(e),o!==null){for(t.flags|=128,Ic(r,!1),e=o.updateQueue,t.updateQueue=e,Fc(t,e),t.subtreeFlags=0,e=n,n=t.child;n!==null;)fi(n,e),n=n.sibling;return P(fo,fo.current&1|2),R&&ki(t,r.treeForkCount),t.child}e=e.sibling}r.tail!==null&&Oe()>tu&&(t.flags|=128,i=!0,Ic(r,!1),t.lanes=4194304)}else{if(!i)if(e=po(o),e!==null){if(t.flags|=128,i=!0,e=e.updateQueue,t.updateQueue=e,Fc(t,e),Ic(r,!0),r.tail===null&&r.tailMode===`hidden`&&!o.alternate&&!R)return Lc(t),null}else 2*Oe()-r.renderingStartTime>tu&&n!==536870912&&(t.flags|=128,i=!0,Ic(r,!1),t.lanes=4194304);r.isBackwards?(o.sibling=t.child,t.child=o):(e=r.last,e===null?t.child=o:e.sibling=o,r.last=o)}return r.tail===null?(Lc(t),null):(e=r.tail,r.rendering=e,r.tail=e.sibling,r.renderingStartTime=Oe(),e.sibling=null,n=fo.current,P(fo,i?n&1|2:n&1),R&&ki(t,r.treeForkCount),e);case 22:case 23:return uo(t),ro(),r=t.memoizedState!==null,e===null?r&&(t.flags|=8192):e.memoizedState!==null!==r&&(t.flags|=8192),r?n&536870912&&!(t.flags&128)&&(Lc(t),t.subtreeFlags&6&&(t.flags|=8192)):Lc(t),n=t.updateQueue,n!==null&&Fc(t,n.retryQueue),n=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),r=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(r=t.memoizedState.cachePool.pool),r!==n&&(t.flags|=2048),e!==null&&N(ya),null;case 24:return n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),Xi(ca),Lc(t),null;case 25:return null;case 30:return null}throw Error(a(156,t.tag))}function zc(e,t){switch(Mi(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return Xi(ca),me(),e=t.flags,e&65536&&!(e&128)?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return ge(t),null;case 31:if(t.memoizedState!==null){if(uo(t),t.alternate===null)throw Error(a(340));Ui()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(uo(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(a(340));Ui()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return N(fo),null;case 4:return me(),null;case 10:return Xi(t.type),null;case 22:case 23:return uo(t),ro(),e!==null&&N(ya),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return Xi(ca),null;case 25:return null;default:return null}}function Bc(e,t){switch(Mi(t),t.tag){case 3:Xi(ca),me();break;case 26:case 27:case 5:ge(t);break;case 4:me();break;case 31:t.memoizedState!==null&&uo(t);break;case 13:uo(t);break;case 19:N(fo);break;case 10:Xi(t.type);break;case 22:case 23:uo(t),ro(),e!==null&&N(ya);break;case 24:Xi(ca)}}function Vc(e,t){try{var n=t.updateQueue,r=n===null?null:n.lastEffect;if(r!==null){var i=r.next;n=i;do{if((n.tag&e)===e){r=void 0;var a=n.create,o=n.inst;r=a(),o.destroy=r}n=n.next}while(n!==i)}}catch(e){Z(t,t.return,e)}}function Hc(e,t,n){try{var r=t.updateQueue,i=r===null?null:r.lastEffect;if(i!==null){var a=i.next;r=a;do{if((r.tag&e)===e){var o=r.inst,s=o.destroy;if(s!==void 0){o.destroy=void 0,i=t;var c=n,l=s;try{l()}catch(e){Z(i,c,e)}}}r=r.next}while(r!==a)}}catch(e){Z(t,t.return,e)}}function Uc(e){var t=e.updateQueue;if(t!==null){var n=e.stateNode;try{Qa(t,n)}catch(t){Z(e,e.return,t)}}}function Wc(e,t,n){n.props=Gs(e.type,e.memoizedProps),n.state=e.memoizedState;try{n.componentWillUnmount()}catch(n){Z(e,t,n)}}function Gc(e,t){try{var n=e.ref;if(n!==null){switch(e.tag){case 26:case 27:case 5:var r=e.stateNode;break;case 30:r=e.stateNode;break;default:r=e.stateNode}typeof n==`function`?e.refCleanup=n(r):n.current=r}}catch(n){Z(e,t,n)}}function Kc(e,t){var n=e.ref,r=e.refCleanup;if(n!==null)if(typeof r==`function`)try{r()}catch(n){Z(e,t,n)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof n==`function`)try{n(null)}catch(n){Z(e,t,n)}else n.current=null}function qc(e){var t=e.type,n=e.memoizedProps,r=e.stateNode;try{a:switch(t){case`button`:case`input`:case`select`:case`textarea`:n.autoFocus&&r.focus();break a;case`img`:n.src?r.src=n.src:n.srcSet&&(r.srcset=n.srcSet)}}catch(t){Z(e,e.return,t)}}function Jc(e,t,n){try{var r=e.stateNode;Fd(r,e.type,n,t),r[I]=t}catch(t){Z(e,e.return,t)}}function Yc(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&Zd(e.type)||e.tag===4}function Xc(e){a:for(;;){for(;e.sibling===null;){if(e.return===null||Yc(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&Zd(e.type)||e.flags&2||e.child===null||e.tag===4)continue a;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function Zc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?(n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n).insertBefore(e,t):(t=n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n,t.appendChild(e),n=n._reactRootContainer,n!=null||t.onclick!==null||(t.onclick=tn));else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode,t=null),e=e.child,e!==null))for(Zc(e,t,n),e=e.sibling;e!==null;)Zc(e,t,n),e=e.sibling}function Qc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?n.insertBefore(e,t):n.appendChild(e);else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode),e=e.child,e!==null))for(Qc(e,t,n),e=e.sibling;e!==null;)Qc(e,t,n),e=e.sibling}function $c(e){var t=e.stateNode,n=e.memoizedProps;try{for(var r=e.type,i=t.attributes;i.length;)t.removeAttributeNode(i[0]);Pd(t,r,n),t[lt]=e,t[I]=n}catch(t){Z(e,e.return,t)}}var el=!1,tl=!1,nl=!1,G=typeof WeakSet==`function`?WeakSet:Set,rl=null;function il(e,t){if(e=e.containerInfo,Rd=sp,e=kr(e),Ar(e)){if(`selectionStart`in e)var n={start:e.selectionStart,end:e.selectionEnd};else a:{n=(n=e.ownerDocument)&&n.defaultView||window;var r=n.getSelection&&n.getSelection();if(r&&r.rangeCount!==0){n=r.anchorNode;var i=r.anchorOffset,o=r.focusNode;r=r.focusOffset;try{n.nodeType,o.nodeType}catch{n=null;break a}var s=0,c=-1,l=-1,u=0,d=0,f=e,p=null;b:for(;;){for(var m;f!==n||i!==0&&f.nodeType!==3||(c=s+i),f!==o||r!==0&&f.nodeType!==3||(l=s+r),f.nodeType===3&&(s+=f.nodeValue.length),(m=f.firstChild)!==null;)p=f,f=m;for(;;){if(f===e)break b;if(p===n&&++u===i&&(c=s),p===o&&++d===r&&(l=s),(m=f.nextSibling)!==null)break;f=p,p=f.parentNode}f=m}n=c===-1||l===-1?null:{start:c,end:l}}else n=null}n||={start:0,end:0}}else n=null;for(zd={focusedElem:e,selectionRange:n},sp=!1,rl=t;rl!==null;)if(t=rl,e=t.child,t.subtreeFlags&1028&&e!==null)e.return=t,rl=e;else for(;rl!==null;){switch(t=rl,o=t.alternate,e=t.flags,t.tag){case 0:if(e&4&&(e=t.updateQueue,e=e===null?null:e.events,e!==null))for(n=0;n<e.length;n++)i=e[n],i.ref.impl=i.nextImpl;break;case 11:case 15:break;case 1:if(e&1024&&o!==null){e=void 0,n=t,i=o.memoizedProps,o=o.memoizedState,r=n.stateNode;try{var h=Gs(n.type,i);e=r.getSnapshotBeforeUpdate(h,o),r.__reactInternalSnapshotBeforeUpdate=e}catch(e){Z(n,n.return,e)}}break;case 3:if(e&1024){if(e=t.stateNode.containerInfo,n=e.nodeType,n===9)ef(e);else if(n===1)switch(e.nodeName){case`HEAD`:case`HTML`:case`BODY`:ef(e);break;default:e.textContent=``}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if(e&1024)throw Error(a(163))}if(e=t.sibling,e!==null){e.return=t.return,rl=e;break}rl=t.return}}function al(e,t,n){var r=n.flags;switch(n.tag){case 0:case 11:case 15:bl(e,n),r&4&&Vc(5,n);break;case 1:if(bl(e,n),r&4)if(e=n.stateNode,t===null)try{e.componentDidMount()}catch(e){Z(n,n.return,e)}else{var i=Gs(n.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(i,t,e.__reactInternalSnapshotBeforeUpdate)}catch(e){Z(n,n.return,e)}}r&64&&Uc(n),r&512&&Gc(n,n.return);break;case 3:if(bl(e,n),r&64&&(e=n.updateQueue,e!==null)){if(t=null,n.child!==null)switch(n.child.tag){case 27:case 5:t=n.child.stateNode;break;case 1:t=n.child.stateNode}try{Qa(e,t)}catch(e){Z(n,n.return,e)}}break;case 27:t===null&&r&4&&$c(n);case 26:case 5:bl(e,n),t===null&&r&4&&qc(n),r&512&&Gc(n,n.return);break;case 12:bl(e,n);break;case 31:bl(e,n),r&4&&dl(e,n);break;case 13:bl(e,n),r&4&&fl(e,n),r&64&&(e=n.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(n=Ju.bind(null,n),sf(e,n))));break;case 22:if(r=n.memoizedState!==null||el,!r){t=t!==null&&t.memoizedState!==null||tl,i=el;var a=tl;el=r,(tl=t)&&!a?Sl(e,n,(n.subtreeFlags&8772)!=0):bl(e,n),el=i,tl=a}break;case 30:break;default:bl(e,n)}}function ol(e){var t=e.alternate;t!==null&&(e.alternate=null,ol(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&gt(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var sl=null,cl=!1;function ll(e,t,n){for(n=n.child;n!==null;)ul(e,t,n),n=n.sibling}function ul(e,t,n){if(Re&&typeof Re.onCommitFiberUnmount==`function`)try{Re.onCommitFiberUnmount(Le,n)}catch{}switch(n.tag){case 26:tl||Kc(n,t),ll(e,t,n),n.memoizedState?n.memoizedState.count--:n.stateNode&&(n=n.stateNode,n.parentNode.removeChild(n));break;case 27:tl||Kc(n,t);var r=sl,i=cl;Zd(n.type)&&(sl=n.stateNode,cl=!1),ll(e,t,n),pf(n.stateNode),sl=r,cl=i;break;case 5:tl||Kc(n,t);case 6:if(r=sl,i=cl,sl=null,ll(e,t,n),sl=r,cl=i,sl!==null)if(cl)try{(sl.nodeType===9?sl.body:sl.nodeName===`HTML`?sl.ownerDocument.body:sl).removeChild(n.stateNode)}catch(e){Z(n,t,e)}else try{sl.removeChild(n.stateNode)}catch(e){Z(n,t,e)}break;case 18:sl!==null&&(cl?(e=sl,Qd(e.nodeType===9?e.body:e.nodeName===`HTML`?e.ownerDocument.body:e,n.stateNode),Np(e)):Qd(sl,n.stateNode));break;case 4:r=sl,i=cl,sl=n.stateNode.containerInfo,cl=!0,ll(e,t,n),sl=r,cl=i;break;case 0:case 11:case 14:case 15:Hc(2,n,t),tl||Hc(4,n,t),ll(e,t,n);break;case 1:tl||(Kc(n,t),r=n.stateNode,typeof r.componentWillUnmount==`function`&&Wc(n,t,r)),ll(e,t,n);break;case 21:ll(e,t,n);break;case 22:tl=(r=tl)||n.memoizedState!==null,ll(e,t,n),tl=r;break;default:ll(e,t,n)}}function dl(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{Np(e)}catch(e){Z(t,t.return,e)}}}function fl(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{Np(e)}catch(e){Z(t,t.return,e)}}function pl(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new G),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new G),t;default:throw Error(a(435,e.tag))}}function ml(e,t){var n=pl(e);t.forEach(function(t){if(!n.has(t)){n.add(t);var r=Yu.bind(null,e,t);t.then(r,r)}})}function hl(e,t){var n=t.deletions;if(n!==null)for(var r=0;r<n.length;r++){var i=n[r],o=e,s=t,c=s;a:for(;c!==null;){switch(c.tag){case 27:if(Zd(c.type)){sl=c.stateNode,cl=!1;break a}break;case 5:sl=c.stateNode,cl=!1;break a;case 3:case 4:sl=c.stateNode.containerInfo,cl=!0;break a}c=c.return}if(sl===null)throw Error(a(160));ul(o,s,i),sl=null,cl=!1,o=i.alternate,o!==null&&(o.return=null),i.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)_l(t,e),t=t.sibling}var gl=null;function _l(e,t){var n=e.alternate,r=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:hl(t,e),vl(e),r&4&&(Hc(3,e,e.return),Vc(3,e),Hc(5,e,e.return));break;case 1:hl(t,e),vl(e),r&512&&(tl||n===null||Kc(n,n.return)),r&64&&el&&(e=e.updateQueue,e!==null&&(r=e.callbacks,r!==null&&(n=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=n===null?r:n.concat(r))));break;case 26:var i=gl;if(hl(t,e),vl(e),r&512&&(tl||n===null||Kc(n,n.return)),r&4){var o=n===null?null:n.memoizedState;if(r=e.memoizedState,n===null)if(r===null)if(e.stateNode===null){a:{r=e.type,n=e.memoizedProps,i=i.ownerDocument||i;b:switch(r){case`title`:o=i.getElementsByTagName(`title`)[0],(!o||o[ht]||o[lt]||o.namespaceURI===`http://www.w3.org/2000/svg`||o.hasAttribute(`itemprop`))&&(o=i.createElement(r),i.head.insertBefore(o,i.querySelector(`head > title`))),Pd(o,r,n),o[lt]=e,xt(o),r=o;break a;case`link`:var s=Vf(`link`,`href`,i).get(r+(n.href||``));if(s){for(var c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`href`)===(n.href==null||n.href===``?null:n.href)&&o.getAttribute(`rel`)===(n.rel==null?null:n.rel)&&o.getAttribute(`title`)===(n.title==null?null:n.title)&&o.getAttribute(`crossorigin`)===(n.crossOrigin==null?null:n.crossOrigin)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;case`meta`:if(s=Vf(`meta`,`content`,i).get(r+(n.content||``))){for(c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`content`)===(n.content==null?null:``+n.content)&&o.getAttribute(`name`)===(n.name==null?null:n.name)&&o.getAttribute(`property`)===(n.property==null?null:n.property)&&o.getAttribute(`http-equiv`)===(n.httpEquiv==null?null:n.httpEquiv)&&o.getAttribute(`charset`)===(n.charSet==null?null:n.charSet)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;default:throw Error(a(468,r))}o[lt]=e,xt(o),r=o}e.stateNode=r}else Hf(i,e.type,e.stateNode);else e.stateNode=If(i,r,e.memoizedProps);else o===r?r===null&&e.stateNode!==null&&Jc(e,e.memoizedProps,n.memoizedProps):(o===null?n.stateNode!==null&&(n=n.stateNode,n.parentNode.removeChild(n)):o.count--,r===null?Hf(i,e.type,e.stateNode):If(i,r,e.memoizedProps))}break;case 27:hl(t,e),vl(e),r&512&&(tl||n===null||Kc(n,n.return)),n!==null&&r&4&&Jc(e,e.memoizedProps,n.memoizedProps);break;case 5:if(hl(t,e),vl(e),r&512&&(tl||n===null||Kc(n,n.return)),e.flags&32){i=e.stateNode;try{qt(i,``)}catch(t){Z(e,e.return,t)}}r&4&&e.stateNode!=null&&(i=e.memoizedProps,Jc(e,i,n===null?i:n.memoizedProps)),r&1024&&(nl=!0);break;case 6:if(hl(t,e),vl(e),r&4){if(e.stateNode===null)throw Error(a(162));r=e.memoizedProps,n=e.stateNode;try{n.nodeValue=r}catch(t){Z(e,e.return,t)}}break;case 3:if(Bf=null,i=gl,gl=gf(t.containerInfo),hl(t,e),gl=i,vl(e),r&4&&n!==null&&n.memoizedState.isDehydrated)try{Np(t.containerInfo)}catch(t){Z(e,e.return,t)}nl&&(nl=!1,yl(e));break;case 4:r=gl,gl=gf(e.stateNode.containerInfo),hl(t,e),vl(e),gl=r;break;case 12:hl(t,e),vl(e);break;case 31:hl(t,e),vl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 13:hl(t,e),vl(e),e.child.flags&8192&&e.memoizedState!==null!=(n!==null&&n.memoizedState!==null)&&($l=Oe()),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 22:i=e.memoizedState!==null;var l=n!==null&&n.memoizedState!==null,u=el,d=tl;if(el=u||i,tl=d||l,hl(t,e),tl=d,el=u,vl(e),r&8192)a:for(t=e.stateNode,t._visibility=i?t._visibility&-2:t._visibility|1,i&&(n===null||l||el||tl||xl(e)),n=null,t=e;;){if(t.tag===5||t.tag===26){if(n===null){l=n=t;try{if(o=l.stateNode,i)s=o.style,typeof s.setProperty==`function`?s.setProperty(`display`,`none`,`important`):s.display=`none`;else{c=l.stateNode;var f=l.memoizedProps.style,p=f!=null&&f.hasOwnProperty(`display`)?f.display:null;c.style.display=p==null||typeof p==`boolean`?``:(``+p).trim()}}catch(e){Z(l,l.return,e)}}}else if(t.tag===6){if(n===null){l=t;try{l.stateNode.nodeValue=i?``:l.memoizedProps}catch(e){Z(l,l.return,e)}}}else if(t.tag===18){if(n===null){l=t;try{var m=l.stateNode;i?$d(m,!0):$d(l.stateNode,!1)}catch(e){Z(l,l.return,e)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break a;for(;t.sibling===null;){if(t.return===null||t.return===e)break a;n===t&&(n=null),t=t.return}n===t&&(n=null),t.sibling.return=t.return,t=t.sibling}r&4&&(r=e.updateQueue,r!==null&&(n=r.retryQueue,n!==null&&(r.retryQueue=null,ml(e,n))));break;case 19:hl(t,e),vl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,ml(e,r)));break;case 30:break;case 21:break;default:hl(t,e),vl(e)}}function vl(e){var t=e.flags;if(t&2){try{for(var n,r=e.return;r!==null;){if(Yc(r)){n=r;break}r=r.return}if(n==null)throw Error(a(160));switch(n.tag){case 27:var i=n.stateNode;Qc(e,Xc(e),i);break;case 5:var o=n.stateNode;n.flags&32&&(qt(o,``),n.flags&=-33),Qc(e,Xc(e),o);break;case 3:case 4:var s=n.stateNode.containerInfo;Zc(e,Xc(e),s);break;default:throw Error(a(161))}}catch(t){Z(e,e.return,t)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function yl(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;yl(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function bl(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)al(e,t.alternate,t),t=t.sibling}function xl(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:Hc(4,t,t.return),xl(t);break;case 1:Kc(t,t.return);var n=t.stateNode;typeof n.componentWillUnmount==`function`&&Wc(t,t.return,n),xl(t);break;case 27:pf(t.stateNode);case 26:case 5:Kc(t,t.return),xl(t);break;case 22:t.memoizedState===null&&xl(t);break;case 30:xl(t);break;default:xl(t)}e=e.sibling}}function Sl(e,t,n){for(n&&=(t.subtreeFlags&8772)!=0,t=t.child;t!==null;){var r=t.alternate,i=e,a=t,o=a.flags;switch(a.tag){case 0:case 11:case 15:Sl(i,a,n),Vc(4,a);break;case 1:if(Sl(i,a,n),r=a,i=r.stateNode,typeof i.componentDidMount==`function`)try{i.componentDidMount()}catch(e){Z(r,r.return,e)}if(r=a,i=r.updateQueue,i!==null){var s=r.stateNode;try{var c=i.shared.hiddenCallbacks;if(c!==null)for(i.shared.hiddenCallbacks=null,i=0;i<c.length;i++)Za(c[i],s)}catch(e){Z(r,r.return,e)}}n&&o&64&&Uc(a),Gc(a,a.return);break;case 27:$c(a);case 26:case 5:Sl(i,a,n),n&&r===null&&o&4&&qc(a),Gc(a,a.return);break;case 12:Sl(i,a,n);break;case 31:Sl(i,a,n),n&&o&4&&dl(i,a);break;case 13:Sl(i,a,n),n&&o&4&&fl(i,a);break;case 22:a.memoizedState===null&&Sl(i,a,n),Gc(a,a.return);break;case 30:break;default:Sl(i,a,n)}t=t.sibling}}function Cl(e,t){var n=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==n&&(e!=null&&e.refCount++,n!=null&&ua(n))}function wl(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ua(e))}function Tl(e,t,n,r){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)El(e,t,n,r),t=t.sibling}function El(e,t,n,r){var i=t.flags;switch(t.tag){case 0:case 11:case 15:Tl(e,t,n,r),i&2048&&Vc(9,t);break;case 1:Tl(e,t,n,r);break;case 3:Tl(e,t,n,r),i&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ua(e)));break;case 12:if(i&2048){Tl(e,t,n,r),e=t.stateNode;try{var a=t.memoizedProps,o=a.id,s=a.onPostCommit;typeof s==`function`&&s(o,t.alternate===null?`mount`:`update`,e.passiveEffectDuration,-0)}catch(e){Z(t,t.return,e)}}else Tl(e,t,n,r);break;case 31:Tl(e,t,n,r);break;case 13:Tl(e,t,n,r);break;case 23:break;case 22:a=t.stateNode,o=t.alternate,t.memoizedState===null?a._visibility&2?Tl(e,t,n,r):(a._visibility|=2,Dl(e,t,n,r,(t.subtreeFlags&10256)!=0||!1)):a._visibility&2?Tl(e,t,n,r):Ol(e,t),i&2048&&Cl(o,t);break;case 24:Tl(e,t,n,r),i&2048&&wl(t.alternate,t);break;default:Tl(e,t,n,r)}}function Dl(e,t,n,r,i){for(i&&=(t.subtreeFlags&10256)!=0||!1,t=t.child;t!==null;){var a=e,o=t,s=n,c=r,l=o.flags;switch(o.tag){case 0:case 11:case 15:Dl(a,o,s,c,i),Vc(8,o);break;case 23:break;case 22:var u=o.stateNode;o.memoizedState===null?(u._visibility|=2,Dl(a,o,s,c,i)):u._visibility&2?Dl(a,o,s,c,i):Ol(a,o),i&&l&2048&&Cl(o.alternate,o);break;case 24:Dl(a,o,s,c,i),i&&l&2048&&wl(o.alternate,o);break;default:Dl(a,o,s,c,i)}t=t.sibling}}function Ol(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var n=e,r=t,i=r.flags;switch(r.tag){case 22:Ol(n,r),i&2048&&Cl(r.alternate,r);break;case 24:Ol(n,r),i&2048&&wl(r.alternate,r);break;default:Ol(n,r)}t=t.sibling}}var kl=8192;function Al(e,t,n){if(e.subtreeFlags&kl)for(e=e.child;e!==null;)jl(e,t,n),e=e.sibling}function jl(e,t,n){switch(e.tag){case 26:Al(e,t,n),e.flags&kl&&e.memoizedState!==null&&Gf(n,gl,e.memoizedState,e.memoizedProps);break;case 5:Al(e,t,n);break;case 3:case 4:var r=gl;gl=gf(e.stateNode.containerInfo),Al(e,t,n),gl=r;break;case 22:e.memoizedState===null&&(r=e.alternate,r!==null&&r.memoizedState!==null?(r=kl,kl=16777216,Al(e,t,n),kl=r):Al(e,t,n));break;default:Al(e,t,n)}}function Ml(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function Nl(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Il(r,e)}Ml(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)Pl(e),e=e.sibling}function Pl(e){switch(e.tag){case 0:case 11:case 15:Nl(e),e.flags&2048&&Hc(9,e,e.return);break;case 3:Nl(e);break;case 12:Nl(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,Fl(e)):Nl(e);break;default:Nl(e)}}function Fl(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Il(r,e)}Ml(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:Hc(8,t,t.return),Fl(t);break;case 22:n=t.stateNode,n._visibility&2&&(n._visibility&=-3,Fl(t));break;default:Fl(t)}e=e.sibling}}function Il(e,t){for(;rl!==null;){var n=rl;switch(n.tag){case 0:case 11:case 15:Hc(8,n,t);break;case 23:case 22:if(n.memoizedState!==null&&n.memoizedState.cachePool!==null){var r=n.memoizedState.cachePool.pool;r!=null&&r.refCount++}break;case 24:ua(n.memoizedState.cache)}if(r=n.child,r!==null)r.return=n,rl=r;else a:for(n=e;rl!==null;){r=rl;var i=r.sibling,a=r.return;if(ol(r),r===n){rl=null;break a}if(i!==null){i.return=a,rl=i;break a}rl=a}}}var Ll={getCacheForType:function(e){var t=na(ca),n=t.data.get(e);return n===void 0&&(n=e(),t.data.set(e,n)),n},cacheSignal:function(){return na(ca).controller.signal}},Rl=typeof WeakMap==`function`?WeakMap:Map,K=0,q=null,J=null,Y=0,X=0,zl=null,Bl=!1,Vl=!1,Hl=!1,Ul=0,Wl=0,Gl=0,Kl=0,ql=0,Jl=0,Yl=0,Xl=null,Zl=null,Ql=!1,$l=0,eu=0,tu=1/0,nu=null,ru=null,iu=0,au=null,ou=null,su=0,cu=0,lu=null,uu=null,du=0,fu=null;function pu(){return K&2&&Y!==0?Y&-Y:O.T===null?ot():dd()}function mu(){if(Jl===0)if(!(Y&536870912)||R){var e=Ge;Ge<<=1,!(Ge&3932160)&&(Ge=262144),Jl=e}else Jl=536870912;return e=io.current,e!==null&&(e.flags|=32),Jl}function hu(e,t,n){(e===q&&(X===2||X===9)||e.cancelPendingCommit!==null)&&(Su(e,0),yu(e,Y,Jl,!1)),$e(e,n),(!(K&2)||e!==q)&&(e===q&&(!(K&2)&&(Kl|=n),Wl===4&&yu(e,Y,Jl,!1)),rd(e))}function gu(e,t,n){if(K&6)throw Error(a(327));var r=!n&&(t&127)==0&&(t&e.expiredLanes)===0||Ye(e,t),i=r?Au(e,t):Ou(e,t,!0),o=r;do{if(i===0){Vl&&!r&&yu(e,t,0,!1);break}else{if(n=e.current.alternate,o&&!vu(n)){i=Ou(e,t,!1),o=!1;continue}if(i===2){if(o=t,e.errorRecoveryDisabledLanes&o)var s=0;else s=e.pendingLanes&-536870913,s=s===0?s&536870912?536870912:0:s;if(s!==0){t=s;a:{var c=e;i=Xl;var l=c.current.memoizedState.isDehydrated;if(l&&(Su(c,s).flags|=256),s=Ou(c,s,!1),s!==2){if(Hl&&!l){c.errorRecoveryDisabledLanes|=o,Kl|=o,i=4;break a}o=Zl,Zl=i,o!==null&&(Zl===null?Zl=o:Zl.push.apply(Zl,o))}i=s}if(o=!1,i!==2)continue}}if(i===1){Su(e,0),yu(e,t,0,!0);break}a:{switch(r=e,o=i,o){case 0:case 1:throw Error(a(345));case 4:if((t&4194048)!==t)break;case 6:yu(r,t,Jl,!Bl);break a;case 2:Zl=null;break;case 3:case 5:break;default:throw Error(a(329))}if((t&62914560)===t&&(i=$l+300-Oe(),10<i)){if(yu(r,t,Jl,!Bl),Je(r,0,!0)!==0)break a;su=t,r.timeoutHandle=Kd(_u.bind(null,r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,`Throttled`,-0,0),i);break a}_u(r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,null,-0,0)}}break}while(1);rd(e)}function _u(e,t,n,r,i,a,o,s,c,l,u,d,f,p){if(e.timeoutHandle=-1,d=t.subtreeFlags,d&8192||(d&16785408)==16785408){d={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:tn},jl(t,a,d);var m=(a&62914560)===a?$l-Oe():(a&4194048)===a?eu-Oe():0;if(m=qf(d,m),m!==null){su=a,e.cancelPendingCommit=m(Lu.bind(null,e,t,a,n,r,i,o,s,c,u,d,null,f,p)),yu(e,a,o,!l);return}}Lu(e,t,a,n,r,i,o,s,c)}function vu(e){for(var t=e;;){var n=t.tag;if((n===0||n===11||n===15)&&t.flags&16384&&(n=t.updateQueue,n!==null&&(n=n.stores,n!==null)))for(var r=0;r<n.length;r++){var i=n[r],a=i.getSnapshot;i=i.value;try{if(!wr(a(),i))return!1}catch{return!1}}if(n=t.child,t.subtreeFlags&16384&&n!==null)n.return=t,t=n;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function yu(e,t,n,r){t&=~ql,t&=~Kl,e.suspendedLanes|=t,e.pingedLanes&=~t,r&&(e.warmLanes|=t),r=e.expirationTimes;for(var i=t;0<i;){var a=31-Be(i),o=1<<a;r[a]=-1,i&=~o}n!==0&&tt(e,n,t)}function bu(){return K&6?!0:(id(0,!1),!1)}function xu(){if(J!==null){if(X===0)var e=J.return;else e=J,Ji=qi=null,jo(e),Na=null,Pa=0,e=J;for(;e!==null;)Bc(e.alternate,e),e=e.return;J=null}}function Su(e,t){var n=e.timeoutHandle;n!==-1&&(e.timeoutHandle=-1,qd(n)),n=e.cancelPendingCommit,n!==null&&(e.cancelPendingCommit=null,n()),su=0,xu(),q=e,J=n=di(e.current,null),Y=t,X=0,zl=null,Bl=!1,Vl=Ye(e,t),Hl=!1,Yl=Jl=ql=Kl=Gl=Wl=0,Zl=Xl=null,Ql=!1,t&8&&(t|=t&32);var r=e.entangledLanes;if(r!==0)for(e=e.entanglements,r&=t;0<r;){var i=31-Be(r),a=1<<i;t|=e[i],r&=~a}return Ul=t,ti(),n}function Cu(e,t){z=null,O.H=Ls,t===Ca||t===Ta?(t=ja(),X=3):t===wa?(t=ja(),X=4):X=t===tc?8:typeof t==`object`&&t&&typeof t.then==`function`?6:1,zl=t,J===null&&(Wl=1,Ys(e,yi(t,e.current)))}function wu(){var e=io.current;return e===null?!0:(Y&4194048)===Y?ao===null:(Y&62914560)===Y||Y&536870912?e===ao:!1}function Tu(){var e=O.H;return O.H=Ls,e===null?Ls:e}function Eu(){var e=O.A;return O.A=Ll,e}function Du(){Wl=4,Bl||(Y&4194048)!==Y&&io.current!==null||(Vl=!0),!(Gl&134217727)&&!(Kl&134217727)||q===null||yu(q,Y,Jl,!1)}function Ou(e,t,n){var r=K;K|=2;var i=Tu(),a=Eu();(q!==e||Y!==t)&&(nu=null,Su(e,t)),t=!1;var o=Wl;a:do try{if(X!==0&&J!==null){var s=J,c=zl;switch(X){case 8:xu(),o=6;break a;case 3:case 2:case 9:case 6:io.current===null&&(t=!0);var l=X;if(X=0,zl=null,Pu(e,s,c,l),n&&Vl){o=0;break a}break;default:l=X,X=0,zl=null,Pu(e,s,c,l)}}ku(),o=Wl;break}catch(t){Cu(e,t)}while(1);return t&&e.shellSuspendCounter++,Ji=qi=null,K=r,O.H=i,O.A=a,J===null&&(q=null,Y=0,ti()),o}function ku(){for(;J!==null;)Mu(J)}function Au(e,t){var n=K;K|=2;var r=Tu(),i=Eu();q!==e||Y!==t?(nu=null,tu=Oe()+500,Su(e,t)):Vl=Ye(e,t);a:do try{if(X!==0&&J!==null){t=J;var o=zl;b:switch(X){case 1:X=0,zl=null,Pu(e,t,o,1);break;case 2:case 9:if(Da(o)){X=0,zl=null,Nu(t);break}t=function(){X!==2&&X!==9||q!==e||(X=7),rd(e)},o.then(t,t);break a;case 3:X=7;break a;case 4:X=5;break a;case 7:Da(o)?(X=0,zl=null,Nu(t)):(X=0,zl=null,Pu(e,t,o,7));break;case 5:var s=null;switch(J.tag){case 26:s=J.memoizedState;case 5:case 27:var c=J;if(s?Wf(s):c.stateNode.complete){X=0,zl=null;var l=c.sibling;if(l!==null)J=l;else{var u=c.return;u===null?J=null:(J=u,Fu(u))}break b}}X=0,zl=null,Pu(e,t,o,5);break;case 6:X=0,zl=null,Pu(e,t,o,6);break;case 8:xu(),Wl=6;break a;default:throw Error(a(462))}}ju();break}catch(t){Cu(e,t)}while(1);return Ji=qi=null,O.H=r,O.A=i,K=n,J===null?(q=null,Y=0,ti(),Wl):0}function ju(){for(;J!==null&&!Ee();)Mu(J)}function Mu(e){var t=jc(e.alternate,e,Ul);e.memoizedProps=e.pendingProps,t===null?Fu(e):J=t}function Nu(e){var t=e,n=t.alternate;switch(t.tag){case 15:case 0:t=hc(n,t,t.pendingProps,t.type,void 0,Y);break;case 11:t=hc(n,t,t.pendingProps,t.type.render,t.ref,Y);break;case 5:jo(t);default:Bc(n,t),t=J=fi(t,Ul),t=jc(n,t,Ul)}e.memoizedProps=e.pendingProps,t===null?Fu(e):J=t}function Pu(e,t,n,r){Ji=qi=null,jo(t),Na=null,Pa=0;var i=t.return;try{if(ec(e,i,t,n,Y)){Wl=1,Ys(e,yi(n,e.current)),J=null;return}}catch(t){if(i!==null)throw J=i,t;Wl=1,Ys(e,yi(n,e.current)),J=null;return}t.flags&32768?(R||r===1?e=!0:Vl||Y&536870912?e=!1:(Bl=e=!0,(r===2||r===9||r===3||r===6)&&(r=io.current,r!==null&&r.tag===13&&(r.flags|=16384))),Iu(t,e)):Fu(t)}function Fu(e){var t=e;do{if(t.flags&32768){Iu(t,Bl);return}e=t.return;var n=Rc(t.alternate,t,Ul);if(n!==null){J=n;return}if(t=t.sibling,t!==null){J=t;return}J=t=e}while(t!==null);Wl===0&&(Wl=5)}function Iu(e,t){do{var n=zc(e.alternate,e);if(n!==null){n.flags&=32767,J=n;return}if(n=e.return,n!==null&&(n.flags|=32768,n.subtreeFlags=0,n.deletions=null),!t&&(e=e.sibling,e!==null)){J=e;return}J=e=n}while(e!==null);Wl=6,J=null}function Lu(e,t,n,r,i,o,s,c,l){e.cancelPendingCommit=null;do Hu();while(iu!==0);if(K&6)throw Error(a(327));if(t!==null){if(t===e.current)throw Error(a(177));if(o=t.lanes|t.childLanes,o|=ei,et(e,n,o,s,c,l),e===q&&(J=q=null,Y=0),ou=t,au=e,su=n,cu=o,lu=i,uu=r,t.subtreeFlags&10256||t.flags&10256?(e.callbackNode=null,e.callbackPriority=0,Xu(Me,function(){return Uu(),null})):(e.callbackNode=null,e.callbackPriority=0),r=(t.flags&13878)!=0,t.subtreeFlags&13878||r){r=O.T,O.T=null,i=k.p,k.p=2,s=K,K|=4;try{il(e,t,n)}finally{K=s,k.p=i,O.T=r}}iu=1,Ru(),zu(),Bu()}}function Ru(){if(iu===1){iu=0;var e=au,t=ou,n=(t.flags&13878)!=0;if(t.subtreeFlags&13878||n){n=O.T,O.T=null;var r=k.p;k.p=2;var i=K;K|=4;try{_l(t,e);var a=zd,o=kr(e.containerInfo),s=a.focusedElem,c=a.selectionRange;if(o!==s&&s&&s.ownerDocument&&Or(s.ownerDocument.documentElement,s)){if(c!==null&&Ar(s)){var l=c.start,u=c.end;if(u===void 0&&(u=l),`selectionStart`in s)s.selectionStart=l,s.selectionEnd=Math.min(u,s.value.length);else{var d=s.ownerDocument||document,f=d&&d.defaultView||window;if(f.getSelection){var p=f.getSelection(),m=s.textContent.length,h=Math.min(c.start,m),g=c.end===void 0?h:Math.min(c.end,m);!p.extend&&h>g&&(o=g,g=h,h=o);var _=Dr(s,h),v=Dr(s,g);if(_&&v&&(p.rangeCount!==1||p.anchorNode!==_.node||p.anchorOffset!==_.offset||p.focusNode!==v.node||p.focusOffset!==v.offset)){var y=d.createRange();y.setStart(_.node,_.offset),p.removeAllRanges(),h>g?(p.addRange(y),p.extend(v.node,v.offset)):(y.setEnd(v.node,v.offset),p.addRange(y))}}}}for(d=[],p=s;p=p.parentNode;)p.nodeType===1&&d.push({element:p,left:p.scrollLeft,top:p.scrollTop});for(typeof s.focus==`function`&&s.focus(),s=0;s<d.length;s++){var b=d[s];b.element.scrollLeft=b.left,b.element.scrollTop=b.top}}sp=!!Rd,zd=Rd=null}finally{K=i,k.p=r,O.T=n}}e.current=t,iu=2}}function zu(){if(iu===2){iu=0;var e=au,t=ou,n=(t.flags&8772)!=0;if(t.subtreeFlags&8772||n){n=O.T,O.T=null;var r=k.p;k.p=2;var i=K;K|=4;try{al(e,t.alternate,t)}finally{K=i,k.p=r,O.T=n}}iu=3}}function Bu(){if(iu===4||iu===3){iu=0,De();var e=au,t=ou,n=su,r=uu;t.subtreeFlags&10256||t.flags&10256?iu=5:(iu=0,ou=au=null,Vu(e,e.pendingLanes));var i=e.pendingLanes;if(i===0&&(ru=null),at(n),t=t.stateNode,Re&&typeof Re.onCommitFiberRoot==`function`)try{Re.onCommitFiberRoot(Le,t,void 0,(t.current.flags&128)==128)}catch{}if(r!==null){t=O.T,i=k.p,k.p=2,O.T=null;try{for(var a=e.onRecoverableError,o=0;o<r.length;o++){var s=r[o];a(s.value,{componentStack:s.stack})}}finally{O.T=t,k.p=i}}su&3&&Hu(),rd(e),i=e.pendingLanes,n&261930&&i&42?e===fu?du++:(du=0,fu=e):du=0,id(0,!1)}}function Vu(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,ua(t)))}function Hu(){return Ru(),zu(),Bu(),Uu()}function Uu(){if(iu!==5)return!1;var e=au,t=cu;cu=0;var n=at(su),r=O.T,i=k.p;try{k.p=32>n?32:n,O.T=null,n=lu,lu=null;var o=au,s=su;if(iu=0,ou=au=null,su=0,K&6)throw Error(a(331));var c=K;if(K|=4,Pl(o.current),El(o,o.current,s,n),K=c,id(0,!1),Re&&typeof Re.onPostCommitFiberRoot==`function`)try{Re.onPostCommitFiberRoot(Le,o)}catch{}return!0}finally{k.p=i,O.T=r,Vu(e,t)}}function Wu(e,t,n){t=yi(n,t),t=Zs(e.stateNode,t,2),e=Ga(e,t,2),e!==null&&($e(e,2),rd(e))}function Z(e,t,n){if(e.tag===3)Wu(e,e,n);else for(;t!==null;){if(t.tag===3){Wu(t,e,n);break}else if(t.tag===1){var r=t.stateNode;if(typeof t.type.getDerivedStateFromError==`function`||typeof r.componentDidCatch==`function`&&(ru===null||!ru.has(r))){e=yi(n,e),n=Qs(2),r=Ga(t,n,2),r!==null&&($s(n,r,t,e),$e(r,2),rd(r));break}}t=t.return}}function Gu(e,t,n){var r=e.pingCache;if(r===null){r=e.pingCache=new Rl;var i=new Set;r.set(t,i)}else i=r.get(t),i===void 0&&(i=new Set,r.set(t,i));i.has(n)||(Hl=!0,i.add(n),e=Ku.bind(null,e,t,n),t.then(e,e))}function Ku(e,t,n){var r=e.pingCache;r!==null&&r.delete(t),e.pingedLanes|=e.suspendedLanes&n,e.warmLanes&=~n,q===e&&(Y&n)===n&&(Wl===4||Wl===3&&(Y&62914560)===Y&&300>Oe()-$l?!(K&2)&&Su(e,0):ql|=n,Yl===Y&&(Yl=0)),rd(e)}function qu(e,t){t===0&&(t=Ze()),e=ii(e,t),e!==null&&($e(e,t),rd(e))}function Ju(e){var t=e.memoizedState,n=0;t!==null&&(n=t.retryLane),qu(e,n)}function Yu(e,t){var n=0;switch(e.tag){case 31:case 13:var r=e.stateNode,i=e.memoizedState;i!==null&&(n=i.retryLane);break;case 19:r=e.stateNode;break;case 22:r=e.stateNode._retryCache;break;default:throw Error(a(314))}r!==null&&r.delete(t),qu(e,n)}function Xu(e,t){return we(e,t)}var Zu=null,Qu=null,$u=!1,ed=!1,td=!1,nd=0;function rd(e){e!==Qu&&e.next===null&&(Qu===null?Zu=Qu=e:Qu=Qu.next=e),ed=!0,$u||($u=!0,ud())}function id(e,t){if(!td&&ed){td=!0;do for(var n=!1,r=Zu;r!==null;){if(!t)if(e!==0){var i=r.pendingLanes;if(i===0)var a=0;else{var o=r.suspendedLanes,s=r.pingedLanes;a=(1<<31-Be(42|e)+1)-1,a&=i&~(o&~s),a=a&201326741?a&201326741|1:a?a|2:0}a!==0&&(n=!0,ld(r,a))}else a=Y,a=Je(r,r===q?a:0,r.cancelPendingCommit!==null||r.timeoutHandle!==-1),!(a&3)||Ye(r,a)||(n=!0,ld(r,a));r=r.next}while(n);td=!1}}function ad(){od()}function od(){ed=$u=!1;var e=0;nd!==0&&Gd()&&(e=nd);for(var t=Oe(),n=null,r=Zu;r!==null;){var i=r.next,a=sd(r,t);a===0?(r.next=null,n===null?Zu=i:n.next=i,i===null&&(Qu=n)):(n=r,(e!==0||a&3)&&(ed=!0)),r=i}iu!==0&&iu!==5||id(e,!1),nd!==0&&(nd=0)}function sd(e,t){for(var n=e.suspendedLanes,r=e.pingedLanes,i=e.expirationTimes,a=e.pendingLanes&-62914561;0<a;){var o=31-Be(a),s=1<<o,c=i[o];c===-1?((s&n)===0||(s&r)!==0)&&(i[o]=Xe(s,t)):c<=t&&(e.expiredLanes|=s),a&=~s}if(t=q,n=Y,n=Je(e,e===t?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r=e.callbackNode,n===0||e===t&&(X===2||X===9)||e.cancelPendingCommit!==null)return r!==null&&r!==null&&Te(r),e.callbackNode=null,e.callbackPriority=0;if(!(n&3)||Ye(e,n)){if(t=n&-n,t===e.callbackPriority)return t;switch(r!==null&&Te(r),at(n)){case 2:case 8:n=je;break;case 32:n=Me;break;case 268435456:n=Pe;break;default:n=Me}return r=cd.bind(null,e),n=we(n,r),e.callbackPriority=t,e.callbackNode=n,t}return r!==null&&r!==null&&Te(r),e.callbackPriority=2,e.callbackNode=null,2}function cd(e,t){if(iu!==0&&iu!==5)return e.callbackNode=null,e.callbackPriority=0,null;var n=e.callbackNode;if(Hu()&&e.callbackNode!==n)return null;var r=Y;return r=Je(e,e===q?r:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r===0?null:(gu(e,r,t),sd(e,Oe()),e.callbackNode!=null&&e.callbackNode===n?cd.bind(null,e):null)}function ld(e,t){if(Hu())return null;gu(e,t,!0)}function ud(){Yd(function(){K&6?we(Ae,ad):od()})}function dd(){if(nd===0){var e=pa;e===0&&(e=We,We<<=1,!(We&261888)&&(We=256)),nd=e}return nd}function fd(e){return e==null||typeof e==`symbol`||typeof e==`boolean`?null:typeof e==`function`?e:en(``+e)}function pd(e,t){var n=t.ownerDocument.createElement(`input`);return n.name=t.name,n.value=t.value,e.id&&n.setAttribute(`form`,e.id),t.parentNode.insertBefore(n,t),e=new FormData(e),n.parentNode.removeChild(n),e}function md(e,t,n,r,i){if(t===`submit`&&n&&n.stateNode===i){var a=fd((i[I]||null).action),o=r.submitter;o&&(t=(t=o[I]||null)?fd(t.formAction):o.getAttribute(`formAction`),t!==null&&(a=t,o=null));var s=new Cn(`action`,`action`,null,r,i);e.push({event:s,listeners:[{instance:null,listener:function(){if(r.defaultPrevented){if(nd!==0){var e=o?pd(i,o):new FormData(i);Cs(n,{pending:!0,data:e,method:i.method,action:a},null,e)}}else typeof a==`function`&&(s.preventDefault(),e=o?pd(i,o):new FormData(i),Cs(n,{pending:!0,data:e,method:i.method,action:a},a,e))},currentTarget:i}]})}}for(var hd=0;hd<Yr.length;hd++){var gd=Yr[hd];Xr(gd.toLowerCase(),`on`+(gd[0].toUpperCase()+gd.slice(1)))}Xr(Hr,`onAnimationEnd`),Xr(Ur,`onAnimationIteration`),Xr(Wr,`onAnimationStart`),Xr(`dblclick`,`onDoubleClick`),Xr(`focusin`,`onFocus`),Xr(`focusout`,`onBlur`),Xr(L,`onTransitionRun`),Xr(Gr,`onTransitionStart`),Xr(Kr,`onTransitionCancel`),Xr(qr,`onTransitionEnd`),Tt(`onMouseEnter`,[`mouseout`,`mouseover`]),Tt(`onMouseLeave`,[`mouseout`,`mouseover`]),Tt(`onPointerEnter`,[`pointerout`,`pointerover`]),Tt(`onPointerLeave`,[`pointerout`,`pointerover`]),wt(`onChange`,`change click focusin focusout input keydown keyup selectionchange`.split(` `)),wt(`onSelect`,`focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(` `)),wt(`onBeforeInput`,[`compositionend`,`keypress`,`textInput`,`paste`]),wt(`onCompositionEnd`,`compositionend focusout keydown keypress keyup mousedown`.split(` `)),wt(`onCompositionStart`,`compositionstart focusout keydown keypress keyup mousedown`.split(` `)),wt(`onCompositionUpdate`,`compositionupdate focusout keydown keypress keyup mousedown`.split(` `));var _d=`abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting`.split(` `),vd=new Set(`beforetoggle cancel close invalid load scroll scrollend toggle`.split(` `).concat(_d));function yd(e,t){t=(t&4)!=0;for(var n=0;n<e.length;n++){var r=e[n],i=r.event;r=r.listeners;a:{var a=void 0;if(t)for(var o=r.length-1;0<=o;o--){var s=r[o],c=s.instance,l=s.currentTarget;if(s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){Zr(e)}i.currentTarget=null,a=c}else for(o=0;o<r.length;o++){if(s=r[o],c=s.instance,l=s.currentTarget,s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){Zr(e)}i.currentTarget=null,a=c}}}}function Q(e,t){var n=t[dt];n===void 0&&(n=t[dt]=new Set);var r=e+`__bubble`;n.has(r)||(Cd(t,e,2,!1),n.add(r))}function bd(e,t,n){var r=0;t&&(r|=4),Cd(n,e,r,t)}var xd=`_reactListening`+Math.random().toString(36).slice(2);function Sd(e){if(!e[xd]){e[xd]=!0,St.forEach(function(t){t!==`selectionchange`&&(vd.has(t)||bd(t,!1,e),bd(t,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[xd]||(t[xd]=!0,bd(`selectionchange`,!1,t))}}function Cd(e,t,n,r){switch(mp(t)){case 2:var i=cp;break;case 8:i=lp;break;default:i=up}n=i.bind(null,t,n,e),i=void 0,!fn||t!==`touchstart`&&t!==`touchmove`&&t!==`wheel`||(i=!0),r?i===void 0?e.addEventListener(t,n,!0):e.addEventListener(t,n,{capture:!0,passive:i}):i===void 0?e.addEventListener(t,n,!1):e.addEventListener(t,n,{passive:i})}function wd(e,t,n,r,i){var a=r;if(!(t&1)&&!(t&2)&&r!==null)a:for(;;){if(r===null)return;var o=r.tag;if(o===3||o===4){var c=r.stateNode.containerInfo;if(c===i)break;if(o===4)for(o=r.return;o!==null;){var l=o.tag;if((l===3||l===4)&&o.stateNode.containerInfo===i)return;o=o.return}for(;c!==null;){if(o=_t(c),o===null)return;if(l=o.tag,l===5||l===6||l===26||l===27){r=a=o;continue a}c=c.parentNode}}r=r.return}ln(function(){var r=a,i=rn(n),o=[];a:{var c=Jr.get(e);if(c!==void 0){var l=Cn,u=e;switch(e){case`keypress`:if(vn(n)===0)break a;case`keydown`:case`keyup`:l=Vn;break;case`focusin`:u=`focus`,l=Mn;break;case`focusout`:u=`blur`,l=Mn;break;case`beforeblur`:case`afterblur`:l=Mn;break;case`click`:if(n.button===2)break a;case`auxclick`:case`dblclick`:case`mousedown`:case`mousemove`:case`mouseup`:case`mouseout`:case`mouseover`:case`contextmenu`:l=An;break;case`drag`:case`dragend`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`dragstart`:case`drop`:l=jn;break;case`touchcancel`:case`touchend`:case`touchmove`:case`touchstart`:l=Un;break;case Hr:case Ur:case Wr:l=Nn;break;case qr:l=Wn;break;case`scroll`:case`scrollend`:l=Tn;break;case`wheel`:l=Gn;break;case`copy`:case`cut`:case`paste`:l=Pn;break;case`gotpointercapture`:case`lostpointercapture`:case`pointercancel`:case`pointerdown`:case`pointermove`:case`pointerout`:case`pointerover`:case`pointerup`:l=Hn;break;case`toggle`:case`beforetoggle`:l=Kn}var d=(t&4)!=0,f=!d&&(e===`scroll`||e===`scrollend`),p=d?c===null?null:c+`Capture`:c;d=[];for(var m=r,h;m!==null;){var g=m;if(h=g.stateNode,g=g.tag,g!==5&&g!==26&&g!==27||h===null||p===null||(g=un(m,p),g!=null&&d.push(Td(m,g,h))),f)break;m=m.return}0<d.length&&(c=new l(c,u,null,n,i),o.push({event:c,listeners:d}))}}if(!(t&7)){a:{if(c=e===`mouseover`||e===`pointerover`,l=e===`mouseout`||e===`pointerout`,c&&n!==nn&&(u=n.relatedTarget||n.fromElement)&&(_t(u)||u[ut]))break a;if((l||c)&&(c=i.window===i?i:(c=i.ownerDocument)?c.defaultView||c.parentWindow:window,l?(u=n.relatedTarget||n.toElement,l=r,u=u?_t(u):null,u!==null&&(f=s(u),d=u.tag,u!==f||d!==5&&d!==27&&d!==6)&&(u=null)):(l=null,u=r),l!==u)){if(d=An,g=`onMouseLeave`,p=`onMouseEnter`,m=`mouse`,(e===`pointerout`||e===`pointerover`)&&(d=Hn,g=`onPointerLeave`,p=`onPointerEnter`,m=`pointer`),f=l==null?c:yt(l),h=u==null?c:yt(u),c=new d(g,m+`leave`,l,n,i),c.target=f,c.relatedTarget=h,g=null,_t(i)===r&&(d=new d(p,m+`enter`,u,n,i),d.target=h,d.relatedTarget=f,g=d),f=g,l&&u)b:{for(d=Dd,p=l,m=u,h=0,g=p;g;g=d(g))h++;g=0;for(var _=m;_;_=d(_))g++;for(;0<h-g;)p=d(p),h--;for(;0<g-h;)m=d(m),g--;for(;h--;){if(p===m||m!==null&&p===m.alternate){d=p;break b}p=d(p),m=d(m)}d=null}else d=null;l!==null&&Od(o,c,l,d,!1),u!==null&&f!==null&&Od(o,f,u,d,!0)}}a:{if(c=r?yt(r):window,l=c.nodeName&&c.nodeName.toLowerCase(),l===`select`||l===`input`&&c.type===`file`)var v=fr;else if(or(c))if(pr)v=Sr;else{v=br;var y=yr}else l=c.nodeName,!l||l.toLowerCase()!==`input`||c.type!==`checkbox`&&c.type!==`radio`?r&&Zt(r.elementType)&&(v=fr):v=xr;if(v&&=v(e,r)){sr(o,v,n,i);break a}y&&y(e,c,r),e===`focusout`&&r&&c.type===`number`&&r.memoizedProps.value!=null&&Ut(c,`number`,c.value)}switch(y=r?yt(r):window,e){case`focusin`:(or(y)||y.contentEditable===`true`)&&(Mr=y,Nr=r,Pr=null);break;case`focusout`:Pr=Nr=Mr=null;break;case`mousedown`:Fr=!0;break;case`contextmenu`:case`mouseup`:case`dragend`:Fr=!1,Ir(o,n,i);break;case`selectionchange`:if(jr)break;case`keydown`:case`keyup`:Ir(o,n,i)}var b;if(Jn)b:{switch(e){case`compositionstart`:var x=`onCompositionStart`;break b;case`compositionend`:x=`onCompositionEnd`;break b;case`compositionupdate`:x=`onCompositionUpdate`;break b}x=void 0}else nr?er(e,n)&&(x=`onCompositionEnd`):e===`keydown`&&n.keyCode===229&&(x=`onCompositionStart`);x&&(Zn&&n.locale!==`ko`&&(nr||x!==`onCompositionStart`?x===`onCompositionEnd`&&nr&&(b=_n()):(mn=i,hn=`value`in mn?mn.value:mn.textContent,nr=!0)),y=Ed(r,x),0<y.length&&(x=new Fn(x,e,null,n,i),o.push({event:x,listeners:y}),b?x.data=b:(b=tr(n),b!==null&&(x.data=b)))),(b=Xn?rr(e,n):ir(e,n))&&(x=Ed(r,`onBeforeInput`),0<x.length&&(y=new Fn(`onBeforeInput`,`beforeinput`,null,n,i),o.push({event:y,listeners:x}),y.data=b)),md(o,e,r,n,i)}yd(o,t)})}function Td(e,t,n){return{instance:e,listener:t,currentTarget:n}}function Ed(e,t){for(var n=t+`Capture`,r=[];e!==null;){var i=e,a=i.stateNode;if(i=i.tag,i!==5&&i!==26&&i!==27||a===null||(i=un(e,n),i!=null&&r.unshift(Td(e,i,a)),i=un(e,t),i!=null&&r.push(Td(e,i,a))),e.tag===3)return r;e=e.return}return[]}function Dd(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function Od(e,t,n,r,i){for(var a=t._reactName,o=[];n!==null&&n!==r;){var s=n,c=s.alternate,l=s.stateNode;if(s=s.tag,c!==null&&c===r)break;s!==5&&s!==26&&s!==27||l===null||(c=l,i?(l=un(n,a),l!=null&&o.unshift(Td(n,l,c))):i||(l=un(n,a),l!=null&&o.push(Td(n,l,c)))),n=n.return}o.length!==0&&e.push({event:t,listeners:o})}var kd=/\r\n?/g,Ad=/\u0000|\uFFFD/g;function jd(e){return(typeof e==`string`?e:``+e).replace(kd,`
`).replace(Ad,``)}function Md(e,t){return t=jd(t),jd(e)===t}function $(e,t,n,r,i,o){switch(n){case`children`:typeof r==`string`?t===`body`||t===`textarea`&&r===``||qt(e,r):(typeof r==`number`||typeof r==`bigint`)&&t!==`body`&&qt(e,``+r);break;case`className`:jt(e,`class`,r);break;case`tabIndex`:jt(e,`tabindex`,r);break;case`dir`:case`role`:case`viewBox`:case`width`:case`height`:jt(e,n,r);break;case`style`:Xt(e,r,o);break;case`data`:if(t!==`object`){jt(e,`data`,r);break}case`src`:case`href`:if(r===``&&(t!==`a`||n!==`href`)){e.removeAttribute(n);break}if(r==null||typeof r==`function`||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=en(``+r),e.setAttribute(n,r);break;case`action`:case`formAction`:if(typeof r==`function`){e.setAttribute(n,`javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')`);break}else typeof o==`function`&&(n===`formAction`?(t!==`input`&&$(e,t,`name`,i.name,i,null),$(e,t,`formEncType`,i.formEncType,i,null),$(e,t,`formMethod`,i.formMethod,i,null),$(e,t,`formTarget`,i.formTarget,i,null)):($(e,t,`encType`,i.encType,i,null),$(e,t,`method`,i.method,i,null),$(e,t,`target`,i.target,i,null)));if(r==null||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=en(``+r),e.setAttribute(n,r);break;case`onClick`:r!=null&&(e.onclick=tn);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`multiple`:e.multiple=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`muted`:e.muted=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`defaultValue`:case`defaultChecked`:case`innerHTML`:case`ref`:break;case`autoFocus`:break;case`xlinkHref`:if(r==null||typeof r==`function`||typeof r==`boolean`||typeof r==`symbol`){e.removeAttribute(`xlink:href`);break}n=en(``+r),e.setAttributeNS(`http://www.w3.org/1999/xlink`,`xlink:href`,n);break;case`contentEditable`:case`spellCheck`:case`draggable`:case`value`:case`autoReverse`:case`externalResourcesRequired`:case`focusable`:case`preserveAlpha`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``+r):e.removeAttribute(n);break;case`inert`:case`allowFullScreen`:case`async`:case`autoPlay`:case`controls`:case`default`:case`defer`:case`disabled`:case`disablePictureInPicture`:case`disableRemotePlayback`:case`formNoValidate`:case`hidden`:case`loop`:case`noModule`:case`noValidate`:case`open`:case`playsInline`:case`readOnly`:case`required`:case`reversed`:case`scoped`:case`seamless`:case`itemScope`:r&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``):e.removeAttribute(n);break;case`capture`:case`download`:!0===r?e.setAttribute(n,``):!1!==r&&r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,r):e.removeAttribute(n);break;case`cols`:case`rows`:case`size`:case`span`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`&&!isNaN(r)&&1<=r?e.setAttribute(n,r):e.removeAttribute(n);break;case`rowSpan`:case`start`:r==null||typeof r==`function`||typeof r==`symbol`||isNaN(r)?e.removeAttribute(n):e.setAttribute(n,r);break;case`popover`:Q(`beforetoggle`,e),Q(`toggle`,e),At(e,`popover`,r);break;case`xlinkActuate`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:actuate`,r);break;case`xlinkArcrole`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:arcrole`,r);break;case`xlinkRole`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:role`,r);break;case`xlinkShow`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:show`,r);break;case`xlinkTitle`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:title`,r);break;case`xlinkType`:Mt(e,`http://www.w3.org/1999/xlink`,`xlink:type`,r);break;case`xmlBase`:Mt(e,`http://www.w3.org/XML/1998/namespace`,`xml:base`,r);break;case`xmlLang`:Mt(e,`http://www.w3.org/XML/1998/namespace`,`xml:lang`,r);break;case`xmlSpace`:Mt(e,`http://www.w3.org/XML/1998/namespace`,`xml:space`,r);break;case`is`:At(e,`is`,r);break;case`innerText`:case`textContent`:break;default:(!(2<n.length)||n[0]!==`o`&&n[0]!==`O`||n[1]!==`n`&&n[1]!==`N`)&&(n=Qt.get(n)||n,At(e,n,r))}}function Nd(e,t,n,r,i,o){switch(n){case`style`:Xt(e,r,o);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`children`:typeof r==`string`?qt(e,r):(typeof r==`number`||typeof r==`bigint`)&&qt(e,``+r);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`onClick`:r!=null&&(e.onclick=tn);break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`innerHTML`:case`ref`:break;case`innerText`:case`textContent`:break;default:if(!Ct.hasOwnProperty(n))a:{if(n[0]===`o`&&n[1]===`n`&&(i=n.endsWith(`Capture`),t=n.slice(2,i?n.length-7:void 0),o=e[I]||null,o=o==null?null:o[n],typeof o==`function`&&e.removeEventListener(t,o,i),typeof r==`function`)){typeof o!=`function`&&o!==null&&(n in e?e[n]=null:e.hasAttribute(n)&&e.removeAttribute(n)),e.addEventListener(t,r,i);break a}n in e?e[n]=r:!0===r?e.setAttribute(n,``):At(e,n,r)}}}function Pd(e,t,n){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`img`:Q(`error`,e),Q(`load`,e);var r=!1,i=!1,o;for(o in n)if(n.hasOwnProperty(o)){var s=n[o];if(s!=null)switch(o){case`src`:r=!0;break;case`srcSet`:i=!0;break;case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:$(e,t,o,s,n,null)}}i&&$(e,t,`srcSet`,n.srcSet,n,null),r&&$(e,t,`src`,n.src,n,null);return;case`input`:Q(`invalid`,e);var c=o=s=i=null,l=null,u=null;for(r in n)if(n.hasOwnProperty(r)){var d=n[r];if(d!=null)switch(r){case`name`:i=d;break;case`type`:s=d;break;case`checked`:l=d;break;case`defaultChecked`:u=d;break;case`value`:o=d;break;case`defaultValue`:c=d;break;case`children`:case`dangerouslySetInnerHTML`:if(d!=null)throw Error(a(137,t));break;default:$(e,t,r,d,n,null)}}Ht(e,o,c,l,u,s,i,!1);return;case`select`:for(i in Q(`invalid`,e),r=s=o=null,n)if(n.hasOwnProperty(i)&&(c=n[i],c!=null))switch(i){case`value`:o=c;break;case`defaultValue`:s=c;break;case`multiple`:r=c;default:$(e,t,i,c,n,null)}t=o,n=s,e.multiple=!!r,t==null?n!=null&&Wt(e,!!r,n,!0):Wt(e,!!r,t,!1);return;case`textarea`:for(s in Q(`invalid`,e),o=i=r=null,n)if(n.hasOwnProperty(s)&&(c=n[s],c!=null))switch(s){case`value`:r=c;break;case`defaultValue`:i=c;break;case`children`:o=c;break;case`dangerouslySetInnerHTML`:if(c!=null)throw Error(a(91));break;default:$(e,t,s,c,n,null)}Kt(e,r,i,o);return;case`option`:for(l in n)if(n.hasOwnProperty(l)&&(r=n[l],r!=null))switch(l){case`selected`:e.selected=r&&typeof r!=`function`&&typeof r!=`symbol`;break;default:$(e,t,l,r,n,null)}return;case`dialog`:Q(`beforetoggle`,e),Q(`toggle`,e),Q(`cancel`,e),Q(`close`,e);break;case`iframe`:case`object`:Q(`load`,e);break;case`video`:case`audio`:for(r=0;r<_d.length;r++)Q(_d[r],e);break;case`image`:Q(`error`,e),Q(`load`,e);break;case`details`:Q(`toggle`,e);break;case`embed`:case`source`:case`link`:Q(`error`,e),Q(`load`,e);case`area`:case`base`:case`br`:case`col`:case`hr`:case`keygen`:case`meta`:case`param`:case`track`:case`wbr`:case`menuitem`:for(u in n)if(n.hasOwnProperty(u)&&(r=n[u],r!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:$(e,t,u,r,n,null)}return;default:if(Zt(t)){for(d in n)n.hasOwnProperty(d)&&(r=n[d],r!==void 0&&Nd(e,t,d,r,n,void 0));return}}for(c in n)n.hasOwnProperty(c)&&(r=n[c],r!=null&&$(e,t,c,r,n,null))}function Fd(e,t,n,r){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`input`:var i=null,o=null,s=null,c=null,l=null,u=null,d=null;for(m in n){var f=n[m];if(n.hasOwnProperty(m)&&f!=null)switch(m){case`checked`:break;case`value`:break;case`defaultValue`:l=f;default:r.hasOwnProperty(m)||$(e,t,m,null,r,f)}}for(var p in r){var m=r[p];if(f=n[p],r.hasOwnProperty(p)&&(m!=null||f!=null))switch(p){case`type`:o=m;break;case`name`:i=m;break;case`checked`:u=m;break;case`defaultChecked`:d=m;break;case`value`:s=m;break;case`defaultValue`:c=m;break;case`children`:case`dangerouslySetInnerHTML`:if(m!=null)throw Error(a(137,t));break;default:m!==f&&$(e,t,p,m,r,f)}}Vt(e,s,c,l,u,d,o,i);return;case`select`:for(o in m=s=c=p=null,n)if(l=n[o],n.hasOwnProperty(o)&&l!=null)switch(o){case`value`:break;case`multiple`:m=l;default:r.hasOwnProperty(o)||$(e,t,o,null,r,l)}for(i in r)if(o=r[i],l=n[i],r.hasOwnProperty(i)&&(o!=null||l!=null))switch(i){case`value`:p=o;break;case`defaultValue`:c=o;break;case`multiple`:s=o;default:o!==l&&$(e,t,i,o,r,l)}t=c,n=s,r=m,p==null?!!r!=!!n&&(t==null?Wt(e,!!n,n?[]:``,!1):Wt(e,!!n,t,!0)):Wt(e,!!n,p,!1);return;case`textarea`:for(c in m=p=null,n)if(i=n[c],n.hasOwnProperty(c)&&i!=null&&!r.hasOwnProperty(c))switch(c){case`value`:break;case`children`:break;default:$(e,t,c,null,r,i)}for(s in r)if(i=r[s],o=n[s],r.hasOwnProperty(s)&&(i!=null||o!=null))switch(s){case`value`:p=i;break;case`defaultValue`:m=i;break;case`children`:break;case`dangerouslySetInnerHTML`:if(i!=null)throw Error(a(91));break;default:i!==o&&$(e,t,s,i,r,o)}Gt(e,p,m);return;case`option`:for(var h in n)if(p=n[h],n.hasOwnProperty(h)&&p!=null&&!r.hasOwnProperty(h))switch(h){case`selected`:e.selected=!1;break;default:$(e,t,h,null,r,p)}for(l in r)if(p=r[l],m=n[l],r.hasOwnProperty(l)&&p!==m&&(p!=null||m!=null))switch(l){case`selected`:e.selected=p&&typeof p!=`function`&&typeof p!=`symbol`;break;default:$(e,t,l,p,r,m)}return;case`img`:case`link`:case`area`:case`base`:case`br`:case`col`:case`embed`:case`hr`:case`keygen`:case`meta`:case`param`:case`source`:case`track`:case`wbr`:case`menuitem`:for(var g in n)p=n[g],n.hasOwnProperty(g)&&p!=null&&!r.hasOwnProperty(g)&&$(e,t,g,null,r,p);for(u in r)if(p=r[u],m=n[u],r.hasOwnProperty(u)&&p!==m&&(p!=null||m!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:if(p!=null)throw Error(a(137,t));break;default:$(e,t,u,p,r,m)}return;default:if(Zt(t)){for(var _ in n)p=n[_],n.hasOwnProperty(_)&&p!==void 0&&!r.hasOwnProperty(_)&&Nd(e,t,_,void 0,r,p);for(d in r)p=r[d],m=n[d],!r.hasOwnProperty(d)||p===m||p===void 0&&m===void 0||Nd(e,t,d,p,r,m);return}}for(var v in n)p=n[v],n.hasOwnProperty(v)&&p!=null&&!r.hasOwnProperty(v)&&$(e,t,v,null,r,p);for(f in r)p=r[f],m=n[f],!r.hasOwnProperty(f)||p===m||p==null&&m==null||$(e,t,f,p,r,m)}function Id(e){switch(e){case`css`:case`script`:case`font`:case`img`:case`image`:case`input`:case`link`:return!0;default:return!1}}function Ld(){if(typeof performance.getEntriesByType==`function`){for(var e=0,t=0,n=performance.getEntriesByType(`resource`),r=0;r<n.length;r++){var i=n[r],a=i.transferSize,o=i.initiatorType,s=i.duration;if(a&&s&&Id(o)){for(o=0,s=i.responseEnd,r+=1;r<n.length;r++){var c=n[r],l=c.startTime;if(l>s)break;var u=c.transferSize,d=c.initiatorType;u&&Id(d)&&(c=c.responseEnd,o+=u*(c<s?1:(s-l)/(c-l)))}if(--r,t+=8*(a+o)/(i.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e==`number`)?e:5}var Rd=null,zd=null;function Bd(e){return e.nodeType===9?e:e.ownerDocument}function Vd(e){switch(e){case`http://www.w3.org/2000/svg`:return 1;case`http://www.w3.org/1998/Math/MathML`:return 2;default:return 0}}function Hd(e,t){if(e===0)switch(t){case`svg`:return 1;case`math`:return 2;default:return 0}return e===1&&t===`foreignObject`?0:e}function Ud(e,t){return e===`textarea`||e===`noscript`||typeof t.children==`string`||typeof t.children==`number`||typeof t.children==`bigint`||typeof t.dangerouslySetInnerHTML==`object`&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var Wd=null;function Gd(){var e=window.event;return e&&e.type===`popstate`?e===Wd?!1:(Wd=e,!0):(Wd=null,!1)}var Kd=typeof setTimeout==`function`?setTimeout:void 0,qd=typeof clearTimeout==`function`?clearTimeout:void 0,Jd=typeof Promise==`function`?Promise:void 0,Yd=typeof queueMicrotask==`function`?queueMicrotask:Jd===void 0?Kd:function(e){return Jd.resolve(null).then(e).catch(Xd)};function Xd(e){setTimeout(function(){throw e})}function Zd(e){return e===`head`}function Qd(e,t){var n=t,r=0;do{var i=n.nextSibling;if(e.removeChild(n),i&&i.nodeType===8)if(n=i.data,n===`/$`||n===`/&`){if(r===0){e.removeChild(i),Np(t);return}r--}else if(n===`$`||n===`$?`||n===`$~`||n===`$!`||n===`&`)r++;else if(n===`html`)pf(e.ownerDocument.documentElement);else if(n===`head`){n=e.ownerDocument.head,pf(n);for(var a=n.firstChild;a;){var o=a.nextSibling,s=a.nodeName;a[ht]||s===`SCRIPT`||s===`STYLE`||s===`LINK`&&a.rel.toLowerCase()===`stylesheet`||n.removeChild(a),a=o}}else n===`body`&&pf(e.ownerDocument.body);n=i}while(n);Np(t)}function $d(e,t){var n=e;e=0;do{var r=n.nextSibling;if(n.nodeType===1?t?(n._stashedDisplay=n.style.display,n.style.display=`none`):(n.style.display=n._stashedDisplay||``,n.getAttribute(`style`)===``&&n.removeAttribute(`style`)):n.nodeType===3&&(t?(n._stashedText=n.nodeValue,n.nodeValue=``):n.nodeValue=n._stashedText||``),r&&r.nodeType===8)if(n=r.data,n===`/$`){if(e===0)break;e--}else n!==`$`&&n!==`$?`&&n!==`$~`&&n!==`$!`||e++;n=r}while(n)}function ef(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var n=t;switch(t=t.nextSibling,n.nodeName){case`HTML`:case`HEAD`:case`BODY`:ef(n),gt(n);continue;case`SCRIPT`:case`STYLE`:continue;case`LINK`:if(n.rel.toLowerCase()===`stylesheet`)continue}e.removeChild(n)}}function tf(e,t,n,r){for(;e.nodeType===1;){var i=n;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!r&&(e.nodeName!==`INPUT`||e.type!==`hidden`))break}else if(!r)if(t===`input`&&e.type===`hidden`){var a=i.name==null?null:``+i.name;if(i.type===`hidden`&&e.getAttribute(`name`)===a)return e}else return e;else if(!e[ht])switch(t){case`meta`:if(!e.hasAttribute(`itemprop`))break;return e;case`link`:if(a=e.getAttribute(`rel`),a===`stylesheet`&&e.hasAttribute(`data-precedence`)||a!==i.rel||e.getAttribute(`href`)!==(i.href==null||i.href===``?null:i.href)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin)||e.getAttribute(`title`)!==(i.title==null?null:i.title))break;return e;case`style`:if(e.hasAttribute(`data-precedence`))break;return e;case`script`:if(a=e.getAttribute(`src`),(a!==(i.src==null?null:i.src)||e.getAttribute(`type`)!==(i.type==null?null:i.type)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin))&&a&&e.hasAttribute(`async`)&&!e.hasAttribute(`itemprop`))break;return e;default:return e}if(e=cf(e.nextSibling),e===null)break}return null}function nf(e,t,n){if(t===``)return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!n||(e=cf(e.nextSibling),e===null))return null;return e}function rf(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!t||(e=cf(e.nextSibling),e===null))return null;return e}function af(e){return e.data===`$?`||e.data===`$~`}function of(e){return e.data===`$!`||e.data===`$?`&&e.ownerDocument.readyState!==`loading`}function sf(e,t){var n=e.ownerDocument;if(e.data===`$~`)e._reactRetry=t;else if(e.data!==`$?`||n.readyState!==`loading`)t();else{var r=function(){t(),n.removeEventListener(`DOMContentLoaded`,r)};n.addEventListener(`DOMContentLoaded`,r),e._reactRetry=r}}function cf(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t===`$`||t===`$!`||t===`$?`||t===`$~`||t===`&`||t===`F!`||t===`F`)break;if(t===`/$`||t===`/&`)return null}}return e}var lf=null;function uf(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`/$`||n===`/&`){if(t===0)return cf(e.nextSibling);t--}else n!==`$`&&n!==`$!`&&n!==`$?`&&n!==`$~`&&n!==`&`||t++}e=e.nextSibling}return null}function df(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`$`||n===`$!`||n===`$?`||n===`$~`||n===`&`){if(t===0)return e;t--}else n!==`/$`&&n!==`/&`||t++}e=e.previousSibling}return null}function ff(e,t,n){switch(t=Bd(n),e){case`html`:if(e=t.documentElement,!e)throw Error(a(452));return e;case`head`:if(e=t.head,!e)throw Error(a(453));return e;case`body`:if(e=t.body,!e)throw Error(a(454));return e;default:throw Error(a(451))}}function pf(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);gt(e)}var mf=new Map,hf=new Set;function gf(e){return typeof e.getRootNode==`function`?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var _f=k.d;k.d={f:vf,r:yf,D:Sf,C:Cf,L:wf,m:Tf,X:Df,S:Ef,M:Of};function vf(){var e=_f.f(),t=bu();return e||t}function yf(e){var t=vt(e);t!==null&&t.tag===5&&t.type===`form`?Ts(t):_f.r(e)}var bf=typeof document>`u`?null:document;function xf(e,t,n){var r=bf;if(r&&typeof t==`string`&&t){var i=Bt(t);i=`link[rel="`+e+`"][href="`+i+`"]`,typeof n==`string`&&(i+=`[crossorigin="`+n+`"]`),hf.has(i)||(hf.add(i),e={rel:e,crossOrigin:n,href:t},r.querySelector(i)===null&&(t=r.createElement(`link`),Pd(t,`link`,e),xt(t),r.head.appendChild(t)))}}function Sf(e){_f.D(e),xf(`dns-prefetch`,e,null)}function Cf(e,t){_f.C(e,t),xf(`preconnect`,e,t)}function wf(e,t,n){_f.L(e,t,n);var r=bf;if(r&&e&&t){var i=`link[rel="preload"][as="`+Bt(t)+`"]`;t===`image`&&n&&n.imageSrcSet?(i+=`[imagesrcset="`+Bt(n.imageSrcSet)+`"]`,typeof n.imageSizes==`string`&&(i+=`[imagesizes="`+Bt(n.imageSizes)+`"]`)):i+=`[href="`+Bt(e)+`"]`;var a=i;switch(t){case`style`:a=Af(e);break;case`script`:a=Pf(e)}mf.has(a)||(e=m({rel:`preload`,href:t===`image`&&n&&n.imageSrcSet?void 0:e,as:t},n),mf.set(a,e),r.querySelector(i)!==null||t===`style`&&r.querySelector(jf(a))||t===`script`&&r.querySelector(Ff(a))||(t=r.createElement(`link`),Pd(t,`link`,e),xt(t),r.head.appendChild(t)))}}function Tf(e,t){_f.m(e,t);var n=bf;if(n&&e){var r=t&&typeof t.as==`string`?t.as:`script`,i=`link[rel="modulepreload"][as="`+Bt(r)+`"][href="`+Bt(e)+`"]`,a=i;switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:a=Pf(e)}if(!mf.has(a)&&(e=m({rel:`modulepreload`,href:e},t),mf.set(a,e),n.querySelector(i)===null)){switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:if(n.querySelector(Ff(a)))return}r=n.createElement(`link`),Pd(r,`link`,e),xt(r),n.head.appendChild(r)}}}function Ef(e,t,n){_f.S(e,t,n);var r=bf;if(r&&e){var i=bt(r).hoistableStyles,a=Af(e);t||=`default`;var o=i.get(a);if(!o){var s={loading:0,preload:null};if(o=r.querySelector(jf(a)))s.loading=5;else{e=m({rel:`stylesheet`,href:e,"data-precedence":t},n),(n=mf.get(a))&&Rf(e,n);var c=o=r.createElement(`link`);xt(c),Pd(c,`link`,e),c._p=new Promise(function(e,t){c.onload=e,c.onerror=t}),c.addEventListener(`load`,function(){s.loading|=1}),c.addEventListener(`error`,function(){s.loading|=2}),s.loading|=4,Lf(o,t,r)}o={type:`stylesheet`,instance:o,count:1,state:s},i.set(a,o)}}}function Df(e,t){_f.X(e,t);var n=bf;if(n&&e){var r=bt(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),xt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function Of(e,t){_f.M(e,t);var n=bf;if(n&&e){var r=bt(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0,type:`module`},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),xt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function kf(e,t,n,r){var i=(i=de.current)?gf(i):null;if(!i)throw Error(a(446));switch(e){case`meta`:case`title`:return null;case`style`:return typeof n.precedence==`string`&&typeof n.href==`string`?(t=Af(n.href),n=bt(i).hoistableStyles,r=n.get(t),r||(r={type:`style`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};case`link`:if(n.rel===`stylesheet`&&typeof n.href==`string`&&typeof n.precedence==`string`){e=Af(n.href);var o=bt(i).hoistableStyles,s=o.get(e);if(s||(i=i.ownerDocument||i,s={type:`stylesheet`,instance:null,count:0,state:{loading:0,preload:null}},o.set(e,s),(o=i.querySelector(jf(e)))&&!o._p&&(s.instance=o,s.state.loading=5),mf.has(e)||(n={rel:`preload`,as:`style`,href:n.href,crossOrigin:n.crossOrigin,integrity:n.integrity,media:n.media,hrefLang:n.hrefLang,referrerPolicy:n.referrerPolicy},mf.set(e,n),o||Nf(i,e,n,s.state))),t&&r===null)throw Error(a(528,``));return s}if(t&&r!==null)throw Error(a(529,``));return null;case`script`:return t=n.async,n=n.src,typeof n==`string`&&t&&typeof t!=`function`&&typeof t!=`symbol`?(t=Pf(n),n=bt(i).hoistableScripts,r=n.get(t),r||(r={type:`script`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};default:throw Error(a(444,e))}}function Af(e){return`href="`+Bt(e)+`"`}function jf(e){return`link[rel="stylesheet"][`+e+`]`}function Mf(e){return m({},e,{"data-precedence":e.precedence,precedence:null})}function Nf(e,t,n,r){e.querySelector(`link[rel="preload"][as="style"][`+t+`]`)?r.loading=1:(t=e.createElement(`link`),r.preload=t,t.addEventListener(`load`,function(){return r.loading|=1}),t.addEventListener(`error`,function(){return r.loading|=2}),Pd(t,`link`,n),xt(t),e.head.appendChild(t))}function Pf(e){return`[src="`+Bt(e)+`"]`}function Ff(e){return`script[async]`+e}function If(e,t,n){if(t.count++,t.instance===null)switch(t.type){case`style`:var r=e.querySelector(`style[data-href~="`+Bt(n.href)+`"]`);if(r)return t.instance=r,xt(r),r;var i=m({},n,{"data-href":n.href,"data-precedence":n.precedence,href:null,precedence:null});return r=(e.ownerDocument||e).createElement(`style`),xt(r),Pd(r,`style`,i),Lf(r,n.precedence,e),t.instance=r;case`stylesheet`:i=Af(n.href);var o=e.querySelector(jf(i));if(o)return t.state.loading|=4,t.instance=o,xt(o),o;r=Mf(n),(i=mf.get(i))&&Rf(r,i),o=(e.ownerDocument||e).createElement(`link`),xt(o);var s=o;return s._p=new Promise(function(e,t){s.onload=e,s.onerror=t}),Pd(o,`link`,r),t.state.loading|=4,Lf(o,n.precedence,e),t.instance=o;case`script`:return o=Pf(n.src),(i=e.querySelector(Ff(o)))?(t.instance=i,xt(i),i):(r=n,(i=mf.get(o))&&(r=m({},n),zf(r,i)),e=e.ownerDocument||e,i=e.createElement(`script`),xt(i),Pd(i,`link`,r),e.head.appendChild(i),t.instance=i);case`void`:return null;default:throw Error(a(443,t.type))}else t.type===`stylesheet`&&!(t.state.loading&4)&&(r=t.instance,t.state.loading|=4,Lf(r,n.precedence,e));return t.instance}function Lf(e,t,n){for(var r=n.querySelectorAll(`link[rel="stylesheet"][data-precedence],style[data-precedence]`),i=r.length?r[r.length-1]:null,a=i,o=0;o<r.length;o++){var s=r[o];if(s.dataset.precedence===t)a=s;else if(a!==i)break}a?a.parentNode.insertBefore(e,a.nextSibling):(t=n.nodeType===9?n.head:n,t.insertBefore(e,t.firstChild))}function Rf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.title??=t.title}function zf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.integrity??=t.integrity}var Bf=null;function Vf(e,t,n){if(Bf===null){var r=new Map,i=Bf=new Map;i.set(n,r)}else i=Bf,r=i.get(n),r||(r=new Map,i.set(n,r));if(r.has(e))return r;for(r.set(e,null),n=n.getElementsByTagName(e),i=0;i<n.length;i++){var a=n[i];if(!(a[ht]||a[lt]||e===`link`&&a.getAttribute(`rel`)===`stylesheet`)&&a.namespaceURI!==`http://www.w3.org/2000/svg`){var o=a.getAttribute(t)||``;o=e+o;var s=r.get(o);s?s.push(a):r.set(o,[a])}}return r}function Hf(e,t,n){e=e.ownerDocument||e,e.head.insertBefore(n,t===`title`?e.querySelector(`head > title`):null)}function Uf(e,t,n){if(n===1||t.itemProp!=null)return!1;switch(e){case`meta`:case`title`:return!0;case`style`:if(typeof t.precedence!=`string`||typeof t.href!=`string`||t.href===``)break;return!0;case`link`:if(typeof t.rel!=`string`||typeof t.href!=`string`||t.href===``||t.onLoad||t.onError)break;switch(t.rel){case`stylesheet`:return e=t.disabled,typeof t.precedence==`string`&&e==null;default:return!0}case`script`:if(t.async&&typeof t.async!=`function`&&typeof t.async!=`symbol`&&!t.onLoad&&!t.onError&&t.src&&typeof t.src==`string`)return!0}return!1}function Wf(e){return!(e.type===`stylesheet`&&!(e.state.loading&3))}function Gf(e,t,n,r){if(n.type===`stylesheet`&&(typeof r.media!=`string`||!1!==matchMedia(r.media).matches)&&!(n.state.loading&4)){if(n.instance===null){var i=Af(r.href),a=t.querySelector(jf(i));if(a){t=a._p,typeof t==`object`&&t&&typeof t.then==`function`&&(e.count++,e=Jf.bind(e),t.then(e,e)),n.state.loading|=4,n.instance=a,xt(a);return}a=t.ownerDocument||t,r=Mf(r),(i=mf.get(i))&&Rf(r,i),a=a.createElement(`link`),xt(a);var o=a;o._p=new Promise(function(e,t){o.onload=e,o.onerror=t}),Pd(a,`link`,r),n.instance=a}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(n,t),(t=n.state.preload)&&!(n.state.loading&3)&&(e.count++,n=Jf.bind(e),t.addEventListener(`load`,n),t.addEventListener(`error`,n))}}var Kf=0;function qf(e,t){return e.stylesheets&&e.count===0&&Xf(e,e.stylesheets),0<e.count||0<e.imgCount?function(n){var r=setTimeout(function(){if(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend){var t=e.unsuspend;e.unsuspend=null,t()}},6e4+t);0<e.imgBytes&&Kf===0&&(Kf=62500*Ld());var i=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend)){var t=e.unsuspend;e.unsuspend=null,t()}},(e.imgBytes>Kf?50:800)+t);return e.unsuspend=n,function(){e.unsuspend=null,clearTimeout(r),clearTimeout(i)}}:null}function Jf(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)Xf(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var Yf=null;function Xf(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,Yf=new Map,t.forEach(Zf,e),Yf=null,Jf.call(e))}function Zf(e,t){if(!(t.state.loading&4)){var n=Yf.get(e);if(n)var r=n.get(null);else{n=new Map,Yf.set(e,n);for(var i=e.querySelectorAll(`link[data-precedence],style[data-precedence]`),a=0;a<i.length;a++){var o=i[a];(o.nodeName===`LINK`||o.getAttribute(`media`)!==`not all`)&&(n.set(o.dataset.precedence,o),r=o)}r&&n.set(null,r)}i=t.instance,o=i.getAttribute(`data-precedence`),a=n.get(o)||r,a===r&&n.set(null,i),n.set(o,i),this.count++,r=Jf.bind(this),i.addEventListener(`load`,r),i.addEventListener(`error`,r),a?a.parentNode.insertBefore(i,a.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(i,e.firstChild)),t.state.loading|=4}}var Qf={$$typeof:S,Provider:null,Consumer:null,_currentValue:ce,_currentValue2:ce,_threadCount:0};function $f(e,t,n,r,i,a,o,s,c){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=Qe(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=Qe(0),this.hiddenUpdates=Qe(null),this.identifierPrefix=r,this.onUncaughtError=i,this.onCaughtError=a,this.onRecoverableError=o,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=c,this.incompleteTransitions=new Map}function ep(e,t,n,r,i,a,o,s,c,l,u,d){return e=new $f(e,t,n,o,c,l,u,d,s),t=1,!0===a&&(t|=24),a=li(3,null,null,t),e.current=a,a.stateNode=e,t=la(),t.refCount++,e.pooledCache=t,t.refCount++,a.memoizedState={element:r,isDehydrated:n,cache:t},Ha(a),e}function tp(e){return e?(e=si,e):si}function np(e,t,n,r,i,a){i=tp(i),r.context===null?r.context=i:r.pendingContext=i,r=Wa(t),r.payload={element:n},a=a===void 0?null:a,a!==null&&(r.callback=a),n=Ga(e,r,t),n!==null&&(hu(n,e,t),Ka(n,e,t))}function rp(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var n=e.retryLane;e.retryLane=n!==0&&n<t?n:t}}function ip(e,t){rp(e,t),(e=e.alternate)&&rp(e,t)}function ap(e){if(e.tag===13||e.tag===31){var t=ii(e,67108864);t!==null&&hu(t,e,67108864),ip(e,67108864)}}function op(e){if(e.tag===13||e.tag===31){var t=pu();t=it(t);var n=ii(e,t);n!==null&&hu(n,e,t),ip(e,t)}}var sp=!0;function cp(e,t,n,r){var i=O.T;O.T=null;var a=k.p;try{k.p=2,up(e,t,n,r)}finally{k.p=a,O.T=i}}function lp(e,t,n,r){var i=O.T;O.T=null;var a=k.p;try{k.p=8,up(e,t,n,r)}finally{k.p=a,O.T=i}}function up(e,t,n,r){if(sp){var i=dp(r);if(i===null)wd(e,t,r,fp,n),Cp(e,r);else if(Tp(i,e,t,n,r))r.stopPropagation();else if(Cp(e,r),t&4&&-1<Sp.indexOf(e)){for(;i!==null;){var a=vt(i);if(a!==null)switch(a.tag){case 3:if(a=a.stateNode,a.current.memoizedState.isDehydrated){var o=qe(a.pendingLanes);if(o!==0){var s=a;for(s.pendingLanes|=2,s.entangledLanes|=2;o;){var c=1<<31-Be(o);s.entanglements[1]|=c,o&=~c}rd(a),!(K&6)&&(tu=Oe()+500,id(0,!1))}}break;case 31:case 13:s=ii(a,2),s!==null&&hu(s,a,2),bu(),ip(a,2)}if(a=dp(r),a===null&&wd(e,t,r,fp,n),a===i)break;i=a}i!==null&&r.stopPropagation()}else wd(e,t,r,null,n)}}function dp(e){return e=rn(e),pp(e)}var fp=null;function pp(e){if(fp=null,e=_t(e),e!==null){var t=s(e);if(t===null)e=null;else{var n=t.tag;if(n===13){if(e=c(t),e!==null)return e;e=null}else if(n===31){if(e=l(t),e!==null)return e;e=null}else if(n===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return fp=e,null}function mp(e){switch(e){case`beforetoggle`:case`cancel`:case`click`:case`close`:case`contextmenu`:case`copy`:case`cut`:case`auxclick`:case`dblclick`:case`dragend`:case`dragstart`:case`drop`:case`focusin`:case`focusout`:case`input`:case`invalid`:case`keydown`:case`keypress`:case`keyup`:case`mousedown`:case`mouseup`:case`paste`:case`pause`:case`play`:case`pointercancel`:case`pointerdown`:case`pointerup`:case`ratechange`:case`reset`:case`resize`:case`seeked`:case`submit`:case`toggle`:case`touchcancel`:case`touchend`:case`touchstart`:case`volumechange`:case`change`:case`selectionchange`:case`textInput`:case`compositionstart`:case`compositionend`:case`compositionupdate`:case`beforeblur`:case`afterblur`:case`beforeinput`:case`blur`:case`fullscreenchange`:case`focus`:case`hashchange`:case`popstate`:case`select`:case`selectstart`:return 2;case`drag`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`mousemove`:case`mouseout`:case`mouseover`:case`pointermove`:case`pointerout`:case`pointerover`:case`scroll`:case`touchmove`:case`wheel`:case`mouseenter`:case`mouseleave`:case`pointerenter`:case`pointerleave`:return 8;case`message`:switch(ke()){case Ae:return 2;case je:return 8;case Me:case Ne:return 32;case Pe:return 268435456;default:return 32}default:return 32}}var hp=!1,gp=null,_p=null,vp=null,yp=new Map,bp=new Map,xp=[],Sp=`mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset`.split(` `);function Cp(e,t){switch(e){case`focusin`:case`focusout`:gp=null;break;case`dragenter`:case`dragleave`:_p=null;break;case`mouseover`:case`mouseout`:vp=null;break;case`pointerover`:case`pointerout`:yp.delete(t.pointerId);break;case`gotpointercapture`:case`lostpointercapture`:bp.delete(t.pointerId)}}function wp(e,t,n,r,i,a){return e===null||e.nativeEvent!==a?(e={blockedOn:t,domEventName:n,eventSystemFlags:r,nativeEvent:a,targetContainers:[i]},t!==null&&(t=vt(t),t!==null&&ap(t)),e):(e.eventSystemFlags|=r,t=e.targetContainers,i!==null&&t.indexOf(i)===-1&&t.push(i),e)}function Tp(e,t,n,r,i){switch(t){case`focusin`:return gp=wp(gp,e,t,n,r,i),!0;case`dragenter`:return _p=wp(_p,e,t,n,r,i),!0;case`mouseover`:return vp=wp(vp,e,t,n,r,i),!0;case`pointerover`:var a=i.pointerId;return yp.set(a,wp(yp.get(a)||null,e,t,n,r,i)),!0;case`gotpointercapture`:return a=i.pointerId,bp.set(a,wp(bp.get(a)||null,e,t,n,r,i)),!0}return!1}function Ep(e){var t=_t(e.target);if(t!==null){var n=s(t);if(n!==null){if(t=n.tag,t===13){if(t=c(n),t!==null){e.blockedOn=t,st(e.priority,function(){op(n)});return}}else if(t===31){if(t=l(n),t!==null){e.blockedOn=t,st(e.priority,function(){op(n)});return}}else if(t===3&&n.stateNode.current.memoizedState.isDehydrated){e.blockedOn=n.tag===3?n.stateNode.containerInfo:null;return}}}e.blockedOn=null}function Dp(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var n=dp(e.nativeEvent);if(n===null){n=e.nativeEvent;var r=new n.constructor(n.type,n);nn=r,n.target.dispatchEvent(r),nn=null}else return t=vt(n),t!==null&&ap(t),e.blockedOn=n,!1;t.shift()}return!0}function Op(e,t,n){Dp(e)&&n.delete(t)}function kp(){hp=!1,gp!==null&&Dp(gp)&&(gp=null),_p!==null&&Dp(_p)&&(_p=null),vp!==null&&Dp(vp)&&(vp=null),yp.forEach(Op),bp.forEach(Op)}function Ap(e,n){e.blockedOn===n&&(e.blockedOn=null,hp||(hp=!0,t.unstable_scheduleCallback(t.unstable_NormalPriority,kp)))}var jp=null;function Mp(e){jp!==e&&(jp=e,t.unstable_scheduleCallback(t.unstable_NormalPriority,function(){jp===e&&(jp=null);for(var t=0;t<e.length;t+=3){var n=e[t],r=e[t+1],i=e[t+2];if(typeof r!=`function`){if(pp(r||n)===null)continue;break}var a=vt(n);a!==null&&(e.splice(t,3),t-=3,Cs(a,{pending:!0,data:i,method:n.method,action:r},r,i))}}))}function Np(e){function t(t){return Ap(t,e)}gp!==null&&Ap(gp,e),_p!==null&&Ap(_p,e),vp!==null&&Ap(vp,e),yp.forEach(t),bp.forEach(t);for(var n=0;n<xp.length;n++){var r=xp[n];r.blockedOn===e&&(r.blockedOn=null)}for(;0<xp.length&&(n=xp[0],n.blockedOn===null);)Ep(n),n.blockedOn===null&&xp.shift();if(n=(e.ownerDocument||e).$$reactFormReplay,n!=null)for(r=0;r<n.length;r+=3){var i=n[r],a=n[r+1],o=i[I]||null;if(typeof a==`function`)o||Mp(n);else if(o){var s=null;if(a&&a.hasAttribute(`formAction`)){if(i=a,o=a[I]||null)s=o.formAction;else if(pp(i)!==null)continue}else s=o.action;typeof s==`function`?n[r+1]=s:(n.splice(r,3),r-=3),Mp(n)}}}function Pp(){function e(e){e.canIntercept&&e.info===`react-transition`&&e.intercept({handler:function(){return new Promise(function(e){return i=e})},focusReset:`manual`,scroll:`manual`})}function t(){i!==null&&(i(),i=null),r||setTimeout(n,20)}function n(){if(!r&&!navigation.transition){var e=navigation.currentEntry;e&&e.url!=null&&navigation.navigate(e.url,{state:e.getState(),info:`react-transition`,history:`replace`})}}if(typeof navigation==`object`){var r=!1,i=null;return navigation.addEventListener(`navigate`,e),navigation.addEventListener(`navigatesuccess`,t),navigation.addEventListener(`navigateerror`,t),setTimeout(n,100),function(){r=!0,navigation.removeEventListener(`navigate`,e),navigation.removeEventListener(`navigatesuccess`,t),navigation.removeEventListener(`navigateerror`,t),i!==null&&(i(),i=null)}}}function Fp(e){this._internalRoot=e}Ip.prototype.render=Fp.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(a(409));var n=t.current;np(n,pu(),e,t,null,null)},Ip.prototype.unmount=Fp.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;np(e.current,2,null,e,null,null),bu(),t[ut]=null}};function Ip(e){this._internalRoot=e}Ip.prototype.unstable_scheduleHydration=function(e){if(e){var t=ot();e={blockedOn:null,target:e,priority:t};for(var n=0;n<xp.length&&t!==0&&t<xp[n].priority;n++);xp.splice(n,0,e),n===0&&Ep(e)}};var Lp=r.version;if(Lp!==`19.2.8`)throw Error(a(527,Lp,`19.2.8`));k.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render==`function`?Error(a(188)):(e=Object.keys(e).join(`,`),Error(a(268,e)));return e=f(t),e=e===null?null:p(e),e=e===null?null:e.stateNode,e};var Rp={bundleType:0,version:`19.2.8`,rendererPackageName:`react-dom`,currentDispatcherRef:O,reconcilerVersion:`19.2.8`};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<`u`){var zp=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!zp.isDisabled&&zp.supportsFiber)try{Le=zp.inject(Rp),Re=zp}catch{}}e.createRoot=function(e,t){if(!o(e))throw Error(a(299));var n=!1,r=``,i=Ks,s=qs,c=Js;return t!=null&&(!0===t.unstable_strictMode&&(n=!0),t.identifierPrefix!==void 0&&(r=t.identifierPrefix),t.onUncaughtError!==void 0&&(i=t.onUncaughtError),t.onCaughtError!==void 0&&(s=t.onCaughtError),t.onRecoverableError!==void 0&&(c=t.onRecoverableError)),t=ep(e,1,!1,null,null,n,r,null,i,s,c,Pp),e[ut]=t.current,Sd(e),new Fp(t)}})),ce=t(((e,t)=>{function n(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>`u`||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!=`function`))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n)}catch(e){console.error(e)}}n(),t.exports=k()})),A=e(n(),1),j=ce(),M={wrapper:`_wrapper_1xh50_2`,header:`_header_1xh50_10`,headerActions:`_headerActions_1xh50_21`,title:`_title_1xh50_27`,panelGroup:`_panelGroup_1xh50_36`,panelToggle:`_panelToggle_1xh50_43`,helpToggle:`_helpToggle_1xh50_66`,helpButtonWrapper:`_helpButtonWrapper_1xh50_93`,helpTogglePulsing:`_helpTogglePulsing_1xh50_97`,helpPulse:`_helpPulse_1xh50_1`,helpHint:`_helpHint_1xh50_112`,helpHintFading:`_helpHintFading_1xh50_139`,helpHintKbd:`_helpHintKbd_1xh50_144`,resizeHandle:`_resizeHandle_1xh50_153`},N=e=>{try{return!new DOMParser().parseFromString(e.trim(),`text/xml`).querySelector(`parsererror`)}catch{return!1}},P=e=>{try{return JSON.parse(e),!0}catch{return!1}},le=e=>e.trim()?P(e)?{valid:!0,error:null,type:`json`}:N(e)?{valid:!0,error:null,type:`xml`}:{valid:!1,error:`Invalid JSON/XML format`,type:null}:{valid:!0,error:null,type:null},ue=e=>{try{let t=JSON.parse(e);return JSON.stringify(t,null,2)}catch{return e}},de=()=>{let[e,t]=(0,A.useState)([]),n=(0,A.useRef)(0),r=(0,A.useRef)(new Set);return(0,A.useEffect)(()=>()=>{r.current.forEach(clearTimeout)},[]),{toasts:e,addToast:(0,A.useCallback)((e,i=`info`,a)=>{let o=++n.current;t(t=>[...t,{id:o,message:e,type:i,action:a?.action}]);let s=setTimeout(()=>{r.current.delete(s),t(e=>e.filter(e=>e.id!==o))},a?.durationMs??3e3);r.current.add(s)},[]),removeToast:(0,A.useCallback)(e=>{t(t=>t.filter(t=>t.id!==e))},[])}},fe=(e,t)=>{let n=(0,A.useCallback)(()=>{try{let n=window.localStorage.getItem(e);return n?JSON.parse(n):t}catch{return t}},[e]),[r,i]=(0,A.useState)(n);return(0,A.useEffect)(()=>{i(n())},[e]),(0,A.useEffect)(()=>{try{window.localStorage.setItem(e,JSON.stringify(r))}catch(t){console.error(`Error setting localStorage key "${e}":`,t)}},[e,r]),(0,A.useEffect)(()=>{let t=t=>{(t.key===e||t.key===null)&&i(n())};return window.addEventListener(`storage`,t),()=>window.removeEventListener(`storage`,t)},[e,n]),(0,A.useEffect)(()=>{let e=()=>i(n());return window.addEventListener(`focus`,e),document.addEventListener(`visibilitychange`,e),()=>{window.removeEventListener(`focus`,e),document.removeEventListener(`visibilitychange`,e)}},[n]),[r,i]},pe=2e4,me=[{path:`/json-path`,label:`JSON-Path`,title:`JSON-Path Playground`,wsPath:`/ws/json/path`,storageKeyPayload:`jsonpath-last-payload`,storageKeyHistory:`jsonpath-command-history`,storageKeyTab:`jsonpath-right-tab`,storageKeyHelpTopic:`jsonpath-help-topic`,supportsUpload:!0,supportsHelp:!0,helpContentProfile:`json-path`,tabs:[`payload`]},{path:`/`,label:`Minigraph`,title:`Minigraph Playground`,wsPath:`/ws/graph/playground`,storageKeyPayload:`minigraph-last-payload`,storageKeyHistory:`minigraph-command-history`,storageKeyTab:`minigraph-right-tab`,storageKeySavedGraphs:`minigraph-saved-graphs`,storageKeyHelpTopic:`minigraph-help-topic`,supportsClipboard:!0,supportsHelp:!0,helpContentProfile:`minigraph`,supportsAuthoring:!0,supportsGraphRun:!0,supportsSessionCollaboration:!0,tabs:[`graph`,`graph-data`]}],he={json_simple:JSON.stringify({name:`John Doe`,age:30,city:`New York`},null,2),json_nested:JSON.stringify({user:{name:`Jane Smith`,profile:{email:`jane@example.com`,address:{city:`San Francisco`,country:`USA`}}}},null,2),json_array:JSON.stringify([{id:1,name:`Item 1`,status:`active`},{id:2,name:`Item 2`,status:`pending`},{id:3,name:`Item 3`,status:`inactive`}],null,2),xml_simple:`<?xml version="1.0" encoding="UTF-8"?>
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
</items>`};function ge(e){return`ws://${window.location.host}${e}`}var F=ne();function _e(e,t,n,r){let i=e[t]??{phase:`idle`,connectionEpoch:null,messages:[]},a=[...i.messages,{id:n,raw:r}];return a.length>200&&a.shift(),{...e,[t]:{...i,messages:a}}}function ve(e,t){let n=e[t.path]??{phase:`idle`,connectionEpoch:null,messages:[]};switch(t.type){case`CONNECTING`:return{...e,[t.path]:{...n,phase:`connecting`}};case`CONNECTED`:return _e({...e,[t.path]:{...n,phase:`connected`,connectionEpoch:t.id}},t.path,t.id,t.msg);case`MESSAGE_RECEIVED`:return _e(e,t.path,t.id,t.msg);case`DISCONNECTED`:return _e({...e,[t.path]:{...n,phase:`idle`}},t.path,t.id,t.msg);case`CONNECT_ERROR`:return{...e,[t.path]:{...n,phase:`idle`}};case`CLEAR_MESSAGES`:return{...e,[t.path]:{...n,messages:[]}};default:return e}}var ye=(0,A.createContext)(null);function be({children:e}){let[t,n]=(0,A.useReducer)(ve,{}),r=(0,A.useRef)({}),i=(0,A.useRef)({}),a=(0,A.useRef)({});(0,A.useEffect)(()=>()=>{Object.entries(r.current).forEach(([e,t])=>{t?.close();let n=i.current[e];n&&clearInterval(n)})},[]);let o=e=>ge(e),s=e=>(a.current[e]=(a.current[e]??0)+1,a.current[e]),c=()=>{let e=new Date().toString(),t=e.indexOf(`GMT`);return t>0?e.substring(0,t).trim():e},l=(e,t)=>JSON.stringify({type:e,message:t,time:c()}),u=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t){let e=t.type;return e===`ping`||e===`pong`}}catch{}return!1},d=(0,A.useCallback)((e,t)=>{if(!window.WebSocket){t?.(`WebSocket not supported by your browser`,`error`);return}let a=r.current[e];if(a&&(a.readyState===WebSocket.OPEN||a.readyState===WebSocket.CONNECTING)){t?.(`Already connected`,`error`);return}n({type:`CONNECTING`,path:e});let c=new WebSocket(o(e));r.current[e]=c,c.onopen=()=>{n({type:`CONNECTED`,path:e,id:s(e),msg:l(`info`,`connected`)}),t?.(`Connected to WebSocket`,`success`),c.send(JSON.stringify({type:`welcome`})),i.current[e]=setInterval(()=>{c.readyState===WebSocket.OPEN&&c.send(l(`ping`,`keep alive`))},pe)},c.onmessage=t=>{u(t.data)||n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t.data})},c.onerror=()=>{n({type:`CONNECT_ERROR`,path:e})},c.onclose=a=>{let o=i.current[e];o&&(clearInterval(o),i.current[e]=null),n({type:`DISCONNECTED`,path:e,id:s(e),msg:l(`info`,`disconnected - (${a.code}) ${a.reason}`)}),t?.(`Disconnected from WebSocket`,`info`),r.current[e]===c&&(r.current[e]=null)}},[]),f=(0,A.useCallback)(e=>{let t=r.current[e];t?t.close():n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:l(`error`,`already disconnected`)})},[]);(0,A.useEffect)(()=>(me.forEach(e=>{d(e.wsPath)}),()=>{me.forEach(e=>{let t=r.current[e.wsPath];t&&t.close()})}),[]);let p=(0,A.useCallback)((e,t)=>{let n=r.current[e];return n&&n.readyState===WebSocket.OPEN?(n.send(t),!0):!1},[]),m=(0,A.useCallback)((e,t)=>{n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t})},[]),h=(0,A.useCallback)(e=>{n({type:`CLEAR_MESSAGES`,path:e})},[]),[g,_]=(0,A.useState)({}),v=(0,A.useCallback)((e,t)=>{_(n=>{if(t===null){let t={...n};return delete t[e],t}return{...n,[e]:t}})},[]),y=(0,A.useCallback)(e=>g[e]??null,[g]),b=(0,A.useCallback)(e=>{let t=g[e]??null;return t!==null&&_(t=>{let n={...t};return delete n[e],n}),t},[g]),x=(0,A.useCallback)(e=>t[e]??{phase:`idle`,connectionEpoch:null,messages:[]},[t]),S=(0,A.useMemo)(()=>({getSlot:x,connect:d,disconnect:f,send:p,appendMessage:m,clearMessages:h,setPendingPayload:v,peekPendingPayload:y,takePendingPayload:b}),[x,d,f,p,m,h,v,y,b]);return(0,F.jsx)(ye.Provider,{value:S,children:e})}function xe(){let e=(0,A.useContext)(ye);if(!e)throw Error(`useWebSocketContext must be used inside <WebSocketProvider>`);return e}var Se=e=>{try{let t=JSON.parse(e);return{type:t.type||`info`,message:t.message||e,time:t.time,raw:e}}catch{return{type:`raw`,message:e,time:null,raw:e}}},Ce=e=>({info:`ℹ️`,error:`❌`,ping:`🔄`,welcome:`👋`,raw:``})[e]??`•`,we=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t)return{isJSON:!0,data:t}}catch{}return{isJSON:!1,data:null}};function Te(e){if(!e.includes(`Graph exported to `))return null;let t=Oe(e);if(!t)return null;let n=t.split(`/`)[4];return n?{graphName:n,apiPath:t}:null}function Ee(e){return e.includes(`Invalid filename`)?{reason:`invalid-name`}:e.includes(`Expect root node name`)?{reason:`root-name-conflict`}:null}function De(e){let t=we(e);return t.isJSON?(t.data.type,!1):!0}function Oe(e){let t=e.match(/\/api\/graph\/model\/([^\s'"]+)/);return t?t[0]:null}function ke(e){return De(e)?Oe(e)!==null:!1}function Ae(e){let t=e.match(/\/api\/json\/content\/([\w-]+)/);return t?t[0]:null}function je(e){let t=e.match(/Large payload \((\d+)\)\s*->\s*GET\s+(\/api\/inspect\/[^\s]+)/i);if(!t)return null;let n=parseInt(t[1],10),r=t[2];return{apiPath:r,byteSize:n,filename:`${r.split(`/`).filter(Boolean).pop()??`payload`}.json`}}function Me(e){let t=e.match(/You may upload .*?->\s*POST\s+(\/api\/mock\/[\w-]+)/i);return t?t[1]:null}function Ne(e){if(!e.startsWith(`> `))return!1;let t=e.slice(2).trim().toLowerCase();return t===`help`||t.startsWith(`help `)?!0:t.startsWith(`describe `)?!t.slice(9).trim().startsWith(`graph`):!1}function Pe(e){if(!e.startsWith(`> `)||!e.slice(2).trimStart().toLowerCase().startsWith(`import graph from `))return null;let t=e.slice(2).trimStart().slice(18).trim();return t.length>0?t:null}var Fe=/^node ([A-Za-z0-9_-]+) created$/i,Ie=/^node ([A-Za-z0-9_-]+) already exists$/i,Le=/^node ([A-Za-z0-9_-]+) updated$/i,Re=/^node ([A-Za-z0-9_-]+) deleted$/i,ze=/^node ([A-Za-z0-9_-]+) connected to ([A-Za-z0-9_-]+)$/i,Be=/^node ([A-Za-z0-9_-]+) not found$/i,Ve=/^Source and target nodes must be different$/i,He=/^Syntax: connect \{node-A\} to \{node-B\} with \{relation\}$/i,Ue=/^ERROR: (.+)$/;function We(e){let t=e.trim();if(t.startsWith(`> `))return null;let n=t.match(Fe);if(n)return{status:`accepted`,action:`create-node`,alias:n[1],targetAlias:null,message:t};let r=t.match(Ie);if(r)return{status:`rejected`,action:`create-node`,alias:r[1],targetAlias:null,message:t};let i=t.match(Le);if(i)return{status:`accepted`,action:`edit-node`,alias:i[1],targetAlias:null,message:t};let a=t.match(Re);if(a)return{status:`accepted`,action:`delete-node`,alias:a[1],targetAlias:null,message:t};let o=t.match(ze);if(o)return{status:`accepted`,action:`create-connection`,alias:o[1],targetAlias:o[2],message:t};let s=t.match(Be);return s?{status:`rejected`,action:null,alias:s[1],targetAlias:null,message:t}:Ve.test(t)||He.test(t)?{status:`rejected`,action:`create-connection`,alias:null,targetAlias:null,message:t}:t.match(Ue)?{status:`error`,action:null,alias:null,targetAlias:null,message:t}:null}function Ge(e){if(!De(e)||e.startsWith(`> `)||ke(e))return null;let t=e.toLowerCase();return t.includes(`graph model imported as draft`)?`import-graph`:t.includes(` -> `)&&t.includes(`removed`)||t.startsWith(`node `)&&(t.includes(` created`)||t.includes(` updated`)||t.includes(` deleted`)||t.includes(` connected to `)||t.includes(` imported from `)||t.includes(` overwritten by node from `))?`node-mutation`:null}var Ke={command:``,historyIndex:-1,draftCommand:``};function qe(e,t){switch(t.type){case`SET_COMMAND`:return{...e,command:t.value,historyIndex:-1,draftCommand:``};case`CLEAR_COMMAND`:return{...e,command:``,historyIndex:-1,draftCommand:``};case`SET_HISTORY_INDEX`:return{...e,historyIndex:t.index,command:t.command};case`ENTER_HISTORY`:return{...e,historyIndex:0,command:t.command,draftCommand:e.command};case`EXIT_HISTORY`:return{...e,historyIndex:-1,command:e.draftCommand,draftCommand:``};default:return e}}function Je({wsPath:e,storageKeyHistory:t,payload:n,addToast:r,bus:i,handleLocalCommand:a}){let o=xe(),{phase:s,connectionEpoch:c,messages:l}=o.getSlot(e),u=s===`connected`,d=s===`connecting`,[f,p]=(0,A.useReducer)(qe,Ke),{command:m,historyIndex:h}=f,[g,_]=fe(t,[]),v=(0,A.useRef)(null),y=(0,A.useRef)(!1);(0,A.useEffect)(()=>{v.current&&(v.current.scrollTop=v.current.scrollHeight)},[l]);let b=(0,A.useCallback)(()=>{o.connect(e,r)},[o,e,r]),x=(0,A.useCallback)(()=>{o.disconnect(e)},[o,e]),S=(0,A.useCallback)(()=>{if(s!==`connected`)return;let t=m.trim();if(t.length!==0){if(a?.(t)===!0){g[0]!==t&&_(e=>[t,...e].slice(0,50)),o.appendMessage(e,`> `+t),p({type:`CLEAR_COMMAND`});return}o.send(e,t),g[0]!==t&&_(e=>[t,...e].slice(0,50)),t===`load`&&(n.length===0?o.appendMessage(e,`ERROR: please paste JSON/XML payload in input text area`):o.send(e,n)),p({type:`CLEAR_COMMAND`})}},[o,e,s,m,n,g,_,a]),C=(0,A.useCallback)(e=>{if(e.key===`ArrowUp`){if(e.preventDefault(),g.length===0)return;if(h===-1)p({type:`ENTER_HISTORY`,command:g[0]});else if(h<g.length-1){let e=h+1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}}else if(e.key===`ArrowDown`)if(e.preventDefault(),h<=0)h===0&&p({type:`EXIT_HISTORY`});else{let e=h-1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}},[g,h]);(0,A.useEffect)(()=>{if(i)return i.on(`upload.contentPath`,t=>{if(!y.current)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let i;try{i=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(t.uploadPath,{method:`POST`,headers:{"Content-Type":`application/json`},body:i}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})})},[i,n,e,o,r]),(0,A.useEffect)(()=>{if(i||!y.current||l.length===0)return;let t=l[l.length-1].raw,a=Ae(t);if(!a)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let s;try{s=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(a,{method:`POST`,headers:{"Content-Type":`application/json`},body:s}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})},[i,l,n,e,o,r]);let w=(0,A.useCallback)(()=>{if(s===`connected`){if(n.length===0){r(`Nothing to upload — paste a JSON payload first`,`error`);return}y.current=!0,o.send(e,`upload`)}},[o,e,s,n,r]),ee=(0,A.useCallback)(t=>s===`connected`&&o.send(e,t),[o,e,s]),te=(0,A.useCallback)(()=>{navigator.clipboard.writeText(l.map(e=>e.raw).join(`
`)),r(`Console copied to clipboard!`,`success`)},[l,r]),T=(0,A.useCallback)(()=>{o.clearMessages(e),r(`Console cleared`,`info`)},[o,e,r]),E=(0,A.useCallback)(t=>{o.appendMessage(e,t)},[o,e]);return{connected:u,connecting:d,connectionEpoch:c,messages:l,command:m,setCommand:(0,A.useCallback)(e=>p({type:`SET_COMMAND`,value:e}),[]),connect:b,disconnect:x,sendCommand:S,handleKeyDown:C,consoleRef:v,copyMessages:te,clearMessages:T,uploadPayload:w,sendRawText:ee,appendMessage:E,history:g}}function Ye(e){let[t,n]=(0,A.useState)(()=>window.matchMedia(e).matches);return(0,A.useEffect)(()=>{let t=window.matchMedia(e),r=e=>n(e.matches);return t.addEventListener(`change`,r),()=>t.removeEventListener(`change`,r)},[e]),t}function Xe(e){if(typeof e!=`object`||!e)return!1;let t=e;return Array.isArray(t.nodes)}function Ze(e,t,n){let r=t.includes(n)?n:t[0]??`graph`;return typeof e==`string`&&t.includes(e)?e:r}function Qe(e,t,n,r,i,a=!1){let[o,s]=(0,A.useState)(null),[c,l]=(0,A.useState)(!1),[u,d]=fe(i,n),f=Ze(u,r,n),[p,m]=(0,A.useState)(!1),h=(0,A.useCallback)(e=>{d(t=>{let i=Ze(t,r,n);return Ze(typeof e==`function`?e(i):e,r,n)})},[d,r,n]);(0,A.useEffect)(()=>{u!==f&&d(f)},[u,f,d]);let g=(0,A.useRef)(e);(0,A.useEffect)(()=>{g.current=e},[e]);let _=(0,A.useRef)(null);(0,A.useEffect)(()=>{if(l(!1),!e){s(null);return}let n=new AbortController;return s(null),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{Xe(e)?(s(e),h(`graph`)):l(!0)}).catch(e=>{e.name!==`AbortError`&&(l(!0),a||t(`Graph fetch failed: ${e.message}`,`error`))}),()=>{n.abort()}},[e,t,a]);let v=(0,A.useCallback)(()=>{let e=g.current;if(!e)return;_.current?.abort();let n=new AbortController;_.current=n,m(!0),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{Xe(e)&&s(e),m(!1)}).catch(e=>{e.name!==`AbortError`&&(t(`Graph refresh failed: ${e.message}`,`error`),m(!1))})},[]);return(0,A.useEffect)(()=>()=>{_.current?.abort()},[]),{graphData:o,setGraphData:s,rightTab:f,setRightTab:h,isRefreshing:p,initialFetchFailed:c,refetchGraph:v}}function $e({enabled:e,sessionGraphPath:t,pinnedGraphPath:n,initialFetchFailed:r,graphData:i,setGraphData:a,setRightTab:o}){let s=(0,A.useRef)(null);(0,A.useEffect)(()=>{if(!e||t===null||i!==null||n!==null&&!r)return;let c=`${t}|${n??``}`;if(s.current===c)return;s.current=c;let l=new AbortController;return fetch(t,{signal:l.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{Xe(e)&&(a(e),o(`graph`))}).catch(()=>{}),()=>{l.abort()}},[e,i,r,n,t,a,o])}function et({bus:e,pinnedGraphPath:t,setPinnedGraphPath:n,connected:r,sendRawText:i,addToast:a}){let o=(0,A.useRef)(null),s=(0,A.useRef)(!1),c=(0,A.useRef)(t),l=(0,A.useRef)(r),u=(0,A.useRef)(i);(0,A.useEffect)(()=>{c.current=t},[t]),(0,A.useEffect)(()=>{l.current=r},[r]),(0,A.useEffect)(()=>{u.current=i},[i]),(0,A.useEffect)(()=>{r||(s.current=!1,o.current!==null&&(clearTimeout(o.current),o.current=null))},[r]),(0,A.useEffect)(()=>e.on(`graph.link`,e=>{s.current&&(s.current=!1,n(e.apiPath))}),[e,n]),(0,A.useEffect)(()=>e.on(`graph.mutation`,e=>{if(l.current){if(e.mutationType===`import-graph`){o.current!==null&&(clearTimeout(o.current),o.current=null),s.current=!0,u.current(`describe graph`),a(`Graph imported — refreshing view…`,`info`);return}s.current=!0,o.current!==null&&clearTimeout(o.current),o.current=setTimeout(()=>{o.current=null,l.current&&(s.current=!0,u.current(`describe graph`),a(c.current===null?`Graph updated — opening Graph tab…`:`Graph updated — refreshing…`,`info`))},300)}}),[e,a]),(0,A.useEffect)(()=>e.on(`session.reset`,()=>{o.current!==null&&(clearTimeout(o.current),o.current=null),s.current=!1,n(null)}),[e,n]),(0,A.useEffect)(()=>()=>{o.current!==null&&clearTimeout(o.current)},[])}var tt=`Connect two nodes
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
`,nt=`Create a node
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
`,rt=`Data Dictionary
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
`,it=`Delete a node, a connection or the fetch cache
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
`,at=`Describe graph, node, connection or skill
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
`,ot=`Edit a node
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
`,st=`Execute a single node
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
`,ct=`Export a graph model
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
`,lt=`Skill: Graph API Fetcher
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

- {node}.status and {node}.error are set (the engine's error record; {node}.stack is
  added when the failure carries a stack trace)
- the output[] mappings are SKIPPED
- with exception={handler-node}, traversal JUMPS to the handler; without
  it, the run ABORTS and the error is returned to the caller.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph. A generic
handler reads the context in its data mapping without naming any failing node:

\`\`\`
mapping[]=error.source -> output.body.failed_at
mapping[]=error.code -> output.body.status
mapping[]=error.message -> output.body.message
\`\`\`

Anchor a shared handler from an island (root -> island -> handler): the handler is reached by
jumping, so the island keeps it non-orphan while plain traversal stops at the island. A handler
node may also connect onward to more nodes for sophisticated recovery (e.g. a graph.task
invoking a composable function). Note that a node is visited at most once per run unless RESET,
so if two parallel branches fail together, only the first jump enters a shared handler. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error". When the failing node is retried successfully, the context resolves to
code=200 with the source kept and the failure details removed - the source match ensures a
parallel node's success never clears a different node's outstanding failure.

A retry handler is typically a graph.math decision node that inspects the
fetcher's status/error, counts attempts, and retries with a bound:

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]=RESET: {error.source}, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: recovery-node
ELSE: next
'''
statement[]=NEXT: {error.source}
statement[]=DELAY: 50
\`\`\`

The handler is fully GENERIC: every statement command resolves {dynamic variables}, so
RESET: {error.source} and NEXT: {error.source} retry whichever node routed here - one handler
serves every fetcher and task in the graph. See tutorial 12 for the full walkthrough.

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
`,I=`Skill: Graph Data Mapper
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
`,ut=`Skill: Graph Extension
----------------------
Delegates to another graph model (a sub-graph) or to an Event Script flow,
so larger capabilities compose from smaller ones. The node passes named
inputs to the target, and the target's response body becomes this node's
result. This is the seam between the knowledge-graph layer and the Event
Script layer beneath it.

The delegated graph or flow inherits the caller's business correlation ID (model.cid), the same
way an Event Script sub-flow does. A delegated subgraph that suspends therefore persists its
state under the shared business correlation ID scoped by its own graph ID - re-invoking with the
same correlation ID resumes it. This makes a parent graph a natural orchestrator of independently
resumable subgraph paths (see the workflow-suspension guide's orchestrator pattern).

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
  aborts. The jump also stages the generic exception context
  (error.source/code/message and error.stack when available) so ONE
  island-anchored handler can serve every node - error.source is the
  extension node in THIS graph; failures inside the delegated subgraph or
  flow route to that graph's own handlers. The bounded-retry pattern and
  the full error-context contract are shown under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the target once per
  element of a runtime list, with bounded parallel fan-out (concurrency
  1-30, default 3). The shared iteration rules are under
  'help graph-api-fetcher'.
- Use graph.extension for multi-step orchestration; use graph.task for a
  single composable-function call.
`,dt=`Skill: Graph Island
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
`,ft=`Skill: Graph Join
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
`,pt=`Skill: Graph JS (retired)
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
`,mt=`Skill: Graph Math
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

Dynamic variables in statement commands
---------------------------------------
Every statement command resolves {dynamic variables}, not only expressions. A NEXT: or THEN:/ELSE:
jump target, a RESET: list entry and a DELAY: value may each be a {namespace.key} reference
resolved at execution time. This is what makes a GENERIC error handler possible - it retries
whichever node routed to it without naming any node:

\`\`\`
statement[]=RESET: {error.source}, error-handler
statement[]=NEXT: {error.source}
statement[]=DELAY: {model.backoff}
\`\`\`

An unresolved variable renders "null": a RESET: entry is then a safe no-op, a DELAY: is skipped,
and a jump target fails the run loudly ("Next node 'null' does not exist") - correct for a jump,
so seed the variable before relying on it. See tutorial 12 for the full generic retry handler.

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
`,ht=`Skill: Graph Resume
-------------------
When a graph run starts with the same business correlation ID as a previously suspended
transaction, the node with this skill restores the persisted workflow state and continues
traversal from the recorded suspension point - without re-executing it.

This skill is a superset of "graph.task": the "task" property names the pluggable store
function, but restoration is encapsulated by the skill, so the node needs no input or
output data mapping.

Place the resume node early in the traversal - conventionally named "resume" and connected
right after "root", or after nodes that perform setup and initialization. When the store
has a record for the business correlation ID (model.cid), the skill merges the persisted
model key-values into the state machine (the current run's reserved keys such as model.cid
and model.instance always win), restores the traversal bookkeeping so downstream join
barriers still see branches completed before suspension, and jumps past the suspension
point onto its normal forward path.

When there is no record - a fresh transaction, which is the normal first-run case, or an
expired record - traversal simply continues along the resume node's own forward path.

Either way, the skill records the outcome in "model.run" - "resume" when a record was
restored, "fresh" when there was none. The engine does not distinguish an absent record
from an expired one; with several checkpoints in one graph, no single fallback could be
right for all of them. Handling the fresh-or-expired condition is application logic:
gate the resume node's forward path with a graph.math IF-THEN-ELSE (on model.run or on
the request shape - see "help tutorial 14") to reject the request, advise the UI, or
jump to a recovery node.

The default store behavior consumes the record on retrieval, so a duplicate resume request
cannot execute the continuation twice.

Route name
----------
"graph.resume"

Setup
-----
To enable this skill, set "skill=graph.resume" as a property in a node.

The following parameter is required in the properties of the node:

1. task - the route name of the state-store function (e.g. "v1.redis.retrieve.model")

The store function receives headers "type=get" and a body of {"cid": "...", "graph": "..."}
(the running graph's ID scopes the lookup, so a resume only ever sees records written by its
own graph - parent and subgraphs are self-contained) and returns
the persisted record, or nothing (null or an empty map) when absent or expired.

Example
-------
create node resume
with type Resume
with properties
purpose=Restore workflow state from the external state store
skill=graph.resume
task=v1.redis.retrieve.model
`,gt=`Skill: Graph Suspend
--------------------
When a graph reaches the node with this skill, the workflow state of the graph instance is
persisted to an external state store and the graph run completes normally - the transaction
can resume later through the "graph.resume" skill using the same business correlation ID.

This skill is a superset of "graph.task": the "task" property names the pluggable store
function, but the persistence envelope is assembled by the skill itself, so the node needs
no input or output data mapping.

The node carrying this skill MUST be named "suspend" - a reserved alias like "root" and
"end". There is exactly one suspend node per graph, and two patterns reach it - named
after the node that pauses:

1. Checkpoint node - a working node with a DRAWN EDGE to the "suspend" node pauses when
   its skill completes normally: the walker redirects to "suspend" instead of following
   the node's continuation edge. The drawn edge is the declaration - no node property is
   needed. The node must also have at least one other edge (the continuation): a resumed
   run continues along it, and the node itself is never re-executed. A checkpoint never
   decides - reaching it IS the decision to pause. It is a complete working node: it
   executes its skill in full (input mapping, skill, output mapping) before pausing, so
   it may carry any non-routing skill (graph.data.mapper, graph.task, graph.api.fetcher,
   graph.extension), capture the actor's input into the model, and stage the caller's
   reply in output.* - only its exit changes.

2. Decision node - a decision (graph.math) pauses by returning "suspend" from its
   IF-THEN-ELSE. On resume the decision is RE-EXECUTED against the new request input,
   so it re-decides every time: an approval proceeds, a rejection terminates, and
   anything else returns "suspend" again - a wait loop with no extra nodes. A decision
   must NOT draw an edge to the suspend node (its drawn edges are outcome alternatives,
   and the gate rejects the shape); it may stage the caller's waiting reply in output.*
   before its IFs - the outcome paths overwrite it.

(The ADRs and compiler internals call these shapes "edge mode" and "jump mode".)

When the suspend node is reachable only by jumps, anchor it behind an island so the graph
has no orphan nodes: "root -> island -> suspend" - traversal stops at the island, so the
anchor edge is never walked. The suspend node cannot be an exception handler
(exception=suspend is rejected). The retired "suspend=true" property is accepted and
ignored for one deprecation window (the gate logs a WARN) - every valid earlier model
already draws its checkpoint edge, which now declares the same behavior.

A suspension point must be the sole active branch - do not suspend between a fan-out and
its join; suspend after the join instead. Anything a later step needs must be mapped into
the "model" namespace before the suspension point, because a node's transient "result"
properties do not survive suspension - the model is the workflow's durable memory.

Unless the graph staged its own output before suspension, the skill stages a default
response body so the caller of the suspended run receives a meaningful reply:

{
  "type": "suspended",
  "cid": "<business correlation ID>"
}

Route name
----------
"graph.suspend"

Setup
-----
To enable this skill, create a node named "suspend" with "skill=graph.suspend".

The following parameters are required in the properties of the node:

1. task - the route name of the state-store function (e.g. "v1.redis.persist.model")
2. ttl - the record's time-to-live using duration syntax, e.g. 20s, 5m, 2h, 2d

The store function receives headers "type=put" and a body of:

{
  "cid":   "<business correlation ID>",
  "graph": "<the graph that suspended - cid + graph form the retrieval key>",
  "node":  "<the suspension point - the node that routed here>",
  "ttl":   <seconds>,
  "model": { the model namespace minus the per-run reserved keys },
  "seen":  { traversal bookkeeping },
  "run":   { traversal bookkeeping }
}

The record is scoped by graph + cid (the Redis reference implementation keys it
"graph:{graph_id}:{cid}"), so the same business transaction may suspend independently in
each domain's graph and in each subgraph, and a resume only ever sees its own graph's
record. The store must acknowledge with a 2xx reply before the graph completes - a failed
store call fails the node (the optional "exception" property routes it to a handler node).

Example
-------
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to the external state store
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
`,_t=`Skill: Graph Task
-----------------
When a node is configured with this skill of "graph task", it will invoke a composable function
through its route name and collect the function's response into the "result" property of the node.
In case of exception, the "status" and "error" fields will be set to the node's properties and the
graph execution will stop unless an exception handler node is configured.

A composable function is a TypedLambdaFunction registered using the PreLoad annotation. This provides
a lightweight method to extend a knowledge graph's capability with a small piece of business logic,
without writing a new skill - more complex business logic should be delegated to a flow extension
or a subgraph using the "graph.extension" skill.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.task"

Setup
-----
To enable this skill for a node, set "skill=graph.task" as a property in a node.

The following parameters are required in the properties of the node:

1. task - the route name of the composable function to invoke
2. input - one or more data mapping entries as input to the composable function

The system uses the same syntax of Event Script for data mapping.

Properties
----------
\`\`\`
skill=graph.task
task=route.name.of.composable.function
input[]={mapping of key-values from input, model or another node to the function's request}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
\`\`\`

Optional properties
-------------------
\`\`\`
for_each[]={map an array parameter for iterative function execution}
concurrency={controls parallel function calls for an "iterative task request". Default 3, max 30}
exception={error-handler-node-name}
\`\`\`

Input data mapping
------------------
source.composite.key -> target

The source (LHS) can use a key-value from the \`input.\` namespace, the \`model.\` namespace, another
node or a constant such as text(hello). The target (RHS) addresses the function's request:

1. \`*\` - the LHS value becomes the whole request body (same as Event Script). Data mapping entries
   are processed in order, so later entries can merge additional key-values into a request body
   that was seeded with \`*\`.
2. \`header.{name}\` - sets a request header of the function call
3. \`model.{key}\` - stages a variable in the graph's state machine instead of the request body, so
   that later entries can reference it as a **dynamic variable** (same as Event Script). e.g. after
   \`input.body.token -> model.token\`, the entry \`text(Bearer {model.token}) -> auth\` resolves the
   \`{model.token}\` reference. Engine-managed model metadata (model.cid, model.ttl, etc.) is
   immutable - a mapping that targets it is rejected.
4. any other composite key - a key-value in the request body

Example:
\`\`\`
input[]=input.body -> *
input[]=input.header.hello -> header.hello
input[]=input.body.amount -> amount
input[]=input.body.person_id -> model.person_id
input[]=text(/api/mdm/profile/{model.person_id}) -> url
\`\`\`

If the function is declared as a TypedLambdaFunction with a PoJo input class, the request body map
is automatically converted to the PoJo at the function boundary.

Result set
----------
Upon successful execution, the function's response body is stored in the "result" parameter, the
response status in "status" and the response headers in "header" in the properties of the node.
The optional output data mapping can copy them to the 'model.' or 'output.' namespace.

Example:
\`\`\`
output[]=result -> model.soap_request_payload
\`\`\`

Timeout
-------
The function call uses the graph instance's time-to-live from "model.ttl" (default 30000 ms).

This deadline bounds the event call to the composable function - it cannot reach inside a
generic function. When the function has its own downstream timeout contract, express it in the
input data mapping. For example, the AsyncHttpClient (async.http.request) takes its HTTP timeout
from the "x-ttl" key-value under "headers" in milliseconds, e.g. \`text(5000) -> headers.x-ttl\`
(see tutorial 13).

Exception handling
------------------
If the function throws an exception (e.g. AppException with a status code) or the call times out,
the "error" and "status" parameters of the node are set (plus "stack" when the failure carries a
stack trace). When the node has an "exception" property, the graph jumps to that error handler
node. Otherwise, the error is returned as the graph output.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph - a graph.task
node, an API fetcher and an extension can all share the same handler, and error.source tells
them apart. Anchor a shared handler from an island (root -> island -> handler) because it is
reached by jumping, and note that a node is visited at most once per run unless RESET. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error". See "describe skill graph.api.fetcher" for the canonical bounded-retry handler.

Example
-------
\`\`\`
create node prepare-soap-request
with type Task
with properties
task=v1.prepare.soap.request
input[]=input.body -> *
input[]=input.header.hello -> header.hello
output[]=result -> model.soap_request_payload
skill=graph.task
\`\`\`
`,vt=`Import a graph model or a node
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
`,yt=`Inspect the state machine
-------------------------
Read a value from the current graph instance's state machine: node
properties, and the input, output and model namespaces.

Syntax
------
\`\`\`
inspect {key}
\`\`\`

\`{key}\` is a placeholder - substitute your key and do not type the braces.
A whole namespace (input | output | model | error) is also valid, e.g.
'inspect output'.

After a failed node routes to its exception handler, 'inspect error' shows the staged
exception context - error.source (the failing node), error.code, error.message and
error.stack when available. When the failing node is later retried successfully, the
context resolves: code becomes 200, the source stays, and the failure details are removed
- so an empty context means nothing failed, {source, code: 200} means recovered, and a
full context means an outstanding failure. The 'error' namespace is a first-class
state-machine citizen like 'model', which is why 'error' is a reserved node alias.

Example
-------
\`\`\`
inspect output
inspect input.body.user_id
inspect model.some_variable
inspect output.body.some_key
inspect book.price
inspect error
inspect error.source
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Keys may be composite (dot-bracket), e.g. output.body.profile[0].name.
- A node's properties and results are addressed by node name, e.g.
  book.price or fetcher.result.name.
- A value too large for the console is redirected: the reply prints a
  GET /api/inspect/... URL to download it instead.
`,bt=`Instantiate a graph instance
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
`,xt=`List nodes, connections, graphs or flows
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
`,St=`Run a graph instance
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
- Before traversal begins, the graph is checked against the same whole-graph
  rules that the CompileGraph deployment gate enforces (the suspend/resume
  contract). Draft authoring allows partial models, but a runnable graph must
  honor these rules - a violation is reported as "Unable to run - <reason>"
  and the run is aborted.
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
`,Ct=`Display nodes that have been 'seen'
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
`,wt=`Session commands
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
`,Tt=`Tutorial 1
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
`,Et=`Tutorial 10
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
GET http://127.0.0.1:8085/api/mdm/profile/100, with [person_id], ttl=30000
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-10 \\
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
`,Dt=`Tutorial 11
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-11 \\
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
`,Ot=`Tutorial 12
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

This handler is GENERIC: it never names the failing node. When a failed node routes to its
"exception" handler, the engine stages the exception context in the state machine —
error.source (the failing node's alias), error.code (the status code), error.message and
error.stack when available — and every statement command resolves {dynamic variables}, so the
handler reads and jumps back through the context ('inspect error' shows it in a dry-run
session; the node alias "error" is reserved for this namespace).

The handler's statements run in order:

1. The first IF tests "{error.code}". It is good practice to test for exactly 200 so an
   unintended configuration error cannot slip through. On HTTP-200 the THEN branch jumps to the
   end node — a taken node-jump ends the statement list immediately. Otherwise the ELSE branch
   resolves to "next" and falls through to the following statements.
2. RESET comes **first among the action statements**: it clears the run-once guard and state of a
   comma-separated list of nodes so they can be executed again — here "{error.source}" (whichever
   node routed here) and the error-handler itself (a node may reset itself because the run-once
   mark is set before its statements execute). Placing RESET early guarantees it runs on every
   path — a later taken IF jump would skip it — and everything the node stores afterwards (such
   as the pending DELAY) survives the self-wipe. Keep RESET **after** any check that reads state
   it would wipe: the status IF above must run first, because RESET clears the failing node's
   "status" and an IF on a wiped variable aborts the run (the staged "error.*" context itself is
   not node state, so it survives).
3. The two MAPPING statements increment the retry counter "model.attempts" (\`f:defaultValue()\`
   seeds it to 0 on the first pass). The "model" namespace is not touched by RESET.
4. The second IF bounds the retry loop: after 3 attempts it jumps to the "clear-exception" node.
5. "NEXT: {error.source}" tells the traversal system to jump back to the failing node — whichever
   node routed here. Unlike a taken IF jump, NEXT does not stop the statement list — the jump is
   applied after the whole list completes.
6. "DELAY: 50" pauses for 50 milliseconds after this node completes, before the next retry. Pacing
   retries is a best practice: it avoids very rapid retries that can cause a "recovery storm" — an
   unintended denial-of-service attack on the target service. (A DELAY value may also be a dynamic
   variable, e.g. "DELAY: {model.backoff}" for a computed backoff.)

After the successful retry, the virtual "error" node reports the RECOVERY instead of the stale
failure: "inspect error" shows code=200 with the source kept and the failure details removed.
An empty context means nothing failed; a full context means an outstanding failure.

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]='''
IF: {error.code} == 200
THEN: end
ELSE: next
'''
statement[]=RESET: {error.source}, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: clear-exception
ELSE: next
'''
statement[]=NEXT: {error.source}
statement[]=DELAY: 50
\`\`\`

Create the clear-exception node
-------------------------------
In the clear-exception node, the RESET comes first (nothing before it reads node state), clearing
the failing node — again via the dynamic "{error.source}" reference, since the exception context
stays readable for the rest of the run — and the clear-exception node itself so that the system
can execute them again. You then set the variable "model.exception" to false so that the mock
service returns a normal response instead of an exception, and clear "model.attempts" to zero.

\`\`\`
create node clear-exception
with type Decision
with properties
skill=graph.math
statement[]=RESET: {error.source}, clear-exception
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-12 \\
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
`,kt=`Tutorial 13
-----------
In this session, you will create a graph model that invokes a composable function using the
"graph.task" skill. The composable function is the AsyncHttpClient (route "async.http.request")
provided by the platform-core module, turning the task node into an HTTP client by configuration.

Pre-requisite
-------------
You would need some working knowledge of composable functions. A composable function is a
TypedLambdaFunction registered with the PreLoad annotation. For more details, please refer to the
[Developer Guide](https://accenture.github.io/mercury-composable/).

What is a task?
---------------
A task is a node that invokes a composable function through its route name. MiniGraph is designed to be
zero-code with built-in skills for data mapping, decision-making and API fetching. More complex business
logic is delegated to a flow extension or a subgraph (tutorials 10 and 11). A task node sits in between -
it provides a lightweight method to extend a knowledge graph's capability with a small piece of business
logic, without writing a new skill.

In this tutorial, the "small piece of business logic" is not custom code at all - it is the framework's
own AsyncHttpClient. Any function registered in the platform is callable by route name, so the task node
can drive an HTTP call purely by configuration.

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
input[]=input.body.person_id -> model.person_id
input[]=text(http://127.0.0.1:\${rest.server.port:8080}) -> host
input[]=text(/api/mdm/profile/{model.person_id}) -> url
input[]=text(GET) -> method
input[]=text(application/json) -> headers.accept
input[]=text(5000) -> headers.x-ttl
output[]=result -> output.body
purpose=Invoke AsyncHttpClient with route 'async.http.request' to fetch a user profile
skill=graph.task
task=async.http.request
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

1. \`input.body.person_id -> model.person_id\` stages a variable in the graph's state machine
   (the \`model.\` namespace). It does not become part of the function's request - it is kept for
   later entries to reference.
2. \`text(/api/mdm/profile/{model.person_id}) -> url\` demonstrates a **dynamic variable**: the
   \`{model.person_id}\` reference inside the text constant is substituted with the model value
   staged by the earlier entry. This is the same idiom as Event Script's
   \`text(Bearer {model.token}) -> headers.Authorization\`.
3. \`text(http://127.0.0.1:\${rest.server.port:8080}) -> host\` demonstrates **environment variable
   substitution**. The \`\${name:default}\` reference is resolved by the configuration system when the
   model is loaded - at 'instantiate graph' for a dry-run and at deployment time for a deployed
   model - so both lanes behave the same. The authored model (and any export) keeps the \`\${...}\`
   placeholder, making the model portable across environments.
4. \`text(application/json) -> headers.accept\` declares the response type this client accepts.
   Always declare it instead of relying on an HTTP library's implicit default - with an explicit
   accept, the profile service replies with \`content-type: application/json\` and the
   AsyncHttpClient decodes the response body into a map.
5. \`text(5000) -> headers.x-ttl\` sets the **HTTP timeout** of the AsyncHttpClient. The graph's
   regular ttl propagation (a node's optional \`ttl\` property, else \`model.ttl\`) bounds only the
   event call to the composable function - it cannot reach inside a generic function, so the
   HTTP client would otherwise run on its own 30-second default. The X-TTL value is expressed
   in **milliseconds**. It also rides the wire as the \`X-TTL\` request header, so a downstream
   Mercury service adopts it as its processing deadline (end-to-end deadline propagation).
6. Any other RHS such as \`url\` and \`method\` is a composite key path in the function's request body.
   RHS \`*\` would map the LHS value as the whole request body, and \`header.{name}\` would set a
   request header of the function call.

About async.http.request
------------------------
The input data mapping above builds a map of key-values. The AsyncHttpRequest class in platform-core
renders that map into an HTTP request through its "fromMap" method at the function boundary. The
commonly used keys are:

\`\`\`
host              target host, e.g. http://127.0.0.1:8080
url               URI path, e.g. /api/mdm/profile/100
method            GET, POST, PUT, DELETE, etc.
headers.{name}    an HTTP request header
headers.x-ttl     HTTP timeout in milliseconds (default 30000); also propagates on the wire
body              the HTTP request body (for POST/PUT)
parameters.query.{name}   a query parameter
\`\`\`

The mock MDM profile service (GET /api/mdm/profile/{id}) is preloaded in dev mode, serving
person IDs 100 and 200.

Perform a dry-run
-----------------
To test the graph model, you can instantiate the graph with mock input as follows:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

Then enter 'run' to execute the graph.

\`\`\`
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to hello-task
Executed hello-task with skill graph.task in 18.5 ms
Walk to end
{
  "output": {
    "body": {
      "profile": {
        "id": "100",
        "name": "Peter",
        "address": "100 World Blvd"
      },
      "accounts": ["a101", "b202", "c303", "d400", "e500"],
      "observed_ttl": "5000"
    }
  }
}
Graph traversal completed in 21 ms
\`\`\`

Note that 'instantiate graph' resolved \`\${rest.server.port:8080}\` to the application's actual port,
and the task node resolved \`{model.person_id}\` to 100 before calling the HTTP endpoint. The
"observed_ttl" field is the mock service echoing the \`X-TTL\` request header it received - proof
that the 5000 ms deadline arrived on the wire.

You can also check the application log. Telemetry and tracing information are shown, proving that the
composable function was executed by the graph instance with full trace propagation.

\`\`\`
GraphTask:144 - Call task async.http.request, ttl=30000
Telemetry:81 - {trace={path=/graph/playground, service=graph.task...
Telemetry:81 - {trace={path=/graph/playground, service=async.http.request...
Telemetry:81 - {trace={path=/graph/playground, service=mock.mdm.profile...
\`\`\`

Error handling
--------------
If the composable function throws an exception (e.g. AppException with a status code) or the call times
out, the "error" and "status" parameters of the node are set. You can add an "exception" property to the
task node to route the error to a handler node, e.g. \`exception=on-error\`.

You can see this without any extra configuration - instantiate with an unknown person ID such as
\`int(999) -> input.body.person_id\` and the HTTP error from the profile service becomes the graph
output.

Iterative execution
-------------------
Like the API fetcher and the flow extension, a task node supports iterative fork-join execution with the
"for_each" and "concurrency" properties. Please enter 'describe skill graph.task' for details.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-13
Graph exported to /tmp/graph/tutorial-13.json
Described in /api/graph/model/tutorial-13/431-3
\`\`\`

The exported file keeps the \`\${rest.server.port:8080}\` placeholder, so the same model resolves to
the correct port in each environment it is deployed to.

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's \`main/resources/graph\`
folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-13 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
}'
\`\`\`

Summary
-------
In this session, we have discussed the use of the "graph.task" skill to invoke a composable function
through its route name, with Event Script style input and output data mapping. Along the way you used
a model variable as a dynamic variable in a later data mapping entry, and an environment variable
reference that resolves when the model is loaded.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without writing
any code, and flow extensions or subgraphs handle complex orchestration. A task node completes the
picture - any custom business logic can now be packaged as a composable function and plugged into a
graph as if it were a custom skill. As this tutorial shows, that includes functions the framework
already provides: the AsyncHttpClient became an HTTP client by configuration, with no code at all.
`,At=`Tutorial 14
-----------
In this session, you will build a purchase workflow with THREE human checkpoints - a customer
orders, the store manager approves (or rejects with a reason, which ends the workflow), the
delivery department releases the shipment, and the parcel ships to the customer. One graph
model, four short runs, one correlation ID.

When to use suspension
----------------------
Any multi-step process that must WAIT mid-way fits this pattern - waiting for a person (an
approval, missing information requested by email) or waiting for another system (a batch job
that takes hours and calls back when done). The workflow pauses instead of ending: its state
is kept in an external store under the business correlation ID - the ticket or order number -
and any application instance resumes it when the reply arrives.

Pre-requisite
-------------
Workflow suspension persists state to an external store through two composable functions. This
tutorial uses the Redis store from the "minigraph-state-redis" extension - the playground
application already includes it, so "v1.redis.persist.model" and "v1.redis.retrieve.model" are
registered automatically. Start a Redis before you run the graph (the "redis-standalone" helper
application works out of the box).

What is workflow suspension?
----------------------------
An approval may take minutes or days. Instead of parking a live graph instance, the graph
persists its workflow state - the "model" namespace - under the business correlation ID and the
run completes normally. A later request with the same correlation ID restores that state and
continues past the checkpoint without re-executing it. Three vocabulary pieces make this work:

1. the "suspend" node - a reserved node name (like root and end) with the "graph.suspend" skill.
   ONE suspend node serves every suspension point in the graph, reached two ways: a CHECKPOINT
   NODE - a working node with a DRAWN EDGE to it - pauses when its skill completes (the edge is
   the declaration, no property needed), and a DECISION NODE pauses by returning "suspend" from
   its IF-THEN-ELSE (the decision is RE-EXECUTED against the new input on every resume)
2. a checkpoint node - any working node (graph.data.mapper here; graph.task, graph.api.fetcher
   and graph.extension work the same way) that draws its checkpoint edge to "suspend" plus a
   continuation edge to the next step; a resumed run continues along the continuation and never
   re-executes the node. A checkpoint never decides - reaching it IS the decision to pause
3. the resume node - the "graph.resume" skill placed right after root; it restores a persisted
   record and continues at the LAST suspension point: past a checkpoint along its continuation,
   or by re-executing the decision that paused the workflow. A fresh transaction flows through -
   either way it sets "model.run" to "resume" or "fresh" so the graph's own logic can react

The graph navigation is:

\`\`\`
root -> resume -> order -> check-approval -> approval -> delivery -> ship -> end
              (checkpoint)     |      \\-> manager-reject -> end     (order, approval and
                               |                                     delivery draw edges
                               +--returns 'suspend' when no valid    to suspend)
                                  decision - and re-decides on
                                  every resume
\`\`\`

Each checkpoint node captures its actor's input into the model and suspends; each following
run resumes one checkpoint further. The model is the workflow's durable memory - anything a
later step needs must be mapped into "model.*" before the checkpoint. The manager's decision
lands at a graph.math decision node on the order checkpoint's continuation with THREE
outcomes: an approved decision routes to the next suspension point, an explicit rejection
routes to a terminal node that reports the manager's reason (the workflow ends), and anything
else - a missing or unrecognized decision - returns "suspend" to pause again, so an invalid
request can never end a long-running workflow by accident: the decision re-executes on the
next resume and re-evaluates whatever arrives.

Create the graph model
----------------------
Create the root node:

\`\`\`
create node root
with properties
purpose=Purchase workflow with three human checkpoints
name=tutorial-14
\`\`\`

Create the resume node. A resumed run jumps past its last checkpoint; a fresh transaction
(no record - never suspended, or expired) continues along the forward path into the
"check-fresh" validation gate with "model.run" set to "fresh":

\`\`\`
create node resume
with type Resume
with properties
purpose=Restore workflow state if this transaction was suspended earlier
skill=graph.resume
task=v1.redis.retrieve.model
\`\`\`

Create the input validation gate. The variable substitution inside the text() constant is
null-safe: when the request has no "item" field it is not an order submission, so a later-stage
request without a suspended record is rejected:

\`\`\`
create node check-fresh
with type Decision
with properties
purpose=A fresh transaction must be an order submission
skill=graph.math
statement[]=MAPPING: text(={input.body.item}) -> model.order_probe
statement[]='''
IF: {model.order_probe} == '=null'
THEN: reject
ELSE: order
'''
\`\`\`

Create the three checkpoint nodes. Each captures its actor's input into the model and stages a
stage-specific reply for the caller (overriding the default suspended response). No property is
needed: the drawn edge to the suspend node (you will connect it below) IS the suspension
declaration. A checkpoint node is a complete working node - it executes its skill in full and
may carry any non-routing skill (graph.data.mapper here; graph.task, graph.api.fetcher and
graph.extension work the same way) - only its exit changes. The "Suspensible" type is purely
visual - it picks the node color in the Playground:

\`\`\`
create node order
with type Suspensible
with properties
purpose=Capture the customer order, then suspend for the store manager
skill=graph.data.mapper
mapping[]=input.body -> model.order
mapping[]=text(order-submitted; waiting for store manager approval) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node approval
with type Suspensible
with properties
purpose=Capture the store manager approval, then suspend for the delivery department
skill=graph.data.mapper
mapping[]=input.body -> model.approval
mapping[]=text(approved; waiting for the delivery department to release the shipment) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node delivery
with type Suspensible
with properties
purpose=Capture the shipment release, then suspend for shipment confirmation
skill=graph.data.mapper
mapping[]=input.body -> model.delivery
mapping[]=text(released; waiting for shipment confirmation) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

Create the manager decision. It sits on the order checkpoint's continuation, so every resumed
run lands here with the manager's input. Three outcomes: "approved" continues to the approval
checkpoint, "rejected" ends the workflow with the manager's reason, and anything else RETURNS
"suspend" to pause - the decision pattern. A pausing decision draws NO edge to the suspend node
(its drawn edges are outcome alternatives, and the gate rejects a decision-to-suspend edge); it
is re-executed against the new request input on every resume, so the workflow simply keeps
waiting until an explicit "approved" or "rejected" arrives - a wait loop with no extra nodes.
The probe reuses the same null-safe idiom as "check-fresh". The awaiting reply is staged
unconditionally before the IFs; the approval and rejection paths overwrite it downstream:

\`\`\`
create node check-approval
with type Decision
with properties
purpose=Approved continues, rejected ends the workflow, anything else keeps waiting
skill=graph.math
statement[]=MAPPING: text(={input.body.decision}) -> model.approval_probe
statement[]=MAPPING: text(awaiting-decision; supply decision approved or rejected for the store manager) -> output.body.stage
statement[]=MAPPING: model.run -> output.body.run
statement[]=MAPPING: model.cid -> output.body.cid
statement[]='''
IF: {model.approval_probe} == '=approved'
THEN: approval
ELSE: next
'''
statement[]='''
IF: {model.approval_probe} == '=rejected'
THEN: manager-reject
ELSE: suspend
'''
\`\`\`

\`\`\`
create node manager-reject
with type mapper
with properties
purpose=The manager rejected the purchase: report the reason and end the workflow
skill=graph.data.mapper
mapping[]=text(rejected) -> output.body.stage
mapping[]=input.body.reason -> output.body.reason
mapping[]=model.order -> output.body.order
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

Create the completion, rejection, suspend and end nodes:

\`\`\`
create node ship
with type mapper
with properties
purpose=Ship to the customer with the full order history
skill=graph.data.mapper
mapping[]=text(shipped) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.order -> output.body.order
mapping[]=model.approval -> output.body.approval
mapping[]=model.delivery -> output.body.delivery
mapping[]=input.body -> output.body.shipment
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node reject
with type mapper
with properties
purpose=Reject a request that has no suspended transaction and is not an order
skill=graph.data.mapper
mapping[]=int(404) -> output.status
mapping[]=text(rejected) -> output.body.type
mapping[]=text(Transaction not found. Submit the order first) -> output.body.message
mapping[]=model.run -> output.body.run
\`\`\`

\`\`\`
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to Redis and wait for the next actor
skill=graph.suspend
task=v1.redis.persist.model
ttl=1h
\`\`\`

\`\`\`
create node end
\`\`\`

Connect the nodes. Every checkpoint node draws BOTH edges - the checkpoint edge to "suspend"
(the suspension declaration) and the continuation edge to the next step (where a resumed run
continues). The check-approval decision draws only its outcome edges - its waiting path is
the jump inside the IF-THEN-ELSE:

\`\`\`
connect root to resume with then
connect resume to check-fresh with fresh
connect check-fresh to order with submission
connect check-fresh to reject with no-transaction
connect order to suspend with checkpoint
connect order to check-approval with next
connect check-approval to approval with approved
connect check-approval to manager-reject with rejected
connect manager-reject to end with then
connect approval to suspend with checkpoint
connect approval to delivery with next
connect delivery to suspend with checkpoint
connect delivery to ship with next
connect ship to end with then
connect reject to end with then
connect suspend to end with then
\`\`\`

For your convenience, this graph model is preloaded as "tutorial-14".

Dry-run the workflow interactively
----------------------------------
You can exercise all three checkpoints without leaving the playground. Two things to
remember: instantiate before every run so each round starts with a fresh state machine
(on this engine 'run' may repeat on one instance and model values persist across runs -
see 'help run' - which would pollute a short-run simulation); and the SAME model.cid
must be supplied each time - it is the resume key. Redis must be running.

Import the deployed model as a draft:

\`\`\`
import graph from tutorial-14
\`\`\`

Run 1 - the customer orders a laptop:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(laptop) -> input.body.item
int(2000) -> input.body.amount
\`\`\`

\`\`\`
run
\`\`\`

The traversal walks root -> resume -> check-fresh -> order -> suspend -> end and the run
completes normally - the workflow state now lives in Redis, not in memory. Inspect the
staged reply:

\`\`\`
inspect output.body
\`\`\`

It shows stage=order-submitted..., run=fresh (a new transaction) and cid=order-1001.

Run 2 - the store manager approves. Instantiate again with the same correlation ID and
the manager's input:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(approved) -> input.body.decision
text(store-88) -> input.body.manager
\`\`\`

\`\`\`
run
\`\`\`

Watch the console: the resume node restores the persisted state and the traversal
continues at the check-approval decision - the order checkpoint is NOT re-executed. The
approved decision routes to the approval checkpoint. Now "inspect output.body" shows
stage=approved... and run=resume, and the "seen" command lists the order node as visited
even though this run never executed it - that is the restored traversal bookkeeping.

(The manager could reject instead: the same run with
"text(rejected) -> input.body.decision" and "text(budget exceeded) -> input.body.reason"
routes to manager-reject - the reply carries stage=rejected with the reason and the
original order, and the workflow ends. And if the run carries no valid decision at all -
including a replay against a leftover suspended record from an earlier exercise - the
workflow does NOT end: it replies stage=awaiting-decision and re-suspends, waiting for a
proper "approved" or "rejected". You will try both over REST below. Tip: records are
consumed on resume and re-created on each suspension, so if you repeat these exercises,
use a fresh correlation ID for each clean start.)

Run 3 - the delivery department releases the shipment:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
boolean(true) -> input.body.release
text(express) -> input.body.courier
\`\`\`

\`\`\`
run
\`\`\`

Run 4 - shipment confirmation completes the workflow:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(TRK-12345) -> input.body.tracking
\`\`\`

\`\`\`
run
\`\`\`

Inspect the final reply - the model accumulated state across all four short runs:

\`\`\`
inspect output.body
\`\`\`

It shows stage=shipped, run=resume, and the full history: order (laptop/2000), approval
(approved/store-88), delivery (release/express) and shipment (TRK-12345).

To see the input validation, start over with a correlation ID that never ordered:

\`\`\`
instantiate graph
text(order-9999) -> model.cid
text(approved) -> input.body.decision
\`\`\`

\`\`\`
run
\`\`\`

"inspect output" shows status=404 with type=rejected and run=fresh - the order must come
first, and the run flag tells the caller why. Each record is consumed on resume, so
repeating any middle run behaves the same way: no record means a fresh transaction.

Test the workflow over REST
---------------------------
Run 1 - the customer orders a laptop:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"item": "laptop", "amount": 2000}'
\`\`\`

The reply is {"stage": "order-submitted; waiting for store manager approval", "run": "fresh",
"cid": "order-1001"} and the run is over - nothing stays in memory. Every stage reply carries
the "run" flag ("fresh" on run 1, "resume" on runs 2 to 4) so the caller always knows whether
it is looking at a new transaction or a resumed continuation. Run 2 - the store manager approves:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"decision": "approved", "manager": "store-88"}'
\`\`\`

Run 3 - the delivery department releases the shipment:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"release": true, "courier": "express"}'
\`\`\`

Run 4 - shipment confirmation completes the workflow:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"tracking": "TRK-12345"}'
\`\`\`

The final reply carries the whole history - the order from run 1, the approval from run 2, the
release from run 3 and the shipment from run 4 - proof that the workflow state crossed every
suspension. Now try a decision with a correlation ID that never ordered:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-9999" \\
  -d '{"decision": "approved"}'
\`\`\`

The workflow rejects it with HTTP-404 - the order must come first - and the reply's
"run": "fresh" tells the UI why: the record expired or never existed. Each record is consumed on
resume, so a duplicated request at any stage behaves like a fresh transaction instead of
executing that stage twice.

Finally, try the manager's other option - reject with a reason. Submit a new order, then
reject it:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-2002" \\
  -d '{"item": "monitor", "amount": 300}'
\`\`\`

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-2002" \\
  -d '{"decision": "rejected", "reason": "budget exceeded"}'
\`\`\`

The reply is {"stage": "rejected", "reason": "budget exceeded", "order": {...}, "run": "resume",
"cid": "order-2002"} and the workflow is over - the record was consumed on resume and nothing
re-suspended, so any further request under order-2002 is a fresh 404 rejection.

An invalid or missing decision behaves differently - the workflow stays alive. Submit another
order under order-3003, then send a request with no decision:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-3003" \\
  -d '{"note": "no decision here"}'
\`\`\`

The reply is {"stage": "awaiting-decision; supply decision approved or rejected for the store
manager", "run": "resume", "cid": "order-3003"} and the workflow re-suspended - repeat with
{"decision": "approved"} and it continues to the delivery stage as usual. Only an explicit
"approved" or "rejected" moves the workflow forward.

Summary
-------
In this session, we expressed a purchase workflow with three human checkpoints as four short
graph runs keyed by one business correlation ID: one reserved "suspend" node served every
checkpoint, each checkpoint node declared its suspension with a drawn edge, captured its
actor's input into the model and staged its own stage response, a graph.math decision at the
manager's resumption point routed an approval to the next checkpoint, a rejection (with the
manager's reason) to the end, and anything else returned "suspend" to pause again (a pausing
decision re-executes and re-decides on every resume, so no extra wait nodes are needed),
input validation enforced the order-before-decision sequence, and the engine-managed
"model.run" flag told every reply whether the run was fresh or resumed.

Why suspend and resume?
-----------------------
Real business processes wait on people - repeatedly. Suspension turns each wait into a durable
record instead of a parked runtime: any application instance sharing the state store can resume
the workflow, restarts lose nothing, and each run stays short and observable. The state store is
pluggable - Redis is the packaged implementation, and any composable function honoring the
documented store contract can replace it.
`,jt=`Tutorial 2
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

The temp graph folder and the graph manifest are set in the application configuration
file (application.properties or application.yml):

\`\`\`properties
#
# temp graph working location
# (temp graph location must use "file:/" prefix because of READ/WRITE requirements)
#
location.graph.temp=file:/tmp/graph
#
# the graph manifest - the quality gate and the only door to deployed execution
#
graph.model.automation=classpath:/graphs.yaml
\`\`\`

The deployed graph folder is declared in the graph manifest itself - like flows.yaml, the
manifest carries the location of its own models. Add your graph ID to the manifest so the
CompileGraph quality gate validates it at startup; only graphs that pass become executable:

\`\`\`yaml
graphs:
  - 'tutorial-1'

location: 'classpath:/graph'
\`\`\`

The 'location' entry is optional (default 'classpath:/graph'; a read-only folder, so
'file:/' or 'classpath:/' both work).

Invoke the graph API REST endpoint
----------------------------------
The generic graph API endpoint is \`POST /api/graph/{graph_id}\`, where 'graph_id' is
the name of the graph model.

To make a request to the 'tutorial-1' graph model, enter the following curl command.

\`\`\`
> curl -X POST http://127.0.0.1:8085/api/graph/tutorial-1
hello world
\`\`\`

It returns 'hello world'.

Since the "hello world" graph model does not require any input parameter, you can also
use HTTP GET to execute the graph.

\`\`\`
> curl http://127.0.0.1:8085/api/graph/tutorial-1
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-2 \\
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
`,Mt=`Tutorial 3
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-3 \\
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
one HTTP call to \`http://127.0.0.1:8085/api/mdm/profile/10\`.

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
`,Nt=`Tutorial 4
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
`,Pt=`Tutorial 5
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
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8085/api/mdm/profile/100, with ["person_id"], ttl=30000
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8085/api/mdm/profile/200, with ["person_id"], ttl=30000
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-5 \\
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
`,Ft=`Tutorial 6
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-6 \\
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
`,It=`Tutorial 7
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-7 \\
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
`,Lt=`Tutorial 8
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-8 \\
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
`,Rt=`Tutorial 9
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
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-9 \\
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
`,zt=`Update a node
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
`,Bt=`Upload mock data
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
`,Vt=`MiniGraph
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

Keyboard shortcuts
------------------
- \`Ctrl + M\` - Toggle the minimap while viewing the Graph tab
- \`Ctrl + backtick\` - Toggle the Help panel

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
9. graph.suspend - persist workflow state at a suspension point (the reserved 'suspend' node)
10. graph.resume - restore workflow state and continue past the suspension point

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
- help tutorial 14 (workflow suspension - a purchase workflow with three human checkpoints)
`,Ht=[`# JSON-Path Playground Overview`,``,`Use the JSON-Path Playground to load a JSON or XML document and evaluate expressions against it.`,`This starter Overview is intentionally the only section for now and can grow into a fuller guide later.`,``,`## Quick start`,``,`1. Click **Start** to connect to the JSON-Path playground.`,`2. Paste a document into the Payload Editor, or choose a Quick load sample.`,"3. Enter `load` to send the document to the active session.","4. Enter a JSONPath expression beginning with `$`, such as `$.response.user.name`.",``,`## Example expressions`,``,`| Expression | Purpose |`,`| --- | --- |`,"| `$.response` | Read the loaded response object |","| `$.response.items[*]` | Select every item in an array |","| `$.response.items[0].name` | Read a field from the first item |",``,`## Help controls`,``,"- `Ctrl + backtick` - Toggle the Help panel",`- Use the maximize button to expand Help and the close button to return to the editor`].join(`
`),Ut=Object.assign({"../../../resources/help/help connect.md":tt,"../../../resources/help/help create.md":nt,"../../../resources/help/help data-dictionary.md":rt,"../../../resources/help/help delete.md":it,"../../../resources/help/help describe.md":at,"../../../resources/help/help edit.md":ot,"../../../resources/help/help execute.md":st,"../../../resources/help/help export.md":ct,"../../../resources/help/help graph-api-fetcher.md":lt,"../../../resources/help/help graph-data-mapper.md":I,"../../../resources/help/help graph-extension.md":ut,"../../../resources/help/help graph-island.md":dt,"../../../resources/help/help graph-join.md":ft,"../../../resources/help/help graph-js.md":pt,"../../../resources/help/help graph-math.md":mt,"../../../resources/help/help graph-resume.md":ht,"../../../resources/help/help graph-suspend.md":gt,"../../../resources/help/help graph-task.md":_t,"../../../resources/help/help import.md":vt,"../../../resources/help/help inspect.md":yt,"../../../resources/help/help instantiate.md":bt,"../../../resources/help/help list.md":xt,"../../../resources/help/help run.md":St,"../../../resources/help/help seen.md":Ct,"../../../resources/help/help session.md":wt,"../../../resources/help/help tutorial 1.md":Tt,"../../../resources/help/help tutorial 10.md":Et,"../../../resources/help/help tutorial 11.md":Dt,"../../../resources/help/help tutorial 12.md":Ot,"../../../resources/help/help tutorial 13.md":kt,"../../../resources/help/help tutorial 14.md":At,"../../../resources/help/help tutorial 2.md":jt,"../../../resources/help/help tutorial 3.md":Mt,"../../../resources/help/help tutorial 4.md":Nt,"../../../resources/help/help tutorial 5.md":Pt,"../../../resources/help/help tutorial 6.md":Ft,"../../../resources/help/help tutorial 7.md":It,"../../../resources/help/help tutorial 8.md":Lt,"../../../resources/help/help tutorial 9.md":Rt,"../../../resources/help/help update.md":zt,"../../../resources/help/help upload.md":Bt,"../../../resources/help/help.md":Vt});function Wt(e){let t=e.split(`/`);return(t[t.length-1]??e).replace(/\.md$/,``)}var Gt=Object.fromEntries(Object.entries(Ut).map(([e,t])=>[Wt(e),t])),Kt={help:Ht};function qt(e){return e===`json-path`?Kt:Gt}function Jt(e,t=`minigraph`){let n=e===``?`help`:`help ${e}`;return qt(t)[n]??null}var Yt=Object.keys(Gt).filter(e=>e!==`help`).map(e=>e.replace(/^help\s+/,``)).sort(),Xt=[{id:`overview`,label:`Overview`},{id:`graph-model`,label:`Graph Model`},{id:`graph-skills`,label:`Graph Skills`},{id:`instance-model`,label:`Instance Model`},{id:`tutorials`,label:`Tutorials`,chipStripLabel:`Chapters`}],Zt=[{id:`overview`,label:`Overview`}];function Qt(e=`minigraph`){return e===`json-path`?Zt:Xt}var $t=new Set([`execute`,`inspect`,`instantiate`,`run`,`seen`,`upload`]);function en(e,t=`minigraph`){return t===`json-path`||e===``?`overview`:e.startsWith(`tutorial `)?`tutorials`:e.startsWith(`graph-`)?`graph-skills`:$t.has(e)?`instance-model`:`graph-model`}function tn(e,t=`minigraph`){if(e===`overview`)return[``];if(t===`json-path`)return[];let n=Yt.filter(n=>en(n,t)===e);return e===`tutorials`?[...n].sort((e,t)=>parseInt(e.replace(/^tutorial\s+/,``),10)-parseInt(t.replace(/^tutorial\s+/,``),10)):n}function nn(e,t){return e===``?`Overview`:t===`tutorials`?e.replace(/^tutorial\s+/,``):e}function rn(e=`minigraph`){return Qt(e).flatMap(t=>tn(t.id,e))}rn();function an(e){return e.replace(/^help\s*/i,``).trim().toLowerCase()}function on({bus:e,setHelpTopic:t,onTabSwitch:n,enabled:r=!0,contentProfile:i=`minigraph`}){let a=(0,A.useRef)(n);(0,A.useEffect)(()=>{a.current=n}),(0,A.useEffect)(()=>{if(r)return e.on(`command.helpOrDescribe`,e=>{if(!e.commandText.trim().toLowerCase().startsWith(`help`))return;let n=an(e.commandText);Jt(n,i)!==null&&(t(n),a.current())})},[e,t,r,i])}function sn({ctx:e,navigate:t,addToast:n,wsPath:r}){let i=me.find(e=>e.tabs.includes(`payload`)&&e.supportsUpload),a=(0,A.useRef)(null),o=i?.wsPath;(0,A.useEffect)(()=>{if(!(!o||!a.current)&&e.getSlot(o).phase===`connected`){let{wsPath:r,json:o}=a.current;a.current=null,e.setPendingPayload(r,o),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)}},[o,e,t,n,i]);let s=(0,A.useCallback)(r=>{if(!i)return;let o=e.getSlot(i.wsPath);o.phase===`connected`?(e.setPendingPayload(i.wsPath,r),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)):o.phase===`connecting`?(a.current={wsPath:i.wsPath,json:r},n(`Updated pending JSON transfer — latest payload will open when connected`,`info`)):(a.current={wsPath:i.wsPath,json:r},e.connect(i.wsPath,n),n(`Connecting to JSON-Path Playground…`,`info`))},[e,t,n,i]);return{handleSendToJsonPath:i&&r!==i.wsPath?s:void 0}}function cn({bus:e,onOpenPanel:t}){(0,A.useEffect)(()=>e.on(`upload.invitation`,e=>{t(e.uploadPath)}),[e,t])}function ln({bus:e,addToast:t}){let[n,r]=(0,A.useState)(null),i=(0,A.useRef)(null),a=(0,A.useRef)([]),o=(0,A.useRef)(null),[s,c]=(0,A.useState)(new Set),l=(0,A.useCallback)(e=>{let t=i.current;if(t!==null){t!==e&&!a.current.includes(e)&&a.current.push(e);return}o.current=document.activeElement,i.current=e,r(e)},[]),u=(0,A.useCallback)(()=>{let e=a.current.shift()??null;i.current=e,r(e),e===null&&setTimeout(()=>o.current?.focus(),0)},[]),d=(0,A.useCallback)(()=>{u()},[u]),f=(0,A.useCallback)(e=>{if(i.current===e)return u(),!0;let t=a.current.indexOf(e);return t===-1?!1:(a.current.splice(t,1),!0)},[u]),p=(0,A.useCallback)(e=>{let n=i.current;n&&c(e=>new Set([...e,n])),u(),t(`Mock data uploaded successfully ✓`,`success`)},[t,u]),m=(0,A.useCallback)(e=>{t(`Upload failed: ${e}`,`error`)},[t]),h=(0,A.useCallback)(()=>{c(new Set)},[]);return cn({bus:e,onOpenPanel:l}),{uploadPanelPath:n,successfulUploadPaths:s,handleOpenUploadPanel:l,handleCloseUploadPanel:d,handleCloseUploadPath:f,handleUploadSuccess:p,handleUploadError:m,resetSuccessfulPaths:h}}function un({bus:e,connected:t,appendMessage:n,addToast:r}){let i=(0,A.useRef)(null),a=(0,A.useRef)(!1),o=(0,A.useRef)(n);(0,A.useEffect)(()=>{o.current=n},[n]);let s=(0,A.useRef)(r);(0,A.useEffect)(()=>{s.current=r},[r]),(0,A.useEffect)(()=>{t||(i.current?.abort(),i.current=null,a.current=!1)},[t]),(0,A.useEffect)(()=>()=>{i.current?.abort()},[]),(0,A.useEffect)(()=>e.on(`payload.large`,e=>{if(a.current)return;let{apiPath:t,byteSize:n}=e;i.current?.abort();let r=new AbortController;i.current=r;let c=(n/(1024*1024)).toFixed(2);s.current(`Fetching large payload (${c} MB)…`,`info`),a.current=!0,fetch(t,{signal:r.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.text()}).then(e=>{if(!e.trim())throw Error(`empty response body`);let t=e;try{t=JSON.stringify(JSON.parse(e),null,2)}catch{}o.current(t),a.current=!1,i.current=null}).catch(e=>{e.name!==`AbortError`&&(a.current=!1,i.current=null,o.current(`ERROR: payload fetch failed — ${e.message}`),s.current(`Payload fetch failed: ${e.message}`,`error`))})}),[e])}function dn(e){let[t,n]=fe(e,{}),r=(0,A.useCallback)(e=>{n(t=>({...t,[e]:{name:e,savedAt:new Date().toISOString()}}))},[n]),i=(0,A.useCallback)(e=>{n(t=>{let n={...t};return delete n[e],n})},[n]),a=(0,A.useCallback)(e=>Object.prototype.hasOwnProperty.call(t,e),[t]);return{savedGraphs:(0,A.useMemo)(()=>Object.values(t).sort((e,t)=>new Date(t.savedAt).getTime()-new Date(e.savedAt).getTime()),[t]),saveGraph:r,deleteGraph:i,hasGraph:a}}var fn={importedName:null,lastSavedName:null,isSaved:!1,untitledSlotConsumed:!1};function pn(e,t){switch(t.type){case`imported`:return{...e,importedName:t.name,lastSavedName:null,isSaved:!1};case`exported`:return{...e,lastSavedName:t.name,isSaved:!0,untitledSlotConsumed:e.untitledSlotConsumed||t.consumesUntitled};case`dirty`:return{...e,isSaved:!1};case`reset`:return{...fn}}}var mn=new Map;function hn(e,t,n){return t&&n!==null&&e===n}function gn(e,t){let n=e.on(`command.importGraph`,e=>{t.onImported(e.graphName)}),r=e.on(`graph.exported`,e=>{t.onExported(e.graphName)}),i=e.on(`graph.mutation`,()=>{t.onDirty()}),a=e.on(`session.reset`,()=>{t.onReset()});return()=>{n(),r(),i(),a()}}function _n(e,t,n,r){let[i,a]=fe(e,1),[o,s]=(0,A.useState)(()=>{let t=mn.get(e);return t&&hn(t.connectionEpoch,n,r)?t.state:{...fn}}),c=(0,A.useRef)(o),l=(0,A.useCallback)(t=>{let n=pn(c.current,t);c.current=n,t.type===`reset`?mn.delete(e):r!==null&&mn.set(e,{connectionEpoch:r,state:n}),s(n)},[r,e]),u=(0,A.useCallback)(e=>{l({type:`exported`,name:e,consumesUntitled:e===`untitled-${i}`})},[i,l]),d=(0,A.useCallback)(()=>{let t=mn.get(e)?.state;(c.current.untitledSlotConsumed||t?.untitledSlotConsumed)&&a(e=>e+1),l({type:`reset`})},[a,e,l]),f=(0,A.useRef)(r);return(0,A.useEffect)(()=>{let t=mn.get(e)?.connectionEpoch;(!n||f.current!==r||t!==void 0&&t!==r)&&d(),f.current=r},[n,r,d,e]),(0,A.useEffect)(()=>gn(t,{onImported:e=>l({type:`imported`,name:e}),onExported:u,onDirty:()=>l({type:`dirty`}),onReset:d}),[t,d,u,l]),{defaultName:o.lastSavedName??o.importedName??`untitled-${i}`,savedName:o.isSaved?o.lastSavedName:null,resetName:d}}var vn=new Set([`description`,`question`,`purpose`]),yn=`input.body`;function bn(e){return/[A-Za-z0-9_.]/.test(e)}function xn(e){return/[A-Za-z0-9_.*\[\]-]/.test(e)}function Sn(e,t){let n=0;for(;n<e.length;){let r=e.indexOf(yn,n);if(r===-1)return;let i=r>0?e[r-1]:``,a=r>1?e[r-2]:``,o=e[r+10]??``;if(i&&bn(i)&&!(i===`.`&&a===`$`)||o&&/[A-Za-z0-9_]/.test(o)){n=r+10;continue}let s=r+10;for(;s<e.length&&xn(e[s]);)s+=1;let c=e.slice(r,s);for(;c.endsWith(`.`)||c.endsWith(`-`)||c.endsWith(`[`);)c=c.slice(0,-1);for(;c.endsWith(`]`)&&!c.includes(`[`);)c=c.slice(0,-1);t.add(c),n=Math.max(s,r+10)}}function Cn(e,t,n){if(!(n&&vn.has(n.toLowerCase()))){if(typeof e==`string`){Sn(e,t);return}if(Array.isArray(e)){for(let n of e)Cn(n,t);return}if(typeof e==`object`&&e)for(let[n,r]of Object.entries(e))Cn(r,t,n)}}function wn(e){if(!e)return[];let t=new Set;for(let n of e.nodes)Cn(n.properties,t);return Array.from(t).sort()}var Tn={instantiate:`instantiate graph`,requestInputUpload:`upload mock data`,run:`run`},En=/^Graph instance created\. Loaded (\d+) mock (?:entry|entries), model\.ttl = (\d+) ms$/,Dn=/^Graph traversal completed in (\d+) ms$/,On=/^ERROR:\s*(.+)$/;function kn(e){let t=e.match(En);return t?{mockEntries:Number.parseInt(t[1],10),ttlMs:Number.parseInt(t[2],10)}:null}function An(e){return e===`Graph instance cleared`}function jn(e){let t=e.match(Dn);return t?{status:`completed`,elapsedMs:Number.parseInt(t[1],10)}:e===`Graph traversal aborted`?{status:`aborted`,elapsedMs:null}:null}function Mn(e){return e.match(On)?.[1]?.trim()||null}function Nn(e){let t=e.trim().toLowerCase();return t===Tn.instantiate||t===`${Tn.instantiate}...`}function Pn(e){return e.trim().toLowerCase()===Tn.run}var Fn=1e4,In={phase:`idle`,intent:null,pendingSignal:null,invalidated:!1};function Ln({enabled:e,bus:t,connected:n,connectionEpoch:r,graphData:i,graphIdentity:a,isPrimary:o,sendRawText:s,addToast:c,onWorkflowInputInvalidated:l}){let[u,d]=(0,A.useState)(In),f=(0,A.useRef)(In),p=(0,A.useRef)(null),m=(0,A.useRef)(s),h=(0,A.useRef)(c),g=(0,A.useRef)(l),_=(0,A.useMemo)(()=>wn(i),[i]),v=(0,A.useRef)(_);(0,A.useEffect)(()=>{m.current=s},[s]),(0,A.useEffect)(()=>{h.current=c},[c]),(0,A.useEffect)(()=>{g.current=l},[l]),(0,A.useEffect)(()=>{v.current=_},[_]);let y=(0,A.useCallback)(e=>{f.current=e,d(e)},[]),b=(0,A.useCallback)(()=>{p.current=null,y({...In})},[y]),x=(0,A.useCallback)(()=>{let e=p.current;p.current=null,e&&g.current?.(e)},[]),S=(0,A.useCallback)(()=>{x(),b()},[x,b]),C=(0,A.useCallback)(()=>{let e=f.current;if(x(),e.pendingSignal!==null){y({...e,phase:`outcome-uncertain`,invalidated:!0});return}b()},[x,b,y]),w=(0,A.useCallback)((e,t,n)=>(y(t),m.current(e)?!0:(b(),h.current(n,`error`),!1)),[b,y]),ee=e&&n&&i!==null&&o,te=ee&&(u.phase===`idle`||u.phase===`ready`&&_.length>0),T=ee&&u.phase===`ready`,E=e?i?n?o?``:`Run the graph from the host session`:`Connect first to run the graph`:`Load a graph first`:`Graph run controls are unavailable`,ne=(0,A.useCallback)(()=>!T||f.current.phase!==`ready`?!1:w(Tn.run,{phase:`running`,intent:null,pendingSignal:`run-terminal`,invalidated:!1},`Could not run graph because the WebSocket is not open.`),[T,w]),D=(0,A.useCallback)(()=>{if(!te)return!1;let e=f.current;return e.phase!==`idle`&&e.phase!==`ready`?!1:w(Tn.instantiate,{phase:`instantiating`,intent:`instantiate-only`,pendingSignal:`instance-created`,invalidated:!1},`Could not instantiate graph because the WebSocket is not open.`)},[te,w]),re=(0,A.useCallback)(e=>{let t=f.current;return t.phase!==`awaiting-input`&&t.phase!==`requesting-input`||t.intent===null||p.current!==e?!1:(p.current=null,y({phase:`ready`,intent:null,pendingSignal:null,invalidated:!1}),h.current(`Graph instantiated and ready to run.`,`success`),!0)},[y]),ie=(0,A.useCallback)(e=>{let t=f.current;return t.phase!==`awaiting-input`&&t.phase!==`requesting-input`||t.intent===null||p.current!==e?!1:(b(),h.current(`Graph instantiation cancelled.`,`info`),!0)},[b]);(0,A.useEffect)(()=>{let e=t.on(`graph.instance.created`,()=>{let e=f.current;if(e.pendingSignal===`instance-created`){if(e.invalidated){b();return}if(e.intent===null){y({phase:`ready`,intent:null,pendingSignal:null,invalidated:!1});return}v.current.length>0?w(Tn.requestInputUpload,{phase:`requesting-input`,intent:e.intent,pendingSignal:`upload-invitation`,invalidated:!1},`Graph was instantiated, but the input upload could not be requested.`):(y({phase:`ready`,intent:null,pendingSignal:null,invalidated:!1}),h.current(`Graph instantiated and ready to run.`,`success`))}}),n=t.on(`graph.instance.cleared`,S),r=t.on(`graph.mutation`,C),i=t.on(`session.reset`,S),a=t.on(`graph.exported`,C),o=t.on(`upload.invitation`,e=>{let t=f.current;if(t.pendingSignal===`upload-invitation`){if(t.invalidated){g.current?.(e.uploadPath),b();return}t.intent!==null&&(p.current=e.uploadPath,y({phase:`awaiting-input`,intent:t.intent,pendingSignal:null,invalidated:!1}))}}),s=t.on(`graph.run.terminal`,e=>{let t=f.current;if(t.pendingSignal!==`run-terminal`)return;let n=!t.invalidated&&e.status===`aborted`;b(),n&&h.current(`Graph run aborted. See the console for details.`,`error`)}),c=t.on(`command.error`,e=>{let t=f.current;if(t.pendingSignal===`instance-created`){let n=!t.invalidated;b(),n&&h.current(`Could not instantiate graph: ${e.message}`,`error`)}else if(t.pendingSignal===`upload-invitation`){let n=!t.invalidated;b(),n&&h.current(`Could not request graph input: ${e.message}`,`error`)}}),l=t.on(`command.echo`,e=>{let t=f.current;Nn(e.commandText)?(t.phase===`idle`||t.phase===`ready`)&&y({phase:`instantiating`,intent:null,pendingSignal:`instance-created`,invalidated:!1}):Pn(e.commandText)&&(t.phase===`idle`||t.phase===`ready`)&&y({phase:`running`,intent:null,pendingSignal:`run-terminal`,invalidated:!1})});return()=>{e(),n(),r(),i(),a(),o(),s(),c(),l()}},[t,S,C,b,w,y]),(0,A.useEffect)(()=>{if(u.phase!==`instantiating`&&u.phase!==`requesting-input`)return;let e=setTimeout(()=>{let e=f.current;(e.phase===`instantiating`||e.phase===`requesting-input`)&&(y({...e,phase:`outcome-uncertain`}),h.current(`Graph setup is taking longer than expected. Waiting for the backend outcome…`,`info`))},Fn);return()=>clearTimeout(e)},[u.phase,y]);let ae=(0,A.useRef)(a);(0,A.useEffect)(()=>{ae.current!==a&&C(),ae.current=a},[a,C]);let oe=(0,A.useRef)(r);(0,A.useEffect)(()=>{oe.current!==r&&S(),oe.current=r},[r,S]),(0,A.useEffect)(()=>{!e||!n||!o?S():i||C()},[e,n,i,S,C,o]);let O=u.phase!==`idle`&&u.phase!==`ready`;return{phase:u.phase,ready:u.phase===`ready`,busy:O,canInteract:ee,canInstantiate:te,canRun:T,disabledReason:E,inputBodyPaths:_,inputIntent:u.intent,workflowUploadPath:p.current,isWorkflowInputPanel:u.intent!==null&&(u.phase===`requesting-input`||u.phase===`awaiting-input`),runGraph:ne,instantiateGraph:D,handleInputUploadSuccess:re,handleInputCancelled:ie}}function Rn(e,t){return e.on(`graph.exported`,e=>t(e.graphName))}function zn({bus:e,connected:t,sendRawText:n,saveGraph:r,addToast:i}){let a=(0,A.useRef)(null),o=(0,A.useCallback)(e=>{if(!t){i(`Save failed: connection required to export graph`,`error`);return}let r=setTimeout(()=>{a.current!==null&&(a.current=null,i(`Save failed: export confirmation timed out`,`error`))},1e4);a.current={graphName:e,timeoutId:r},n(`export graph as ${e}`)},[t,n,i]);return(0,A.useEffect)(()=>{if(r!==null)return Rn(e,r)},[e,r]),(0,A.useEffect)(()=>e.on(`graph.exported`,e=>{if(a.current===null||e.graphName!==a.current.graphName)return;clearTimeout(a.current.timeoutId);let t=a.current.graphName;a.current=null,i(`Graph saved as "${t}"`,`success`)}),[e,i]),(0,A.useEffect)(()=>e.on(`graph.export.failed`,e=>{a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,e.reason===`invalid-name`?i(`Save failed: invalid filename (a–z, A–Z, 0–9, hyphen only)`,`error`):i(`Save failed: root node name does not match existing graph`,`error`))}),[e,i]),(0,A.useEffect)(()=>{!t&&a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,i(`Save failed: connection closed before export confirmation`,`error`))},[t,i]),(0,A.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current.timeoutId)},[]),{handleSaveGraph:o,handleLoadGraph:(0,A.useCallback)(e=>{t&&(n(`import graph from ${e}`),i(`Importing graph "${e}"…`,`info`))},[t,n,i])}}var Bn=new Map;function Vn(e){let[t,n]=(0,A.useState)(()=>Bn.get(e)??null);return[t,(0,A.useCallback)(t=>{n(t),t===null?Bn.delete(e):Bn.set(e,t)},[e])]}function Hn(e){if(e==null)return``;let t=typeof e==`string`?e:JSON.stringify(e);return t.includes(`'''`)&&console.warn(`[commandBuilder] Property value contains "'''" which cannot be escaped in the backend grammar. The value may be truncated on paste.`),t.includes(`
`)?`'''\n${t}\n'''`:t}function Un(e,t){let n=[`${e} node ${t.alias}`];t.types.length>0&&n.push(`with type ${t.types[0]}`);let r=Object.entries(t.properties).filter(([,e])=>e!=null);if(r.length>0){n.push(`with properties`);for(let[e,t]of r)if(Array.isArray(t))for(let r of t)n.push(`${e}[]=${Hn(r)}`);else n.push(`${e}[]=${Hn(t)}`)}return n.join(`
`)}function Wn(e,t){let n=t?.nodes.some(t=>t.alias===e.node.alias)?`update`:`create`;return{verb:n,command:Un(n,e.node)}}function Gn(e){return`${e} ${e===1?`node`:`nodes`} clipped to workspace`}function Kn(e){let{added:t,duplicates:n,failed:r}=e;if(t===0&&n===0&&r===0)return{message:`No selected nodes are available to clip.`,type:`info`};if(t===0&&n>0&&r===0)return{message:`All selected nodes already exist in workspace.`,type:`info`};if(t===0&&n===0&&r>0)return{message:`Failed to clip selected nodes to workspace.`,type:`error`};let i=Gn(t);return n>0&&(i+=`. ${n} already existed.`),r>0&&(i+=`. ${r} failed.`),{message:i,type:r>0?`error`:`success`}}function qn(e){return{execute(t){return e(t)}}}var Jn=/^[A-Za-z0-9_-]+$/,Yn=/^[A-Za-z0-9_-]+(?:\[(?:0|[1-9]\d*)?\])*$/,Xn=new Set([`input`,`output`,`model`,`response`,`result`,`parameter`,`none`,`next`,`api`,`error`]);function Zn(e,t){return`properties.${e}.${t}`}function Qn(e){return e.split(`.`).every(e=>Yn.test(e))}function $n(e){return Qn(e)}function er(e,t={}){let n={},r=t.mode??`create`,i=e.alias.trim(),a=t.originalAlias?.trim()??``,o=e.nodeType.trim();r===`edit`?a?Jn.test(a)||(n.alias=`Use only letters, numbers, underscore, and hyphen.`):n.alias=`Original alias is required.`:i?Jn.test(i)?Xn.has(i.toLowerCase())?n.alias=`"${i}" is reserved.`:t.graphData?.nodes.some(e=>e.alias.toLowerCase()===i.toLowerCase())&&(n.alias=`Node "${i}" already exists in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,o&&!Jn.test(o)&&(n.nodeType=`Use only letters, numbers, underscore, and hyphen.`);for(let t of e.properties){let e=t.key.trim(),r=t.value.trim();!e&&!r||(!e&&r?n[Zn(t.id,`key`)]=`Property key is required when value is present.`:$n(e)||(n[Zn(t.id,`key`)]=`Use a property name or dot/bracket path, for example mapping[] or config.value.`),r.includes(`'''`)&&(n[Zn(t.id,`value`)]=`Property value cannot contain '''.`))}return{valid:Object.keys(n).length===0,errors:n}}function tr(e,t={}){let n={},r=e.trim();return r?Jn.test(r)?t.graphData&&!t.graphData.nodes.some(e=>e.alias.toLowerCase()===r.toLowerCase())&&(n.alias=`Node "${r}" is no longer available in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,{valid:Object.keys(n).length===0,errors:n}}function nr(e,t){let n={},r=e.trim(),i=t.trim();return(!r||!Jn.test(r))&&(n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`),(!i||!Jn.test(i))&&(n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`),!n.sourceAlias&&!n.targetAlias&&r===i&&(n.targetAlias=`Source and target must be different nodes.`),{valid:Object.keys(n).length===0,errors:n}}function rr(e,t){return!!e?.nodes.some(e=>e.alias.toLowerCase()===t.toLowerCase())}function ir(e,t={}){let n={},r=e.sourceAlias.trim(),i=e.targetAlias.trim(),a=e.relation.trim();return t.connected===!1&&(n.command=`Connection disconnected. Reconnect before creating a connection.`),r?Jn.test(r)?t.graphData&&!rr(t.graphData,r)&&(n.sourceAlias=`Node "${r}" is no longer available in the current graph.`):n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`:n.sourceAlias=`Source is required.`,i?Jn.test(i)?t.graphData&&!rr(t.graphData,i)&&(n.targetAlias=`Node "${i}" is no longer available in the current graph.`):n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`:n.targetAlias=`Target is required.`,r&&i&&r.toLowerCase()===i.toLowerCase()&&(n.targetAlias=`Source and target nodes must be different.`),a?Jn.test(a)||(n.relation=`Use only letters, numbers, underscore, and hyphen.`):n.relation=`Relation is required.`,{valid:Object.keys(n).length===0,errors:n}}function ar(e){return e.length<=63488?{valid:!0,errors:{}}:{valid:!1,errors:{command:`The node command is too large. Shorten property values before submitting.`}}}function or(e,t=!1){return e.properties.map(e=>({key:e.key.trim(),value:t?e.value.replace(/\r\n/g,`
`).replace(/\r/g,`
`):e.value.trim()})).filter(e=>e.key||e.value.trim())}function sr(e){let t=ar(e);if(!t.valid)throw Error(t.errors.command)}function cr(e,t,n){if(n.includes(`
`)){e.push(`${t}='''`),e.push(n),e.push(`'''`);return}e.push(`${t}=${n}`)}function lr(e){let t=er(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid node form state.`);let n=e.alias.trim(),r=e.nodeType.trim(),i=or(e,!0),a=[`create node ${n}`];if(r&&a.push(`with type ${r}`),i.length>0){a.push(`with properties`);for(let e of i)cr(a,e.key,e.value)}let o=a.join(`
`);return sr(o),o}function ur(e,t){let n=t.trim(),r=er(e,{mode:`edit`,originalAlias:n});if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node form state.`);let i=e.nodeType.trim(),a=or(e,!0),o=[`update node ${n}`];if(i&&o.push(`with type ${i}`),a.length>0){o.push(`with properties`);for(let e of a)cr(o,e.key,e.value)}let s=o.join(`
`);return sr(s),s}function dr(e,t={}){let n=e.trim(),r=tr(n,t);if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node alias.`);let i=`delete node ${n}`;return sr(i),i}function fr(e,t){let n=e.trim(),r=t.trim(),i=nr(n,r);if(!i.valid)throw Error(Object.values(i.errors)[0]??`Invalid connection endpoints.`);let a=`delete connection ${n} and ${r}`;return sr(a),a}function pr(e){let t=ir(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid connection form state.`);let n=`connect ${e.sourceAlias.trim()} to ${e.targetAlias.trim()} with ${e.relation.trim()}`;return sr(n),n}function mr(e,t,n){let r=[];for(let i of e.connections??[])if(!(i.source!==t||i.target!==n))for(let e of i.relations)r.push(e.type);return r}function hr(e,t){try{let n=new Map,r=(t,r)=>{let i=`${t}\t${r}`,a=n.get(i);return a||(a=mr(e,t,r),n.set(i,a)),a},i=[],a=[],o=new Set;for(let{source:e,target:n,relation:s}of t){let t=r(e,n);if(s===void 0){if(t.length===0)continue;for(let r of t)i.push({source:e,target:n,relation:r});t.length=0}else{let r=t.indexOf(s);if(r===-1)continue;t.splice(r,1),i.push({source:e,target:n,relation:s})}let c=[e,n].sort().join(`	`);o.has(c)||(o.add(c),a.push({a:e,b:n}))}if(i.length===0)return null;let s=[];for(let{a:e,b:t}of a){s.push(fr(e,t));for(let n of r(e,t))s.push(pr({sourceAlias:e,targetAlias:t,relation:n}));for(let n of r(t,e))s.push(pr({sourceAlias:t,targetAlias:e,relation:n}))}return{commands:s,removed:i}}catch{return null}}function gr(e){let t=e[0];return e.length===1?`relation '${t.relation}' (${t.source} → ${t.target})`:e.every(e=>e.source===t.source&&e.target===t.target)?`connection ${t.source} → ${t.target}`:`${e.length} relations`}var _r=0;function vr(e=``,t=``){return _r+=1,{id:`property-row-${_r}`,key:e,value:t}}function yr(e){return{alias:e===`empty-graph`?`root`:``,nodeType:e===`empty-graph`?`Root`:``,properties:[vr()],source:e}}function br(e){let t=e.indexOf(`[`);if(t===-1)return null;let n=e.indexOf(`]`,t+1);return n===-1||e.indexOf(`[`,n+1)!==-1?null:{open:t,close:n}}function xr(e){let t=br(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+n.padStart(3,`0`)+e.slice(t.close):e}function Sr(e){let t=br(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+e.slice(t.close):e}function Cr(e){return e.slice().sort((e,t)=>{let n=xr(e.key.trim()),r=xr(t.key.trim());return!n&&!r?0:n?!r||n<r?-1:+(n>r):1})}var wr=`This node contains data that cannot be safely represented in the edit form. Use the console edit command for this node.`;function Tr(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function Er(e){return e===null?`null`:String(e)}function Dr(e,t,n){if(!Qn(e))return!1;if(Array.isArray(t))return t.length!==0&&t.every((t,r)=>Dr(`${e}[${r}]`,t,n));if(Tr(t)){let r=Object.entries(t);return r.length!==0&&r.every(([t,r])=>Dr(`${e}.${t}`,r,n))}let r=Er(t);return r.includes(`'''`)?!1:(n.push(vr(e,r)),!0)}function Or(e){if(!Jn.test(e.alias)||e.types.length>1)return{valid:!1,formState:null,message:wr};let t=Object.entries(e.properties),n=[];for(let[e,r]of t)if(!Dr(e,r,n))return{valid:!1,formState:null,message:wr};let r=Cr(n).map(e=>({...e,key:Sr(e.key)}));return{valid:!0,formState:{alias:e.alias,nodeType:e.types[0]??``,properties:r.length>0?r:[vr()],source:`edit-node`},message:null}}function kr(e,t,n){let r=[];for(let i of e.connections??[]){let e=i.source===t&&i.target===n,a=i.source===n&&i.target===t;if(!(!e&&!a))for(let e of i.relations)r.push(pr({sourceAlias:i.source,targetAlias:i.target,relation:e.type}))}return r}function Ar(e){if(e.length===0)return null;try{return{label:`delete ${gr(e)}`,inverseCommands:e.map(({source:e,target:t,relation:n})=>pr({sourceAlias:e,targetAlias:t,relation:n}))}}catch{return null}}function jr(e){let t=Or(e);if(!t.valid||t.formState===null)return null;try{return{label:`edit node ${e.alias}`,inverseCommands:[ur(t.formState,e.alias)]}}catch{return null}}function Mr(e){try{return{label:`create node ${e}`,inverseCommands:[dr(e)]}}catch{return null}}function Nr(e,t,n){try{let r=kr(e,t,n);return{label:`create connection ${t} → ${n}`,inverseCommands:[fr(t,n),...r]}}catch{return null}}function Pr(e,t){let n=Or(t);if(!n.valid||n.formState===null)return null;try{let r=[lr(n.formState)];for(let n of e.connections??[])if(!(n.source!==t.alias&&n.target!==t.alias))for(let e of n.relations)r.push(pr({sourceAlias:n.source,targetAlias:n.target,relation:e.type}));return{label:`delete node ${t.alias}`,inverseCommands:r}}catch{return null}}var Fr=2500;function Ir({bus:e,connected:t,sendRawText:n,addToast:r}){let i=(0,A.useRef)([]),a=(0,A.useRef)(0),o=(0,A.useCallback)(()=>{i.current=[]},[]);(0,A.useEffect)(()=>{t||o()},[o,t]),(0,A.useEffect)(()=>e.on(`graph.mutation`,e=>{e.mutationType===`import-graph`&&o()}),[e,o]);let s=(0,A.useCallback)(e=>{if(e===null||e.inverseCommands.length===0)return null;let t=++a.current;return i.current=[...i.current.slice(-19),{...e,id:t}],t},[]),c=(0,A.useRef)(!1),l=(0,A.useCallback)(()=>new Promise(t=>{let n=!1,r=()=>{n||(n=!0,i(),clearTimeout(a),t())},i=e.on(`graph.mutation`,r),a=setTimeout(r,Fr)}),[e]),u=(0,A.useRef)([]),d=(0,A.useCallback)(e=>{e.length!==0&&(u.current.push(e),!c.current&&(c.current=!0,(async()=>{try{for(;;){let e=u.current.shift();if(!e)break;for(let t of e){if(!n(t)){r(`Could not send the command because the WebSocket is not open.`,`error`),u.current=[];return}await l()}}}finally{c.current=!1}})()))},[r,n,l]),f=(0,A.useCallback)(e=>{if(c.current){r(`A graph edit is already in progress.`,`info`);return}let t=i.current,a=t[t.length-1];if(!a){r(`Nothing to undo.`,`info`);return}if(e!==void 0&&a.id!==e){r(`Newer changes exist — press Ctrl+Z to undo them in order.`,`info`);return}i.current=t.slice(0,-1),c.current=!0,(async()=>{try{for(let e of a.inverseCommands){if(!n(e)){r(`Could not send the undo command because the WebSocket is not open.`,`error`),o();return}await l()}r(`Undo: ${a.label}`,`success`)}finally{c.current=!1}})()},[r,o,n,l]);return{push:s,undoLast:(0,A.useCallback)(()=>f(void 0),[f]),undoEntry:f,runCommands:d,hasEntries:(0,A.useCallback)(()=>i.current.length>0,[]),clear:o}}function Lr(e){return e.trim().toLowerCase()}function Rr(e,t){if(!t.alias||t.action!==null&&t.action!==`delete-node`)return null;let n=e.aliases.find(n=>Lr(n)===Lr(t.alias)&&e.results[n]===void 0);return n?{...e,results:{...e.results,[n]:t.status===`accepted`?`success`:`error`}}:null}function zr(e){return e.aliases.every(t=>e.results[t]!==void 0)}function Br(e){let t=Object.values(e.results).filter(e=>e===`success`).length,n=Object.values(e.results).filter(e=>e===`error`).length,r=`${t} selected ${t===1?`node`:`nodes`} deleted.`;return t===0&&n>0?{message:`Failed to delete selected nodes.`,type:`error`}:n>0?{message:`${r} ${n} failed.`,type:`error`}:{message:r,type:`success`}}var Vr={toastContainer:`_toastContainer_1rn7u_1`,toast:`_toast_1rn7u_1`,slideIn:`_slideIn_1rn7u_1`,success:`_success_1rn7u_36`,error:`_error_1rn7u_40`,info:`_info_1rn7u_44`,toastIcon:`_toastIcon_1rn7u_48`,toastMessage:`_toastMessage_1rn7u_53`,toastAction:`_toastAction_1rn7u_59`},Hr=({toasts:e,onRemove:t})=>e.length===0?null:(0,F.jsx)(`div`,{className:Vr.toastContainer,children:e.map(e=>(0,F.jsxs)(`div`,{className:`${Vr.toast} ${Vr[e.type]}`,onClick:()=>t(e.id),children:[(0,F.jsxs)(`span`,{className:Vr.toastIcon,children:[e.type===`success`&&`✅`,e.type===`error`&&`❌`,e.type===`info`&&`ℹ️`]}),(0,F.jsx)(`span`,{className:Vr.toastMessage,children:e.message}),e.action&&(0,F.jsx)(`button`,{type:`button`,className:Vr.toastAction,onClick:n=>{n.stopPropagation(),e.action?.onClick(),t(e.id)},children:e.action.label})]},e.id))}),Ur={container:`_container_9dbh2_3`,trigger:`_trigger_9dbh2_7`,chevron:`_chevron_9dbh2_37`,chevronOpen:`_chevronOpen_9dbh2_43`,dot:`_dot_9dbh2_49`,dotIdle:`_dotIdle_9dbh2_56`,dotConnecting:`_dotConnecting_9dbh2_57`,pulse:`_pulse_9dbh2_1`,dotConnected:`_dotConnected_9dbh2_58`,dotPartial:`_dotPartial_9dbh2_59`,dropdown:`_dropdown_9dbh2_65`,fadeIn:`_fadeIn_9dbh2_1`};function Wr({label:e,dotStatus:t,children:n}){let[r,i]=(0,A.useState)(!1),a=(0,A.useRef)(null);(0,A.useEffect)(()=>{if(!r)return;let e=e=>{a.current&&!a.current.contains(e.target)&&i(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[r]);let o=e=>{e.key===`Escape`&&(i(!1),a.current?.querySelector(`button[aria-haspopup]`)?.focus())},s=t===`connected`?Ur.dotConnected:t===`connecting`?Ur.dotConnecting:t===`partial`?Ur.dotPartial:t===`idle`?Ur.dotIdle:void 0;return(0,F.jsxs)(`div`,{className:Ur.container,ref:a,onKeyDown:o,children:[(0,F.jsxs)(`button`,{className:Ur.trigger,onClick:()=>i(e=>!e),"aria-haspopup":`true`,"aria-expanded":r,children:[t!==void 0&&(0,F.jsx)(`span`,{className:`${Ur.dot} ${s??``}`,"aria-hidden":`true`}),(0,F.jsx)(`span`,{children:e}),(0,F.jsx)(`span`,{className:`${Ur.chevron} ${r?Ur.chevronOpen:``}`,"aria-hidden":`true`,children:`▾`})]}),r&&(0,F.jsx)(`div`,{className:Ur.dropdown,role:`menu`,children:n})]})}var L={panel:`_panel_1ws0d_1`,section:`_section_1ws0d_7`,sectionHeader:`_sectionHeader_1ws0d_20`,sessionHeaderRow:`_sessionHeaderRow_1ws0d_29`,iconButton:`_iconButton_1ws0d_36`,copyButton:`_copyButton_1ws0d_63`,copyButtonCopied:`_copyButtonCopied_1ws0d_95`,relationshipRow:`_relationshipRow_1ws0d_102`,subscriberRow:`_subscriberRow_1ws0d_103`,sessionId:`_sessionId_1ws0d_114`,statusDot:`_statusDot_1ws0d_127`,metaText:`_metaText_1ws0d_136`,emptyMessage:`_emptyMessage_1ws0d_137`,subscriberList:`_subscriberList_1ws0d_148`,subscribeForm:`_subscribeForm_1ws0d_157`,subscribeInput:`_subscribeInput_1ws0d_164`,subscribeButton:`_subscribeButton_1ws0d_189`,resetButton:`_resetButton_1ws0d_190`,unsubscribeButton:`_unsubscribeButton_1ws0d_218`,actionsRow:`_actionsRow_1ws0d_241`,errorMessage:`_errorMessage_1ws0d_261`,infoMessage:`_infoMessage_1ws0d_262`};function Gr({copied:e}){return e?(0,F.jsx)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:(0,F.jsx)(`path`,{d:`M6.2 11.4 2.9 8.1l1.1-1.1 2.2 2.2 5.8-5.8 1.1 1.1-6.9 6.9Z`})}):(0,F.jsxs)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:[(0,F.jsx)(`path`,{d:`M5 2.5A1.5 1.5 0 0 1 6.5 1h6A1.5 1.5 0 0 1 14 2.5v6A1.5 1.5 0 0 1 12.5 10H11V8.7h1.5a.2.2 0 0 0 .2-.2v-6a.2.2 0 0 0-.2-.2h-6a.2.2 0 0 0-.2.2V4H5V2.5Z`}),(0,F.jsx)(`path`,{d:`M2 6.5A1.5 1.5 0 0 1 3.5 5h6A1.5 1.5 0 0 1 11 6.5v7A1.5 1.5 0 0 1 9.5 15h-6A1.5 1.5 0 0 1 2 13.5v-7Zm1.5-.2a.2.2 0 0 0-.2.2v7a.2.2 0 0 0 .2.2h6a.2.2 0 0 0 .2-.2v-7a.2.2 0 0 0-.2-.2h-6Z`})]})}function Kr(e){return e.connected?e.state.pendingCommand===`refresh`||e.state.loading?`connecting`:e.state.sessionId?`connected`:`partial`:`idle`}function qr({controller:e}){let{state:t}=e,[n,r]=(0,A.useState)(!1),[i,a]=(0,A.useState)(``),[o,s]=(0,A.useState)(null),[c,l]=(0,A.useState)(null),u=(0,A.useRef)(null),d=e.canReset;(0,A.useEffect)(()=>()=>{u.current!==null&&window.clearTimeout(u.current)},[]),(0,A.useEffect)(()=>{t.subscribedTo&&t.pendingCommand!==`subscribe`&&(r(!1),a(``))},[t.pendingCommand,t.subscribedTo]);let f=t=>{t.preventDefault(),e.subscribeToSession(i)},p=async e=>{try{await navigator.clipboard.writeText(e),s(null),l(e),u.current!==null&&window.clearTimeout(u.current),u.current=window.setTimeout(()=>{l(null),u.current=null},1400)}catch{l(null),s(`Could not copy the session ID. Please copy it manually.`)}};return e.connected?(0,F.jsxs)(`div`,{className:L.panel,children:[(0,F.jsxs)(`section`,{className:L.section,children:[(0,F.jsx)(`div`,{className:L.sectionHeader,children:`This session`}),(0,F.jsxs)(`div`,{className:L.sessionHeaderRow,children:[t.sessionId?(0,F.jsx)(`code`,{className:L.sessionId,title:t.sessionId,children:t.sessionId}):(0,F.jsx)(`p`,{className:L.emptyMessage,children:`Session details are not loaded yet.`}),t.sessionId&&(0,F.jsx)(`button`,{type:`button`,className:`${L.copyButton} ${c===t.sessionId?L.copyButtonCopied:``}`,onClick:()=>void p(t.sessionId),"aria-label":`Copy session ID`,title:c===t.sessionId?`Copied`:`Copy session ID`,children:(0,F.jsx)(Gr,{copied:c===t.sessionId})}),e.canSubscribe&&(0,F.jsx)(`button`,{type:`button`,className:L.iconButton,onClick:()=>{e.clearMessage(),r(e=>!e)},"aria-expanded":n,"aria-label":n?`Close subscribe form`:`Subscribe to another session`,title:n?`Close subscribe form`:`Subscribe to another session`,children:n?`×`:`+`})]}),t.startedSince&&(0,F.jsxs)(`div`,{className:L.metaText,children:[`Started since `,t.startedSince]}),n&&(0,F.jsxs)(`form`,{className:L.subscribeForm,onSubmit:f,children:[(0,F.jsx)(`input`,{className:L.subscribeInput,value:i,onChange:t=>{a(t.target.value),e.clearMessage()},placeholder:`ws-123456-1`,autoComplete:`off`,disabled:!e.canSubscribe}),(0,F.jsx)(`button`,{type:`submit`,className:L.subscribeButton,disabled:!e.canSubscribe||i.trim().length===0,children:t.pendingCommand===`subscribe`?`...`:`Subscribe`})]})]}),t.subscribedTo&&(0,F.jsxs)(`section`,{className:L.section,children:[(0,F.jsx)(`div`,{className:L.sectionHeader,children:`Subscribed to`}),(0,F.jsxs)(`div`,{className:L.relationshipRow,children:[(0,F.jsx)(`span`,{className:L.statusDot,"aria-hidden":`true`}),(0,F.jsx)(`code`,{className:L.sessionId,title:t.subscribedTo,children:t.subscribedTo}),(0,F.jsx)(`button`,{type:`button`,className:L.unsubscribeButton,onClick:e.unsubscribe,disabled:!e.canUnsubscribe,children:t.pendingCommand===`unsubscribe`?`...`:`Unsubscribe`})]})]}),t.subscribers.length>0&&(0,F.jsxs)(`section`,{className:L.section,children:[(0,F.jsx)(`div`,{className:L.sectionHeader,children:`Subscribers`}),(0,F.jsx)(`ul`,{className:L.subscriberList,children:t.subscribers.map(e=>(0,F.jsxs)(`li`,{className:L.subscriberRow,children:[(0,F.jsx)(`span`,{className:L.statusDot,"aria-hidden":`true`}),(0,F.jsx)(`code`,{className:L.sessionId,title:e,children:e})]},e))})]}),d&&(0,F.jsx)(`div`,{className:L.actionsRow,children:(0,F.jsx)(`button`,{type:`button`,className:L.resetButton,onClick:e.resetSession,disabled:!e.canReset,children:t.pendingCommand===`reset`?`Resetting...`:`Reset Session`})}),t.error&&(0,F.jsx)(`div`,{className:L.errorMessage,role:`alert`,children:t.error}),o&&(0,F.jsx)(`div`,{className:L.errorMessage,role:`alert`,children:o})]}):(0,F.jsx)(`div`,{className:L.panel,children:(0,F.jsx)(`p`,{className:L.emptyMessage,children:`Connect Minigraph to view session details.`})})}function Jr({controller:e}){return(0,F.jsx)(Wr,{label:`Session`,dotStatus:Kr(e),children:(0,F.jsx)(qr,{controller:e})})}var Yr={nav:`_nav_1hfby_3`,menuList:`_menuList_1hfby_11`,menuItem:`_menuItem_1hfby_19`,toolRow:`_toolRow_1hfby_56`,toolLink:`_toolLink_1hfby_67`,toolLinkActive:`_toolLinkActive_1hfby_92`,toolDot:`_toolDot_1hfby_99`,toolDotIdle:`_toolDotIdle_1hfby_106`,toolDotConnecting:`_toolDotConnecting_1hfby_107`,pulse:`_pulse_1hfby_1`,toolDotConnected:`_toolDotConnected_1hfby_108`,connectAllRow:`_connectAllRow_1hfby_112`,connectAllBtn:`_connectAllBtn_1hfby_118`,connectAllBtnStop:`_connectAllBtnStop_1hfby_142`,toolConnectBtn:`_toolConnectBtn_1hfby_154`,toolConnectBtnStop:`_toolConnectBtnStop_1hfby_180`,externalIcon:`_externalIcon_1hfby_192`};function Xr(e){return e.every(e=>e===`connected`)?`connected`:e.every(e=>e===`idle`)?`idle`:e.some(e=>e===`connecting`)?`connecting`:`partial`}function Zr(e){return e===`connected`?`connected`:e===`connecting`?`connecting`:`idle`}var Qr=[{href:`/info`,label:`Info`},{href:`/info/lib`,label:`Libraries`},{href:`/info/routes`,label:`Services`},{href:`/health`,label:`Health`},{href:`/env`,label:`Environment`},{href:`http://localhost:8085/api/ws/json`,label:`Legacy JSON`},{href:`http://localhost:8085/api/ws/graph`,label:`Legacy Graph`}];function $r({addToast:e,sessionCollaboration:t}){let n=xe(),r=me.map(e=>n.getSlot(e.wsPath).phase),i=Xr(r),a=r.every(e=>e===`connected`),o=r.some(e=>e===`connecting`);function s(){me.forEach(t=>{n.getSlot(t.wsPath).phase===`idle`&&n.connect(t.wsPath,e)})}function c(){me.forEach(e=>{let{phase:t}=n.getSlot(e.wsPath);(t===`connected`||t===`connecting`)&&n.disconnect(e.wsPath)})}return(0,F.jsxs)(`nav`,{className:Yr.nav,"aria-label":`Main navigation`,children:[t&&(0,F.jsx)(Jr,{controller:t}),(0,F.jsxs)(Wr,{label:`Tools`,dotStatus:i,children:[(0,F.jsx)(`div`,{className:Yr.connectAllRow,children:(0,F.jsx)(`button`,{className:`${Yr.connectAllBtn} ${a?Yr.connectAllBtnStop:``}`,onClick:a?c:s,disabled:o,"aria-label":o?`Connecting…`:a?`Disconnect all WebSockets`:`Connect all WebSockets`,children:o?`Connecting…`:a?`Disconnect All`:`Connect All`})}),(0,F.jsx)(`ul`,{className:Yr.menuList,role:`none`,children:me.map(t=>{let{phase:r}=n.getSlot(t.wsPath),i=Zr(r),a=r===`connected`,o=r===`connecting`,s=i===`connected`?Yr.toolDotConnected:i===`connecting`?Yr.toolDotConnecting:Yr.toolDotIdle;return(0,F.jsxs)(`li`,{role:`none`,className:Yr.toolRow,children:[(0,F.jsxs)(w,{to:t.path,role:`menuitem`,className:({isActive:e})=>`${Yr.toolLink} ${e?Yr.toolLinkActive:``}`,children:[(0,F.jsx)(`span`,{className:`${Yr.toolDot} ${s}`,"aria-hidden":`true`}),(0,F.jsx)(`span`,{className:Yr.toolLabel,children:t.label})]}),(0,F.jsx)(`button`,{className:`${Yr.toolConnectBtn} ${a?Yr.toolConnectBtnStop:``}`,onClick:()=>a||o?n.disconnect(t.wsPath):n.connect(t.wsPath,e),disabled:o,"aria-label":o?`Connecting…`:a?`Disconnect ${t.label}`:`Connect ${t.label}`,title:o?`Connecting…`:ge(t.wsPath),children:o?`…`:a?`Stop`:`Start`})]},t.path)})})]}),(0,F.jsx)(Wr,{label:`Quick Links`,children:(0,F.jsx)(`ul`,{className:Yr.menuList,role:`none`,children:Qr.map(e=>(0,F.jsx)(`li`,{role:`none`,children:(0,F.jsxs)(`a`,{href:e.href,role:`menuitem`,className:Yr.menuItem,target:`_blank`,rel:`noopener noreferrer`,children:[e.label,(0,F.jsx)(`span`,{className:Yr.externalIcon,"aria-hidden":`true`,children:`↗`})]})},e.href))})})]})}var ei={saveBtn:`_saveBtn_ek34s_3`,saveBtnSaved:`_saveBtnSaved_ek34s_25`,saveBtnLabel:`_saveBtnLabel_ek34s_35`,saveForm:`_saveForm_ek34s_50`,saveInput:`_saveInput_ek34s_56`,saveInputWarn:`_saveInputWarn_ek34s_72`,saveWarnLabel:`_saveWarnLabel_ek34s_76`,saveActionBtn:`_saveActionBtn_ek34s_82`};function ti(e){return e?`Saved: ${e}`:`Save Graph`}function ni({disabled:e,defaultName:t,savedName:n=null,onSave:r,nameExists:i,connected:a=!1}){let[o,s]=(0,A.useState)(!1),[c,l]=(0,A.useState)(``),u=(0,A.useRef)(null),d=(0,A.useRef)(null),f=(0,A.useRef)(!1),p=(0,A.useCallback)(()=>{l(t),s(!0)},[t]),m=(0,A.useCallback)(()=>{f.current=!0,s(!1),l(``)},[]),h=(0,A.useCallback)(()=>{let e=c.trim();e&&(r(e),f.current=!0,s(!1),l(``))},[c,r]),g=(0,A.useCallback)(e=>{e.key===`Enter`&&(e.preventDefault(),h()),e.key===`Escape`&&(e.preventDefault(),m())},[h,m]);return(0,A.useEffect)(()=>{o?u.current?.focus():f.current&&(f.current=!1,d.current?.focus())},[o]),o?(0,F.jsxs)(`div`,{className:ei.saveForm,children:[(0,F.jsx)(`input`,{ref:u,className:`${ei.saveInput}${i?.(c.trim())?` ${ei.saveInputWarn}`:``}`,type:`text`,value:c,onChange:e=>l(e.target.value),onKeyDown:g,placeholder:`Enter a name…`,"aria-label":`Graph save name`,maxLength:80}),i?.(c.trim())&&(0,F.jsx)(`span`,{className:ei.saveWarnLabel,role:`status`,children:`Overwrite?`}),(0,F.jsx)(`button`,{className:ei.saveActionBtn,onClick:h,disabled:!c.trim(),"aria-label":`Confirm save`,children:`✅`}),(0,F.jsx)(`button`,{className:ei.saveActionBtn,onClick:m,"aria-label":`Cancel save`,children:`❌`})]}):(0,F.jsxs)(`button`,{ref:d,className:`${ei.saveBtn}${n?` ${ei.saveBtnSaved}`:``}`,onClick:p,disabled:e||!a,title:e?`No graph loaded`:a?n?`Graph saved as ${n}. Click to save again`:`Export graph snapshot to server and save bookmark`:`Connect first to save`,"aria-label":n?`Graph saved as ${n}. Save again`:`Save graph snapshot`,children:[(0,F.jsx)(`span`,{"aria-hidden":`true`,children:n?`✅`:`💾`}),(0,F.jsx)(`span`,{className:ei.saveBtnLabel,children:ti(n)})]})}var ri={empty:`_empty_tpeii_3`,hint:`_hint_tpeii_12`,list:`_list_tpeii_21`,row:`_row_tpeii_31`,rowInfo:`_rowInfo_tpeii_50`,rowName:`_rowName_tpeii_58`,rowMeta:`_rowMeta_tpeii_67`,rowActions:`_rowActions_tpeii_78`,loadBtn:`_loadBtn_tpeii_84`,deleteBtn:`_deleteBtn_tpeii_85`};function ii({savedGraphs:e,onLoad:t,onDelete:n,connected:r}){return(0,F.jsx)(Wr,{label:e.length>0?`Load Graph (${e.length})`:`Load Graph`,children:e.length===0?(0,F.jsx)(`p`,{className:ri.empty,children:`No saved graphs yet.`}):(0,F.jsxs)(F.Fragment,{children:[!r&&(0,F.jsx)(`p`,{className:ri.hint,children:`Connect to load a graph`}),(0,F.jsx)(`ul`,{className:ri.list,role:`list`,children:e.map(e=>(0,F.jsxs)(`li`,{className:ri.row,children:[(0,F.jsxs)(`div`,{className:ri.rowInfo,children:[(0,F.jsx)(`span`,{className:ri.rowName,title:e.name,children:e.name}),(0,F.jsx)(`span`,{className:ri.rowMeta,children:new Date(e.savedAt).toLocaleString()})]}),(0,F.jsxs)(`div`,{className:ri.rowActions,children:[(0,F.jsx)(`button`,{className:ri.loadBtn,onClick:()=>t(e.name),disabled:!r,title:r?`Run: import graph from ${e.name}`:`Connect to the playground first`,"aria-label":`Load graph ${e.name}`,children:`Load`}),(0,F.jsx)(`button`,{className:ri.deleteBtn,onClick:()=>n(e.name),title:`Remove "${e.name}" from local storage`,"aria-label":`Delete saved graph ${e.name}`,children:`Delete`})]})]},e.name))})]})})}var ai={payloadRoot:`_payloadRoot_6u47x_2`,labelRow:`_labelRow_6u47x_10`,label:`_label_6u47x_10`,payloadControls:`_payloadControls_6u47x_26`,charCounter:`_charCounter_6u47x_32`,typeIndicator:`_typeIndicator_6u47x_38`,validationIcon:`_validationIcon_6u47x_49`,formatButton:`_formatButton_6u47x_53`,uploadButton:`_uploadButton_6u47x_67`,textarea:`_textarea_6u47x_82`,textareaError:`_textareaError_6u47x_107`,errorMessage:`_errorMessage_6u47x_109`,sampleButtonsRow:`_sampleButtonsRow_6u47x_117`,sampleButtons:`_sampleButtons_6u47x_117`,sampleLabel:`_sampleLabel_6u47x_130`,sampleGroup:`_sampleGroup_6u47x_136`,sampleGroupLabel:`_sampleGroupLabel_6u47x_143`,sampleButton:`_sampleButton_6u47x_117`};function oi({onLoad:e}){let t=Object.keys(he).filter(e=>e.startsWith(`json_`)),n=Object.keys(he).filter(e=>e.startsWith(`xml_`)),r=e=>e.replace(/^(json|xml)_/,``).replace(/_/g,` `);return(0,F.jsxs)(`div`,{className:ai.sampleButtons,children:[(0,F.jsx)(`span`,{className:ai.sampleLabel,children:`Quick load:`}),(0,F.jsxs)(`div`,{className:ai.sampleGroup,children:[(0,F.jsx)(`span`,{className:ai.sampleGroupLabel,children:`JSON:`}),t.map(t=>(0,F.jsx)(`button`,{className:ai.sampleButton,onClick:()=>e(he[t]),children:r(t)},t))]}),(0,F.jsxs)(`div`,{className:ai.sampleGroup,children:[(0,F.jsx)(`span`,{className:ai.sampleGroupLabel,children:`XML:`}),n.map(t=>(0,F.jsx)(`button`,{className:ai.sampleButton,onClick:()=>e(he[t]),children:r(t)},t))]})]})}function si({payload:e,onChange:t,validation:n,onFormat:r,onUpload:i}){return(0,F.jsxs)(`div`,{className:ai.payloadRoot,children:[(0,F.jsxs)(`div`,{className:ai.labelRow,children:[(0,F.jsx)(`label`,{htmlFor:`payload`,className:ai.label,children:`JSON/XML Payload`}),(0,F.jsxs)(`div`,{className:ai.payloadControls,children:[(0,F.jsxs)(`span`,{className:ai.charCounter,children:[`size: `,e.length]}),e&&n.type&&(0,F.jsx)(`span`,{className:ai.typeIndicator,children:n.type.toUpperCase()}),e&&(0,F.jsx)(`span`,{className:ai.validationIcon,children:n.valid?`✅`:`❌`}),(0,F.jsx)(`button`,{className:ai.formatButton,onClick:r,disabled:!e||n.type!==`json`,title:n.type===`xml`?`Format only available for JSON`:`Format JSON`,children:`Format`}),i!==void 0&&(0,F.jsx)(`button`,{className:ai.uploadButton,onClick:i,disabled:!e||!n.valid||n.type!==`json`,title:`Upload JSON payload to current session via REST`,children:`Upload`})]})]}),(0,F.jsx)(`textarea`,{id:`payload`,className:`${ai.textarea} ${n.valid?``:ai.textareaError}`,placeholder:`Paste your JSON/XML payload here`,value:e,onChange:e=>t(e.target.value)}),!n.valid&&(0,F.jsx)(`div`,{className:ai.errorMessage,children:n.error}),(0,F.jsx)(`div`,{className:ai.sampleButtonsRow,children:(0,F.jsx)(oi,{onLoad:t})})]})}var ci={Root:{icon:`🚀`,label:`Root`},End:{icon:`🏁`,label:`End`},Fetcher:{icon:`🌐`,label:`Fetcher`},mapper:{icon:`🗺️`,label:`Mapper`},Math:{icon:`🔢`,label:`Math`},JavaScript:{icon:`📜`,label:`JavaScript`},Provider:{icon:`🔌`,label:`Provider`},Dictionary:{icon:`📖`,label:`Dictionary`},Join:{icon:`🔀`,label:`Join`},Extension:{icon:`🧩`,label:`Extension`},Island:{icon:`🏝️`,label:`Island`},Decision:{icon:`❓`,label:`Decision`},Suspend:{icon:`⏸️`,label:`Suspend`},Resume:{icon:`▶️`,label:`Resume`},Suspensible:{icon:`⏯️`,label:`Suspensible`}},li={boxSizing:`border-box`,borderRadius:`8px`,borderWidth:`1.5px`,borderStyle:`solid`,background:`var(--bg-secondary, #1e1e2e)`,color:`var(--text-primary, #cdd6f4)`,fontSize:`0.75rem`,boxShadow:`0 2px 8px rgba(0,0,0,0.45)`,overflow:`visible`,padding:0},ui={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`,Suspend:`#0d9488`,Resume:`#0284c7`,Suspensible:`#c026d3`},di=`#6c7086`;function fi(e){return ci[e]??{icon:`📦`,label:e}}function pi(e){return ui[e]??di}function mi(e){let t=ui[e]??di;return{...li,borderColor:t,"--node-accent":t}}var hi={content:`_content_1g9w1_8`,header:`_header_1g9w1_24`,icon:`_icon_1g9w1_44`,alias:`_alias_1g9w1_49`,badge:`_badge_1g9w1_55`,body:`_body_1g9w1_67`,bodyPeek:`_bodyPeek_1g9w1_77`,row:`_row_1g9w1_85`,label:`_label_1g9w1_98`,value:`_value_1g9w1_104`,edgeHandle:`_edgeHandle_1g9w1_118`,authoringRing:`_authoringRing_1g9w1_134`,authoringTargetOverlay:`_authoringTargetOverlay_1g9w1_197`,authoringTargetOverlayActive:`_authoringTargetOverlayActive_1g9w1_213`};function gi({label:e,value:t}){return(0,F.jsxs)(`div`,{className:hi.row,children:[(0,F.jsx)(`span`,{className:hi.label,children:e}),(0,F.jsx)(`span`,{className:hi.value,title:t,children:t})]})}function _i({properties:e}){let t=Object.entries(e).filter(([,e])=>e!=null);return t.length===0?null:(0,F.jsx)(F.Fragment,{children:t.map(([e,t])=>Array.isArray(t)?t.map((t,n)=>{let r=typeof t==`string`?t:JSON.stringify(t);return(0,F.jsx)(gi,{label:n===0?e:``,value:r},`${e}-${n}`)}):(0,F.jsx)(gi,{label:e,value:typeof t==`string`?t:JSON.stringify(t)},e))})}function vi({alias:e,nodeType:t,properties:n,compact:r=!1}){let i=fi(t);return(0,F.jsx)(A.Fragment,{children:(0,F.jsxs)(`div`,{className:hi.content,children:[(0,F.jsxs)(`div`,{className:hi.header,children:[(0,F.jsx)(`span`,{className:hi.icon,children:i.icon}),(0,F.jsx)(`span`,{className:hi.alias,children:e}),(0,F.jsx)(`span`,{className:hi.badge,children:i.label})]}),(0,F.jsx)(`div`,{className:r?`${hi.body} ${hi.bodyPeek}`:hi.body,children:(0,F.jsx)(_i,{properties:n})})]})})}function yi({id:e,data:t,isConnectable:n,selected:r}){let[i,a]=(0,A.useState)(!1),o=t.supportsConnectionAuthoring&&!i,c=p(),l=o&&c.inProgress&&c.fromNode?.id!==e;return(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(y,{minWidth:180,minHeight:t.minHeight,isVisible:r,onResizeStart:()=>a(!0),onResizeEnd:()=>a(!1)}),t.targetHandles.map(({id:e,offset:t})=>(0,F.jsx)(s,{id:e,type:`target`,position:_.Left,isConnectable:n,className:hi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backSourceHandles.map(({id:e,offset:t})=>(0,F.jsx)(s,{id:e,type:`source`,position:_.Left,isConnectable:n,className:hi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),(0,F.jsx)(vi,{alias:t.alias,nodeType:t.nodeType,properties:t.properties,compact:t.compact}),o&&(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(s,{id:`authoring-source`,type:`source`,position:_.Right,isConnectable:n,isConnectableEnd:!1,className:hi.authoringRing}),(0,F.jsx)(s,{id:`authoring-target`,type:`target`,position:_.Left,isConnectable:n,isConnectableStart:!1,className:l?`${hi.authoringTargetOverlay} ${hi.authoringTargetOverlayActive}`:hi.authoringTargetOverlay})]}),t.sourceHandles.map(({id:e,offset:t})=>(0,F.jsx)(s,{id:e,type:`source`,position:_.Right,isConnectable:n,className:hi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backTargetHandles.map(({id:e,offset:t})=>(0,F.jsx)(s,{id:e,type:`target`,position:_.Right,isConnectable:n,className:hi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e))]})}var bi={Root:yi,End:yi,Fetcher:yi,mapper:yi,Math:yi,JavaScript:yi,Provider:yi,Dictionary:yi,Join:yi,Extension:yi,Island:yi,Decision:yi,default:yi},xi={graphWrapper:`_graphWrapper_1pee8_15`,graphSurface:`_graphSurface_1pee8_24`,empty:`_empty_1pee8_30`,emptyIcon:`_emptyIcon_1pee8_43`,emptyCreateButton:`_emptyCreateButton_1pee8_48`,emptyHint:`_emptyHint_1pee8_70`,refreshingOverlay:`_refreshingOverlay_1pee8_135`,clipboardDropOverlay:`_clipboardDropOverlay_1pee8_147`,clipboardDropMessage:`_clipboardDropMessage_1pee8_160`,refreshingSpinner:`_refreshingSpinner_1pee8_175`,graphRefreshSpin:`_graphRefreshSpin_1pee8_1`,connectBanner:`_connectBanner_1pee8_189`,connectBannerCancel:`_connectBannerCancel_1pee8_209`,detailToggleIcon:`_detailToggleIcon_1pee8_248`,detailToggleHeader:`_detailToggleHeader_1pee8_259`,detailToggleLine:`_detailToggleLine_1pee8_266`},Si=class extends A.Component{constructor(...e){super(...e),this.state={caughtError:null}}static getDerivedStateFromError(e){return{caughtError:e instanceof Error?e.message:String(e)}}componentDidCatch(e,t){let n=e instanceof Error?e.message:String(e);console.error(`[GraphView] Render error:`,n,t.componentStack),this.props.onRenderError?.(`Graph render failed: ${n}`)}render(){return this.state.caughtError?(0,F.jsxs)(`div`,{className:xi.empty,children:[(0,F.jsx)(`span`,{className:xi.emptyIcon,children:`⚠️`}),(0,F.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,F.jsx)(`span`,{children:this.state.caughtError})]}):this.props.children}},Ci=[`fetch`,`details`,`ext-call`,`mapping`,`compute`,`calculate`,`evaluate`,`fork`,`join`,`one`,`two`,`three`,`more`,`done`,`complete`,`finish`,`positive`,`negative`],wi={fetch:`#0369a1`,details:`#0369a1`,"ext-call":`#0369a1`,mapping:`#b45309`,compute:`#b45309`,calculate:`#b45309`,evaluate:`#b45309`,fork:`#7e22ce`,join:`#7e22ce`,one:`#7e22ce`,two:`#6d28d9`,three:`#5b21b6`,more:`#4c1d95`,done:`#15803d`,complete:`#15803d`,finish:`#15803d`,positive:`#15803d`,negative:`#b91c1c`},Ti=[`#0369a1`,`#15803d`,`#b45309`,`#7e22ce`,`#b91c1c`,`#0f766e`,`#c2410c`,`#a16207`];function Ei(e){let t=0;for(let n=0;n<e.length;n++)t=(t<<5)-t+e.charCodeAt(n),t|=0;return Math.abs(t)}function Di(e,t){if(e.length===0)return t;let n=e[0].trim().toLowerCase();return wi[n]||Ti[Ei(n)%Ti.length]}var Oi=240,ki=100,Ai=100,ji=60,Mi=360,Ni=120,Pi=80,Fi=4,R=1e4,Ii=2e3,Li=64,Ri=256,zi=256,Bi=512,Vi=.001,Hi=8,Ui=16,Wi=`rgba(148, 163, 184, 0.42)`,Gi=`var(--bg-secondary)`,Ki=24,qi=32;function Ji(e){return Di(e,Wi)}function Yi(e){return`source-${e}`}function Xi(e){return`target-${e}`}function Zi(e){return`back-source-${e}`}function Qi(e){return`back-target-${e}`}function $i(e,t){return t<=1?0:t===2?e===0?-24:Ki:(e-(t-1)/2)*Ki}function ea(e,t=ki){return e<=1?t:Math.max(t,(e-1)*Ki+qi*2)}var ta=40,na=9,ra=18,ia=22;function aa(e){let t=typeof e==`string`?e:JSON.stringify(e)??``;return Math.max(1,Math.ceil(t.length/ia))}function oa(e){let t=ta;for(let n of Object.values(e.properties??{})){if(n==null)continue;let e=Array.isArray(n)?n:[n];for(let n of e)t+=na+aa(n)*ra}return Math.max(ki,t)}var sa=new Set([`graph.math`,`graph.js`]),ca=[`Dictionary`,`Provider`,`Module`,`Entity`],la={ROOT_TREE:0,DEFAULT_TREE:1,END_TREE:2};function ua(e){return e.alias.toLowerCase()===`root`||e.types.includes(`Root`)||e.types.includes(`entry_point`)}function da(e){return e.alias.toLowerCase()===`end`||e.types.includes(`End`)}function fa(e){return e.hasRoot?la.ROOT_TREE:e.hasEnd?la.END_TREE:la.DEFAULT_TREE}function pa(e,t){let n=fa(e)-fa(t);return n===0?e.sortKey.localeCompare(t.sortKey):n}function ma(e){return`real:${e}`}function ha(e){return new Map([...e].map(([e,t])=>[e,t.slice()]))}function ga(e){return new Map(e.map((e,t)=>[e.id,t]))}function _a(e,t,n){let r=(t.get(e)??[]).map(e=>n.get(e)).filter(e=>e!==void 0);if(r.length!==0)return r.reduce((e,t)=>e+t,0)/r.length}function va(e,t,n){let r=ga(e);return e.slice().sort((e,i)=>{let a=_a(e.id,t,n),o=_a(i.id,t,n);if(a!==void 0&&o!==void 0){let e=a-o;if(Math.abs(e)>2**-52)return e}let s=r.get(e.id)-r.get(i.id);return s===0?e.stableKey.localeCompare(i.stableKey):s})}function ya(e,t){let n=Array(t+1).fill(0),r=e=>{let t=0;for(let r=e;r>0;r-=r&-r)t+=n[r];return t},i=e=>{for(let t=e;t<n.length;t+=t&-t)n[t]+=1},a=0;for(let t=e.length-1;t>=0;t--){let n=e[t]+1;a+=r(n-1),i(n)}return a}function ba(e,t){let n=new Map([...e].map(([e,t])=>[e,ga(t)])),r=new Map;for(let e of t)r.has(e.level)||r.set(e.level,[]),r.get(e.level).push(e);let i=0;for(let[e,t]of r){let r=n.get(e),a=n.get(e+1);if(!r||!a)continue;let o=t.map(e=>({sourceId:e.sourceId,targetId:e.targetId,sourcePosition:r.get(e.sourceId),targetPosition:a.get(e.targetId)})).sort((e,t)=>e.sourcePosition-t.sourcePosition||e.targetPosition-t.targetPosition||e.sourceId.localeCompare(t.sourceId)||e.targetId.localeCompare(t.targetId)).map(e=>e.targetPosition);i+=ya(o,a.size)}return i}function xa(e,t,n){e.has(t)||e.set(t,[]),e.get(t).push(n)}function Sa(e){return[e.source,e.target,...e.relations.map(e=>e.type)].join(`	`)}function Ca(e,t){return e.nodeIntrusions<t.nodeIntrusions||e.nodeIntrusions===t.nodeIntrusions&&e.edgeCrossings<t.edgeCrossings}function wa(e){let t=new Map;for(let[n,r]of e){let e=-(r.reduce((e,t)=>e+t.height,0)+Math.max(0,r.length-1)*ji)/2;for(let i of r)i.alias&&t.set(i.alias,{alias:i.alias,level:n,x:n*360,y:e,height:i.height}),e+=i.height+ji}return t}function Ta(e,t,n,r,i){let a=1-i;return a*a*a*e+3*a*a*i*t+3*a*i*i*n+i*i*i*r}function Ea(e,t,n,r){let i=0,a=1;for(let o=0;o<32;o++){let o=(i+a)/2;Ta(e,t,t,n,o)<r?i=o:a=o}return(i+a)/2}function Da(e,t,n,r,i){let a=e.x+Oi,o=t.x,s=i.x+Vi,c=i.x+Oi-Vi;if(s>=o||c<=a)return null;let l=a+(o-a)/2,u=Ea(a,l,o,s),d=Ea(a,l,o,c),f=e.y+e.height/2+n,p=t.y+t.height/2+r,m=Ta(f,f,p,p,u),h=Ta(f,f,p,p,d);return{yLow:Math.min(m,h),yHigh:Math.max(m,h)}}function Oa(e,t,n,r,i){let a=Da(e,t,n,r,i);if(!a)return!1;let o=i.y+Vi,s=i.y+i.height-Vi;return a.yHigh>o&&a.yLow<s}function ka(e,t){let n=new Map;for(let[e,r]of t)n.has(r)||n.set(r,[]),n.get(r).push(e);for(let e of n.values())e.sort();let r=[];for(let[i,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(!(e===void 0||o===void 0||o-e<=1)){for(let t=e+1;t<o;t++)for(let e of n.get(t)??[])if(r.push({connectionIndex:i,nodeAlias:e}),r.length>Ii)return null}}return r}function Aa(e,t,n){let r=new Map,i=new Map;for(let e of t.keys())r.set(e,[]),i.set(e,[]);for(let[n,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(e===void 0||o===void 0)continue;let s=e>=o,c=Sa(a);s?(i.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),r.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c})):(r.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),i.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c}))}let a=(e,t)=>n(e.peerAlias)-n(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex;for(let e of r.values())e.sort(a);for(let e of i.values())e.sort(a);let o=new Map,s=new Map;for(let e of r.values())for(let[t,n]of e.entries()){let r=$i(t,e.length);n.isBack?s.set(n.connectionIndex,r):o.set(n.connectionIndex,r)}for(let e of i.values())for(let[t,n]of e.entries()){let r=$i(t,e.length);n.isBack?o.set(n.connectionIndex,r):s.set(n.connectionIndex,r)}return{sourceOffsets:o,targetOffsets:s}}function ja(e,t,n,r){let i=wa(e),{sourceOffsets:a,targetOffsets:o}=Aa(n,t,e=>i.get(e)?.y??0),s=0;for(let{connectionIndex:e,nodeAlias:t}of r){let r=n[e],c=i.get(r.source),l=i.get(r.target),u=i.get(t);!c||!l||!u||Oa(c,l,a.get(e)??0,o.get(e)??0,u)&&(s+=1)}return s}function Ma(e,t,n,r){let i=e=>t.map(t=>e.get(t).map(e=>e.id).join(`	`)).join(`
`),a=ha(e),o=1,s=a,c=n(a),l=[{candidate:a,depth:0}],u=new Set([i(a)]),d=0;for(;d<l.length&&o<r;){let{candidate:e,depth:a}=l[d++];if(!(a>=2))for(let d of t){let t=e.get(d);for(let f=0;f<t.length;f++){for(let p=0;p<t.length;p++){if(f===p)continue;let t=ha(e),m=t.get(d),[h]=m.splice(f,1);m.splice(p,0,h);let g=i(t);if(u.has(g))continue;u.add(g);let _=n(t);if(o+=1,Ca(_,c)&&(s=t,c=_),a+1<2&&l.push({candidate:t,depth:a+1}),o>=r)break}if(o>=r)break}if(o>=r)break}}for(let n of t)e.set(n,s.get(n).slice());return o}function Na(e,t,n,r){let i=new Map;for(let[t,n]of e)i.has(n)||i.set(n,[]),i.get(n).push({id:ma(t),alias:t,height:r.get(t)??ki,stableKey:`real:${t}`});for(let e of i.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let a=ha(i),o=ha(i),s=t.map((t,n)=>({connectionIndex:n,source:t.source,target:t.target,sourceLevel:e.get(t.source),targetLevel:e.get(t.target),stableKey:Sa(t)})).filter(e=>e.sourceLevel!==void 0&&e.targetLevel!==void 0&&!n.has(`${e.source}\t${e.target}`)&&e.sourceLevel<e.targetLevel).sort((e,t)=>e.source.localeCompare(t.source)||e.target.localeCompare(t.target)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex),c=s.reduce((e,t)=>e+(t.targetLevel-t.sourceLevel),0)<=R,l=new Map,u=new Map,d=[];for(let[e,t]of s.entries()){let n=t.sourceLevel,r=t.targetLevel;if(!c&&r-n>1)continue;let i=ma(t.source),a=n;for(let s=n+1;s<r;s++){let n=`dummy:${t.source}\t${t.target}\t${e}\t${s}`;o.has(s)||o.set(s,[]),o.get(s).push({id:n,height:ki,stableKey:n}),d.push({sourceId:i,targetId:n,level:a}),xa(u,i,n),xa(l,n,i),i=n,a=s}let s=ma(t.target);d.push({sourceId:i,targetId:s,level:a}),xa(u,i,s),xa(l,s,i)}for(let e of o.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let f=[...o.keys()].sort((e,t)=>e-t);if(f.length<=1||d.length===0)return a;let p=ha(o),m=ha(o),h=ka(t,e),g=h?.length??2001,_=h!==null,v=n=>({nodeIntrusions:_?ja(n,e,t,h??[]):0,edgeCrossings:ba(n,d)}),y=v(m),b=()=>{let e=v(p);Ca(e,y)&&(y=e,m=ha(p))},x=[...o.values()].reduce((e,t)=>e+t.length,0)<=Li&&d.length<=Ri&&g<=zi,S=0,C=()=>{let e=Bi-S;!x||e<=1||(S+=Ma(p,f,v,e),b())};for(let e=0;e<Fi;e++){for(let e=1;e<f.length;e++){let t=f[e],n=f[e-1];p.set(t,va(p.get(t),l,ga(p.get(n))))}b(),C();for(let e=f.length-2;e>=0;e--){let t=f[e],n=f[e+1];p.set(t,va(p.get(t),u,ga(p.get(n))))}b(),C()}return _&&ja(a,e,t,h??[])<y.nodeIntrusions?a:m}function Pa(e,t,n,r,i){let a=n.slice().sort((e,n)=>Sa(t[e.connectionIndex]).localeCompare(Sa(t[n.connectionIndex]))||e.nodeAlias.localeCompare(n.nodeAlias)||e.connectionIndex-n.connectionIndex);for(let{connectionIndex:n,nodeAlias:o}of a){let a=t[n],s=e.get(a.source),c=e.get(a.target),l=e.get(o);if(!s||!c||!l)continue;let u=Da(s,c,r.get(n)??0,i.get(n)??0,l);if(!u)continue;let d=l.y+Vi,f=l.y+l.height-Vi;if(u.yHigh<=d||u.yLow>=f)continue;let p=u.yHigh+Ui-l.y,m=l.y+l.height-(u.yLow-Ui);return p<=m?{x:l.x,y:l.y,delta:p,direction:1}:{x:l.x,y:l.y,delta:m,direction:-1}}return null}function Fa(e,t,n,r){let i=ka(n,t);if(!(i===null||i.length===0))for(let a=0;a<Hi;a++){let a=new Map;for(let[n,i]of e){let e=t.get(n);e!==void 0&&a.set(n,{alias:n,level:e,x:i.x,y:i.y,height:r.get(n)??ki})}let{sourceOffsets:o,targetOffsets:s}=Aa(n,t,e=>a.get(e)?.y??0),c=Pa(a,n,i,o,s);if(!c)return;for(let[t,n]of e)n.x===c.x&&(c.direction===1?n.y>=c.y:n.y<=c.y)&&e.set(t,{x:n.x,y:n.y+c.delta*c.direction})}}function Ia(e,t){if(t.has(e.alias))return`flow`;let n=e.types[0]??``,r=typeof e.properties.skill==`string`?e.properties.skill:void 0;return n===`Dictionary`?`Dictionary`:n===`Provider`?`Provider`:r&&sa.has(r)?`Module`:r?`__unknown__`:`Entity`}function La(e,t,n){let r=new Set;for(let e of t??[])r.add(e.source),r.add(e.target);let i=[],a=[],o=new Map;for(let t of e){let e=Ia(t,r);o.set(t.alias,e),e===`flow`?i.push(t):a.push(t)}let s=new Set(i.map(e=>e.alias)),c=new Map(i.map(e=>[e.alias,e])),l=new Map,u=new Map,d=new Map;for(let e of i)l.set(e.alias,[]),u.set(e.alias,new Set),d.set(e.alias,0);for(let e of t??[])!s.has(e.source)||!s.has(e.target)||(l.get(e.source)?.push(e.target),u.get(e.source)?.add(e.target),u.get(e.target)?.add(e.source),d.set(e.target,(d.get(e.target)??0)+1));for(let e of l.values())e.sort();let f=i.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||ua(e)).map(e=>e.alias).sort(),p=new Set;{let e=new Map;for(let t of i)e.set(t.alias,0);function t(t){if(e.get(t)!==0)return;e.set(t,1);let n=[{node:t,childIdx:0}];for(;n.length>0;){let t=n[n.length-1],r=l.get(t.node)??[];if(t.childIdx>=r.length){e.set(t.node,2),n.pop();continue}let i=r[t.childIdx++],a=e.get(i);a===1?p.add(`${t.node}\t${i}`):a===0&&(e.set(i,1),n.push({node:i,childIdx:0}))}}for(let e of f)t(e);for(let e of[...s].sort())t(e)}let m=[],h=new Set;for(let e of Array.from(s).sort()){if(h.has(e))continue;let t=[],n=[e];for(h.add(e);n.length>0;){let e=n.pop();t.push(e);for(let t of u.get(e)??[])h.has(t)||(h.add(t),n.push(t))}t.sort();let r=t.map(e=>c.get(e)).filter(e=>!!e);m.push({aliases:t,nodes:r,hasRoot:r.some(ua),hasEnd:r.some(da),sortKey:t[0]??``})}m.sort(pa);let g=new Map;m.forEach((e,t)=>{e.aliases.forEach(e=>g.set(e,t))});let _=m.map(()=>[]);for(let e of t){let t=g.get(e.source),n=g.get(e.target);t!==void 0&&t===n&&_[t].push(e)}let v=new Map,y=new Map,b=0,x=0;for(let[e,t]of m.entries()){let r=new Set(t.aliases),i=t.nodes.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||ua(e)).map(e=>e.alias).sort();i.length===0&&t.aliases.length>0&&i.push(t.aliases[0]);let a=new Map,o=[...i];i.forEach(e=>a.set(e,0));let s=0;for(;s<o.length;){let e=o[s++],t=a.get(e)??0;for(let n of l.get(e)??[])r.has(n)&&(p.has(`${e}\t${n}`)||(!a.has(n)||a.get(n)<=t)&&(a.set(n,t+1),o.push(n)))}let c=a.size>0?Math.max(...a.values()):0;for(let e of t.aliases)a.has(e)||a.set(e,c+1);let u=Na(a,_[e],p,n),f=x;for(let[e,t]of[...u].sort(([e],[t])=>e-t)){let n=-(t.reduce((e,t)=>e+t.height,0)+Math.max(0,t.length-1)*ji)/2,r=b+e,i=x+e*360;f=Math.max(f,i);for(let e of t)e.alias&&(v.set(e.alias,r),y.set(e.alias,{x:i,y:n})),n+=e.height+ji}let m=a.size>0?Math.max(...a.values()):0;b+=m+1,x=f+Oi+Mi}Fa(y,v,t,n);let S=0;for(let[e,t]of y)S=Math.max(S,t.y+(n.get(e)??ki));let C=S+(y.size>0?Ni:0),w=new Map;for(let e of ca)w.set(e,[]);w.set(`__unknown__`,[]);for(let e of a){let t=o.get(e.alias);w.get(t).push(e.alias)}for(let e of[...ca,`__unknown__`]){let t=(w.get(e)??[]).slice().sort();if(t.length===0)continue;let r=t.reduce((e,t)=>Math.max(e,n.get(t)??ki),0);t.forEach((e,t)=>{y.set(e,{x:0+t*360,y:C})}),C+=r+Pi}return{positions:y,levelOf:v}}function Ra(e,t,n){let r=new Map,i=new Map;for(let e of t)r.set(e.source,(r.get(e.source)??0)+1),i.set(e.target,(i.get(e.target)??0)+1);return new Map(e.map(e=>{let t=ea(Math.max(r.get(e.alias)??0,i.get(e.alias)??0),n?Ai:ki);return[e.alias,n?t:Math.max(t,oa(e))]}))}function za(e,t,n={}){let r=e.connections??[],i=Ra(e.nodes,r,n.compactNodes===!0);for(let[e,n]of t)i.has(e)&&Number.isFinite(n)&&n>0&&i.set(e,n);return La(e.nodes,r,i).positions}function Ba(e,t={}){let n=e.connections??[],r=t.supportsConnectionAuthoring===!0,i=t.compactNodes===!0,a=Ra(e.nodes,n,i),{positions:o,levelOf:s}=La(e.nodes,n,a),c=new Set;for(let[e,t]of n.entries()){let n=s.get(t.source),r=s.get(t.target);n!==void 0&&r!==void 0&&n>=r&&c.add(e)}let l=new Map,d=new Map;for(let t of e.nodes)l.set(t.alias,[]),d.set(t.alias,[]);for(let[e,t]of n.entries()){let n=Sa(t);c.has(e)?(d.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!0,stableKey:n}),l.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!0,stableKey:n})):(l.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!1,stableKey:n}),d.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!1,stableKey:n}))}let f=e=>o.get(e)?.y??0,p=(e,t)=>f(e.peerAlias)-f(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connIndex-t.connIndex;for(let e of l.values())e.sort(p);for(let e of d.values())e.sort(p);let m=new Map,h=new Map,g=e.nodes.map(e=>{let t=l.get(e.alias)??[],n=d.get(e.alias)??[],s=ea(Math.max(t.length,n.length),i?Ai:ki),c=Math.max(s,a.get(e.alias)??ki),u=[],f=[],p=0,g=0;for(let e=0;e<t.length;e++){let n=t[e],r=$i(e,t.length);if(n.isBack){let e=Qi(g++);f.push({id:e,offset:r}),h.set(n.connIndex,e)}else{let e=Yi(p++);u.push({id:e,offset:r}),m.set(n.connIndex,e)}}let _=[],v=[],y=0,b=0;for(let e=0;e<n.length;e++){let t=n[e],r=$i(e,n.length);if(t.isBack){let e=Zi(b++);v.push({id:e,offset:r}),m.set(t.connIndex,e)}else{let e=Xi(y++);_.push({id:e,offset:r}),h.set(t.connIndex,e)}}return{id:e.alias,type:e.types[0]??`default`,className:`nokey`,position:o.get(e.alias)??{x:0,y:0},width:Oi,...i?{height:c}:{initialHeight:c},style:{...mi(e.types[0]??`unknown`),minHeight:s},data:{alias:e.alias,nodeType:e.types[0]??`unknown`,properties:e.properties,sourceHandles:u,targetHandles:_,backSourceHandles:v,backTargetHandles:f,supportsConnectionAuthoring:r,compact:i,minHeight:s}}}),_=[];for(let[e,t]of n.entries()){let n=t.relations.map(e=>e.type),r=`${t.source}__${t.target}__${e}`,i=Ji(n);_.push({id:r,source:t.source,target:t.target,sourceHandle:m.get(e),targetHandle:h.get(e),label:n.join(`, `),type:`bezier`,markerEnd:{type:u.ArrowClosed,width:16,height:16,color:Wi},style:{stroke:Wi,strokeWidth:2},labelStyle:{fill:i,fontSize:10,fontWeight:700},labelBgStyle:{fill:Gi,fillOpacity:.94,stroke:`rgba(15, 23, 42, 0.16)`,strokeWidth:1},labelBgPadding:[5,2],labelBgBorderRadius:6,data:{relationTypes:n}})}return{nodes:g,edges:_}}var Va=`application/x-minigraph-clipboard-item`;function Ha(e){return e.includes(Va)}function Ua(e,t){e.effectAllowed=`copy`,e.setData(Va,t)}function Wa(e){let t=e?.getData(`application/x-minigraph-clipboard-item`)??``;return t.trim()?t:null}function Ga(e,t){return e.nodes.find(e=>e.alias===t)}function Ka(e,t){return(e.connections??[]).filter(e=>e.source!==e.target&&(e.source===t||e.target===t))}var qa={viewBox:`0 0 16 16`,width:16,height:16,fill:`none`,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,focusable:`false`};function Ja(e){return(0,F.jsxs)(`svg`,{...qa,...e,children:[(0,F.jsx)(`path`,{d:`M8 1.75 13.25 4.75v6.5L8 14.25l-5.25-3v-6.5L8 1.75Z`}),(0,F.jsx)(`path`,{d:`m2.95 4.9 5.05 2.9 5.05-2.9M8 7.8v6.15`})]})}function Ya(e){return(0,F.jsx)(`svg`,{...qa,...e,children:(0,F.jsx)(`path`,{d:`m5.25 3 6.5 5-6.5 5V3Z`})})}function Xa(e){return(0,F.jsxs)(`svg`,{...qa,...e,children:[(0,F.jsx)(`rect`,{x:`5`,y:`2`,width:`8.5`,height:`9`,rx:`1.25`}),(0,F.jsx)(`path`,{d:`M10.75 14h-7A1.25 1.25 0 0 1 2.5 12.75V6A1.25 1.25 0 0 1 3.75 4.75H5`})]})}var Za={toolbar:`_toolbar_stok4_2`,nameGroup:`_nameGroup_stok4_17`,graphName:`_graphName_stok4_24`,stats:`_stats_stok4_33`,toolbarActions:`_toolbarActions_stok4_53`,runControls:`_runControls_stok4_59`,tooltipAnchor:`_tooltipAnchor_stok4_65`,actionTooltip:`_actionTooltip_stok4_76`,toolbarButton:`_toolbarButton_stok4_116`,toolbarIconButton:`_toolbarIconButton_stok4_134`,toolbarIcon:`_toolbarIcon_stok4_134`};function Qa({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i}){let a=(0,A.useCallback)(()=>{e&&navigator.clipboard.writeText(JSON.stringify(e,null,2)).then(()=>n?.()).catch(()=>r?.())},[e,n,r]),o=e?.nodes.length??0,s=(e?.connections??[]).length;return(0,F.jsxs)(`div`,{className:Za.toolbar,children:[(0,F.jsxs)(`div`,{className:Za.nameGroup,children:[(0,F.jsx)(`span`,{className:Za.graphName,children:t??`Untitled`}),(0,F.jsxs)(`span`,{className:Za.stats,children:[o,` node`,o===1?``:`s`,` · `,s,` connection`,s===1?``:`s`]})]}),(0,F.jsxs)(`div`,{className:Za.toolbarActions,children:[i,(0,F.jsx)(`button`,{type:`button`,className:`${Za.toolbarButton} ${Za.toolbarIconButton}`,onClick:a,title:`Copy raw graph JSON to clipboard`,"aria-label":`Copy raw graph JSON to clipboard`,children:(0,F.jsx)(Xa,{className:Za.toolbarIcon,"aria-hidden":`true`,focusable:`false`})})]})]})}var $a=new Set([`instantiating`,`requesting-input`,`awaiting-input`,`outcome-uncertain`]);function eo(e){switch(e){case`instantiating`:return`Instantiating…`;case`requesting-input`:return`Preparing…`;case`awaiting-input`:return`Input required`;case`outcome-uncertain`:return`Waiting…`;default:return`Instantiate`}}function to(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`requesting-input`:return`Graph input is being prepared`;case`awaiting-input`:return`Graph is waiting for input`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return`Instantiate graph`}}function no(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`requesting-input`:return`Preparing graph input`;case`awaiting-input`:return`Complete or cancel the mock graph input panel`;case`running`:return`Graph is running`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return``}}function ro(e,t){return t?`${e} ${/[.!?]$/.test(t)?t:`${t}.`}`:e}function io({id:e,text:t,keyboardFallback:n,children:r}){let[i,a]=(0,A.useState)(!1),[o,s]=(0,A.useState)(!1);return(0,F.jsxs)(`span`,{className:Za.tooltipAnchor,onMouseEnter:()=>a(!0),onMouseLeave:()=>a(!1),onFocusCapture:()=>s(!0),onBlurCapture:e=>{e.currentTarget.contains(e.relatedTarget)||s(!1)},tabIndex:n?0:void 0,"aria-describedby":n?e:void 0,children:[r,(0,F.jsx)(`span`,{id:e,className:Za.actionTooltip,role:`tooltip`,"data-state":i||o?`open`:`closed`,children:t})]})}function ao({phase:e,canInstantiate:t,canRun:n,disabledReason:r,onInstantiate:i,onRun:a}){let o=(0,A.useId)(),s=`${o}-instantiate`,c=`${o}-run`,l=$a.has(e),u=no(e),d=r||u||(e===`ready`?`The graph is already instantiated`:``),f=r||u||(n?``:`Instantiate the graph first`),p=ro(`Prepare the current graph to run and add input if required.`,d),m=ro(`Run the graph after it has been instantiated.`,f);return(0,F.jsxs)(`div`,{className:Za.runControls,role:`group`,"aria-label":`Graph run controls`,children:[(0,F.jsx)(io,{id:s,text:p,keyboardFallback:!t,children:(0,F.jsxs)(`button`,{type:`button`,className:Za.toolbarButton,onClick:i,disabled:!t,"aria-label":to(e),"aria-describedby":s,"aria-busy":l,children:[(0,F.jsx)(Ja,{className:Za.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,F.jsx)(`span`,{children:eo(e)})]})}),(0,F.jsx)(io,{id:c,text:m,keyboardFallback:!n,children:(0,F.jsxs)(`button`,{type:`button`,className:Za.toolbarButton,onClick:a,disabled:!n,"aria-label":e===`ready`?`Run instantiated graph`:e===`running`?`Graph is running`:`Run graph`,"aria-describedby":c,"aria-busy":e===`running`,children:[(0,F.jsx)(Ya,{className:Za.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,F.jsx)(`span`,{children:e===`running`?`Running…`:`Run`})]})})]})}var oo={menu:`_menu_13qxg_1`,menuItem:`_menuItem_13qxg_12`};function so({open:e,x:t,y:n,canCreateNode:r,onCreateNode:i,onClose:a}){let o=(0,A.useRef)(null),s=(0,A.useRef)(null);return(0,A.useEffect)(()=>{if(!e)return;s.current?.focus();let t=e=>{o.current&&!o.current.contains(e.target)&&a()},n=e=>{e.key===`Escape`&&(e.preventDefault(),a())};return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n)}},[e,a]),e?(0,F.jsx)(`div`,{ref:o,className:oo.menu,style:{left:t,top:n},role:`menu`,"aria-label":`Graph actions`,children:(0,F.jsx)(`button`,{ref:s,role:`menuitem`,type:`button`,className:oo.menuItem,disabled:!r,onClick:()=>{r&&(i(),a())},children:`Create Node`})}):null}var co={menu:`_menu_1trgd_1`,menuItem:`_menuItem_1trgd_12`,dangerItem:`_dangerItem_1trgd_38`,confirmation:`_confirmation_1trgd_51`,confirmationText:`_confirmationText_1trgd_57`,confirmationActions:`_confirmationActions_1trgd_65`},lo=8;function uo(e){let{open:t,x:n,y:r,onClose:i}=e,[a,o]=(0,A.useState)(!1),[s,c]=(0,A.useState)({left:n,top:r}),l=(0,A.useRef)(null),u=(0,A.useRef)(null),d=(0,A.useRef)(null),f=e.mode===`multi-node`?e.selectedCount:null,p=f!==null&&f>1,m=e.mode===`multi-node`?p&&e.canClipSelectedNodes:e.canClipNode,h=e.mode===`single-node`&&e.canConnectNode,g=e.mode===`single-node`&&e.canEditNode,_=e.mode===`multi-node`?p&&e.canDeleteSelectedNodes:e.canDeleteNode,v=m||h||g||_,y=p?`${f} selected nodes`:e.mode===`single-node`?e.nodeAlias:``;return(0,A.useLayoutEffect)(()=>{t&&o(!1)},[t,y,n,r]),(0,A.useLayoutEffect)(()=>{if(!t)return;let e=l.current;if(!e){c({left:n,top:r});return}let i=e.getBoundingClientRect(),a=Math.max(lo,window.innerWidth-i.width-lo),o=Math.max(lo,window.innerHeight-i.height-lo);c({left:Math.min(Math.max(n,lo),a),top:Math.min(Math.max(r,lo),o)})},[m,_,g,a,t,y,n,r]),(0,A.useEffect)(()=>{if(!t){o(!1);return}a?d.current?.focus():u.current?.focus()},[a,t]),(0,A.useEffect)(()=>{if(!t)return;let e=e=>{l.current&&!l.current.contains(e.target)&&i()},n=e=>{e.key===`Escape`&&(e.preventDefault(),i())},r=()=>i();return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[i,t]),!t||!v?null:(0,F.jsx)(`div`,{ref:l,className:co.menu,style:{left:s.left,top:s.top},role:`menu`,"aria-label":p?`Actions for ${f} selected nodes`:`Node actions for ${y}`,children:a?(0,F.jsxs)(`div`,{className:co.confirmation,role:`group`,"aria-label":`Confirm delete ${y}`,children:[(0,F.jsx)(`div`,{className:co.confirmationText,children:p?`Delete ${f} selected nodes?`:`Delete "${y}"?`}),(0,F.jsxs)(`div`,{className:co.confirmationActions,children:[(0,F.jsx)(`button`,{ref:d,type:`button`,className:`${co.menuItem} ${co.dangerItem}`,onClick:()=>{e.mode===`multi-node`?e.onDeleteSelectedNodes():e.onDeleteNode(),i()},children:`Delete`}),(0,F.jsx)(`button`,{type:`button`,className:co.menuItem,onClick:()=>o(!1),children:`Cancel`})]})]}):(0,F.jsxs)(F.Fragment,{children:[m&&(0,F.jsx)(`button`,{ref:u,role:`menuitem`,type:`button`,className:co.menuItem,onClick:()=>{e.mode===`multi-node`?e.onClipSelectedNodes():e.onClipNode(),i()},children:p?`Clip ${f} selected nodes to Workspace`:`Clip to Workspace`}),h&&e.mode===`single-node`&&(0,F.jsx)(`button`,{ref:m?void 0:u,role:`menuitem`,type:`button`,className:co.menuItem,onClick:()=>{e.onConnectNode(),i()},children:`Connect to…`}),g&&e.mode===`single-node`&&(0,F.jsx)(`button`,{ref:m||h?void 0:u,role:`menuitem`,type:`button`,className:co.menuItem,onClick:()=>{e.onEditNode(),i()},children:`Edit Node`}),_&&(0,F.jsx)(`button`,{ref:!m&&!h&&!g?u:void 0,role:`menuitem`,type:`button`,className:`${co.menuItem} ${co.dangerItem}`,onClick:()=>o(!0),children:p?`Delete ${f} selected nodes`:`Delete Node`})]})})}var fo=8;function po({open:e,x:t,y:n,sourceAlias:r,targetAlias:i,relations:a,onDeleteRelation:o,onClose:s}){let[c,l]=(0,A.useState)({left:t,top:n}),u=(0,A.useRef)(null),d=(0,A.useRef)(null);return(0,A.useLayoutEffect)(()=>{if(!e)return;let r=u.current;if(!r){l({left:t,top:n});return}let i=r.getBoundingClientRect(),a=Math.max(fo,window.innerWidth-i.width-fo),o=Math.max(fo,window.innerHeight-i.height-fo);l({left:Math.min(Math.max(t,fo),a),top:Math.min(Math.max(n,fo),o)})},[e,a,t,n]),(0,A.useEffect)(()=>{e&&d.current?.focus()},[e]),(0,A.useEffect)(()=>{if(!e)return;let t=e=>{u.current&&!u.current.contains(e.target)&&s()},n=e=>{e.key===`Escape`&&(e.preventDefault(),s())},r=()=>s();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[s,e]),!e||a.length===0?null:(0,F.jsxs)(`div`,{ref:u,className:co.menu,style:{left:c.left,top:c.top},role:`menu`,"aria-label":`Connection actions for ${r} → ${i}`,children:[a.map((e,t)=>(0,F.jsxs)(`button`,{ref:t===0?d:void 0,role:`menuitem`,type:`button`,className:`${co.menuItem} ${co.dangerItem}`,onClick:()=>{o(e),s()},children:[`Delete '`,e,`'`]},`${e}-${t}`)),a.length>1&&(0,F.jsxs)(`button`,{role:`menuitem`,type:`button`,className:`${co.menuItem} ${co.dangerItem}`,onClick:()=>{o(void 0),s()},children:[`Delete all (`,a.length,`)`]})]})}var mo={island:`_island_had6p_6`,islandDragging:`_islandDragging_had6p_18`,islandGrip:`_islandGrip_had6p_22`,islandGripDots:`_islandGripDots_had6p_46`,islandGripLabel:`_islandGripLabel_had6p_51`,minimap:`_minimap_had6p_55`,controls:`_controls_had6p_59`,toggleButton:`_toggleButton_had6p_63`,toggleIcon:`_toggleIcon_had6p_68`},z={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`},B={left:50,bottom:15},ho=8,go=`graph-minimap-position`;function _o(e){return z[e.type??``]??`#6c7086`}function vo(e){return e instanceof Element&&e.closest(`input, textarea, select, [contenteditable]:not([contenteditable="false"])`)!==null}function yo(e,t,n){if(t.clientWidth<=0||t.clientHeight<=0||n.offsetWidth<=0||n.offsetHeight<=0)return e;let r=Math.max(t.clientWidth-n.offsetWidth-ho,ho),i=Math.max(t.clientHeight-n.offsetHeight-ho,ho);return{left:Math.min(Math.max(e.left,ho),r),bottom:Math.min(Math.max(e.bottom,ho),i)}}function bo({open:e,onOpenChange:t,hotkeyEnabled:n,children:r}){let i=e?`Hide minimap`:`Show minimap`,a=(0,A.useCallback)(()=>{t(!e)},[t,e]);(0,A.useEffect)(()=>{if(!n)return;let e=e=>{e.defaultPrevented||e.repeat||!e.ctrlKey||e.metaKey||e.altKey||e.shiftKey||e.code!==`KeyM`||vo(e.target)||(e.preventDefault(),a())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[n,a]);let[o,s]=fe(go,B),c=(0,A.useRef)(null),[l,u]=(0,A.useState)(null),d=(0,A.useCallback)(e=>{e.pointerType===`mouse`&&e.button!==0||(e.preventDefault(),u({pointerId:e.pointerId,startX:e.clientX,startY:e.clientY,origin:o}))},[o]);return(0,A.useEffect)(()=>{if(l===null)return;let e=e=>{if(e.pointerId!==l.pointerId)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null,r={left:l.origin.left+(e.clientX-l.startX),bottom:l.origin.bottom-(e.clientY-l.startY)};s(t&&n?yo(r,n,t):r)},t=e=>{e.pointerId===l.pointerId&&u(null)};return window.addEventListener(`pointermove`,e),window.addEventListener(`pointerup`,t),window.addEventListener(`pointercancel`,t),()=>{window.removeEventListener(`pointermove`,e),window.removeEventListener(`pointerup`,t),window.removeEventListener(`pointercancel`,t)}},[l,s]),(0,A.useEffect)(()=>{if(!e)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null;if(!t||!n)return;let r=()=>{s(e=>{let r=yo(e,n,t);return r.left===e.left&&r.bottom===e.bottom?e:r})};if(r(),typeof ResizeObserver>`u`)return;let i=new ResizeObserver(r);return i.observe(n),()=>i.disconnect()},[e,s]),(0,F.jsxs)(F.Fragment,{children:[e&&(0,F.jsxs)(`div`,{ref:c,className:l===null?mo.island:`${mo.island} ${mo.islandDragging}`,style:{left:o.left,bottom:o.bottom},role:`group`,"aria-label":`Graph minimap`,children:[(0,F.jsxs)(`div`,{className:mo.islandGrip,role:`button`,"aria-label":`Move minimap`,title:`Drag to move the minimap`,onPointerDown:d,children:[(0,F.jsx)(`span`,{className:mo.islandGripDots,"aria-hidden":`true`,children:`⠿`}),(0,F.jsx)(`span`,{className:mo.islandGripLabel,children:`Minimap`})]}),(0,F.jsx)(g,{className:mo.minimap,nodeColor:_o,maskColor:`rgba(0,0,0,0.3)`,pannable:!0,style:{position:`relative`,margin:0,background:`#fff`}})]}),(0,F.jsxs)(f,{position:`bottom-left`,showInteractive:!1,className:mo.controls,children:[r,(0,F.jsx)(v,{className:`${mo.toggleButton} nodrag nopan`,"aria-label":i,"aria-keyshortcuts":`Control+M`,"aria-pressed":e,title:`${i} (Ctrl + M)`,onClick:a,children:(0,F.jsxs)(`svg`,{className:mo.toggleIcon,viewBox:`1.5 2 17 16`,fill:`none`,"aria-hidden":`true`,focusable:`false`,children:[(0,F.jsx)(`rect`,{x:`2.5`,y:`3`,width:`15`,height:`14`,rx:`1.5`}),(0,F.jsx)(`path`,{d:`M6 12.5 9 8l2.5 2 2.5-3`}),(0,F.jsx)(`circle`,{cx:`6`,cy:`12.5`,r:`1`}),(0,F.jsx)(`circle`,{cx:`9`,cy:`8`,r:`1`}),(0,F.jsx)(`circle`,{cx:`11.5`,cy:`10`,r:`1`}),(0,F.jsx)(`circle`,{cx:`14`,cy:`7`,r:`1`})]})})]})]})}var xo={tip:`_tip_a9o1b_1`,fading:`_fading_a9o1b_11`,tipButton:`_tipButton_a9o1b_15`};function So({visible:e,fading:t,onDismiss:n}){return e?(0,F.jsx)(`div`,{className:`${xo.tip}${t?` ${xo.fading}`:``}`,role:`status`,children:(0,F.jsx)(`button`,{type:`button`,className:xo.tipButton,onClick:n,children:`Multi-select: Shift/Ctrl/Cmd + click nodes, or Shift + drag on the canvas.`})}):null}function Co(e){return e.trim().toLowerCase()}function wo(e){let t=new Set;return e.filter(e=>{let n=Co(e);return t.has(n)?!1:(t.add(n),!0)})}function To(e,t){let n=wo(t),r=Co(e);return n.length>1&&n.some(e=>Co(e)===r)?{kind:`multi-node`,aliases:n}:{kind:`single-node`,alias:e}}function Eo(e,t){let n=new Map(t.nodes.map(e=>[Co(e.alias),e]));return wo(e).map(e=>n.get(Co(e))).filter(e=>e!==void 0)}var Do=[],Oo=[],ko=[`Shift`,`Control`,`Meta`];function Ao(e,t){return e.length===t.length&&e.every((e,n)=>e===t[n])}function jo({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,graphRunControls:i,onRenderError:a,isRefreshing:o=!1,onClipNode:s,onClipNodes:u,onClipboardDrop:d,isActive:f,isConnected:p,supportsAuthoring:g=!1,onCreateNode:_,onCreateConnection:y,onEditNode:S,onDeleteNode:C,onDeleteNodes:w,onDeleteConnections:ee,panelLayoutKey:te}){let[T,E]=(0,A.useState)(null),[ne,D]=(0,A.useState)(null),[re,ie]=(0,A.useState)(null),[ae,oe]=(0,A.useState)([]),[O,se]=(0,A.useState)(!1),[k,ce]=(0,A.useState)(!1),[j,M]=(0,A.useState)(!1),[N,P]=(0,A.useState)(!1),le=(0,A.useRef)(0),ue=(0,A.useRef)(!1),de=(0,A.useRef)(null),pe=!!(g&&_&&p),me=!!(g&&y&&p),he=!!s,ge=!!u,_e=!!(g&&S&&p),ve=!!(g&&C&&p),ye=!!(g&&w&&p),be=he||_e||ve||me,xe=ge||ye,Se=be||xe,Ce=!!(d&&p),we=(0,A.useCallback)(()=>{le.current=0,se(!1)},[]);(0,A.useEffect)(()=>{if(!ne)return;let e=e=>{e.key===`Escape`&&D(null)},t=()=>D(null);return document.addEventListener(`keydown`,e),window.addEventListener(`scroll`,t,!0),window.addEventListener(`resize`,t),()=>{document.removeEventListener(`keydown`,e),window.removeEventListener(`scroll`,t,!0),window.removeEventListener(`resize`,t)}},[ne]),(0,A.useEffect)(()=>{let e=()=>we();return window.addEventListener(`dragend`,e),window.addEventListener(`drop`,e),()=>{window.removeEventListener(`dragend`,e),window.removeEventListener(`drop`,e),we()}},[we]);let Te=(0,A.useRef)(a);(0,A.useEffect)(()=>{Te.current=a},[a]);let[Ee,De]=fe(`graph-nodes-compact`,!1),{nodes:Oe,edges:ke,transformError:Ae}=(0,A.useMemo)(()=>{if(!e)return{nodes:Do,edges:Oo,transformError:null};try{return{...Ba(e,{supportsConnectionAuthoring:me,compactNodes:Ee}),transformError:null}}catch(e){return{nodes:Do,edges:Oo,transformError:e instanceof Error?e.message:String(e)}}},[me,Ee,e]);(0,A.useEffect)(()=>{Ae&&Te.current?.(`Graph render failed: ${Ae}`)},[Ae]);let je=(0,A.useMemo)(()=>e?JSON.stringify(e.nodes.map(e=>e.alias)):`empty`,[e]),[Me,Ne,Pe]=l(Oe),[Fe,Ie,Le]=x(ke),Re=(0,A.useRef)(null),ze=!!(e&&e.nodes.length>0),Be=(0,A.useCallback)(({nodes:e})=>{let t=e.map(e=>e.data.alias);oe(e=>Ao(e,t)?e:t)},[]);(0,A.useEffect)(()=>{Ne(Oe),Ie(ke),oe([]),E(null),ie(null)},[Oe,ke,Ne,Ie]);let Ve=(0,A.useRef)(null),He=(0,A.useRef)(null);(0,A.useEffect)(()=>{if(!e||e.nodes.length===0)return;let t=He.current;if(t&&t.graph===e&&t.compact===Ee)return;let n=new Map(e.nodes.map(e=>[e.alias,e.properties]));if(!(Me.length===n.size&&Me.every(e=>n.get(e.id)===e.data.properties&&e.data.compact===Ee)))return;let r=new Map;for(let e of Me){let t=e.measured?.height;if(typeof t!=`number`)return;r.set(e.id,t)}He.current={graph:e,compact:Ee};let i=za(e,r,{compactNodes:Ee});Me.some(e=>{let t=i.get(e.id);return t!==void 0&&(t.x!==e.position.x||t.y!==e.position.y)})&&Ne(e=>e.map(e=>{let t=i.get(e.id);return t?{...e,position:t}:e})),requestAnimationFrame(()=>{Ve.current?.fitView({padding:.1})})},[Ee,e,Me,Ne]);let Ue=(0,A.useCallback)(()=>{!k||j||(M(!0),de.current!==null&&clearTimeout(de.current),de.current=setTimeout(()=>{ce(!1),de.current=null},400))},[j,k]);(0,A.useEffect)(()=>{!ze||Ae||ue.current||(ue.current=!0,M(!1),ce(!0))},[ze,Ae]),(0,A.useEffect)(()=>{if(!k||j)return;let e=setTimeout(Ue,5e3);return()=>clearTimeout(e)},[Ue,j,k]),(0,A.useEffect)(()=>()=>{de.current!==null&&clearTimeout(de.current)},[]);let We=e=>{Ce&&Ha(Array.from(e.dataTransfer.types))&&(e.preventDefault(),le.current+=1,se(!0))},Ge=e=>{Ce&&Ha(Array.from(e.dataTransfer.types))&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,se(!0))},Ke=e=>{Ha(Array.from(e.dataTransfer.types))&&(le.current=Math.max(0,le.current-1),le.current===0&&se(!1))},qe=e=>{if(!Ce||!Ha(Array.from(e.dataTransfer.types)))return;e.preventDefault();let t=Wa(e.dataTransfer);we(),t&&d?.(t)},Je=(0,A.useMemo)(()=>new Set(e?.nodes.map(e=>e.alias)??[]),[e]),Ye=T?.target.kind===`single-node`&&e?Ga(e,T.target.alias):null,Xe=T?.target.kind===`multi-node`?T.target.aliases:[],Ze=e?Eo(Xe,e):[],Qe=(0,A.useCallback)(e=>!me||!e.source||!e.target||e.source===e.target||!Je.has(e.source)||!Je.has(e.target)?!1:e.sourceHandle===`authoring-source`&&e.targetHandle===`authoring-target`,[me,Je]),$e=(0,A.useRef)(null);(0,A.useEffect)(()=>{let e=e=>{$e.current={x:e.clientX,y:e.clientY}};return document.addEventListener(`pointerup`,e,!0),()=>document.removeEventListener(`pointerup`,e,!0)},[]);let et=(0,A.useCallback)(e=>{Qe(e)&&(!e.source||!e.target||y?.(e.source,e.target,$e.current??void 0))},[Qe,y]),[tt,nt]=(0,A.useState)(null);(0,A.useEffect)(()=>{nt(null)},[e,me]),(0,A.useEffect)(()=>{if(tt===null)return;let e=e=>{e.key===`Escape`&&(e.preventDefault(),nt(null))};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[tt]);let rt=(0,A.useCallback)((e,t)=>{t.handleId!==`authoring-source`||t.handleType!==`source`||(Re.current=t.nodeId,E(null),D(null),ie(null))},[]),it=(0,A.useCallback)(()=>{Re.current=null},[]),at=(0,A.useRef)(te);(0,A.useEffect)(()=>{if(at.current===te||(at.current=te,!ze))return;let e=null,t=requestAnimationFrame(()=>{e=requestAnimationFrame(()=>{Ve.current?.fitView({padding:.1})})});return()=>{cancelAnimationFrame(t),e!==null&&cancelAnimationFrame(e)}},[ze,te]);let ot=!!(g&&ee&&p),st=(0,A.useCallback)(async({edges:e})=>{if(ot&&e.length>0){let t=new Set,n=[];for(let r of e){let e=`${r.source}\t${r.target}`;t.has(e)||(t.add(e),n.push({source:r.source,target:r.target}))}ee?.(n)}return!1},[ot,ee]);return Ae?(0,F.jsxs)(`div`,{className:xi.empty,children:[(0,F.jsx)(`span`,{className:xi.emptyIcon,children:`⚠️`}),(0,F.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,F.jsx)(`span`,{children:Ae})]}):(0,F.jsx)(Si,{onRenderError:a,children:(0,F.jsxs)(`div`,{className:xi.graphWrapper,"aria-busy":o,children:[ze&&e&&(0,F.jsx)(Qa,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i?(0,F.jsx)(ao,{...i}):void 0}),(0,F.jsxs)(`div`,{className:xi.graphSurface,"data-connect-picking":tt!==null||void 0,onDragEnter:We,onDragOver:Ge,onDragLeave:Ke,onDrop:qe,onWheelCapture:Ue,children:[ze?(0,F.jsxs)(c,{nodes:Me,edges:Fe,onInit:e=>{Ve.current=e},onNodesChange:Pe,onEdgesChange:Le,nodesConnectable:me,edgesReconnectable:!1,connectOnClick:!1,isValidConnection:Qe,onConnect:et,onConnectStart:rt,onConnectEnd:it,deleteKeyCode:[`Delete`,`Backspace`],onBeforeDelete:st,nodeTypes:bi,fitView:!0,fitViewOptions:{padding:.1},minZoom:.2,maxZoom:4,zoomOnScroll:!0,zoomOnPinch:!0,zoomOnDoubleClick:!0,panOnScroll:!1,selectionKeyCode:`Shift`,multiSelectionKeyCode:ko,selectionOnDrag:!1,selectionMode:m.Partial,proOptions:{hideAttribution:!1},onSelectionChange:Be,onNodeContextMenu:(e,t)=>{if(e.preventDefault(),e.stopPropagation(),Ue(),D(null),ie(null),!Se)return;let n=To(t.data.alias,ae);n.kind===`single-node`&&ae.length>1&&(Ne(e=>e.map(e=>({...e,selected:e.data.alias===t.data.alias}))),oe([t.data.alias])),E({x:e.clientX,y:e.clientY,target:n})},onEdgeContextMenu:(e,t)=>{e.preventDefault(),e.stopPropagation(),Ue(),E(null),D(null),ot&&ie({x:e.clientX,y:e.clientY,source:t.source,target:t.target,relations:t.data?.relationTypes??[]})},onPaneContextMenu:e=>{e.preventDefault(),Ue(),E(null),ie(null),pe&&D({x:e.clientX,y:e.clientY})},onPaneClick:()=>{Ue(),E(null),D(null),ie(null),nt(null)},onNodeClick:(e,t)=>{if(Ue(),tt===null)return;e.preventDefault(),e.stopPropagation();let n=t.data.alias;n!==tt&&y?.(tt,n,{x:e.clientX,y:e.clientY}),nt(null)},onNodeDragStart:()=>Ue(),onSelectionStart:()=>Ue(),onMoveStart:e=>{e&&Ue()},children:[(0,F.jsx)(b,{variant:h.Dots,gap:18,size:1,color:`rgba(255,255,255,0.07)`}),(0,F.jsx)(bo,{open:N,onOpenChange:P,hotkeyEnabled:f,children:(0,F.jsx)(v,{onClick:()=>De(e=>!e),title:Ee?`Show node details`:`Show thumbnail nodes`,"aria-label":Ee?`Show node details`:`Show thumbnail nodes`,"aria-pressed":Ee,children:(0,F.jsxs)(`span`,{className:xi.detailToggleIcon,"aria-hidden":`true`,children:[(0,F.jsx)(`span`,{className:xi.detailToggleHeader}),Ee&&(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(`span`,{className:xi.detailToggleLine}),(0,F.jsx)(`span`,{className:xi.detailToggleLine}),(0,F.jsx)(`span`,{className:xi.detailToggleLine})]})]})})})]}):(0,F.jsxs)(`div`,{className:xi.empty,children:[(0,F.jsx)(`span`,{className:xi.emptyIcon,children:`🕸️`}),(0,F.jsx)(`span`,{children:`No graph data yet.`}),(0,F.jsxs)(`span`,{children:[`Run `,(0,F.jsx)(`strong`,{children:`describe graph`}),` or `,(0,F.jsx)(`strong`,{children:`export graph`}),` in the playground.`]}),g&&_&&(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(`button`,{type:`button`,className:xi.emptyCreateButton,disabled:!p,onClick:()=>_(`empty-graph`),children:`Create Node`}),!p&&(0,F.jsx)(`span`,{className:xi.emptyHint,children:`Connect WebSocket to create a node.`})]})]}),(0,F.jsx)(So,{visible:k,fading:j,onDismiss:Ue}),tt!==null&&(0,F.jsxs)(`div`,{className:xi.connectBanner,role:`status`,children:[(0,F.jsxs)(`span`,{children:[`Connecting from `,(0,F.jsx)(`strong`,{children:tt}),` — click a target node`]}),(0,F.jsx)(`button`,{type:`button`,className:xi.connectBannerCancel,onClick:()=>nt(null),children:`Cancel (Esc)`})]}),o&&(0,F.jsx)(`div`,{className:xi.refreshingOverlay,children:(0,F.jsx)(`div`,{className:xi.refreshingSpinner,role:`status`,"aria-label":`Graph refreshing`})}),O&&(0,F.jsx)(`div`,{className:xi.clipboardDropOverlay,children:(0,F.jsx)(`div`,{className:xi.clipboardDropMessage,children:`Drop to paste workspace node`})}),(0,F.jsx)(so,{open:ne!==null,x:ne?.x??0,y:ne?.y??0,canCreateNode:pe,onCreateNode:()=>_?.(`pane-context-menu`),onClose:()=>D(null)}),T?.target.kind===`multi-node`?(0,F.jsx)(uo,{mode:`multi-node`,open:Ze.length>1&&xe,x:T.x,y:T.y,selectedCount:Xe.length,canClipSelectedNodes:ge,canDeleteSelectedNodes:ye,onClipSelectedNodes:()=>{if(!e){u?.([]);return}let t=Ze.map(t=>({node:t,connections:Ka(e,t.alias)}));u?.(t)},onDeleteSelectedNodes:()=>{let e=Ze.length===Xe.length;w?.(e?Ze:[])},onClose:()=>E(null)}):(0,F.jsx)(uo,{mode:`single-node`,open:T!==null&&Ye!==null&&be,x:T?.x??0,y:T?.y??0,nodeAlias:T?.target.kind===`single-node`?T.target.alias:``,canClipNode:he&&Ye!==null,canConnectNode:me&&Ye!==null,canEditNode:_e&&Ye!==null,canDeleteNode:ve&&Ye!==null,onConnectNode:()=>{Ye&&nt(Ye.alias)},onClipNode:()=>{if(!Ye||!e)return;let t=Ka(e,Ye.alias);s?.(Ye,t)},onEditNode:()=>{Ye&&S?.(Ye)},onDeleteNode:()=>{Ye&&C?.(Ye)},onClose:()=>E(null)}),(0,F.jsx)(po,{open:re!==null&&ot,x:re?.x??0,y:re?.y??0,sourceAlias:re?.source??``,targetAlias:re?.target??``,relations:re?.relations??[],onDeleteRelation:e=>{re&&ee?.([{source:re.source,target:re.target,relation:e}])},onClose:()=>ie(null)})]})]})},je)}var V={root:`_root_1yhjs_2`,empty:`_empty_1yhjs_10`,emptyIcon:`_emptyIcon_1yhjs_23`,toolbarButton:`_toolbarButton_1yhjs_29 _toolbarButton_stok4_116`,scrollBody:`_scrollBody_1yhjs_34`,jsonContainer:`_jsonContainer_1yhjs_45`,jsonLabel:`_jsonLabel_1yhjs_46`,jsonString:`_jsonString_1yhjs_47`,jsonNumber:`_jsonNumber_1yhjs_48`,jsonBoolean:`_jsonBoolean_1yhjs_49`,jsonNull:`_jsonNull_1yhjs_50`},Mo={default:e=>e<3,all:i,none:a};function No({graphData:e,graphName:t,onCopySuccess:n,onCopyError:i}){let[a,s]=(0,A.useState)(`all`);return e?(0,F.jsxs)(`div`,{className:V.root,children:[(0,F.jsx)(Qa,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,extraActions:(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(`button`,{className:V.toolbarButton,onClick:()=>s(`all`),title:`Expand all nodes`,"aria-label":`Expand all JSON nodes`,"aria-pressed":a===`all`,children:`➖`}),(0,F.jsx)(`button`,{className:V.toolbarButton,onClick:()=>s(`none`),title:`Collapse all nodes`,"aria-label":`Collapse all JSON nodes`,"aria-pressed":a===`none`,children:`➕`})]})}),(0,F.jsx)(`div`,{className:V.scrollBody,children:(0,F.jsx)(o,{data:e,shouldExpandNode:Mo[a],style:{...r,container:`${r.container} ${V.jsonContainer}`,label:V.jsonLabel,stringValue:V.jsonString,numberValue:V.jsonNumber,booleanValue:V.jsonBoolean,nullValue:V.jsonNull}})})]}):(0,F.jsx)(`div`,{className:V.root,children:(0,F.jsxs)(`div`,{className:V.empty,children:[(0,F.jsx)(`span`,{className:V.emptyIcon,children:`🕸️`}),(0,F.jsx)(`span`,{children:`No graph data yet.`}),(0,F.jsx)(`span`,{children:`Pin a graph-link message in the Console to load the raw data here.`})]})})}var Po={rightPanel:`_rightPanel_xa3j1_2`,tabStrip:`_tabStrip_xa3j1_10`,tab:`_tab_xa3j1_10`,tabActive:`_tabActive_xa3j1_41`,tabBadge:`_tabBadge_xa3j1_45`,tabBody:`_tabBody_xa3j1_51`,tabBodyHidden:`_tabBodyHidden_xa3j1_64`,graphContent:`_graphContent_xa3j1_68`,rightPanelGroup:`_rightPanelGroup_xa3j1_75`,verticalResizeHandle:`_verticalResizeHandle_xa3j1_83`},Fo=`help-split-percent`,Io=`help-split-maximized`,Lo=45,Ro=98;function zo({tabs:e,payload:t,onChange:n,validation:r,onFormat:i,onUpload:a,graphData:o,graphName:s,activeTab:c,onTabChange:l,onGraphRenderError:u,onGraphDataCopySuccess:d,onGraphDataCopyError:f,graphRunControls:p,isGraphRefreshing:m,onClipNode:h,onClipNodes:g,onClipboardDrop:_,isConnected:v,supportsAuthoring:y,onCreateNode:b,onCreateConnection:x,onEditNode:S,onDeleteNode:C,onDeleteNodes:w,onDeleteConnections:ee,panelLayoutKey:te,helpPanel:T}){let E=(0,A.useId)(),ne=`${E}-tab-payload`,D=`${E}-tab-graph`,ae=`${E}-tab-graph-data`,O=!!T,se=(0,A.useRef)(Number(sessionStorage.getItem(Fo))||Lo),k=(0,A.useRef)(null),ce=(0,A.useRef)(null),[j,M]=(0,A.useState)(()=>sessionStorage.getItem(Io)===`1`),N=(0,A.useRef)(j),P=(0,F.jsxs)(`div`,{className:Po.rightPanel,children:[e.length>1&&(0,F.jsxs)(`div`,{className:Po.tabStrip,role:`tablist`,"aria-label":`Right panel tabs`,children:[e.includes(`payload`)&&(0,F.jsx)(`button`,{role:`tab`,"aria-selected":c===`payload`,"aria-controls":ne,className:`${Po.tab}${c===`payload`?` ${Po.tabActive}`:``}`,onClick:()=>l(`payload`),children:`Payload Editor`}),e.includes(`graph`)&&(0,F.jsxs)(`button`,{role:`tab`,"aria-selected":c===`graph`,"aria-controls":D,className:`${Po.tab}${c===`graph`?` ${Po.tabActive}`:``}`,onClick:()=>l(`graph`),children:[`Graph`,o!==null&&(0,F.jsx)(`span`,{className:Po.tabBadge,"aria-label":`Graph data available`,children:`🕸️`})]}),e.includes(`graph-data`)&&(0,F.jsx)(`button`,{role:`tab`,"aria-selected":c===`graph-data`,"aria-controls":ae,className:`${Po.tab}${c===`graph-data`?` ${Po.tabActive}`:``}`,onClick:()=>l(`graph-data`),children:`Graph Data (Raw)`})]}),e.includes(`payload`)&&(0,F.jsx)(`div`,{role:`tabpanel`,id:ne,tabIndex:c===`payload`?0:-1,className:`${Po.tabBody}${c===`payload`?``:` ${Po.tabBodyHidden}`}`,children:(0,F.jsx)(si,{payload:t,onChange:n,validation:r,onFormat:i,onUpload:a})}),e.includes(`graph`)&&(0,F.jsx)(`div`,{role:`tabpanel`,id:D,tabIndex:c===`graph`?0:-1,className:`${Po.tabBody}${c===`graph`?``:` ${Po.tabBodyHidden}`}`,children:(0,F.jsx)(`div`,{className:Po.graphContent,children:(0,F.jsx)(jo,{graphData:o,graphName:s,onRenderError:u,isRefreshing:m,onCopySuccess:d,onCopyError:f,graphRunControls:p,onClipNode:h,onClipNodes:g,onClipboardDrop:_,isActive:c===`graph`,isConnected:v,supportsAuthoring:y,onCreateNode:b,onCreateConnection:x,onEditNode:S,onDeleteNode:C,onDeleteNodes:w,onDeleteConnections:ee,panelLayoutKey:te})})}),e.includes(`graph-data`)&&(0,F.jsx)(`div`,{role:`tabpanel`,id:ae,tabIndex:c===`graph-data`?0:-1,className:`${Po.tabBody}${c===`graph-data`?``:` ${Po.tabBodyHidden}`}`,children:(0,F.jsx)(No,{graphData:o,graphName:s,onCopySuccess:d,onCopyError:f})})]}),le=(0,A.useCallback)(e=>{let t=e[`help-split-help`];if(t===void 0)return;let n=t>=Ro;n!==N.current&&(N.current=n,M(n),sessionStorage.setItem(Io,n?`1`:`0`)),n||(se.current=t,sessionStorage.setItem(Fo,String(t)))},[]),ue=(0,A.useCallback)(()=>{let e=!N.current;if(N.current=e,M(e),sessionStorage.setItem(Io,e?`1`:`0`),e)ce.current?.resize(`0%`),k.current?.resize(`100%`);else{let e=se.current;k.current?.resize(`${e}%`),ce.current?.resize(`${100-e}%`)}},[]);if((0,A.useEffect)(()=>{O&&N.current&&requestAnimationFrame(()=>{ce.current?.resize(`0%`),k.current?.resize(`100%`)})},[O]),!T)return P;let de=typeof T==`function`?T(ue,j):T,fe=N.current?100:se.current,pe=100-fe;return(0,F.jsxs)(oe,{orientation:`vertical`,className:Po.rightPanelGroup,onLayoutChanged:le,children:[(0,F.jsx)(ie,{panelRef:ce,defaultSize:`${pe}%`,minSize:`0%`,children:P}),(0,F.jsx)(re,{className:Po.verticalResizeHandle,"aria-label":`Resize help panel`}),(0,F.jsx)(ie,{id:`help-split-help`,panelRef:k,defaultSize:`${fe}%`,minSize:`15%`,children:de})]})}var Bo=class extends A.Component{constructor(...e){super(...e),this.state={hasError:!1}}static getDerivedStateFromError(){return{hasError:!0}}componentDidCatch(e,t){console.error(`[ConsoleErrorBoundary] Failed to render message:`,e,t.componentStack)}render(){return this.state.hasError?(0,F.jsx)(`span`,{children:this.props.fallback}):this.props.children}},Vo=2e3,Ho=(e={})=>{let{onSuccess:t,onError:n}=e,[r,i]=(0,A.useState)(!1),a=(0,A.useRef)(null);return(0,A.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current)},[]),{copy:(0,A.useCallback)(async e=>{if(!navigator.clipboard)return console.warn(`useCopyToClipboard: Clipboard API not available in this browser.`),n?.(),!1;try{return await navigator.clipboard.writeText(e),i(!0),a.current!==null&&clearTimeout(a.current),a.current=setTimeout(()=>{a.current=null,i(!1)},Vo),t?.(),!0}catch(e){return console.error(`useCopyToClipboard: Failed to write to clipboard.`,e),n?.(),!1}},[t,n]),copied:r}},H={consoleRoot:`_consoleRoot_1lgp1_2`,consoleHeader:`_consoleHeader_1lgp1_10`,consoleTitle:`_consoleTitle_1lgp1_20`,consoleControls:`_consoleControls_1lgp1_25`,controlButton:`_controlButton_1lgp1_30`,console:`_console_1lgp1_2`,emptyConsole:`_emptyConsole_1lgp1_67`,consoleMessage:`_consoleMessage_1lgp1_80`,consoleMessageActivatable:`_consoleMessageActivatable_1lgp1_94`,consoleMessageGraphLink:`_consoleMessageGraphLink_1lgp1_104`,consoleMessageLargePayload:`_consoleMessageLargePayload_1lgp1_115`,consoleMessageMockUpload:`_consoleMessageMockUpload_1lgp1_122`,uploadMockButton:`_uploadMockButton_1lgp1_131`,copyButton:`_copyButton_1lgp1_172`,copyButtonCopied:`_copyButtonCopied_1lgp1_225`,sendToJsonPathButton:`_sendToJsonPathButton_1lgp1_234`,messageIcon:`_messageIcon_1lgp1_268`,messageContent:`_messageContent_1lgp1_272`,messageText:`_messageText_1lgp1_278`,messageTime:`_messageTime_1lgp1_283`,"messageType-error":`_messageType-error_1lgp1_290`,"messageType-info":`_messageType-info_1lgp1_291`,"messageType-welcome":`_messageType-welcome_1lgp1_292`,jsonViewWrapper:`_jsonViewWrapper_1lgp1_295`,jsonContainer:`_jsonContainer_1lgp1_301`,jsonLabel:`_jsonLabel_1lgp1_302`,jsonString:`_jsonString_1lgp1_303`,jsonNumber:`_jsonNumber_1lgp1_304`,jsonBoolean:`_jsonBoolean_1lgp1_305`,jsonNull:`_jsonNull_1lgp1_306`};function Uo({message:e,msgId:t,classificationMap:n,onGraphLink:i,onCopyMessage:a,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){let u=Se(e),d=Ce(u.type),f=we(u.message),p=(t===void 0?void 0:n?.get(t))??[],m=p.some(e=>e.kind===`graph.link`),h=p.some(e=>e.kind===`payload.large`),g=p.some(e=>e.kind===`upload.invitation`),_=p.find(e=>e.kind===`upload.invitation`)?.uploadPath??null,v=!!c&&g&&_!==null,y=v&&!!l?.has(_),b=!!i&&m&&!g&&!h,x=!!s&&f.isJSON,{copy:S,copied:C}=Ho({onSuccess:a}),w=t=>{t.stopPropagation(),S(e)},ee=t=>{(t.key===`Enter`||t.key===` `)&&(t.preventDefault(),t.stopPropagation(),S(e))},te=e=>{e.stopPropagation(),!(!s||!f.isJSON)&&s(JSON.stringify(f.data,null,2))},T=e=>{e.stopPropagation(),!(!c||!_)&&c(_)};return(0,F.jsxs)(`div`,{className:[H.consoleMessage,H[`messageType-${u.type}`],b?H.consoleMessageActivatable:``,m?H.consoleMessageGraphLink:``,h?H.consoleMessageLargePayload:``,g?H.consoleMessageMockUpload:``].filter(Boolean).join(` `),onClick:b?()=>i():void 0,title:b?`Click to load graph in Graph View`:void 0,role:b?`button`:void 0,tabIndex:b?0:void 0,onKeyDown:b?e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),i())}:void 0,"aria-label":b?`Load graph in Graph View`:void 0,children:[(0,F.jsx)(`span`,{className:H.messageIcon,children:g?`⬆️`:h?`⬇️`:m?`🕸️`:d}),(0,F.jsx)(`div`,{className:H.messageContent,children:f.isJSON?(0,F.jsx)(`div`,{className:H.jsonViewWrapper,children:(0,F.jsx)(o,{data:f.data,shouldExpandNode:e=>e<1,style:{...r,container:`${r.container} ${H.jsonContainer}`,label:H.jsonLabel,stringValue:H.jsonString,numberValue:H.jsonNumber,booleanValue:H.jsonBoolean,nullValue:H.jsonNull}})}):(0,F.jsxs)(`span`,{className:H.messageText,children:[u.message,y&&(0,F.jsx)(`span`,{title:`Upload succeeded`,children:` ✅`})]})}),(0,F.jsx)(`button`,{className:`${H.copyButton} ${C?H.copyButtonCopied:``}`,onClick:w,onKeyDown:ee,title:C?`Copied!`:`Copy message`,"aria-label":C?`Copied to clipboard`:`Copy message to clipboard`,tabIndex:0,children:C?`✅`:`📄`}),x&&(0,F.jsx)(`button`,{className:H.sendToJsonPathButton,onClick:te,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&te(e)},title:`Open in JSON-Path Playground`,"aria-label":`Open this JSON in the JSON-Path Playground`,tabIndex:0,children:`➡️`}),v&&(0,F.jsx)(`button`,{className:H.uploadMockButton,onClick:T,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&T(e)},title:`Re-open upload dialog`,"aria-label":`Re-open mock data upload dialog`,tabIndex:0,children:`⬆️ Upload JSON…`}),u.time&&(0,F.jsx)(`span`,{className:H.messageTime,children:u.time})]})}function Wo({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){return(0,F.jsxs)(`div`,{className:H.consoleRoot,children:[(0,F.jsxs)(`div`,{className:H.consoleHeader,children:[(0,F.jsx)(`span`,{className:H.consoleTitle,children:`Console Output`}),(0,F.jsxs)(`div`,{className:H.consoleControls,children:[(0,F.jsx)(`button`,{className:H.controlButton,onClick:n,title:`Copy console output`,"aria-label":`Copy console output to clipboard`,children:`📑`}),(0,F.jsx)(`button`,{className:H.controlButton,onClick:r,title:`Clear console`,"aria-label":`Clear console`,children:`🗑️`})]})]}),(0,F.jsxs)(`div`,{className:H.console,ref:i,role:`log`,"aria-live":`polite`,children:[e.map(e=>(0,F.jsx)(Bo,{fallback:e.raw,children:(0,F.jsx)(Uo,{message:e.raw,msgId:e.id,classificationMap:t,onGraphLink:a?()=>a(e):void 0,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l})},e.id)),e.length===0&&(0,F.jsxs)(`div`,{className:H.emptyConsole,children:[`No messages yet. Use the `,(0,F.jsx)(`strong`,{children:`Start`}),` button in the header to connect.`]})]})]})}var Go={commandInput:`_commandInput_188pe_2`,labelRow:`_labelRow_188pe_8`,labelGroup:`_labelGroup_188pe_16`,label:`_label_188pe_8`,infoWrapper:`_infoWrapper_188pe_28`,paletteToggle:`_paletteToggle_188pe_34`,paletteToggleActive:`_paletteToggleActive_188pe_66`,popover:`_popover_188pe_73`,popoverOpen:`_popoverOpen_188pe_95`,popoverTitle:`_popoverTitle_188pe_121`,popoverRow:`_popoverRow_188pe_135`,popoverKeyword:`_popoverKeyword_188pe_156`,popoverDesc:`_popoverDesc_188pe_168`,popoverAlias:`_popoverAlias_188pe_174`,inputRow:`_inputRow_188pe_181`,inputWrapper:`_inputWrapper_188pe_187`,textarea:`_textarea_188pe_197`,sendButton:`_sendButton_188pe_236`,hint:`_hint_188pe_253`,dropup:`_dropup_188pe_261`,dropupHeader:`_dropupHeader_188pe_276`,dropupItem:`_dropupItem_188pe_292`,dropupItemText:`_dropupItemText_188pe_315`,matchHighlight:`_matchHighlight_188pe_323`,multilineIndicator:`_multilineIndicator_188pe_329`},Ko=[`graph.data.mapper`,`graph.math`,`graph.js`,`graph.api.fetcher`,`graph.extension`,`graph.island`,`graph.join`],qo=[{keyword:`help`,description:`List all help topics, or get help for a specific command`,template:`help`},{keyword:`create`,description:`Create a new graph node`,template:`create node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`update`,description:`Update an existing node`,template:`update node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`edit`,description:`Print raw node data ready for editing and re-submitting`,template:`edit node {name}`},{keyword:`delete node`,description:`Delete a node by name`,alias:`clear node`,template:`delete node {name}`},{keyword:`delete connection`,description:`Delete connection(s) between two nodes`,alias:`clear connection`,template:`delete connection {nodeA} and {nodeB}`},{keyword:`delete cache`,description:`Clear cached API fetcher results`,alias:`clear cache`,template:`delete cache`},{keyword:`connect`,description:`Connect two nodes with a named relation`,template:`connect {node-A} to {node-B} with {relation}`},{keyword:`list nodes`,description:`List all nodes in the current graph`,template:`list nodes`},{keyword:`list connections`,description:`List all connections in the current graph`,template:`list connections`},{keyword:`describe graph`,description:`Describe the current graph model`,template:`describe graph`},{keyword:`describe node`,description:`Describe a specific node and its connections`,template:`describe node {name}`},{keyword:`describe connection`,description:`Describe connection(s) between two nodes`,template:`describe connection {nodeA} and {nodeB}`},{keyword:`describe skill`,description:`Show documentation for a skill by route name`,template:`describe skill {skill.route}`},{keyword:`export`,description:`Export the graph model to a JSON file`,template:`export graph as {name}`},{keyword:`import graph`,description:`Import a graph model from a saved file`,template:`import graph from {name}`},{keyword:`import node`,description:`Import a single node from another saved graph`,template:`import node {node-name} from {graph-name}`},{keyword:`instantiate`,description:`Create a runnable graph instance with mock input`,alias:`start`,template:`instantiate graph
{constant} -> input.body.{key}`,multiline:!0},{keyword:`upload mock data`,description:`Print the URL to POST a JSON payload as mock input.body`,template:`upload mock data`},{keyword:`execute`,description:`Execute a single node skill in isolation`,template:`execute node {name}`},{keyword:`inspect`,description:`Inspect a state-machine variable`,template:`inspect {variable_name}`},{keyword:`run`,description:`Run the graph instance from root to end`,template:`run`}];[...Ko.map(e=>({tokens:[`describe`,`skill`,e],template:`describe skill ${e}`,hint:`Describe built-in skill: ${e}`}))];function Jo(e,t){let[n,r]=(0,A.useState)(!1),[i,a]=(0,A.useState)(-1),o=(0,A.useMemo)(()=>{let n=t.trimStart();if(n.length===0)return[];let r=n.toLowerCase(),i=e.filter(e=>e.toLowerCase().startsWith(r)),a=new Set;return i.filter(e=>a.has(e)?!1:(a.add(e),!0)).slice(0,8)},[e,t]),s=()=>{r(!0),a(-1)},c=e=>{let t=o.length;t!==0&&a(n=>e===1?n<0?0:(n+1)%t:n<=0?t-1:n-1)},l=(e,t)=>{e>=0&&e<o.length&&t(o[e]),r(!1),a(-1)};return{suggestions:o,isOpen:n,activeIndex:i,onCommandChange:s,navigate:c,accept:l,onTab:e=>{!n||o.length===0||l(i>=0?i:0,e)},dismiss:()=>{r(!1),a(-1)}}}var Yo=e=>(0,F.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:14,height:14,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,F.jsx)(`polyline`,{points:`2,4 6,8 2,12`}),(0,F.jsx)(`line`,{x1:7,y1:12,x2:14,y2:12})]});function Xo({command:e,onChange:t,onKeyDown:n,onSend:r,sendDisabled:i,disabled:a,history:o}){let s=(0,A.useRef)(null),c=(0,A.useRef)(null),l=(0,A.useRef)(null),[u,d]=(0,A.useState)(!1);(0,A.useEffect)(()=>{if(!u)return;let e=e=>{c.current&&!c.current.contains(e.target)&&d(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[u]);let f=Jo(o,e),p=(0,A.useCallback)(()=>{let e=s.current;e&&(e.style.height=`auto`,e.style.height=`${e.scrollHeight}px`)},[]);(0,A.useEffect)(()=>{p()},[e,p]),(0,A.useEffect)(()=>{let e=s.current?.parentElement;if(!e||typeof ResizeObserver>`u`)return;let t=null,n=new ResizeObserver(()=>{t===null&&(t=requestAnimationFrame(()=>{t=null,p()}))});return n.observe(e),()=>{n.disconnect(),t!==null&&cancelAnimationFrame(t)}},[p]);let m=a?`Not connected`:`Enter command (Enter to send · Shift+Enter for new line)`,h=a?`Enter your test message once it is connected`:`Enter to send · Shift+Enter for new line · ↑↓ for history`;return(0,F.jsxs)(`div`,{className:Go.commandInput,children:[(0,F.jsx)(`div`,{className:Go.labelRow,children:(0,F.jsxs)(`div`,{className:Go.labelGroup,children:[(0,F.jsx)(`label`,{htmlFor:`command`,className:Go.label,children:`Command`}),(0,F.jsxs)(`span`,{ref:c,className:Go.infoWrapper,children:[(0,F.jsx)(`button`,{type:`button`,className:`${Go.paletteToggle}${u?` ${Go.paletteToggleActive}`:``}`,"aria-label":`Toggle command palette`,"aria-expanded":u,"aria-controls":`command-palette`,onClick:()=>d(e=>!e),onKeyDown:e=>{e.key===`ArrowDown`&&u&&(e.preventDefault(),(l.current?.querySelector(`[role="option"]`))?.focus())},title:`Command palette`,children:(0,F.jsx)(Yo,{"aria-hidden":`true`,focusable:`false`})}),(0,F.jsxs)(`div`,{id:`command-palette`,ref:l,className:`${Go.popover}${u?` ${Go.popoverOpen}`:``}`,role:`listbox`,"aria-label":`Command palette`,onKeyDown:e=>{if(e.key===`ArrowDown`||e.key===`ArrowUp`){e.preventDefault();let t=l.current?.querySelectorAll(`[role="option"]`);if(!t||t.length===0)return;let n=Array.from(t).indexOf(document.activeElement);e.key===`ArrowDown`?t[n<0?0:(n+1)%t.length].focus():t[n<=0?t.length-1:n-1].focus()}else e.key===`Escape`&&(e.preventDefault(),d(!1),s.current?.focus())},children:[(0,F.jsx)(`p`,{className:Go.popoverTitle,children:`Command palette — click to insert`}),qo.map(({keyword:e,alias:n,description:r,template:i})=>(0,F.jsxs)(`div`,{className:Go.popoverRow,role:`option`,"aria-selected":!1,tabIndex:u?0:-1,onMouseDown:e=>e.preventDefault(),onClick:()=>{t(i),d(!1),s.current?.focus()},onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),t(i),d(!1),s.current?.focus())},children:[(0,F.jsx)(`span`,{className:Go.popoverKeyword,children:e}),(0,F.jsxs)(`span`,{className:Go.popoverDesc,children:[r,n&&(0,F.jsxs)(`span`,{className:Go.popoverAlias,children:[` · alias: `,n]})]})]},e))]})]})]})}),(0,F.jsxs)(`div`,{className:Go.inputRow,children:[(0,F.jsxs)(`div`,{className:Go.inputWrapper,children:[(0,F.jsxs)(`div`,{id:`history-dropup`,role:`listbox`,"aria-label":`Command history suggestions`,className:Go.dropup,hidden:!(f.isOpen&&f.suggestions.length>0),children:[(0,F.jsx)(`div`,{className:Go.dropupHeader,"aria-hidden":`true`,children:`Recent Commands`}),f.isOpen&&f.suggestions.length>0&&f.suggestions.map((n,r)=>{let i=n.split(`
`)[0],a=n.includes(`
`),o=e.trimStart().split(`
`)[0],c=Math.min(o.length,i.length),l=i.slice(0,c),u=i.slice(c);return(0,F.jsxs)(`div`,{id:`history-option-${r}`,role:`option`,"aria-selected":r===f.activeIndex,className:Go.dropupItem,onMouseDown:e=>e.preventDefault(),onClick:()=>{f.accept(r,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)})},children:[(0,F.jsxs)(`span`,{className:Go.dropupItemText,children:[c>0&&(0,F.jsx)(`strong`,{className:Go.matchHighlight,children:l}),u,a?`…`:``]}),a&&(0,F.jsx)(`span`,{className:Go.multilineIndicator,"aria-label":`multi-line command`,children:`↵`})]},n)})]}),(0,F.jsx)(`textarea`,{ref:s,id:`command`,role:`combobox`,"aria-expanded":f.isOpen&&f.suggestions.length>0,"aria-haspopup":`listbox`,"aria-controls":`history-dropup`,"aria-activedescendant":f.isOpen&&f.suggestions.length>0&&f.activeIndex>=0?`history-option-${f.activeIndex}`:void 0,"aria-autocomplete":`list`,className:Go.textarea,rows:1,placeholder:m,value:e,disabled:a,onChange:e=>{t(e.target.value),f.onCommandChange()},onKeyDown:e=>{if(e.key===`Tab`){e.preventDefault(),f.isOpen&&f.suggestions.length>0&&(f.onTab(e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}));return}if(e.key===`Enter`){if(e.shiftKey)return;if(e.preventDefault(),f.isOpen&&f.activeIndex>=0){f.accept(f.activeIndex,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}),s.current?.focus();return}r(),s.current?.focus();return}if(e.key===`Escape`){if(f.isOpen){f.dismiss(),e.preventDefault();return}return}if(e.key===`ArrowUp`||e.key===`ArrowDown`){if(f.isOpen&&f.suggestions.length>0){e.preventDefault(),f.navigate(e.key===`ArrowDown`?1:-1);return}let t=s.current;if(t){let{selectionStart:n,value:r}=t,i=!r.slice(0,n).includes(`
`),a=!r.slice(n).includes(`
`);if(!(e.key===`ArrowUp`&&i||e.key===`ArrowDown`&&a))return}n(e),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)});return}n(e)},onBlur:()=>f.dismiss(),autoComplete:`off`,autoCorrect:`off`,spellCheck:!1})]}),(0,F.jsx)(`button`,{className:Go.sendButton,onClick:()=>{r(),s.current?.focus()},disabled:i,"aria-label":`Send command`,children:`Send`})]}),h&&(0,F.jsx)(`p`,{className:Go.hint,children:h})]})}var Zo={root:`_root_1ac49_1`};function Qo({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l,command:u,onCommandChange:d,onCommandKeyDown:f,onSend:p,sendDisabled:m,inputDisabled:h,commandHistory:g}){return(0,F.jsxs)(`div`,{className:Zo.root,children:[(0,F.jsx)(Wo,{messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}),(0,F.jsx)(Xo,{command:u,onChange:d,onKeyDown:f,onSend:p,disabled:h,sendDisabled:m,history:g})]})}var $o=e=>(0,F.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:16,height:16,stroke:`currentColor`,strokeWidth:1.8,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,F.jsx)(`line`,{x1:4.75,y1:4.75,x2:11.25,y2:11.25}),(0,F.jsx)(`line`,{x1:11.25,y1:4.75,x2:4.75,y2:11.25})]}),U={root:`_root_1qoh5_9`,card:`_card_1qoh5_19`,ribbon:`_ribbon_1qoh5_32`,ribbonIcon:`_ribbonIcon_1qoh5_45`,ribbonAlias:`_ribbonAlias_1qoh5_50`,ribbonAliasInput:`_ribbonAliasInput_1qoh5_57`,ribbonBadge:`_ribbonBadge_1qoh5_82`,ribbonClose:`_ribbonClose_1qoh5_93`,ribbonCloseIcon:`_ribbonCloseIcon_1qoh5_114`,body:`_body_1qoh5_121`,row:`_row_1qoh5_128`,rowDropTarget:`_rowDropTarget_1qoh5_138`,dragGrip:`_dragGrip_1qoh5_142`,rowLabel:`_rowLabel_1qoh5_166`,rowKey:`_rowKey_1qoh5_167`,rowValue:`_rowValue_1qoh5_178`,rowSpacer:`_rowSpacer_1qoh5_185`,keyInput:`_keyInput_1qoh5_189`,valueInput:`_valueInput_1qoh5_190`,removeButton:`_removeButton_1qoh5_228`,removeIcon:`_removeIcon_1qoh5_251`,addRow:`_addRow_1qoh5_257`,addButton:`_addButton_1qoh5_263`,message:`_message_1qoh5_282`,errorMessage:`_errorMessage_1qoh5_283`,warningMessage:`_warningMessage_1qoh5_284`,errorText:`_errorText_1qoh5_306`,footer:`_footer_1qoh5_312`,secondaryButton:`_secondaryButton_1qoh5_321`,primaryButton:`_primaryButton_1qoh5_322`},es=1,ts=10,ns=36;function rs(e){let t=e.split(`
`).reduce((e,t)=>e+Math.max(1,Math.ceil(t.length/ns)),0);return Math.min(Math.max(t,es),ts)}function is({mode:e,formState:t,phase:n,lockReason:r,serverMessage:i,validationErrors:a,onFormStateChange:o,onSubmit:s,onClose:c}){let l=(0,A.useRef)(null),u=(0,A.useRef)(null),d=(0,A.useRef)(new Map),f=(0,A.useRef)(null),p=e===`create`,m=n===`sending`,h=r===`disconnected`,g=m||h,_=p?`Create Node`:`Save Changes`,v=p?`Creating...`:`Saving...`,y=p?`Connection disconnected. Refresh the page and create the node again after the app reconnects.`:`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,b=fi(t.nodeType),x=pi(t.nodeType);(0,A.useEffect)(()=>{p?l.current?.focus():u.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),m||c())};return document.addEventListener(`keydown`,e),()=>{document.removeEventListener(`keydown`,e)}},[p,c,m]),(0,A.useEffect)(()=>{let e=f.current;if(!e)return;let t=d.current.get(e);t&&(t.focus(),f.current=null)},[t.properties]);let S=(0,A.useCallback)(e=>{e.preventDefault(),!g&&s()},[g,s]),C=(0,A.useCallback)(e=>{o({...t,...e})},[t,o]),w=(0,A.useCallback)((e,n)=>{o({...t,properties:t.properties.map(t=>t.id===e?{...t,...n}:t)})},[t,o]),ee=(0,A.useCallback)(()=>{let e=vr();f.current=e.id,o({...t,properties:[...t.properties,e]})},[t,o]),te=(0,A.useCallback)(e=>{let n=t.properties.filter(t=>t.id!==e);o({...t,properties:n.length>0?n:[vr()]})},[t,o]),T=(0,A.useRef)(null),[E,ne]=(0,A.useState)(null),D=(0,A.useCallback)(()=>{T.current=null,ne(null)},[]),re=(0,A.useCallback)(e=>t=>{if(T.current=e,t.dataTransfer){t.dataTransfer.effectAllowed=`move`,t.dataTransfer.setData(`text/plain`,e);let n=t.currentTarget.closest(`[data-row-id]`);n instanceof HTMLElement&&typeof t.dataTransfer.setDragImage==`function`&&t.dataTransfer.setDragImage(n,16,16)}},[]),ie=(0,A.useCallback)(e=>t=>{T.current!==null&&(t.preventDefault(),t.dataTransfer&&(t.dataTransfer.dropEffect=`move`),ne(t=>t===e?t:e))},[]),ae=(0,A.useCallback)(e=>{let n=T.current;if(D(),n===null||n===e)return;let r=t.properties.find(e=>e.id===n);if(!r)return;let i=t.properties.filter(e=>e.id!==n),a=i.length;if(e!==null){let t=i.findIndex(t=>t.id===e);t!==-1&&(a=t)}let s=[...i.slice(0,a),r,...i.slice(a)];o({...t,properties:Cr(s)})},[D,t,o]),oe=(0,A.useCallback)(e=>t=>{T.current!==null&&(t.preventDefault(),ae(e))},[ae]);return(0,F.jsx)(`div`,{className:U.root,children:(0,F.jsxs)(`form`,{className:U.card,style:{borderColor:x,"--node-accent":x},"aria-label":p?`Create node`:`Edit node ${t.alias}`,onSubmit:S,children:[(0,F.jsxs)(`header`,{className:U.ribbon,children:[(0,F.jsx)(`span`,{className:U.ribbonIcon,"aria-hidden":`true`,children:b.icon}),p?(0,F.jsx)(`input`,{ref:l,className:U.ribbonAliasInput,value:t.alias,placeholder:`node-alias`,"aria-label":`Node alias`,disabled:g,"aria-invalid":!!a.alias,onChange:e=>C({alias:e.target.value})}):(0,F.jsx)(`span`,{className:U.ribbonAlias,children:t.alias}),(0,F.jsx)(`span`,{className:U.ribbonBadge,children:b.label}),(0,F.jsx)(`button`,{type:`button`,className:U.ribbonClose,"aria-label":`Close node editor`,title:`Close (Esc)`,onClick:c,disabled:m,children:(0,F.jsx)($o,{className:U.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,F.jsxs)(`div`,{className:U.body,children:[i&&!h&&(0,F.jsx)(`div`,{className:U.message,role:`status`,children:i}),a.command&&(0,F.jsx)(`div`,{className:U.errorMessage,role:`alert`,children:a.command}),a.alias&&(0,F.jsx)(`div`,{className:U.errorMessage,role:`alert`,children:a.alias}),h&&(0,F.jsx)(`div`,{className:U.warningMessage,role:`status`,children:i??y}),(0,F.jsxs)(`div`,{className:U.row,children:[(0,F.jsx)(`span`,{"aria-hidden":`true`}),(0,F.jsx)(`label`,{className:U.rowLabel,htmlFor:`node-edit-type`,children:`type`}),(0,F.jsxs)(`div`,{className:U.rowValue,children:[(0,F.jsx)(`input`,{id:`node-edit-type`,ref:u,className:U.valueInput,value:t.nodeType,disabled:g,"aria-invalid":!!a.nodeType,onChange:e=>C({nodeType:e.target.value})}),a.nodeType&&(0,F.jsx)(`span`,{className:U.errorText,children:a.nodeType})]}),(0,F.jsx)(`span`,{className:U.rowSpacer,"aria-hidden":`true`})]}),t.properties.map(e=>{let t=a[Zn(e.id,`key`)],n=a[Zn(e.id,`value`)];return(0,F.jsxs)(`div`,{"data-row-id":e.id,className:E===e.id?`${U.row} ${U.rowDropTarget}`:U.row,onDragOver:ie(e.id),onDrop:oe(e.id),children:[(0,F.jsx)(`span`,{className:U.dragGrip,role:`button`,"aria-label":`Reorder property ${e.key.trim()||`(empty)`}`,title:`Drag to reorder — same keys append as [0], [1], … in row order`,draggable:!g,onDragStart:re(e.id),onDragEnd:D,children:`⠿`}),(0,F.jsxs)(`div`,{className:U.rowKey,children:[(0,F.jsx)(`input`,{ref:t=>{t?d.current.set(e.id,t):d.current.delete(e.id)},className:U.keyInput,value:e.key,placeholder:`key`,"aria-label":`Property key`,disabled:g,"aria-invalid":!!t,onChange:t=>w(e.id,{key:t.target.value})}),t&&(0,F.jsx)(`span`,{className:U.errorText,children:t})]}),(0,F.jsxs)(`div`,{className:U.rowValue,children:[(0,F.jsx)(`textarea`,{className:U.valueInput,value:e.value,placeholder:`value`,"aria-label":`Property value`,disabled:g,rows:rs(e.value),"aria-invalid":!!n,onChange:t=>w(e.id,{value:t.target.value})}),n&&(0,F.jsx)(`span`,{className:U.errorText,children:n})]}),(0,F.jsx)(`button`,{type:`button`,className:U.removeButton,"aria-label":`Remove property`,disabled:g,onClick:()=>te(e.id),children:(0,F.jsx)($o,{className:U.removeIcon,"aria-hidden":`true`,focusable:`false`})})]},e.id)}),(0,F.jsx)(`div`,{className:E===`end`?`${U.addRow} ${U.rowDropTarget}`:U.addRow,onDragOver:ie(`end`),onDrop:oe(null),children:(0,F.jsxs)(`button`,{type:`button`,className:U.addButton,disabled:g,onClick:ee,children:[(0,F.jsx)(`span`,{"aria-hidden":`true`,children:`+`}),(0,F.jsx)(`span`,{children:`Add Property`})]})})]}),(0,F.jsxs)(`footer`,{className:U.footer,children:[(0,F.jsx)(`button`,{type:`button`,className:U.secondaryButton,onClick:c,disabled:m,children:`Cancel`}),(0,F.jsx)(`button`,{type:`submit`,className:U.primaryButton,disabled:g,children:m?v:_})]})]})})}var W={root:`_root_rdck7_7`,card:`_card_rdck7_17`,ribbon:`_ribbon_rdck7_30`,titleGroup:`_titleGroup_rdck7_41`,title:`_title_rdck7_41`,path:`_path_rdck7_54`,ribbonClose:`_ribbonClose_rdck7_61`,ribbonCloseIcon:`_ribbonCloseIcon_rdck7_90`,body:`_body_rdck7_96`,description:`_description_rdck7_105`,inputHints:`_inputHints_rdck7_112`,inputHintsLabel:`_inputHintsLabel_rdck7_122`,inputHintList:`_inputHintList_rdck7_128`,moreHints:`_moreHints_rdck7_135`,inputHintsNote:`_inputHintsNote_rdck7_147`,dropZone:`_dropZone_rdck7_153`,dropZoneActive:`_dropZoneActive_rdck7_175`,dropZoneIcon:`_dropZoneIcon_rdck7_181`,dropZoneText:`_dropZoneText_rdck7_187`,dropZoneOr:`_dropZoneOr_rdck7_200`,browseButton:`_browseButton_rdck7_207`,fileInputHidden:`_fileInputHidden_rdck7_236`,fileError:`_fileError_rdck7_241`,textareaLabel:`_textareaLabel_rdck7_246`,textarea:`_textarea_rdck7_246`,validationError:`_validationError_rdck7_274`,keyboardHint:`_keyboardHint_rdck7_279`,errorBanner:`_errorBanner_rdck7_284`,footer:`_footer_rdck7_295`,footerActions:`_footerActions_rdck7_305`,formatButton:`_formatButton_rdck7_311`,cancelButton:`_cancelButton_rdck7_312`,uploadButton:`_uploadButton_rdck7_313`,spinner:`_spinner_rdck7_380`,spin:`_spin_rdck7_380`};function as({uploadPath:e,json:t,onSuccess:n,onError:r}){let[i,a]=(0,A.useState)(!1),o=(0,A.useRef)(null),s=(0,A.useCallback)(()=>{o.current?.abort(),o.current=null,a(!1)},[]);return{isUploading:i,upload:(0,A.useCallback)(async()=>{o.current?.abort();let i=new AbortController;o.current=i,a(!0);try{let o=await fetch(e,{method:`POST`,headers:{"Content-Type":`application/json`},body:t,signal:i.signal}),s=await o.text();if(!o.ok){a(!1),r(`HTTP ${o.status} — ${s}`);return}a(!1),n(s)}catch(e){if(e.name===`AbortError`){a(!1);return}a(!1),r(e.message??`Network error`)}},[e,t,n,r]),cancel:s}}var os=(navigator.userAgentData?.platform??navigator.platform).toLowerCase().includes(`mac`);function ss(e){return new Promise((t,n)=>{let r=new FileReader;r.onload=()=>t(r.result),r.onerror=()=>n(Error(`Could not read file "${e.name}"`)),r.readAsText(e,`utf-8`)})}function cs(e){let t=e.name.toLowerCase().endsWith(`.json`),n=e.type===`application/json`||e.type===`text/plain`;return!t&&!n?`"${e.name}" does not appear to be a JSON file. Only .json files are accepted.`:null}function ls({uploadPath:e,onSuccess:t,onClose:n,onError:r,title:i=`⬆️ Upload Mock Data`,description:a,inputPathHints:o=[],submitLabel:s=`Upload`}){let[c,l]=(0,A.useState)(``),[u,d]=(0,A.useState)(null),[f,p]=(0,A.useState)(null),[m,h]=(0,A.useState)(!1),g=(0,A.useRef)(null),_=(0,A.useRef)(null),v=we(c).isJSON,y=v&&c.trim()!==``,{isUploading:b,upload:x,cancel:S}=as({uploadPath:e,json:c,onSuccess:n=>t(n,e),onError:e=>{d(e),r(e)}}),C=(0,A.useCallback)(()=>{S(),n(e)},[S,n,e]),w=(0,A.useRef)(b);(0,A.useEffect)(()=>{w.current=b},[b]),(0,A.useEffect)(()=>{g.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),w.current||C())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[C]);let ee=(0,A.useCallback)(()=>{d(null),x()},[x]),te=(0,A.useCallback)(e=>{e.key===`Enter`&&(e.ctrlKey||e.metaKey)&&(e.preventDefault(),y&&!b&&ee())},[y,b,ee]),T=(0,A.useCallback)(()=>{v&&l(ue(c))},[v,c]),E=(0,A.useCallback)(async e=>{p(null),d(null);let t=cs(e);if(t){p(t);return}try{let t=await ss(e);if(!we(t).isJSON){p(`"${e.name}" contains invalid JSON.`);return}l(ue(t)),g.current?.focus()}catch(e){p(e.message)}},[]),ne=(0,A.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),m||h(!0)},[m]),D=(0,A.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&h(!1)},[]),re=(0,A.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),h(!1);let t=e.dataTransfer.files[0];t&&E(t)},[E]),ie=(0,A.useCallback)(e=>{let t=e.target.files?.[0];t&&(E(t),e.target.value=``)},[E]),ae=!v&&c.trim()!==``;return(0,F.jsx)(`div`,{className:W.root,children:(0,F.jsxs)(`section`,{className:W.card,"aria-label":i,children:[(0,F.jsxs)(`header`,{className:W.ribbon,children:[(0,F.jsxs)(`div`,{className:W.titleGroup,children:[(0,F.jsx)(`span`,{className:W.title,children:i}),(0,F.jsx)(`span`,{className:W.path,children:e})]}),(0,F.jsx)(`button`,{className:W.ribbonClose,onClick:C,"aria-label":`Close upload panel`,title:`Close (Esc)`,disabled:b,children:(0,F.jsx)($o,{className:W.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,F.jsxs)(`div`,{className:W.body,children:[a&&(0,F.jsx)(`p`,{className:W.description,children:a}),o.length>0&&(0,F.jsxs)(`div`,{className:W.inputHints,"aria-label":`Referenced graph input paths`,children:[(0,F.jsx)(`span`,{className:W.inputHintsLabel,children:`Referenced input paths`}),(0,F.jsxs)(`div`,{className:W.inputHintList,children:[o.slice(0,6).map(e=>(0,F.jsx)(`code`,{children:e},e)),o.length>6&&(0,F.jsxs)(`span`,{className:W.moreHints,children:[`+`,o.length-6,` more`]})]}),(0,F.jsx)(`span`,{className:W.inputHintsNote,children:`Hints are derived from graph references.`})]}),(0,F.jsxs)(`div`,{className:`${W.dropZone} ${m?W.dropZoneActive:``}`,onDragOver:ne,onDragLeave:D,onDrop:re,"aria-label":`Drop a JSON file here`,children:[(0,F.jsx)(`span`,{className:W.dropZoneIcon,children:`📂`}),(0,F.jsxs)(`span`,{className:W.dropZoneText,children:[`Drop a `,(0,F.jsx)(`code`,{children:`.json`}),` file here`]}),(0,F.jsx)(`span`,{className:W.dropZoneOr,children:`— or —`}),(0,F.jsx)(`input`,{ref:_,type:`file`,accept:`.json,application/json`,className:W.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:ie}),(0,F.jsx)(`button`,{type:`button`,className:W.browseButton,onClick:()=>_.current?.click(),disabled:b,"aria-label":`Browse for a JSON file`,children:`Browse file…`})]}),f&&(0,F.jsxs)(`span`,{className:W.fileError,role:`alert`,children:[`⚠️ `,f]}),(0,F.jsx)(`label`,{htmlFor:`mock-upload-textarea`,className:W.textareaLabel,children:`JSON Payload`}),(0,F.jsx)(`textarea`,{id:`mock-upload-textarea`,ref:g,className:W.textarea,value:c,onChange:e=>{l(e.target.value),p(null)},onKeyDown:te,placeholder:`Paste JSON here, or drop / browse a .json file above`,rows:10,spellCheck:!1,"aria-describedby":ae?`mock-upload-validation`:void 0}),ae&&(0,F.jsx)(`span`,{id:`mock-upload-validation`,className:W.validationError,role:`status`,children:`⚠️ Invalid JSON — check syntax`}),(0,F.jsx)(`span`,{className:W.keyboardHint,children:os?`⌘+Enter to ${s.toLowerCase()}`:`Ctrl+Enter to ${s.toLowerCase()}`}),u&&(0,F.jsxs)(`div`,{className:W.errorBanner,role:`alert`,children:[`❌ Upload failed: `,u]})]}),(0,F.jsxs)(`footer`,{className:W.footer,children:[(0,F.jsx)(`button`,{className:W.formatButton,onClick:T,disabled:!v||b,title:`Format JSON`,"aria-label":`Format JSON`,children:`Format`}),(0,F.jsxs)(`div`,{className:W.footerActions,children:[(0,F.jsx)(`button`,{className:W.cancelButton,onClick:C,disabled:b,children:`Cancel`}),(0,F.jsx)(`button`,{className:W.uploadButton,onClick:ee,disabled:!y||b,"aria-busy":b,children:b?(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(`span`,{className:W.spinner,"aria-hidden":`true`}),` Uploading…`]}):s===`Upload`?`Upload ▶`:s})]})]})]})})}var us={popover:`_popover_qw337_7`,header:`_header_qw337_19`,endpoint:`_endpoint_qw337_30`,arrow:`_arrow_qw337_41`,chips:`_chips_qw337_46`,chip:`_chip_qw337_46`,customRow:`_customRow_qw337_73`,input:`_input_qw337_78`,submitButton:`_submitButton_qw337_100`,errorText:`_errorText_qw337_122`,statusText:`_statusText_qw337_128`,warningText:`_warningText_qw337_129`},ds=12;function fs({formState:e,phase:t,lockReason:n,serverMessage:r,validationErrors:i,anchor:a,onFormStateChange:o,onSubmit:s,onClose:c}){let l=(0,A.useRef)(null),u=(0,A.useRef)(null),[d,f]=(0,A.useState)(null),p=t===`sending`,m=n===`disconnected`,h=p||m,g=(0,A.useRef)(null);(0,A.useEffect)(()=>{g.current!==null&&e.relation===g.current&&(g.current=null,h||s())},[h,e.relation,s]),(0,A.useEffect)(()=>{u.current?.focus()},[]),(0,A.useLayoutEffect)(()=>{let e={x:window.innerWidth/2,y:96},t=a??e,n=l.current?.getBoundingClientRect(),r=n?.width??280,i=n?.height??180;f({left:Math.min(Math.max(t.x-r/2,ds),Math.max(ds,window.innerWidth-r-ds)),top:Math.min(Math.max(t.y+14,ds),Math.max(ds,window.innerHeight-i-ds))})},[a]),(0,A.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),p||c())},t=e=>{p||l.current&&!l.current.contains(e.target)&&c()};return document.addEventListener(`keydown`,e),document.addEventListener(`pointerdown`,t),()=>{document.removeEventListener(`keydown`,e),document.removeEventListener(`pointerdown`,t)}},[c,p]);let _=(0,A.useCallback)(t=>{o({...e,relation:t})},[e,o]),v=(0,A.useCallback)(t=>{h||(g.current=t,o({...e,relation:t}))},[h,e,o]),y=(0,A.useCallback)(e=>{e.preventDefault(),!h&&s()},[h,s]),b=i.relation??i.command??i.sourceAlias??i.targetAlias;return(0,F.jsxs)(`div`,{ref:l,className:us.popover,style:d?{left:d.left,top:d.top}:{visibility:`hidden`},role:`dialog`,"aria-label":`Create connection from ${e.sourceAlias} to ${e.targetAlias}`,children:[(0,F.jsxs)(`div`,{className:us.header,children:[(0,F.jsx)(`span`,{className:us.endpoint,children:e.sourceAlias}),(0,F.jsx)(`span`,{className:us.arrow,"aria-hidden":`true`,children:`→`}),(0,F.jsx)(`span`,{className:us.endpoint,children:e.targetAlias})]}),(0,F.jsx)(`div`,{className:us.chips,children:Ci.map(e=>(0,F.jsx)(`button`,{type:`button`,className:us.chip,style:{"--chip-color":wi[e]},disabled:h,onClick:()=>v(e),children:e},e))}),(0,F.jsxs)(`form`,{className:us.customRow,onSubmit:y,children:[(0,F.jsx)(`input`,{ref:u,className:us.input,value:e.relation,placeholder:`custom relation…`,autoComplete:`off`,autoCorrect:`off`,spellCheck:!1,disabled:h,"aria-label":`Relation name`,"aria-invalid":!!i.relation,onChange:e=>_(e.target.value)}),(0,F.jsx)(`button`,{type:`submit`,className:us.submitButton,disabled:h||!e.relation.trim(),children:p?`Creating…`:`Connect`})]}),b&&!p&&(0,F.jsx)(`div`,{className:us.errorText,role:`alert`,children:b}),r&&(0,F.jsx)(`div`,{className:m?us.warningText:us.statusText,role:`status`,children:r})]})}var ps=1e4,ms=`A graph authoring action is already pending. Wait for it to finish before starting another.`,hs=`Could not send the create-node command because the WebSocket is not open. The form values remain in this dialog.`,gs=`Could not send the edit-node command because the WebSocket is not open. Your changes remain in this dialog.`,_s=`Could not send the delete-node command because the WebSocket is not open.`,vs=`Could not send the create-connection command because the WebSocket is not open. The form values remain in this dialog.`,ys=`Could not send delete-node commands because the WebSocket is not open.`,bs=`No selected nodes are available to delete.`,xs=`Select 100 or fewer nodes to delete at once.`,Ss=`Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.`,Cs=`This node is no longer available in the current graph.`,ws=`Connection disconnected. Refresh the page and create the node again after the app reconnects.`,Ts=`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,Es=`Connection disconnected. Refresh the page and create the connection again after the app reconnects.`,Ds=`Connection disconnected while the graph authoring action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.`,Os={status:`closed`,pendingSubmit:null,serverMessage:null};function ks(e){return e.pendingSubmit}function As(e){return e.action===`delete-nodes`}function js(e){return e===`create-connection`?vs:e===`edit-node`?gs:e===`delete-node`?_s:hs}function Ms(e){return As(e)?Ss:`The ${e.action} command was sent, but no backend result was observed yet. The outcome is unknown.`}function Ns(e){return e===`create-connection`?Es:e===`edit-node`?Ts:ws}function Ps(e,t){return!e||!t?!1:e.trim().toLowerCase()===t.trim().toLowerCase()}function Fs(e,t){return e?.nodes.find(e=>e.alias.toLowerCase()===t.toLowerCase())??null}function Is(e,t){return e.status===`error`?!0:t.action===`create-connection`?e.action===`create-connection`?e.alias===null?!0:Ps(e.alias,t.alias)?e.status===`accepted`?Ps(e.targetAlias,t.targetAlias):e.targetAlias===null||Ps(e.targetAlias,t.targetAlias):!1:e.action===null?Ps(e.alias,t.alias)||Ps(e.alias,t.targetAlias):!1:Ps(e.alias,t.alias)?e.action===null||e.action===t.action:!1}function Ls(e,t){let n=ks(e);return!n||As(n)||!Is(t,n)?null:t.status===`accepted`?{state:Os,acceptedResult:{status:t.status,action:t.action,alias:t.alias,targetAlias:t.targetAlias,message:t.message}}:e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:t.status===`error`?`Backend returned an error while this submit was pending: ${t.message}`:t.message}}:{state:Os,notification:{message:t.message,type:`error`}}}function Rs(e){return{...e,phase:`editing`,pendingSubmit:null,serverMessage:js(e.action)}}function zs(e){let t=ks(e);if(!t)return null;let n=Ms(t);return e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n}}:{state:Os,notification:{message:n,type:`error`}}}function Bs(e){let t=ks(e);if(e.status===`open`){let n=t?Ds:Ns(e.action);return{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n,connectionLost:!0}}}return t?{state:Os,notification:{message:Ds,type:`error`}}:null}function Vs(e){return`sourceAlias`in e}function Hs({bus:e,connected:t,graphData:n,executor:r,timeoutMs:i=ps,onAccepted:a,onUserMessage:o}){let[s,c]=(0,A.useState)(Os),[l,u]=(0,A.useState)({}),d=(0,A.useRef)(s),f=(0,A.useRef)(null),p=(0,A.useRef)(t),m=(0,A.useRef)(n),h=(0,A.useRef)(a),g=(0,A.useRef)(o);(0,A.useEffect)(()=>{d.current=s},[s]),(0,A.useEffect)(()=>{m.current=n},[n]),(0,A.useEffect)(()=>{h.current=a},[a]),(0,A.useEffect)(()=>{g.current=o},[o]);let _=(0,A.useCallback)((e,t=`error`)=>{g.current?.(e,t)},[]),v=(0,A.useCallback)(e=>{d.current=e,c(e)},[]),y=(0,A.useCallback)(()=>{f.current!==null&&(clearTimeout(f.current),f.current=null)},[]),b=(0,A.useCallback)(()=>{y(),f.current=setTimeout(()=>{let e=zs(d.current);e&&(v(e.state),e.notification&&_(e.notification.message,e.notification.type)),f.current=null},i)},[y,_,v,i]),x=(0,A.useCallback)(e=>{if(!t)return;if(ks(d.current)){_(ms,`error`);return}let n=yr(e);u({}),v({status:`open`,action:`create-node`,phase:`editing`,formState:n,originalAlias:null,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),S=(0,A.useCallback)((e,n)=>{if(!t)return;if(ks(d.current)){_(ms,`error`);return}let r={sourceAlias:e,targetAlias:n,relation:``},{relation:i,...a}=ir(r,{graphData:m.current,connected:t}).errors;Object.keys(a).length>0||(u({}),v({status:`open`,action:`create-connection`,phase:`editing`,formState:r,pendingSubmit:null,serverMessage:null,connectionLost:!1}))},[t,_,v]),C=(0,A.useCallback)(e=>{if(!t){_(Ts,`error`);return}if(ks(d.current)){_(ms,`error`);return}let n=Fs(m.current,e.alias);if(!n){_(Cs,`error`);return}let r=Or(n);if(!r.valid||!r.formState){_(r.message??`This node cannot be edited in the UI.`,`error`);return}u({}),v({status:`open`,action:`edit-node`,phase:`editing`,formState:r.formState,originalAlias:n.alias,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),w=(0,A.useCallback)(e=>{if(!t){_(_s,`error`);return}if(ks(d.current)){_(ms,`error`);return}let n=tr(e.alias,{graphData:m.current});if(!n.valid){_(Object.values(n.errors)[0]??`Invalid node alias.`,`error`);return}let i;try{i=dr(e.alias,{graphData:m.current})}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}if(!r.execute(i)){_(_s,`error`);return}let a={action:`delete-node`,alias:e.alias.trim(),command:i,sentAt:new Date().toISOString()};u({}),v({status:`closed`,pendingSubmit:a,serverMessage:null}),b()},[t,r,_,v,b]),ee=(0,A.useCallback)(e=>{if(!t){_(ys,`error`);return}if(ks(d.current)){_(ms,`error`);return}if(e.length===0){_(bs,`info`);return}if(e.length>100){_(xs,`error`);return}let n=new Set,i=e.filter(e=>{let t=e.alias.trim().toLowerCase();return n.has(t)?!1:(n.add(t),!0)}),a=[],o=[];for(let e of i){let t=tr(e.alias,{graphData:m.current});if(!t.valid){_(Object.values(t.errors)[0]??bs,`error`);return}try{o.push(e.alias.trim()),a.push(dr(e.alias,{graphData:m.current}))}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}}for(let[e,t]of a.entries())if(!r.execute(t)){_(e===0?ys:Ds,`error`);return}let s={action:`delete-nodes`,aliases:o,commands:a,sentAt:new Date().toISOString(),results:{}};u({}),v({status:`closed`,pendingSubmit:s,serverMessage:null}),_(`${o.length} delete-node commands sent. Waiting for backend response.`,`info`),b()},[t,r,_,v,b]),te=(0,A.useCallback)(e=>{let t=d.current;if(t.status===`open`&&!(t.phase===`sending`||t.connectionLost)){if(u({}),t.action===`create-connection`){if(!Vs(e))return;v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1});return}Vs(e)||v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1})}},[v]),T=(0,A.useCallback)(()=>{let e=d.current;if(e.status!==`open`||e.phase===`sending`||e.connectionLost)return;let n=e.action;if(!t){v({...e,serverMessage:js(n)});return}let i=e.action===`create-connection`?ir(e.formState,{graphData:m.current,connected:t}):er(e.formState,e.action===`edit-node`?{mode:`edit`,originalAlias:e.originalAlias}:{graphData:m.current});if(!i.valid){u(i.errors);return}let a,o,s=null;try{if(e.action===`edit-node`)o=e.originalAlias?.trim()??``,a=ur(e.formState,o);else if(e.action===`create-node`)o=e.formState.alias.trim(),a=lr(e.formState);else if(Vs(e.formState))o=e.formState.sourceAlias.trim(),s=e.formState.targetAlias.trim(),a=pr(e.formState);else{u({command:`Invalid connection form state.`});return}}catch(e){u({command:e instanceof Error?e.message:String(e)});return}if(!r.execute(a)){v(Rs(e));return}let c={action:n,alias:o,targetAlias:s,command:a,sentAt:new Date().toISOString()};u({}),v({...e,phase:`sending`,pendingSubmit:c,serverMessage:null,connectionLost:!1}),b()},[t,r,v,b]),E=(0,A.useCallback)(()=>{let e=d.current;e.status===`open`&&e.phase!==`sending`&&(y(),u({}),v(Os))},[y,v]);return(0,A.useEffect)(()=>e.on(`minigraph.nodeAction.textResult`,e=>{let t=d.current,n=ks(t);if(!n)return;if(As(n)){let r=Rr(n,e);if(!r)return;if(e.status===`accepted`&&h.current?.({status:e.status,action:e.action,alias:e.alias,targetAlias:e.targetAlias,message:e.message}),!zr(r)){v({...t,pendingSubmit:r});return}y(),v(Os);let i=Br(r);_(i.message,i.type);return}let r=Ls(t,e);r&&(y(),r.acceptedResult&&u({}),v(r.state),r.acceptedResult&&h.current?.(r.acceptedResult),r.notification&&_(r.notification.message,r.notification.type))}),[e,y,_,v]),(0,A.useEffect)(()=>{if(p.current&&!t){let e=Bs(d.current);e&&(y(),v(e.state),e.notification&&_(e.notification.message,e.notification.type))}p.current=t},[y,t,_,v]),(0,A.useEffect)(()=>()=>{y()},[y]),{state:s,validationErrors:l,openCreateNode:x,openCreateConnection:S,openEditNode:C,deleteNode:w,deleteNodes:ee,updateFormState:te,submit:T,close:E}}var Us=/^ws-\d+-\d+$/;function Ws(e){return Us.test(e.trim())}var Gs={sessionId:null,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null};function Ks(e){return Array.from(new Set(e)).sort()}function qs({enabled:e,connected:t,bus:n,classificationMap:r,sendRawText:i,addToast:a}){let[o,s]=(0,A.useState)(Gs),c=(0,A.useRef)(new Set),l=(0,A.useRef)(0),u=(0,A.useRef)(i),d=(0,A.useRef)(a);(0,A.useEffect)(()=>{u.current=i},[i]),(0,A.useEffect)(()=>{d.current=a},[a]);let f=(0,A.useCallback)(()=>{if(!e||!t)return!1;s(e=>({...e,loading:!0,pendingCommand:`refresh`,error:null,lastInfo:null}));let n=u.current(`session`);if(!n){let e=`Could not load session details because the WebSocket is not open.`;s(t=>({...t,loading:!1,pendingCommand:null,error:e})),d.current(e,`error`)}return n},[t,e]),p=(0,A.useCallback)(e=>{let t=`${e.kind}:${e.msgId}`;if(!c.current.has(t)){if(c.current.add(t),e.kind===`minigraph.session.started`){s({...Gs,sessionId:e.sessionId});return}if(e.kind===`minigraph.session.status`){s(t=>({...t,sessionId:e.sessionId,startedSince:e.startedSince,subscribedTo:e.subscribedTo,subscribers:Ks(e.subscribers),loading:!1,pendingCommand:null,error:null,lastInfo:null}));return}if(e.kind===`minigraph.session.commandResult`){if(e.status===`accepted`){s(t=>({...t,subscribedTo:e.command===`subscribe`?e.sessionId:e.command===`unsubscribe`?null:t.subscribedTo,pendingCommand:null,error:null,lastInfo:null}));return}s(t=>({...t,pendingCommand:null,error:e.message,lastInfo:null}));return}if(e.kind===`minigraph.session.notification`){e.type===`host-closed`?s(t=>({...t,subscribedTo:t.subscribedTo===e.sessionId?null:t.subscribedTo,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null})):e.type===`subscriber-joined`?s(t=>({...t,subscribers:Ks([...t.subscribers,e.sessionId]),error:null,lastInfo:null})):s(t=>({...t,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null}));return}e.kind===`session.reset`&&(s(e=>({...e,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null})),f())}},[f]),m=(0,A.useCallback)(()=>{s(e=>({...e,error:null,lastInfo:null}))},[]),h=(0,A.useCallback)(n=>{let r=n.trim();if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null)return!1;if(!Ws(r))return s(e=>({...e,error:`Enter a valid session ID like ws-123456-1.`,lastInfo:null})),!1;s(e=>({...e,pendingCommand:`subscribe`,error:null,lastInfo:null}));let c=i(`session subscribe ${r}`);if(!c){let e=`Could not subscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return c},[a,t,e,i,o.pendingCommand,o.subscribedTo]),g=(0,A.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo===null)return!1;s(e=>({...e,pendingCommand:`unsubscribe`,error:null,lastInfo:null}));let n=i(`session unsubscribe`);if(!n){let e=`Could not unsubscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo]),_=(0,A.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null||o.subscribers.length===0)return!1;s(e=>({...e,pendingCommand:`reset`,error:null,lastInfo:null}));let n=i(`session reset`);if(!n){let e=`Could not reset because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo,o.subscribers.length]);(0,A.useEffect)(()=>{e&&t||(c.current.clear(),l.current=0,s(Gs))},[t,e]),(0,A.useEffect)(()=>{if(!e)return;let t=n.on(`minigraph.session.started`,e=>{p(e)}),r=n.on(`minigraph.session.status`,e=>{p(e)}),i=n.on(`minigraph.session.commandResult`,e=>{p(e)}),a=n.on(`minigraph.session.notification`,e=>{p(e)}),o=n.on(`session.reset`,e=>{p(e)});return()=>{t(),r(),i(),a(),o()}},[n,e,p]),(0,A.useEffect)(()=>{!e||!t||f()},[t,e,f]),(0,A.useEffect)(()=>{if(!e||!r)return;let t=l.current;for(let[e,n]of r)if(!(e<=l.current)){for(let e of n)(e.kind===`minigraph.session.started`||e.kind===`minigraph.session.status`||e.kind===`minigraph.session.commandResult`||e.kind===`minigraph.session.notification`||e.kind===`session.reset`)&&p(e);t=Math.max(t,e)}l.current=t,c.current.clear()},[r,e,p]);let v=o.subscribedTo===null,y=o.subscribers.length>0;return(0,A.useMemo)(()=>({state:o,connected:t,isPrimary:v,hasSubscribers:y,canSubscribe:e&&t&&o.pendingCommand===null&&o.subscribedTo===null,canUnsubscribe:e&&t&&o.subscribedTo!==null&&o.pendingCommand===null,canReset:e&&t&&o.pendingCommand===null&&o.subscribedTo===null&&o.subscribers.length>0,subscribeToSession:h,unsubscribe:g,resetSession:_,clearMessage:m}),[m,t,e,y,v,_,o,h,g])}var Js=(e,t)=>t.some(t=>e instanceof t),Ys,Xs;function Zs(){return Ys||=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction]}function Qs(){return Xs||=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey]}var $s=new WeakMap,ec=new WeakMap,tc=new WeakMap;function nc(e){let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`success`,i),e.removeEventListener(`error`,a)},i=()=>{t(cc(e.result)),r()},a=()=>{n(e.error),r()};e.addEventListener(`success`,i),e.addEventListener(`error`,a)});return tc.set(t,e),t}function rc(e){if($s.has(e))return;let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`complete`,i),e.removeEventListener(`error`,a),e.removeEventListener(`abort`,a)},i=()=>{t(),r()},a=()=>{n(e.error||new DOMException(`AbortError`,`AbortError`)),r()};e.addEventListener(`complete`,i),e.addEventListener(`error`,a),e.addEventListener(`abort`,a)});$s.set(e,t)}var ic={get(e,t,n){if(e instanceof IDBTransaction){if(t===`done`)return $s.get(e);if(t===`store`)return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return cc(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){return e instanceof IDBTransaction&&(t===`done`||t===`store`)||t in e}};function ac(e){ic=e(ic)}function oc(e){return Qs().includes(e)?function(...t){return e.apply(lc(this),t),cc(this.request)}:function(...t){return cc(e.apply(lc(this),t))}}function sc(e){return typeof e==`function`?oc(e):(e instanceof IDBTransaction&&rc(e),Js(e,Zs())?new Proxy(e,ic):e)}function cc(e){if(e instanceof IDBRequest)return nc(e);if(ec.has(e))return ec.get(e);let t=sc(e);return t!==e&&(ec.set(e,t),tc.set(t,e)),t}var lc=e=>tc.get(e);function uc(e,t,{blocked:n,upgrade:r,blocking:i,terminated:a}={}){let o=indexedDB.open(e,t),s=cc(o);return r&&o.addEventListener(`upgradeneeded`,e=>{r(cc(o.result),e.oldVersion,e.newVersion,cc(o.transaction),e)}),n&&o.addEventListener(`blocked`,e=>n(e.oldVersion,e.newVersion,e)),s.then(e=>{a&&e.addEventListener(`close`,()=>a()),i&&e.addEventListener(`versionchange`,e=>i(e.oldVersion,e.newVersion,e))}).catch(()=>{}),s}function dc(e,{blocked:t}={}){let n=indexedDB.deleteDatabase(e);return t&&n.addEventListener(`blocked`,e=>t(e.oldVersion,e)),cc(n).then(()=>void 0)}var fc=[`get`,`getKey`,`getAll`,`getAllKeys`,`count`],pc=[`put`,`add`,`delete`,`clear`],mc=new Map;function hc(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t==`string`))return;if(mc.get(t))return mc.get(t);let n=t.replace(/FromIndex$/,``),r=t!==n,i=pc.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(i||fc.includes(n)))return;let a=async function(e,...t){let a=this.transaction(e,i?`readwrite`:`readonly`),o=a.store;return r&&(o=o.index(t.shift())),(await Promise.all([o[n](...t),i&&a.done]))[0]};return mc.set(t,a),a}ac(e=>({...e,get:(t,n,r)=>hc(t,n)||e.get(t,n,r),has:(t,n)=>!!hc(t,n)||e.has(t,n)}));var gc=[`continue`,`continuePrimaryKey`,`advance`],_c={},vc=new WeakMap,yc=new WeakMap,bc={get(e,t){if(!gc.includes(t))return e[t];let n=_c[t];return n||=_c[t]=function(...e){vc.set(this,yc.get(this)[t](...e))},n}};async function*xc(...e){let t=this;if(t instanceof IDBCursor||(t=await t.openCursor(...e)),!t)return;t=t;let n=new Proxy(t,bc);for(yc.set(n,t),tc.set(n,lc(t));t;)yield n,t=await(vc.get(n)||t.continue()),vc.delete(n)}function Sc(e,t){return t===Symbol.asyncIterator&&Js(e,[IDBIndex,IDBObjectStore,IDBCursor])||t===`iterate`&&Js(e,[IDBIndex,IDBObjectStore])}ac(e=>({...e,get(t,n,r){return Sc(t,n)?xc:e.get(t,n,r)},has(t,n){return Sc(t,n)||e.has(t,n)}}));var Cc=`minigraph-clipboard`,wc=1,Tc=`items`,Ec=null;function Dc(){return uc(Cc,wc,{upgrade(e){e.objectStoreNames.contains(Tc)&&e.deleteObjectStore(Tc);let t=e.createObjectStore(Tc,{keyPath:`id`});t.createIndex(`by-alias`,`node.alias`,{unique:!0}),t.createIndex(`by-clippedAt`,`clippedAt`)}})}function Oc(){return Ec||=Dc().catch(async e=>(console.warn(`[clipboard/db] openDB failed, deleting and recreating:`,e),Ec=null,await dc(Cc),Dc())),Ec}async function kc(){return(await(await Oc()).getAllFromIndex(Tc,`by-clippedAt`)).reverse()}async function Ac(e){return(await Oc()).getFromIndex(Tc,`by-alias`,e)}async function jc(e){await(await Oc()).add(Tc,e)}async function Mc(e,t){let n=(await Oc()).transaction(Tc,`readwrite`);await n.store.delete(e),await n.store.add(t),await n.done}async function Nc(e){await(await Oc()).delete(Tc,e)}async function Pc(){await(await Oc()).clear(Tc)}var Fc=`minigraph-clipboard-sync`;function Ic(){return new BroadcastChannel(Fc)}function Lc(e,t){switch(t.type){case`HYDRATE`:return{items:t.items,isLoading:!1};case`ITEM_ADDED`:return{...e,items:[t.item,...e.items]};case`ITEM_REPLACED`:{let n=e.items.filter(e=>e.id!==t.previousId);return{...e,items:[t.item,...n]}}case`ITEM_REMOVED`:return{...e,items:e.items.filter(e=>e.id!==t.id)};case`ITEMS_CLEARED`:return{...e,items:[]};default:return e}}var Rc=(0,A.createContext)(null);function zc({children:e}){let[t,n]=(0,A.useReducer)(Lc,{items:[],isLoading:!0}),r=(0,A.useRef)(null);(0,A.useEffect)(()=>{kc().then(e=>n({type:`HYDRATE`,items:e}))},[]),(0,A.useEffect)(()=>{let e;try{e=Ic()}catch{return}return r.current=e,e.onmessage=e=>{let t=e.data;switch(t.type){case`item-added`:n({type:`ITEM_ADDED`,item:t.item});break;case`item-replaced`:n({type:`ITEM_REPLACED`,item:t.item,previousId:t.previousId});break;case`item-removed`:n({type:`ITEM_REMOVED`,id:t.id});break;case`items-cleared`:n({type:`ITEMS_CLEARED`});break}},()=>{e.close(),r.current=null}},[]);let i=(0,A.useCallback)(e=>{r.current?.postMessage(e)},[]),a=(0,A.useCallback)(async(e,t,r)=>{try{let a={id:crypto.randomUUID(),clippedAt:new Date().toISOString(),sourceWsPath:r.sourceWsPath,sourceLabel:r.sourceLabel,node:e,connections:t},o=await Ac(e.alias);if(o)return{status:`duplicate`,existingItem:o,pendingItem:a};try{await jc(a)}catch(t){if(t instanceof DOMException&&t.name===`ConstraintError`){let t=await Ac(e.alias);if(t)return{status:`duplicate`,existingItem:t,pendingItem:a}}throw t}return n({type:`ITEM_ADDED`,item:a}),i({type:`item-added`,item:a}),{status:`added`}}catch(e){return{status:`error`,message:e instanceof Error?e.message:String(e)}}},[i]),o=(0,A.useCallback)(async(e,t)=>{await Mc(t,e),n({type:`ITEM_REPLACED`,item:e,previousId:t}),i({type:`item-replaced`,item:e,previousId:t})},[i]),s=(0,A.useCallback)(async e=>{await Nc(e),n({type:`ITEM_REMOVED`,id:e}),i({type:`item-removed`,id:e})},[i]),c=(0,A.useCallback)(async()=>{await Pc(),n({type:`ITEMS_CLEARED`}),i({type:`items-cleared`})},[i]);return(0,F.jsx)(Rc.Provider,{value:{items:t.items,isLoading:t.isLoading,clipNode:a,confirmReplace:o,removeItem:s,clearAll:c},children:e})}function Bc(){let e=(0,A.useContext)(Rc);if(!e)throw Error(`useClipboardContext must be used inside <ClipboardProvider>`);return e}var Vc=new Intl.Collator(void 0,{sensitivity:`base`,numeric:!0});function Hc(e){return e.node.types[0]?.trim()||`unknown`}function Uc(e,t){return Vc.compare(e,t)}function Wc(e,t){return e-t}function Gc(e){return e===`recent`||e===`connections`?`descending`:`ascending`}function Kc(e,t){return t===`descending`?-e:e}function qc(e,t){let n=t.trim();if(!n)return{missing:!0,value:``};let r=e.node.properties[n];return r==null?{missing:!0,value:``}:typeof r==`string`?{missing:!1,value:r}:typeof r==`number`||typeof r==`boolean`?{missing:!1,value:String(r)}:{missing:!1,value:JSON.stringify(r)}}function Jc(e,t,n,r){let i=qc(e,n),a=qc(t,n);return i.missing&&!a.missing?1:!i.missing&&a.missing?-1:Kc(Uc(i.value,a.value),r)}function Yc(e,t){let n=t.direction??Gc(t.field);return e.map((e,t)=>({item:e,originalIndex:t})).sort((e,r)=>{let i=0;switch(t.field){case`type`:i=Uc(Hc(e.item),Hc(r.item));break;case`alias`:i=Uc(e.item.node.alias,r.item.node.alias);break;case`source`:i=Uc(e.item.sourceLabel,r.item.sourceLabel);break;case`connections`:i=e.item.connections.length-r.item.connections.length;break;case`property`:i=Jc(e.item,r.item,t.propertyKey??``,n);break;default:i=Date.parse(e.item.clippedAt)-Date.parse(r.item.clippedAt);break}return t.field!==`property`&&(i=Kc(i,n)),i===0?Wc(e.originalIndex,r.originalIndex):i}).map(({item:e})=>e)}function Xc(e){let t=Date.now()-new Date(e).getTime();if(t<0)return`just now`;let n=Math.floor(t/1e3);if(n<60)return`just now`;let r=Math.floor(n/60);if(r<60)return`${r} min ago`;let i=Math.floor(r/60);if(i<24)return`${i} hour${i>1?`s`:``} ago`;let a=Math.floor(i/24);return a===1?`yesterday`:a<30?`${a} days ago`:new Date(e).toLocaleDateString()}var Zc={item:`_item_1ne62_1`,previewFrame:`_previewFrame_1ne62_13`,preview:`_preview_1ne62_13`,previewShell:`_previewShell_1ne62_25`,metaBlock:`_metaBlock_1ne62_29`,timestamp:`_timestamp_1ne62_35`,removeChrome:`_removeChrome_1ne62_40`,removeIcon:`_removeIcon_1ne62_71`};function Qc({item:e,onRemove:t,onOpenMenu:n,onCloseMenu:r}){let{node:i,clippedAt:a,sourceLabel:o}=e;return(0,F.jsxs)(`div`,{className:Zc.item,children:[(0,F.jsxs)(`div`,{className:Zc.previewFrame,children:[(0,F.jsx)(`button`,{type:`button`,className:Zc.removeChrome,draggable:!1,"aria-label":`Remove node ${i.alias} from clipboard`,onClick:n=>{n.stopPropagation(),r(),t(e.id)},children:(0,F.jsx)($o,{className:Zc.removeIcon,"aria-hidden":`true`,focusable:`false`})}),(0,F.jsx)(`div`,{className:Zc.preview,role:`group`,draggable:!0,onDragStart:t=>{r(),Ua(t.dataTransfer,e.id)},onContextMenu:t=>{t.preventDefault(),n(e.id,t.clientX,t.clientY)},onKeyDown:t=>{if(t.key===`ContextMenu`||t.key===`F10`&&t.shiftKey){t.preventDefault();let r=t.currentTarget.getBoundingClientRect();n(e.id,Math.round(r.left+8),Math.round(r.top+8))}},tabIndex:0,"aria-label":`Drag node ${i.alias} into the graph to paste`,children:(0,F.jsx)(`div`,{className:Zc.previewShell,style:mi(i.types[0]??`unknown`),children:(0,F.jsx)(vi,{alias:i.alias,nodeType:i.types[0]??`unknown`,properties:i.properties})})})]}),(0,F.jsx)(`div`,{className:Zc.metaBlock,children:(0,F.jsxs)(`div`,{className:Zc.timestamp,children:[`Clipped `,Xc(a),` from `,o]})})]})}var $c={menu:`_menu_164vh_1`,menuItem:`_menuItem_164vh_12`},el=16;function tl(e,t,n){let r=el,i=Math.max(el,n-t-el);return Math.min(Math.max(e,r),i)}function nl({open:e,x:t,y:n,canPasteToInput:r,onPasteToInput:i,onInspect:a,onClose:o}){let s=(0,A.useRef)(null),c=(0,A.useRef)(null),l=(0,A.useRef)(null),[u,d]=(0,A.useState)({left:t,top:n});return(0,A.useLayoutEffect)(()=>{if(!e||!s.current)return;let r=s.current.getBoundingClientRect();d({left:tl(t,r.width,window.innerWidth),top:tl(n,r.height,window.innerHeight)})},[e,t,n]),(0,A.useEffect)(()=>{if(!e)return;r?c.current?.focus():l.current?.focus();let t=e=>{s.current&&!s.current.contains(e.target)&&o()},n=e=>{e.key===`Escape`&&(e.preventDefault(),o())},i=()=>o();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,i,!0),window.addEventListener(`resize`,i),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,i,!0),window.removeEventListener(`resize`,i)}},[e,r,o]),e?(0,F.jsxs)(`div`,{ref:s,className:$c.menu,style:{left:u.left,top:u.top},role:`menu`,"aria-label":`Clipboard item actions`,children:[(0,F.jsx)(`button`,{ref:c,role:`menuitem`,type:`button`,className:$c.menuItem,disabled:!r,onClick:()=>{r&&i()},children:`Paste to Input`}),(0,F.jsx)(`button`,{ref:l,role:`menuitem`,type:`button`,className:$c.menuItem,onClick:a,children:`Inspect`})]}):null}var G={sidebar:`_sidebar_1jo34_2`,header:`_header_1jo34_12`,headerTitle:`_headerTitle_1jo34_25`,clearBtn:`_clearBtn_1jo34_32`,sortBar:`_sortBar_1jo34_48`,sortMenuWrapper:`_sortMenuWrapper_1jo34_63`,sortMenuButton:`_sortMenuButton_1jo34_68`,sortButtonLabel:`_sortButtonLabel_1jo34_93`,sortButtonValue:`_sortButtonValue_1jo34_98`,sortButtonDirection:`_sortButtonDirection_1jo34_106`,sortButtonCaret:`_sortButtonCaret_1jo34_114`,sortButtonCaretOpen:`_sortButtonCaretOpen_1jo34_121`,sortPopover:`_sortPopover_1jo34_125`,sortGroup:`_sortGroup_1jo34_139`,propertySortRow:`_propertySortRow_1jo34_150`,sortGroupTitle:`_sortGroupTitle_1jo34_154`,sortOption:`_sortOption_1jo34_164`,propertyLabel:`_propertyLabel_1jo34_197`,propertyInput:`_propertyInput_1jo34_205`,itemList:`_itemList_1jo34_223`,loading:`_loading_1jo34_233`,emptyState:`_emptyState_1jo34_243`,emptyIcon:`_emptyIcon_1jo34_256`,emptyTitle:`_emptyTitle_1jo34_261`,emptyHint:`_emptyHint_1jo34_265`,inspectPanel:`_inspectPanel_1jo34_271`,inspectHeader:`_inspectHeader_1jo34_279`,inspectClose:`_inspectClose_1jo34_293`,inspectBody:`_inspectBody_1jo34_307`,dialog:`_dialog_1jo34_313`,dialogTitle:`_dialogTitle_1jo34_328`,dialogBody:`_dialogBody_1jo34_335`,dialogActions:`_dialogActions_1jo34_342`,cancelBtn:`_cancelBtn_1jo34_349`,replaceBtn:`_replaceBtn_1jo34_363`};function rl(){return(0,F.jsxs)(`div`,{className:G.emptyState,children:[(0,F.jsx)(`span`,{className:G.emptyIcon,children:`📋`}),(0,F.jsx)(`span`,{className:G.emptyTitle,children:`No items clipped yet.`}),(0,F.jsx)(`span`,{className:G.emptyHint,children:`Right-click a node in the Graph view to get started.`})]})}var il=[{value:`recent`,label:`Recent`},{value:`type`,label:`Type`},{value:`alias`,label:`Alias`},{value:`source`,label:`Source`},{value:`connections`,label:`Connections`},{value:`property`,label:`Property`}],al=[{value:`ascending`,label:`Ascending`},{value:`descending`,label:`Descending`}],ol=il.reduce((e,t)=>({...e,[t.value]:t.label}),{});function sl({connected:e,onPasteToInput:t}){let n=(0,A.useId)(),i=(0,A.useId)(),a=(0,A.useRef)(null),s=Bc(),[c,l]=(0,A.useState)(null),[u,d]=(0,A.useState)(null),[f,p]=(0,A.useState)(`recent`),[m,h]=(0,A.useState)(Gc(`recent`)),[g,_]=(0,A.useState)(``),[v,y]=(0,A.useState)(!1),b=(e,t,n)=>{d({itemId:e,x:t,y:n})},x=()=>{d(null)},S=e=>{x(),t(e)},C=e=>{x(),l(t=>t?.id===e.id?null:e)},w=e=>{x(),l(t=>t?.id===e?null:t),s.removeItem(e)},ee=()=>{x(),l(null),s.clearAll()},te=e=>{p(e),h(Gc(e))};(0,A.useEffect)(()=>{let e=new Set(s.items.map(e=>e.id));u&&!e.has(u.itemId)&&d(null),c&&!e.has(c.id)&&l(null)},[s.items,u,c]),(0,A.useEffect)(()=>{if(!v)return;let e=e=>{a.current?.contains(e.target)||y(!1)},t=e=>{e.key===`Escape`&&y(!1)};return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,t),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,t)}},[v]);let T=(0,A.useMemo)(()=>u?s.items.find(e=>e.id===u.itemId)??null:null,[u,s.items]),E=(0,A.useMemo)(()=>Yc(s.items,{field:f,direction:m,propertyKey:g}),[s.items,m,f,g]);return(0,F.jsxs)(`div`,{className:G.sidebar,children:[(0,F.jsxs)(`div`,{className:G.header,children:[(0,F.jsx)(`span`,{className:G.headerTitle,children:`Workspace`}),s.items.length>0&&(0,F.jsx)(`button`,{className:G.clearBtn,onClick:ee,"aria-label":`Clear all workspace items`,children:`Clear`})]}),s.items.length>0&&(0,F.jsx)(`div`,{className:G.sortBar,children:(0,F.jsxs)(`div`,{className:G.sortMenuWrapper,ref:a,children:[(0,F.jsxs)(`button`,{type:`button`,className:G.sortMenuButton,onClick:()=>y(e=>!e),"aria-expanded":v,"aria-controls":n,children:[(0,F.jsx)(`span`,{className:G.sortButtonLabel,children:`Sort`}),(0,F.jsx)(`span`,{className:G.sortButtonValue,children:ol[f]}),(0,F.jsx)(`span`,{className:G.sortButtonDirection,children:m===`ascending`?`Asc`:`Desc`}),(0,F.jsx)(`span`,{className:`${G.sortButtonCaret}${v?` ${G.sortButtonCaretOpen}`:``}`,"aria-hidden":`true`,children:`▾`})]}),v&&(0,F.jsxs)(`div`,{id:n,className:G.sortPopover,children:[(0,F.jsxs)(`div`,{className:G.sortGroup,role:`group`,"aria-labelledby":`${n}-field-title`,children:[(0,F.jsx)(`div`,{id:`${n}-field-title`,className:G.sortGroupTitle,children:`Sort By`}),il.map(e=>(0,F.jsxs)(`label`,{className:G.sortOption,children:[(0,F.jsx)(`input`,{type:`radio`,name:`${n}-field`,value:e.value,checked:f===e.value,onChange:()=>te(e.value)}),(0,F.jsx)(`span`,{children:e.label})]},e.value))]}),f===`property`&&(0,F.jsxs)(`div`,{className:G.propertySortRow,children:[(0,F.jsx)(`label`,{className:G.propertyLabel,htmlFor:i,children:`Property Key`}),(0,F.jsx)(`input`,{id:i,className:G.propertyInput,value:g,onChange:e=>_(e.target.value),placeholder:`skill`,"aria-label":`Property key to sort by`})]}),(0,F.jsxs)(`div`,{className:G.sortGroup,role:`group`,"aria-labelledby":`${n}-direction-title`,children:[(0,F.jsx)(`div`,{id:`${n}-direction-title`,className:G.sortGroupTitle,children:`Sort Direction`}),al.map(e=>(0,F.jsxs)(`label`,{className:G.sortOption,children:[(0,F.jsx)(`input`,{type:`radio`,name:`${n}-direction`,value:e.value,checked:m===e.value,onChange:()=>h(e.value)}),(0,F.jsx)(`span`,{children:e.label})]},e.value))]})]})]})}),(0,F.jsx)(`div`,{className:G.itemList,children:s.isLoading?(0,F.jsx)(`div`,{className:G.loading,children:`Loading…`}):s.items.length===0?(0,F.jsx)(rl,{}):E.map(e=>(0,F.jsx)(Qc,{item:e,onRemove:w,onOpenMenu:b,onCloseMenu:x},e.id))}),c&&(0,F.jsxs)(`div`,{className:G.inspectPanel,children:[(0,F.jsxs)(`div`,{className:G.inspectHeader,children:[(0,F.jsxs)(`span`,{children:[`Inspect node `,c.node.alias]}),(0,F.jsx)(`button`,{className:G.inspectClose,onClick:()=>l(null),"aria-label":`Close inspect panel`,children:`✕`})]}),(0,F.jsx)(`div`,{className:G.inspectBody,children:(0,F.jsx)(o,{data:{node:c.node,connections:c.connections},style:r})})]}),u&&T&&(0,F.jsx)(nl,{open:!0,x:u.x,y:u.y,canPasteToInput:e,onPasteToInput:()=>S(T),onInspect:()=>C(T),onClose:x})]})}function cl(e){let{wheelTargetRef:t,scrollRef:n,contentWrapperRef:r,currentIndex:i,totalPages:a,onNavigatePrev:o,onNavigateNext:s}=e,c=(0,A.useRef)(0),l=(0,A.useRef)(null),u=(0,A.useRef)(!1),d=(0,A.useRef)(null),f=(0,A.useRef)(o),p=(0,A.useRef)(s),m=(0,A.useRef)(i),h=(0,A.useRef)(a);(0,A.useEffect)(()=>{f.current=o}),(0,A.useEffect)(()=>{p.current=s}),(0,A.useEffect)(()=>{m.current=i}),(0,A.useEffect)(()=>{h.current=a}),(0,A.useEffect)(()=>{d.current!==null&&(clearTimeout(d.current),d.current=null),r.current&&(r.current.style.transition=`none`,r.current.style.transform=`translateY(0)`),c.current=0,l.current=null},[i]),(0,A.useEffect)(()=>{let e=t.current;if(!e)return;function i(){c.current=0,l.current=null,r.current&&(r.current.style.transition=`transform 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94)`,r.current.style.transform=`translateY(0)`)}function a(e){if(e.deltaY===0)return;let t=n.current;if(!t)return;let a=t.scrollTop<=0,o=t.scrollTop+t.clientHeight>=t.scrollHeight-1,s=e.deltaY<0,g=e.deltaY>0,_=a&&s,v=o&&g;if(!_&&!v){i();return}if(u.current)return;let y=m.current,b=h.current;if(_&&y===0||v&&y===b-1)return;let x=_?`prev`:`next`;if(l.current!==null&&l.current!==x&&i(),l.current=x,c.current+=Math.abs(e.deltaY),r.current){let e=x===`prev`?-1:1,t=c.current*(18/120),n=Math.min(t,18)*e;r.current.style.transition=`none`,r.current.style.transform=`translateY(${n}px)`}if(d.current!==null&&clearTimeout(d.current),d.current=setTimeout(i,180),c.current>=120){d.current!==null&&clearTimeout(d.current);let e=l.current;i(),u.current=!0,e===`prev`?f.current():p.current(),setTimeout(()=>{u.current=!1},650)}}return e.addEventListener(`wheel`,a,{passive:!0}),()=>{d.current!==null&&clearTimeout(d.current),e.removeEventListener(`wheel`,a)}},[])}var ll={helpRoot:`_helpRoot_18tja_2`,categoryNav:`_categoryNav_18tja_11`,categoryTabScroller:`_categoryTabScroller_18tja_21`,categoryTab:`_categoryTab_18tja_21`,categoryTabActive:`_categoryTabActive_18tja_71`,maximizeButton:`_maximizeButton_18tja_78`,closeButton:`_closeButton_18tja_100`,helpBody:`_helpBody_18tja_122`,emptyFallback:`_emptyFallback_18tja_130`,helpContent:`_helpContent_18tja_147`,topicLink:`_topicLink_18tja_226`,helpBodyContent:`_helpBodyContent_18tja_271`,chipStrip:`_chipStrip_18tja_276`,chipStripLabel:`_chipStripLabel_18tja_294`,topicChip:`_topicChip_18tja_310`,topicChipActive:`_topicChipActive_18tja_338`};function ul(e){return typeof e==`string`?e:typeof e==`number`?String(e):Array.isArray(e)?e.map(ul).join(``):A.isValidElement(e)?ul(e.props.children):``}function dl(e){if(!e.trim().toLowerCase().startsWith(`help `))return null;let t=e.trim().slice(5).replace(/\s*\(.*\)\s*$/,``).trim().toLowerCase();return t.length>0?t:null}function fl({activeTopic:e,onNavigate:t,onClose:n,onToggleMaximize:r,isMaximized:i,contentProfile:a=`minigraph`}){let o=(0,A.useRef)(null),s=(0,A.useRef)(null),c=(0,A.useRef)(null),l=(0,A.useRef)(null);(0,A.useEffect)(()=>{o.current&&(o.current.scrollTop=0)},[e]),(0,A.useEffect)(()=>{let e=l.current;if(!e)return;let t=e.querySelector(`[aria-current="step"]`);t&&t.scrollIntoView({block:`nearest`,inline:`nearest`,behavior:`smooth`})},[e]);let u=(0,A.useMemo)(()=>Qt(a),[a]),d=(0,A.useMemo)(()=>rn(a),[a]),f=en(e,a),p=(0,A.useMemo)(()=>tn(f,a),[f,a]),m=p.length,h=(0,A.useMemo)(()=>u.find(e=>e.id===f)?.chipStripLabel??null,[f,u]),g=d.indexOf(e),_=g<0?0:g,v=d.length;cl({wheelTargetRef:s,scrollRef:o,contentWrapperRef:c,currentIndex:_,totalPages:v,onNavigatePrev:()=>t(d[_-1]??``),onNavigateNext:()=>t(d[_+1]??d[d.length-1])});let y=Jt(e,a);return(0,F.jsxs)(`div`,{className:ll.helpRoot,role:`region`,"aria-label":`Help browser`,ref:s,children:[(0,F.jsxs)(`nav`,{className:ll.categoryNav,"aria-label":`Help categories`,children:[(0,F.jsx)(`div`,{className:ll.categoryTabScroller,children:u.map(e=>(0,F.jsx)(`button`,{className:[ll.categoryTab,e.id===f?ll.categoryTabActive:``].join(` `).trim(),"aria-current":e.id===f?`true`:void 0,onClick:()=>{t(tn(e.id,a)[0]??``)},children:e.label},e.id))}),r&&(0,F.jsx)(`button`,{className:ll.maximizeButton,onClick:r,"aria-label":i?`Restore help panel`:`Maximize help panel`,children:i?`⊞`:`⛶`}),n&&(0,F.jsx)(`button`,{className:ll.closeButton,onClick:n,"aria-label":`Close help panel`,children:`×`})]}),m>1&&(0,F.jsxs)(`div`,{className:ll.chipStrip,ref:l,children:[h!==null&&(0,F.jsx)(`span`,{className:ll.chipStripLabel,children:h}),p.map(n=>{let r=n===e,i=nn(n,f);return(0,F.jsx)(`button`,{className:[ll.topicChip,r?ll.topicChipActive:``].join(` `).trim(),"aria-current":r?`step`:void 0,onClick:()=>t(n),children:i},n)})]}),(0,F.jsx)(`div`,{className:ll.helpBody,ref:o,children:(0,F.jsx)(`div`,{className:ll.helpBodyContent,ref:c,children:y===null?(0,F.jsxs)(`div`,{className:ll.emptyFallback,children:[(0,F.jsxs)(`code`,{children:[`help `,e||``]}),`\xA0 not found in the local bundle.`]}):(0,F.jsx)(`div`,{className:ll.helpContent,children:(0,F.jsx)(E,{remarkPlugins:[D],components:e===``?{li:({children:e,...n})=>{let r=dl(ul(e).trim());return r!==null&&Jt(r,a)!==null?(0,F.jsx)(`li`,{...n,children:(0,F.jsx)(`button`,{className:ll.topicLink,"aria-label":`Open help topic: ${r}`,onClick:()=>t(r),children:e})}):(0,F.jsx)(`li`,{...n,children:e})}}:void 0,children:y})})})})]})}function pl({existingItem:e,pendingItem:t,onReplace:n,onCancel:r}){let i=(0,A.useRef)(null);return(0,A.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal()},[]),(0,F.jsxs)(`dialog`,{ref:i,className:G.dialog,onClose:r,"aria-labelledby":`duplicate-dialog-title`,children:[(0,F.jsx)(`h2`,{id:`duplicate-dialog-title`,className:G.dialogTitle,children:`Duplicate Node`}),(0,F.jsxs)(`p`,{className:G.dialogBody,children:[`A clipboard item with alias `,(0,F.jsxs)(`strong`,{children:[`"`,t.node.alias,`"`]}),` already exists (clipped `,Xc(e.clippedAt),`).`]}),(0,F.jsx)(`p`,{className:G.dialogBody,children:`Replace it with the new snapshot?`}),(0,F.jsxs)(`div`,{className:G.dialogActions,children:[(0,F.jsx)(`button`,{className:G.cancelBtn,onClick:r,children:`Cancel`}),(0,F.jsx)(`button`,{className:G.replaceBtn,onClick:n,children:`Replace`})]})]})}function ml(e,t,n=`minigraph`){if(!t)return null;let r=e.trim().toLowerCase();if(r!==`help`&&!r.startsWith(`help `))return null;let i=an(e);return Jt(i,n)===null?null:i}var hl=class{constructor(){this.listeners=new Map}on(e,t){let n=e;return this.listeners.has(n)||this.listeners.set(n,new Set),this.listeners.get(n).add(t),()=>{this.listeners.get(n)?.delete(t)}}emit(e){let t=this.listeners.get(e.kind);t&&t.forEach(t=>{try{t(e)}catch(t){console.error(`[ProtocolBus] listener for '${e.kind}' threw:`,t)}})}clear(){this.listeners.clear()}},gl=`(ws-\\d+-\\d+)`,_l=RegExp(`^session ${gl} started(?:\\nCompanion endpoint: (\\/api\\/companion\\/${gl}))?$`,`i`),vl=RegExp(`^Session ${gl} started since (.+)$`),yl=RegExp(`^subscribed to ${gl}$`),bl=/^subscribed by \[(.*)]$/,xl=RegExp(`^Subscribed to ${gl}$`),Sl=RegExp(`^Session unsubscribed from ${gl}$`),Cl=RegExp(`^Session ${gl} not found$`),wl=RegExp(`^${gl} is not a primary session$`),Tl=RegExp(`^You have already subscribed to ${gl}(?:\\nPlease do 'session reset' before subscribing to another session)?$`),El=RegExp(`^${gl} subscribed to your session$`),Dl=RegExp(`^${gl} unsubscribed from your session$`),Ol=RegExp(`^Session ${gl} has closed$`);function kl(e){let t=e.trim();return t.length===0||t.startsWith(`> `)?null:t}function Al(e){return e.split(`,`).map(e=>e.trim()).filter(e=>e.length>0&&Ws(e))}function jl(e){let t=kl(e);if(!t)return null;let n=t.match(_l);return n?{sessionId:n[1],companionEndpoint:n[2]??null}:null}function Ml(e){let t=kl(e);if(!t)return null;let n=t.split(`
`).map(e=>e.trim()).filter(Boolean),r=n[0];if(!r)return null;let i=r.match(vl);if(!i)return null;let a=null,o=[];for(let e of n.slice(1)){let t=e.match(yl);if(t){a=t[1];continue}let n=e.match(bl);n&&(o=Al(n[1]))}return{sessionId:i[1],startedSince:i[2],subscribedTo:a,subscribers:o}}function Nl(e){let t=kl(e);if(!t)return null;let n=t.match(xl);if(n)return{command:`subscribe`,status:`accepted`,sessionId:n[1],message:t};let r=t.match(Sl);if(r)return{command:`unsubscribe`,status:`accepted`,sessionId:r[1],message:t};let i=t.match(Cl);if(i)return{command:`subscribe`,status:`rejected`,sessionId:i[1],message:t};let a=t.match(wl);if(a)return{command:`subscribe`,status:`rejected`,sessionId:a[1],message:t};let o=t.match(Tl);return o?{command:`subscribe`,status:`rejected`,sessionId:o[1],message:t}:t===`You cannot subscribe to yourself`?{command:`subscribe`,status:`rejected`,sessionId:null,message:t}:t===`Nothing to unsubscribe`?{command:`unsubscribe`,status:`rejected`,sessionId:null,message:t}:t===`Invalid session command`?{command:`unknown`,status:`rejected`,sessionId:null,message:t}:null}function Pl(e){let t=kl(e);if(!t)return null;let n=t.match(El);if(n)return{type:`subscriber-joined`,sessionId:n[1],message:t};let r=t.match(Dl);if(r)return{type:`subscriber-left`,sessionId:r[1],message:t};let i=t.match(Ol);return i?{type:`host-closed`,sessionId:i[1],message:t}:null}var Fl=new Set([`info`,`error`,`ping`,`welcome`]);function Il(e,t){let n=[],r={msgId:e,raw:t},i=!1,a=!1,o=!1,s=!1,c=!1,l=!1,u=we(t);if(u.isJSON){let e=u.data;if(typeof e.type==`string`){let i=e.type;return n.push({...r,kind:`lifecycle`,type:i,knownType:Fl.has(i),message:typeof e.message==`string`?e.message:t,time:e.time??null}),n.length>0?n:[{...r,kind:`unclassified`}]}return n.push({...r,kind:`json.response`,data:u.data}),n.length>0?n:[{...r,kind:`unclassified`}]}let d=je(t);d&&(c=!0,n.push({...r,kind:`payload.large`,apiPath:d.apiPath,byteSize:d.byteSize,filename:d.filename}));let f=Me(t);f&&(o=!0,n.push({...r,kind:`upload.invitation`,uploadPath:f}));let p=Ae(t);if(p&&(s=!0,n.push({...r,kind:`upload.contentPath`,uploadPath:p})),ke(t)){a=!0;let e=Oe(t);e&&n.push({...r,kind:`graph.link`,apiPath:e})}if(a){let e=Te(t);e&&n.push({...r,kind:`graph.exported`,graphName:e.graphName,apiPath:e.apiPath})}let m=Ge(t);m&&n.push({...r,kind:`graph.mutation`,mutationType:m});let h=kn(t);h&&n.push({...r,kind:`graph.instance.created`,mockEntries:h.mockEntries,ttlMs:h.ttlMs}),An(t)&&n.push({...r,kind:`graph.instance.cleared`});let g=jn(t);g&&n.push({...r,kind:`graph.run.terminal`,status:g.status,elapsedMs:g.elapsedMs});let _=Mn(t);_&&n.push({...r,kind:`command.error`,message:_});let v=We(t);v&&n.push({...r,kind:`minigraph.nodeAction.textResult`,status:v.status,action:v.action,alias:v.alias,targetAlias:v.targetAlias,message:v.message}),v&&(v.action===`create-node`||v.status===`error`)&&n.push({...r,kind:`minigraph.createNode.textResult`,status:v.status,alias:v.alias,message:v.message}),t===`Session restarted`&&(l=!0,n.push({...r,kind:`session.reset`}));let y=jl(t);y&&(l=!0,n.push({...r,kind:`minigraph.session.started`,sessionId:y.sessionId,companionEndpoint:y.companionEndpoint}));let b=Ml(t);b&&(l=!0,n.push({...r,kind:`minigraph.session.status`,sessionId:b.sessionId,startedSince:b.startedSince,subscribedTo:b.subscribedTo,subscribers:b.subscribers}));let x=Nl(t);x&&(l=!0,n.push({...r,kind:`minigraph.session.commandResult`,command:x.command,status:x.status,sessionId:x.sessionId,message:x.message}));let S=Pl(t);S&&(l=!0,n.push({...r,kind:`minigraph.session.notification`,type:S.type,sessionId:S.sessionId,message:S.message})),t.startsWith(`> `)&&(i=!0,n.push({...r,kind:`command.echo`,commandText:t.slice(2)})),Ne(t)&&n.push({...r,kind:`command.helpOrDescribe`,commandText:t.slice(2)});let C=Pe(t);C&&n.push({...r,kind:`command.importGraph`,graphName:C});let w=Ee(t);return w&&n.push({...r,kind:`graph.export.failed`,reason:w.reason}),!i&&!a&&!o&&!s&&!c&&!l&&De(t)&&n.push({...r,kind:`docs.response`,isMarkdown:!0}),n.length===0&&n.push({...r,kind:`unclassified`}),n}function Ll({messages:e,bus:t}){let n=(0,A.useRef)(-1);(0,A.useEffect)(()=>{e.length>0&&(n.current=e[e.length-1].id)},[]);let r=(0,A.useMemo)(()=>{let t=new Map;for(let n of e)t.set(n.id,Il(n.id,n.raw));return t},[e]);return(0,A.useEffect)(()=>{if(e.length===0)return;let i=e.filter(e=>e.id>n.current);if(i.length!==0){n.current=e[e.length-1].id;for(let e of i){let n=r.get(e.id);if(n)for(let e of n)t.emit(e)}}},[e,t,r]),{classificationMap:r}}var Rl={console:`40%`,"node-edit":`30%`,upload:`30%`};function K({config:e}){let{title:t,wsPath:n,storageKeyPayload:r,storageKeyHistory:i,storageKeyTab:a,storageKeySavedGraphs:o,supportsUpload:s,supportsClipboard:c,supportsHelp:l,helpContentProfile:u=`minigraph`,supportsAuthoring:d,supportsGraphRun:f,supportsSessionCollaboration:p,tabs:m}=e,h=ee(),[g,_]=fe(r,``),v=xe(),[y,b]=(0,A.useState)(()=>v.peekPendingPayload(n)),{takePendingPayload:x}=v;(0,A.useEffect)(()=>{let e=x(n);e!==null&&b(e)},[x,n]);let S=y??g,C=(0,A.useCallback)(e=>{b(null),_(e)},[_]),w=(0,A.useMemo)(()=>S?le(S):{valid:!0,error:null,type:null},[S]),{toasts:te,addToast:T,removeToast:E}=de(),ne=(0,A.useRef)(new hl).current,D=Je({wsPath:n,storageKeyHistory:i,payload:S,addToast:T,bus:ne,handleLocalCommand:(0,A.useCallback)(e=>ml(e,l===!0,u)!==null,[l,u])}),{classificationMap:O}=Ll({messages:D.messages,bus:ne}),[se,k]=Vn(n),ce=p===!0&&m.includes(`graph`),{graphData:j,setGraphData:N,rightTab:P,setRightTab:pe,isRefreshing:me,initialFetchFailed:he}=Qe(se,T,m[0],m,a,ce),{uploadPanelPath:ge,successfulUploadPaths:_e,handleOpenUploadPanel:ve,handleCloseUploadPanel:ye,handleCloseUploadPath:be,handleUploadSuccess:Se,handleUploadError:Ce,resetSuccessfulPaths:we}=ln({bus:ne,addToast:T});et({bus:ne,pinnedGraphPath:se,setPinnedGraphPath:k,connected:D.connected,sendRawText:D.sendRawText,addToast:T});let Te=Ir({bus:ne,connected:D.connected,sendRawText:D.sendRawText,addToast:T}),Ee=(0,A.useRef)(null),De=(0,A.useRef)(null),Oe=(0,A.useRef)(new Map),ke=(0,A.useCallback)((e,t)=>{if(t===null){T(e,`success`);return}T(e,`success`,{durationMs:6e3,action:{label:`Undo`,onClick:()=>Te.undoEntry(t)}})},[T,Te.undoEntry]),Ae=(0,A.useCallback)(e=>{if(e.action===`edit-node`){let t=Ee.current;Ee.current=null,t&&t.alias===e.alias&&ke(`Updated node ${t.alias}`,Te.push(jr(t)));return}if(e.action===`create-node`&&e.alias){ke(`Created node ${e.alias}`,Te.push(Mr(e.alias)));return}if(e.action===`create-connection`){let t=De.current;De.current=null,ke(`Connected ${e.alias??``} → ${e.targetAlias??``}`,Te.push(t));return}if(e.action===`delete-node`&&e.alias){let t=Oe.current.get(e.alias)??null;Oe.current.delete(e.alias),ke(`Deleted node ${e.alias}`,Te.push(t))}},[Te.push,ke]),je=(0,A.useRef)(!1);(0,A.useEffect)(()=>{je.current&&!D.connected&&(k(null),N(null)),je.current=D.connected},[D.connected,k,N]);let[Me,Ne]=fe(e.storageKeyHelpTopic??`help-topic-fallback`,``),[Pe,Fe]=fe(`help-panel-open`,!1),[Ie,Le]=(0,A.useState)(()=>!!l&&!Pe),[Re,ze]=(0,A.useState)(!1),Be=(0,A.useRef)(null),Ve=(0,A.useCallback)(()=>{Ie&&(ze(!0),Be.current=setTimeout(()=>Le(!1),400))},[Ie]);(0,A.useEffect)(()=>{if(!Ie||Re)return;let e=setTimeout(Ve,3e3);return()=>clearTimeout(e)},[Ie,Re,Ve]),(0,A.useEffect)(()=>{Pe&&Ie&&Ve()},[Pe,Ie,Ve]),(0,A.useEffect)(()=>()=>{Be.current&&clearTimeout(Be.current)},[]),(0,A.useEffect)(()=>{if(!l)return;let e=e=>{e.ctrlKey&&e.key==="`"&&(e.preventDefault(),Fe(e=>!e))};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[l,Fe]),on({bus:ne,setHelpTopic:Ne,onTabSwitch:l?()=>Fe(!0):()=>{},enabled:l===!0,contentProfile:u}),un({bus:ne,connected:D.connected,appendMessage:D.appendMessage,addToast:T});let[He,Ue]=fe(`console-panel-open`,!0),We=Bc(),[Ge,Ke]=fe(`clipboard-sidebar-open`,!1),[qe,Xe]=(0,A.useState)(null),Ze=(0,A.useCallback)(e=>{let t=Wn(e,j);D.setCommand(t.command),T(`${t.verb===`create`?`Create`:`Update`} command for "${e.node.alias}" pasted to input`,`info`)},[j,D.setCommand,T]),tt=(0,A.useCallback)(e=>{let t=We.items.find(t=>t.id===e);if(!t){T(`Clipboard item is no longer available. It may have been removed in another tab.`,`error`);return}let n=Wn(t,j);if(!D.sendRawText(n.command)){T(`Could not send clipboard paste command because the WebSocket is not open.`,`error`);return}T(`Clipboard node "${t.node.alias}" sent as ${n.verb}. Waiting for backend response.`,`info`)},[We.items,j,D.sendRawText,T]),nt=(0,A.useCallback)(async(t,r)=>{try{let i=await We.clipNode(t,r,{sourceWsPath:n,sourceLabel:e.label});switch(i.status){case`added`:T(`Node "${t.alias}" clipped to workspace`,`success`);break;case`duplicate`:Xe({pendingItem:i.pendingItem,existingItem:i.existingItem});break;case`error`:T(`Clip failed: ${i.message}`,`error`);break}}catch(e){T(`Clip failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[We,n,e.label,T]),rt=(0,A.useCallback)(async t=>{if(t.length===0){T(`No selected nodes are available to clip.`,`info`);return}if(t.length>100){T(`Select 100 or fewer nodes to clip at once.`,`error`);return}let r={added:0,duplicates:0,failed:0};for(let i of t){if(!i.node?.alias?.trim()){r.failed+=1;continue}try{let t=await We.clipNode(i.node,i.connections,{sourceWsPath:n,sourceLabel:e.label});t.status===`added`&&(r.added+=1),t.status===`duplicate`&&(r.duplicates+=1),t.status===`error`&&(r.failed+=1)}catch{r.failed+=1}}let i=Kn(r);T(i.message,i.type)},[T,We,e.label,n]),it=dn(o??``),{defaultName:at,savedName:ot,resetName:st}=_n(o?`${o}-untitled-counter`:`untitled-counter`,ne,D.connected,D.connectionEpoch),ct=(0,A.useMemo)(()=>{let e=j?.nodes.find(e=>e.types.includes(`Root`)),t=typeof e?.properties?.name==`string`?e.properties.name:void 0;return t?.trim()?t:null},[j])??at,lt=(0,A.useMemo)(()=>qn(D.sendRawText),[D.sendRawText]),I=Hs({bus:ne,connected:D.connected,graphData:j,executor:lt,onAccepted:Ae,onUserMessage:T}),ut=I.state,dt=d&&ut.status===`open`&&(ut.action===`edit-node`||ut.action===`create-node`)?ut:null,ft=d&&ut.status===`open`&&ut.action===`create-connection`?ut:null,[pt,mt]=(0,A.useState)(null),ht=dt!==null||ft!==null;(0,A.useEffect)(()=>{let e=e=>{if(e.key!==`z`&&e.key!==`Z`||!(e.metaKey||e.ctrlKey)||e.shiftKey||e.altKey)return;let t=e.target;t instanceof Element&&t.closest(`input, textarea, select, [contenteditable="true"]`)||ht||Te.hasEntries()&&(e.preventDefault(),Te.undoLast())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[ht,Te.hasEntries,Te.undoLast]);let gt=(0,A.useCallback)((e,t,n)=>{mt(n??null),De.current=j?Nr(j,e,t):null,I.openCreateConnection(e,t)},[I.openCreateConnection,j]),_t=(0,A.useCallback)(e=>{if(!j)return;let t=hr(j,e);t&&(Te.runCommands(t.commands),ke(`Deleted ${gr(t.removed)}`,Te.push(Ar(t.removed))))},[j,Te.push,Te.runCommands,ke]),vt=(0,A.useCallback)(e=>{Ee.current=e,I.openEditNode(e)},[I.openEditNode]),yt=(0,A.useCallback)(e=>{Oe.current.set(e.alias,j?Pr(j,e):null),I.deleteNode(e)},[I.deleteNode,j]),bt=D.consoleRef,xt=He&&dt===null&&ge===null;(0,A.useEffect)(()=>{xt&&bt.current&&(bt.current.scrollTop=bt.current.scrollHeight)},[xt,bt]);let St=qs({enabled:p===!0,bus:ne,classificationMap:O,connected:D.connected,sendRawText:D.sendRawText,addToast:T});$e({enabled:ce,sessionGraphPath:St.state.sessionId===null?null:`/api/graph/session/${St.state.sessionId}`,pinnedGraphPath:se,initialFetchFailed:he,graphData:j,setGraphData:N,setRightTab:pe});let Ct=Ln({enabled:f===!0,bus:ne,connected:D.connected,connectionEpoch:D.connectionEpoch,graphData:j,graphIdentity:se,isPrimary:f!==!0||St.state.sessionId!==null&&St.isPrimary,sendRawText:D.sendRawText,addToast:T,onWorkflowInputInvalidated:be}),{handleSaveGraph:wt,handleLoadGraph:Tt}=zn({bus:ne,connected:D.connected,sendRawText:D.sendRawText,saveGraph:o?it.saveGraph:null,addToast:T}),Et=(0,A.useCallback)(e=>{let t=O.get(e.id)?.find(e=>e.kind===`graph.link`);t&&k(t.apiPath)},[O]),{handleSendToJsonPath:Dt}=sn({ctx:v,navigate:h,addToast:T,wsPath:n}),Ot=Ye(`(max-width: 768px)`),{defaultLayout:kt,onLayoutChanged:At}=ae({id:e.path+`-panel-split-v2`,storage:localStorage,onlySaveAfterUserInteractions:!0}),jt=dt===null?ge===null?He?`console`:null:`upload`:`node-edit`,Mt=(0,A.useRef)(null),Nt=(0,A.useRef)(jt);(0,A.useEffect)(()=>{let e=Nt.current;if(Nt.current=jt,jt===null||jt===e)return;let t=requestAnimationFrame(()=>{Mt.current?.resize(Rl[jt])});return()=>cancelAnimationFrame(t)},[jt]);let Pt=!!c&&Ge,Ft=jt===null?Pt?`80%`:`100%`:Pt?`40%`:`60%`,It=(0,A.useCallback)(()=>C(ue(S)),[S]),Lt=(0,A.useCallback)(()=>{D.clearMessages(),k(null),N(null),we(),st()},[D.clearMessages,N,we,st]),Rt=(0,A.useCallback)((e,t)=>{Se(e),Ct.handleInputUploadSuccess(t)},[Ct.handleInputUploadSuccess,Se]),zt=(0,A.useCallback)(e=>{ye(),Ct.handleInputCancelled(e)},[Ct.handleInputCancelled,ye]),Bt=ge!==null&&Ct.isWorkflowInputPanel&&Ct.workflowUploadPath===ge;return(0,F.jsxs)(`div`,{className:M.wrapper,children:[(0,F.jsx)(Hr,{toasts:te,onRemove:E}),ft!==null&&(0,F.jsx)(fs,{formState:ft.formState,phase:ft.phase,lockReason:ft.phase===`sending`?`sending`:ft.connectionLost?`disconnected`:null,serverMessage:ft.serverMessage,validationErrors:I.validationErrors,anchor:pt,onFormStateChange:I.updateFormState,onSubmit:I.submit,onClose:I.close}),(0,F.jsxs)(`header`,{className:M.header,children:[(0,F.jsx)(`h1`,{className:M.title,children:t}),(0,F.jsxs)(`div`,{className:M.headerActions,children:[o&&(0,F.jsx)(ni,{disabled:!j,defaultName:at,savedName:ot,onSave:wt,nameExists:it.hasGraph,connected:D.connected}),o&&it.savedGraphs.length>0&&(0,F.jsx)(ii,{savedGraphs:it.savedGraphs,onLoad:Tt,onDelete:it.deleteGraph,connected:D.connected}),(0,F.jsx)(`button`,{className:M.panelToggle,onClick:()=>{if(dt!==null){dt.phase!==`sending`&&(I.close(),Ue(!0));return}if(ge!==null){zt(ge),Ue(!0);return}Ue(e=>!e)},"aria-label":dt===null?ge===null?He?`Hide console panel`:`Show console panel`:`Show console panel and close the upload form`:`Show console panel and close the node editor`,"aria-pressed":xt,title:dt===null?ge===null?void 0:`Closes the upload form`:`Closes the node editor`,children:`Console`}),c&&(0,F.jsxs)(`button`,{className:M.panelToggle,onClick:()=>Ke(e=>!e),"aria-label":Ge?`Close workspace sidebar`:`Open workspace sidebar`,"aria-pressed":Ge,children:[`Workspace`,We.items.length>0?` (${We.items.length})`:``]}),(0,F.jsx)($r,{addToast:T,sessionCollaboration:p?St:null}),l&&(0,F.jsxs)(`div`,{className:M.helpButtonWrapper,children:[(0,F.jsx)(`button`,{className:`${M.helpToggle}${Ie&&!Re?` ${M.helpTogglePulsing}`:``}`,onClick:()=>Fe(e=>!e),"aria-label":Pe?`Close help panel`:`Open help panel`,"aria-pressed":Pe,children:`?`}),Ie&&(0,F.jsxs)(`div`,{className:`${M.helpHint}${Re?` ${M.helpHintFading}`:``}`,onClick:Ve,role:`status`,children:[(0,F.jsx)(`kbd`,{className:M.helpHintKbd,children:"Ctrl + `"}),` to toggle help`]})]})]})]}),qe&&(0,F.jsx)(pl,{existingItem:qe.existingItem,pendingItem:qe.pendingItem,onReplace:async()=>{try{await We.confirmReplace(qe.pendingItem,qe.existingItem.id),Xe(null),T(`Clipboard item "${qe.pendingItem.node.alias}" replaced`,`success`)}catch(e){T(`Replace failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},onCancel:()=>{Xe(null),T(`Clip cancelled`,`info`)}}),(0,F.jsxs)(oe,{className:M.panelGroup,orientation:Ot?`vertical`:`horizontal`,defaultLayout:kt,onLayoutChanged:At,children:[jt!==null&&(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(ie,{panelRef:Mt,defaultSize:Rl[jt],minSize:`25%`,children:dt===null?ge===null?(0,F.jsx)(Qo,{messages:D.messages,classificationMap:O,onCopy:D.copyMessages,onClear:Lt,consoleRef:D.consoleRef,command:D.command,onCommandChange:D.setCommand,onCommandKeyDown:D.handleKeyDown,onSend:D.sendCommand,sendDisabled:!D.connected||!D.command.trim(),inputDisabled:!D.connected,commandHistory:D.history,onGraphLinkMessage:Et,onCopyMessage:()=>T(`Copied to clipboard`,`success`),onSendToJsonPath:Dt,onUploadMockData:ve,successfulUploadPaths:_e}):(0,F.jsx)(ls,{uploadPath:ge,onSuccess:Rt,onClose:zt,onError:Ce,title:Bt?`▶ Mock Graph Input`:void 0,description:Bt?`Add the JSON body and keep this graph instantiated for a later run.`:void 0,inputPathHints:Bt?Ct.inputBodyPaths:void 0,submitLabel:Bt?`Upload & Instantiate`:void 0},ge):(0,F.jsx)(is,{mode:dt.action===`edit-node`?`edit`:`create`,formState:dt.formState,phase:dt.phase,lockReason:dt.phase===`sending`?`sending`:dt.connectionLost?`disconnected`:null,serverMessage:dt.serverMessage,validationErrors:I.validationErrors,onFormStateChange:I.updateFormState,onSubmit:I.submit,onClose:I.close})}),(0,F.jsx)(re,{className:M.resizeHandle,"aria-label":`Resize panels`})]}),(0,F.jsx)(ie,{defaultSize:Ft,minSize:`20%`,children:(0,F.jsx)(zo,{tabs:m,payload:S,onChange:C,validation:w,onFormat:It,onUpload:s?D.uploadPayload:void 0,graphData:j,graphName:ct,activeTab:P,onTabChange:pe,onGraphRenderError:e=>T(e,`error`),onGraphDataCopySuccess:()=>T(`Graph JSON copied to clipboard!`,`success`),onGraphDataCopyError:()=>T(`Copy failed`,`error`),graphRunControls:f?{phase:Ct.phase,canInstantiate:Ct.canInstantiate,canRun:Ct.canRun,disabledReason:Ct.disabledReason,onInstantiate:Ct.instantiateGraph,onRun:Ct.runGraph}:void 0,isGraphRefreshing:me,onClipNode:c?nt:void 0,onClipNodes:c?rt:void 0,onClipboardDrop:c?tt:void 0,isConnected:D.connected,supportsAuthoring:d,onCreateNode:d?I.openCreateNode:void 0,onCreateConnection:d?gt:void 0,onEditNode:d?vt:void 0,onDeleteNode:d?yt:void 0,onDeleteNodes:d?I.deleteNodes:void 0,onDeleteConnections:d?_t:void 0,panelLayoutKey:`${jt??`closed`}|${Ge}|${Pe}`,helpPanel:l&&Pe?((e,t)=>(0,F.jsx)(fl,{activeTopic:Me,contentProfile:u,onNavigate:Ne,onClose:()=>Fe(!1),onToggleMaximize:e,isMaximized:t})):void 0})}),c&&Ge&&(0,F.jsxs)(F.Fragment,{children:[(0,F.jsx)(re,{className:M.resizeHandle,"aria-label":`Resize clipboard`}),(0,F.jsx)(ie,{defaultSize:`20%`,minSize:`10%`,maxSize:`40%`,children:(0,F.jsx)(sl,{connected:D.connected,onPasteToInput:Ze})})]})]})]})}function q(){let e=me[0].path;return(0,F.jsx)(be,{children:(0,F.jsx)(zc,{children:(0,F.jsx)(T,{children:(0,F.jsxs)(S,{children:[me.map(e=>(0,F.jsx)(C,{path:e.path,element:(0,F.jsx)(K,{config:e},e.path)},e.path)),(0,F.jsx)(C,{path:`*`,element:(0,F.jsx)(te,{to:e,replace:!0})})]})})})})}(0,j.createRoot)(document.getElementById(`root`)).render((0,F.jsx)(A.StrictMode,{children:(0,F.jsx)(q,{})}));
//# sourceMappingURL=index-CKkgr39d.js.map
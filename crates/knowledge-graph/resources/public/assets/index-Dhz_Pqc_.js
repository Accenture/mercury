import{r as e,t}from"./rolldown-runtime-QTnfLwEv.js";import{a as n,i as r,n as i,r as a,t as o}from"./vendor-json-view-XsbV5yWD.js";import{a as s,c,d as l,f as u,h as d,i as f,l as p,m,n as h,o as g,p as _,r as v,s as y,t as b,u as x}from"./vendor-xyflow-Dg2A4cNM.js";import{a as S,i as C,n as w,o as T,r as E,t as D}from"./vendor-router-DrFk6dZO.js";import{n as O,r as k,t as A}from"./vendor-markdown-Di8s8vbi.js";import{n as ee,r as j,t as te}from"./vendor-panels-Cn6VYsgu.js";(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();var ne=t((e=>{function t(e,t){var n=e.length;e.push(t);a:for(;0<n;){var r=n-1>>>1,a=e[r];if(0<i(a,t))e[r]=t,e[n]=a,n=r;else break a}}function n(e){return e.length===0?null:e[0]}function r(e){if(e.length===0)return null;var t=e[0],n=e.pop();if(n!==t){e[0]=n;a:for(var r=0,a=e.length,o=a>>>1;r<o;){var s=2*(r+1)-1,c=e[s],l=s+1,u=e[l];if(0>i(c,n))l<a&&0>i(u,c)?(e[r]=u,e[l]=n,r=l):(e[r]=c,e[s]=n,r=s);else if(l<a&&0>i(u,n))e[r]=u,e[l]=n,r=l;else break a}}return t}function i(e,t){var n=e.sortIndex-t.sortIndex;return n===0?e.id-t.id:n}if(e.unstable_now=void 0,typeof performance==`object`&&typeof performance.now==`function`){var a=performance;e.unstable_now=function(){return a.now()}}else{var o=Date,s=o.now();e.unstable_now=function(){return o.now()-s}}var c=[],l=[],u=1,d=null,f=3,p=!1,m=!1,h=!1,g=!1,_=typeof setTimeout==`function`?setTimeout:null,v=typeof clearTimeout==`function`?clearTimeout:null,y=typeof setImmediate<`u`?setImmediate:null;function b(e){for(var i=n(l);i!==null;){if(i.callback===null)r(l);else if(i.startTime<=e)r(l),i.sortIndex=i.expirationTime,t(c,i);else break;i=n(l)}}function x(e){if(h=!1,b(e),!m)if(n(c)!==null)m=!0,S||(S=!0,O());else{var t=n(l);t!==null&&ee(x,t.startTime-e)}}var S=!1,C=-1,w=5,T=-1;function E(){return g?!0:!(e.unstable_now()-T<w)}function D(){if(g=!1,S){var t=e.unstable_now();T=t;var i=!0;try{a:{m=!1,h&&(h=!1,v(C),C=-1),p=!0;var a=f;try{b:{for(b(t),d=n(c);d!==null&&!(d.expirationTime>t&&E());){var o=d.callback;if(typeof o==`function`){d.callback=null,f=d.priorityLevel;var s=o(d.expirationTime<=t);if(t=e.unstable_now(),typeof s==`function`){d.callback=s,b(t),i=!0;break b}d===n(c)&&r(c),b(t)}else r(c);d=n(c)}if(d!==null)i=!0;else{var u=n(l);u!==null&&ee(x,u.startTime-t),i=!1}}break a}finally{d=null,f=a,p=!1}i=void 0}}finally{i?O():S=!1}}}var O;if(typeof y==`function`)O=function(){y(D)};else if(typeof MessageChannel<`u`){var k=new MessageChannel,A=k.port2;k.port1.onmessage=D,O=function(){A.postMessage(null)}}else O=function(){_(D,0)};function ee(t,n){C=_(function(){t(e.unstable_now())},n)}e.unstable_IdlePriority=5,e.unstable_ImmediatePriority=1,e.unstable_LowPriority=4,e.unstable_NormalPriority=3,e.unstable_Profiling=null,e.unstable_UserBlockingPriority=2,e.unstable_cancelCallback=function(e){e.callback=null},e.unstable_forceFrameRate=function(e){0>e||125<e?console.error(`forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported`):w=0<e?Math.floor(1e3/e):5},e.unstable_getCurrentPriorityLevel=function(){return f},e.unstable_next=function(e){switch(f){case 1:case 2:case 3:var t=3;break;default:t=f}var n=f;f=t;try{return e()}finally{f=n}},e.unstable_requestPaint=function(){g=!0},e.unstable_runWithPriority=function(e,t){switch(e){case 1:case 2:case 3:case 4:case 5:break;default:e=3}var n=f;f=e;try{return t()}finally{f=n}},e.unstable_scheduleCallback=function(r,i,a){var o=e.unstable_now();switch(typeof a==`object`&&a?(a=a.delay,a=typeof a==`number`&&0<a?o+a:o):a=o,r){case 1:var s=-1;break;case 2:s=250;break;case 5:s=1073741823;break;case 4:s=1e4;break;default:s=5e3}return s=a+s,r={id:u++,callback:i,priorityLevel:r,startTime:a,expirationTime:s,sortIndex:-1},a>o?(r.sortIndex=a,t(l,r),n(c)===null&&r===n(l)&&(h?(v(C),C=-1):h=!0,ee(x,a-o))):(r.sortIndex=s,t(c,r),m||p||(m=!0,S||(S=!0,O()))),r},e.unstable_shouldYield=E,e.unstable_wrapCallback=function(e){var t=f;return function(){var n=f;f=t;try{return e.apply(this,arguments)}finally{f=n}}}})),M=t(((e,t)=>{t.exports=ne()})),N=t((e=>{var t=M(),r=n(),i=d();function a(e){var t=`https://react.dev/errors/`+e;if(1<arguments.length){t+=`?args[]=`+encodeURIComponent(arguments[1]);for(var n=2;n<arguments.length;n++)t+=`&args[]=`+encodeURIComponent(arguments[n])}return`Minified React error #`+e+`; visit `+t+` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`}function o(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function s(e){var t=e,n=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,t.flags&4098&&(n=t.return),e=t.return;while(e)}return t.tag===3?n:null}function c(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function l(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function u(e){if(s(e)!==e)throw Error(a(188))}function f(e){var t=e.alternate;if(!t){if(t=s(e),t===null)throw Error(a(188));return t===e?e:null}for(var n=e,r=t;;){var i=n.return;if(i===null)break;var o=i.alternate;if(o===null){if(r=i.return,r!==null){n=r;continue}break}if(i.child===o.child){for(o=i.child;o;){if(o===n)return u(i),e;if(o===r)return u(i),t;o=o.sibling}throw Error(a(188))}if(n.return!==r.return)n=i,r=o;else{for(var c=!1,l=i.child;l;){if(l===n){c=!0,n=i,r=o;break}if(l===r){c=!0,r=i,n=o;break}l=l.sibling}if(!c){for(l=o.child;l;){if(l===n){c=!0,n=o,r=i;break}if(l===r){c=!0,r=o,n=i;break}l=l.sibling}if(!c)throw Error(a(189))}}if(n.alternate!==r)throw Error(a(190))}if(n.tag!==3)throw Error(a(188));return n.stateNode.current===n?e:t}function p(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=p(e),t!==null)return t;e=e.sibling}return null}var m=Object.assign,h=Symbol.for(`react.element`),g=Symbol.for(`react.transitional.element`),_=Symbol.for(`react.portal`),v=Symbol.for(`react.fragment`),y=Symbol.for(`react.strict_mode`),b=Symbol.for(`react.profiler`),x=Symbol.for(`react.consumer`),S=Symbol.for(`react.context`),C=Symbol.for(`react.forward_ref`),w=Symbol.for(`react.suspense`),T=Symbol.for(`react.suspense_list`),E=Symbol.for(`react.memo`),D=Symbol.for(`react.lazy`),O=Symbol.for(`react.activity`),k=Symbol.for(`react.memo_cache_sentinel`),A=Symbol.iterator;function ee(e){return typeof e!=`object`||!e?null:(e=A&&e[A]||e[`@@iterator`],typeof e==`function`?e:null)}var j=Symbol.for(`react.client.reference`);function te(e){if(e==null)return null;if(typeof e==`function`)return e.$$typeof===j?null:e.displayName||e.name||null;if(typeof e==`string`)return e;switch(e){case v:return`Fragment`;case b:return`Profiler`;case y:return`StrictMode`;case w:return`Suspense`;case T:return`SuspenseList`;case O:return`Activity`}if(typeof e==`object`)switch(e.$$typeof){case _:return`Portal`;case S:return e.displayName||`Context`;case x:return(e._context.displayName||`Context`)+`.Consumer`;case C:var t=e.render;return e=e.displayName,e||=(e=t.displayName||t.name||``,e===``?`ForwardRef`:`ForwardRef(`+e+`)`),e;case E:return t=e.displayName||null,t===null?te(e.type)||`Memo`:t;case D:t=e._payload,e=e._init;try{return te(e(t))}catch{}}return null}var ne=Array.isArray,N=r.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,P=i.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,F={pending:!1,data:null,method:null,action:null},re=[],ie=-1;function ae(e){return{current:e}}function I(e){0>ie||(e.current=re[ie],re[ie]=null,ie--)}function L(e,t){ie++,re[ie]=e.current,e.current=t}var oe=ae(null),se=ae(null),ce=ae(null),le=ae(null);function ue(e,t){switch(L(ce,t),L(se,e),L(oe,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?Hd(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=Hd(t),e=Ud(t,e);else switch(e){case`svg`:e=1;break;case`math`:e=2;break;default:e=0}}I(oe),L(oe,e)}function de(){I(oe),I(se),I(ce)}function fe(e){e.memoizedState!==null&&L(le,e);var t=oe.current,n=Ud(t,e.type);t!==n&&(L(se,e),L(oe,n))}function R(e){se.current===e&&(I(oe),I(se)),le.current===e&&(I(le),Qf._currentValue=F)}var pe,me;function he(e){if(pe===void 0)try{throw Error()}catch(e){var t=e.stack.trim().match(/\n( *(at )?)/);pe=t&&t[1]||``,me=-1<e.stack.indexOf(`
    at`)?` (<anonymous>)`:-1<e.stack.indexOf(`@`)?`@unknown:0:0`:``}return`
`+pe+e+me}var ge=!1;function _e(e,t){if(!e||ge)return``;ge=!0;var n=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var r={DetermineComponentFrameRoot:function(){try{if(t){var n=function(){throw Error()};if(Object.defineProperty(n.prototype,"props",{set:function(){throw Error()}}),typeof Reflect==`object`&&Reflect.construct){try{Reflect.construct(n,[])}catch(e){var r=e}Reflect.construct(e,[],n)}else{try{n.call()}catch(e){r=e}e.call(n.prototype)}}else{try{throw Error()}catch(e){r=e}(n=e())&&typeof n.catch==`function`&&n.catch(function(){})}}catch(e){if(e&&r&&typeof e.stack==`string`)return[e.stack,r.stack]}return[null,null]}};r.DetermineComponentFrameRoot.displayName=`DetermineComponentFrameRoot`;var i=Object.getOwnPropertyDescriptor(r.DetermineComponentFrameRoot,`name`);i&&i.configurable&&Object.defineProperty(r.DetermineComponentFrameRoot,"name",{value:`DetermineComponentFrameRoot`});var a=r.DetermineComponentFrameRoot(),o=a[0],s=a[1];if(o&&s){var c=o.split(`
`),l=s.split(`
`);for(i=r=0;r<c.length&&!c[r].includes(`DetermineComponentFrameRoot`);)r++;for(;i<l.length&&!l[i].includes(`DetermineComponentFrameRoot`);)i++;if(r===c.length||i===l.length)for(r=c.length-1,i=l.length-1;1<=r&&0<=i&&c[r]!==l[i];)i--;for(;1<=r&&0<=i;r--,i--)if(c[r]!==l[i]){if(r!==1||i!==1)do if(r--,i--,0>i||c[r]!==l[i]){var u=`
`+c[r].replace(` at new `,` at `);return e.displayName&&u.includes(`<anonymous>`)&&(u=u.replace(`<anonymous>`,e.displayName)),u}while(1<=r&&0<=i);break}}}finally{ge=!1,Error.prepareStackTrace=n}return(n=e?e.displayName||e.name:``)?he(n):``}function ve(e,t){switch(e.tag){case 26:case 27:case 5:return he(e.type);case 16:return he(`Lazy`);case 13:return e.child!==t&&t!==null?he(`Suspense Fallback`):he(`Suspense`);case 19:return he(`SuspenseList`);case 0:case 15:return _e(e.type,!1);case 11:return _e(e.type.render,!1);case 1:return _e(e.type,!0);case 31:return he(`Activity`);default:return``}}function ye(e){try{var t=``,n=null;do t+=ve(e,n),n=e,e=e.return;while(e);return t}catch(e){return`
Error generating stack: `+e.message+`
`+e.stack}}var be=Object.prototype.hasOwnProperty,xe=t.unstable_scheduleCallback,Se=t.unstable_cancelCallback,Ce=t.unstable_shouldYield,we=t.unstable_requestPaint,Te=t.unstable_now,Ee=t.unstable_getCurrentPriorityLevel,De=t.unstable_ImmediatePriority,Oe=t.unstable_UserBlockingPriority,ke=t.unstable_NormalPriority,Ae=t.unstable_LowPriority,je=t.unstable_IdlePriority,Me=t.log,Ne=t.unstable_setDisableYieldValue,Pe=null,Fe=null;function Ie(e){if(typeof Me==`function`&&Ne(e),Fe&&typeof Fe.setStrictMode==`function`)try{Fe.setStrictMode(Pe,e)}catch{}}var Le=Math.clz32?Math.clz32:Be,Re=Math.log,ze=Math.LN2;function Be(e){return e>>>=0,e===0?32:31-(Re(e)/ze|0)|0}var Ve=256,He=262144,Ue=4194304;function We(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function Ge(e,t,n){var r=e.pendingLanes;if(r===0)return 0;var i=0,a=e.suspendedLanes,o=e.pingedLanes;e=e.warmLanes;var s=r&134217727;return s===0?(s=r&~a,s===0?o===0?n||(n=r&~e,n!==0&&(i=We(n))):i=We(o):i=We(s)):(r=s&~a,r===0?(o&=s,o===0?n||(n=s&~e,n!==0&&(i=We(n))):i=We(o)):i=We(r)),i===0?0:t!==0&&t!==i&&(t&a)===0&&(a=i&-i,n=t&-t,a>=n||a===32&&n&4194048)?t:i}function Ke(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function qe(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function Je(){var e=Ue;return Ue<<=1,!(Ue&62914560)&&(Ue=4194304),e}function Ye(e){for(var t=[],n=0;31>n;n++)t.push(e);return t}function Xe(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function Ze(e,t,n,r,i,a){var o=e.pendingLanes;e.pendingLanes=n,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=n,e.entangledLanes&=n,e.errorRecoveryDisabledLanes&=n,e.shellSuspendCounter=0;var s=e.entanglements,c=e.expirationTimes,l=e.hiddenUpdates;for(n=o&~n;0<n;){var u=31-Le(n),d=1<<u;s[u]=0,c[u]=-1;var f=l[u];if(f!==null)for(l[u]=null,u=0;u<f.length;u++){var p=f[u];p!==null&&(p.lane&=-536870913)}n&=~d}r!==0&&Qe(e,r,0),a!==0&&i===0&&e.tag!==0&&(e.suspendedLanes|=a&~(o&~t))}function Qe(e,t,n){e.pendingLanes|=t,e.suspendedLanes&=~t;var r=31-Le(t);e.entangledLanes|=t,e.entanglements[r]=e.entanglements[r]|1073741824|n&261930}function $e(e,t){var n=e.entangledLanes|=t;for(e=e.entanglements;n;){var r=31-Le(n),i=1<<r;i&t|e[r]&t&&(e[r]|=t),n&=~i}}function et(e,t){var n=t&-t;return n=n&42?1:tt(n),(n&(e.suspendedLanes|t))===0?n:0}function tt(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function nt(e){return e&=-e,2<e?8<e?e&134217727?32:268435456:8:2}function rt(){var e=P.p;return e===0?(e=window.event,e===void 0?32:mp(e.type)):e}function it(e,t){var n=P.p;try{return P.p=e,t()}finally{P.p=n}}var at=Math.random().toString(36).slice(2),ot=`__reactFiber$`+at,st=`__reactProps$`+at,ct=`__reactContainer$`+at,lt=`__reactEvents$`+at,ut=`__reactListeners$`+at,dt=`__reactHandles$`+at,ft=`__reactResources$`+at,pt=`__reactMarker$`+at;function mt(e){delete e[ot],delete e[st],delete e[lt],delete e[ut],delete e[dt]}function ht(e){var t=e[ot];if(t)return t;for(var n=e.parentNode;n;){if(t=n[ct]||n[ot]){if(n=t.alternate,t.child!==null||n!==null&&n.child!==null)for(e=ff(e);e!==null;){if(n=e[ot])return n;e=ff(e)}return t}e=n,n=e.parentNode}return null}function gt(e){if(e=e[ot]||e[ct]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function _t(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(a(33))}function vt(e){var t=e[ft];return t||=e[ft]={hoistableStyles:new Map,hoistableScripts:new Map},t}function yt(e){e[pt]=!0}var bt=new Set,xt={};function St(e,t){Ct(e,t),Ct(e+`Capture`,t)}function Ct(e,t){for(xt[e]=t,e=0;e<t.length;e++)bt.add(t[e])}var wt=RegExp(`^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`),Tt={},Et={};function Dt(e){return be.call(Et,e)?!0:be.call(Tt,e)?!1:wt.test(e)?Et[e]=!0:(Tt[e]=!0,!1)}function Ot(e,t,n){if(Dt(t))if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:e.removeAttribute(t);return;case`boolean`:var r=t.toLowerCase().slice(0,5);if(r!==`data-`&&r!==`aria-`){e.removeAttribute(t);return}}e.setAttribute(t,``+n)}}function kt(e,t,n){if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(t);return}e.setAttribute(t,``+n)}}function At(e,t,n,r){if(r===null)e.removeAttribute(n);else{switch(typeof r){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(n);return}e.setAttributeNS(t,n,``+r)}}function jt(e){switch(typeof e){case`bigint`:case`boolean`:case`number`:case`string`:case`undefined`:return e;case`object`:return e;default:return``}}function Mt(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()===`input`&&(t===`checkbox`||t===`radio`)}function Nt(e,t,n){var r=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&r!==void 0&&typeof r.get==`function`&&typeof r.set==`function`){var i=r.get,a=r.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return i.call(this)},set:function(e){n=``+e,a.call(this,e)}}),Object.defineProperty(e,t,{enumerable:r.enumerable}),{getValue:function(){return n},setValue:function(e){n=``+e},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function Pt(e){if(!e._valueTracker){var t=Mt(e)?`checked`:`value`;e._valueTracker=Nt(e,t,``+e[t])}}function Ft(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var n=t.getValue(),r=``;return e&&(r=Mt(e)?e.checked?`true`:`false`:e.value),e=r,e===n?!1:(t.setValue(e),!0)}function It(e){if(e||=typeof document<`u`?document:void 0,e===void 0)return null;try{return e.activeElement||e.body}catch{return e.body}}var Lt=/[\n"\\]/g;function Rt(e){return e.replace(Lt,function(e){return`\\`+e.charCodeAt(0).toString(16)+` `})}function zt(e,t,n,r,i,a,o,s){e.name=``,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`?e.type=o:e.removeAttribute(`type`),t==null?o!==`submit`&&o!==`reset`||e.removeAttribute(`value`):o===`number`?(t===0&&e.value===``||e.value!=t)&&(e.value=``+jt(t)):e.value!==``+jt(t)&&(e.value=``+jt(t)),t==null?n==null?r!=null&&e.removeAttribute(`value`):Vt(e,o,jt(n)):Vt(e,o,jt(t)),i==null&&a!=null&&(e.defaultChecked=!!a),i!=null&&(e.checked=i&&typeof i!=`function`&&typeof i!=`symbol`),s!=null&&typeof s!=`function`&&typeof s!=`symbol`&&typeof s!=`boolean`?e.name=``+jt(s):e.removeAttribute(`name`)}function Bt(e,t,n,r,i,a,o,s){if(a!=null&&typeof a!=`function`&&typeof a!=`symbol`&&typeof a!=`boolean`&&(e.type=a),t!=null||n!=null){if(!(a!==`submit`&&a!==`reset`||t!=null)){Pt(e);return}n=n==null?``:``+jt(n),t=t==null?n:``+jt(t),s||t===e.value||(e.value=t),e.defaultValue=t}r??=i,r=typeof r!=`function`&&typeof r!=`symbol`&&!!r,e.checked=s?e.checked:!!r,e.defaultChecked=!!r,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`&&(e.name=o),Pt(e)}function Vt(e,t,n){t===`number`&&It(e.ownerDocument)===e||e.defaultValue===``+n||(e.defaultValue=``+n)}function Ht(e,t,n,r){if(e=e.options,t){t={};for(var i=0;i<n.length;i++)t[`$`+n[i]]=!0;for(n=0;n<e.length;n++)i=t.hasOwnProperty(`$`+e[n].value),e[n].selected!==i&&(e[n].selected=i),i&&r&&(e[n].defaultSelected=!0)}else{for(n=``+jt(n),t=null,i=0;i<e.length;i++){if(e[i].value===n){e[i].selected=!0,r&&(e[i].defaultSelected=!0);return}t!==null||e[i].disabled||(t=e[i])}t!==null&&(t.selected=!0)}}function Ut(e,t,n){if(t!=null&&(t=``+jt(t),t!==e.value&&(e.value=t),n==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=n==null?``:``+jt(n)}function Wt(e,t,n,r){if(t==null){if(r!=null){if(n!=null)throw Error(a(92));if(ne(r)){if(1<r.length)throw Error(a(93));r=r[0]}n=r}n??=``,t=n}n=jt(t),e.defaultValue=n,r=e.textContent,r===n&&r!==``&&r!==null&&(e.value=r),Pt(e)}function Gt(e,t){if(t){var n=e.firstChild;if(n&&n===e.lastChild&&n.nodeType===3){n.nodeValue=t;return}}e.textContent=t}var Kt=new Set(`animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(` `));function qt(e,t,n){var r=t.indexOf(`--`)===0;n==null||typeof n==`boolean`||n===``?r?e.setProperty(t,``):t===`float`?e.cssFloat=``:e[t]=``:r?e.setProperty(t,n):typeof n!=`number`||n===0||Kt.has(t)?t===`float`?e.cssFloat=n:e[t]=(``+n).trim():e[t]=n+`px`}function Jt(e,t,n){if(t!=null&&typeof t!=`object`)throw Error(a(62));if(e=e.style,n!=null){for(var r in n)!n.hasOwnProperty(r)||t!=null&&t.hasOwnProperty(r)||(r.indexOf(`--`)===0?e.setProperty(r,``):r===`float`?e.cssFloat=``:e[r]=``);for(var i in t)r=t[i],t.hasOwnProperty(i)&&n[i]!==r&&qt(e,i,r)}else for(var o in t)t.hasOwnProperty(o)&&qt(e,o,t[o])}function Yt(e){if(e.indexOf(`-`)===-1)return!1;switch(e){case`annotation-xml`:case`color-profile`:case`font-face`:case`font-face-src`:case`font-face-uri`:case`font-face-format`:case`font-face-name`:case`missing-glyph`:return!1;default:return!0}}var Xt=new Map([[`acceptCharset`,`accept-charset`],[`htmlFor`,`for`],[`httpEquiv`,`http-equiv`],[`crossOrigin`,`crossorigin`],[`accentHeight`,`accent-height`],[`alignmentBaseline`,`alignment-baseline`],[`arabicForm`,`arabic-form`],[`baselineShift`,`baseline-shift`],[`capHeight`,`cap-height`],[`clipPath`,`clip-path`],[`clipRule`,`clip-rule`],[`colorInterpolation`,`color-interpolation`],[`colorInterpolationFilters`,`color-interpolation-filters`],[`colorProfile`,`color-profile`],[`colorRendering`,`color-rendering`],[`dominantBaseline`,`dominant-baseline`],[`enableBackground`,`enable-background`],[`fillOpacity`,`fill-opacity`],[`fillRule`,`fill-rule`],[`floodColor`,`flood-color`],[`floodOpacity`,`flood-opacity`],[`fontFamily`,`font-family`],[`fontSize`,`font-size`],[`fontSizeAdjust`,`font-size-adjust`],[`fontStretch`,`font-stretch`],[`fontStyle`,`font-style`],[`fontVariant`,`font-variant`],[`fontWeight`,`font-weight`],[`glyphName`,`glyph-name`],[`glyphOrientationHorizontal`,`glyph-orientation-horizontal`],[`glyphOrientationVertical`,`glyph-orientation-vertical`],[`horizAdvX`,`horiz-adv-x`],[`horizOriginX`,`horiz-origin-x`],[`imageRendering`,`image-rendering`],[`letterSpacing`,`letter-spacing`],[`lightingColor`,`lighting-color`],[`markerEnd`,`marker-end`],[`markerMid`,`marker-mid`],[`markerStart`,`marker-start`],[`overlinePosition`,`overline-position`],[`overlineThickness`,`overline-thickness`],[`paintOrder`,`paint-order`],[`panose-1`,`panose-1`],[`pointerEvents`,`pointer-events`],[`renderingIntent`,`rendering-intent`],[`shapeRendering`,`shape-rendering`],[`stopColor`,`stop-color`],[`stopOpacity`,`stop-opacity`],[`strikethroughPosition`,`strikethrough-position`],[`strikethroughThickness`,`strikethrough-thickness`],[`strokeDasharray`,`stroke-dasharray`],[`strokeDashoffset`,`stroke-dashoffset`],[`strokeLinecap`,`stroke-linecap`],[`strokeLinejoin`,`stroke-linejoin`],[`strokeMiterlimit`,`stroke-miterlimit`],[`strokeOpacity`,`stroke-opacity`],[`strokeWidth`,`stroke-width`],[`textAnchor`,`text-anchor`],[`textDecoration`,`text-decoration`],[`textRendering`,`text-rendering`],[`transformOrigin`,`transform-origin`],[`underlinePosition`,`underline-position`],[`underlineThickness`,`underline-thickness`],[`unicodeBidi`,`unicode-bidi`],[`unicodeRange`,`unicode-range`],[`unitsPerEm`,`units-per-em`],[`vAlphabetic`,`v-alphabetic`],[`vHanging`,`v-hanging`],[`vIdeographic`,`v-ideographic`],[`vMathematical`,`v-mathematical`],[`vectorEffect`,`vector-effect`],[`vertAdvY`,`vert-adv-y`],[`vertOriginX`,`vert-origin-x`],[`vertOriginY`,`vert-origin-y`],[`wordSpacing`,`word-spacing`],[`writingMode`,`writing-mode`],[`xmlnsXlink`,`xmlns:xlink`],[`xHeight`,`x-height`]]),Zt=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function Qt(e){return Zt.test(``+e)?`javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`:e}function $t(){}var en=null;function tn(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var nn=null,rn=null;function an(e){var t=gt(e);if(t&&(e=t.stateNode)){var n=e[st]||null;a:switch(e=t.stateNode,t.type){case`input`:if(zt(e,n.value,n.defaultValue,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name),t=n.name,n.type===`radio`&&t!=null){for(n=e;n.parentNode;)n=n.parentNode;for(n=n.querySelectorAll(`input[name="`+Rt(``+t)+`"][type="radio"]`),t=0;t<n.length;t++){var r=n[t];if(r!==e&&r.form===e.form){var i=r[st]||null;if(!i)throw Error(a(90));zt(r,i.value,i.defaultValue,i.defaultValue,i.checked,i.defaultChecked,i.type,i.name)}}for(t=0;t<n.length;t++)r=n[t],r.form===e.form&&Ft(r)}break a;case`textarea`:Ut(e,n.value,n.defaultValue);break a;case`select`:t=n.value,t!=null&&Ht(e,!!n.multiple,t,!1)}}}var on=!1;function sn(e,t,n){if(on)return e(t,n);on=!0;try{return e(t)}finally{if(on=!1,(nn!==null||rn!==null)&&(yu(),nn&&(t=nn,e=rn,rn=nn=null,an(t),e)))for(t=0;t<e.length;t++)an(e[t])}}function cn(e,t){var n=e.stateNode;if(n===null)return null;var r=n[st]||null;if(r===null)return null;n=r[t];a:switch(t){case`onClick`:case`onClickCapture`:case`onDoubleClick`:case`onDoubleClickCapture`:case`onMouseDown`:case`onMouseDownCapture`:case`onMouseMove`:case`onMouseMoveCapture`:case`onMouseUp`:case`onMouseUpCapture`:case`onMouseEnter`:(r=!r.disabled)||(e=e.type,r=!(e===`button`||e===`input`||e===`select`||e===`textarea`)),e=!r;break a;default:e=!1}if(e)return null;if(n&&typeof n!=`function`)throw Error(a(231,t,typeof n));return n}var ln=!(typeof window>`u`||window.document===void 0||window.document.createElement===void 0),un=!1;if(ln)try{var dn={};Object.defineProperty(dn,"passive",{get:function(){un=!0}}),window.addEventListener(`test`,dn,dn),window.removeEventListener(`test`,dn,dn)}catch{un=!1}var fn=null,pn=null,mn=null;function hn(){if(mn)return mn;var e,t=pn,n=t.length,r,i=`value`in fn?fn.value:fn.textContent,a=i.length;for(e=0;e<n&&t[e]===i[e];e++);var o=n-e;for(r=1;r<=o&&t[n-r]===i[a-r];r++);return mn=i.slice(e,1<r?1-r:void 0)}function gn(e){var t=e.keyCode;return`charCode`in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function _n(){return!0}function vn(){return!1}function yn(e){function t(t,n,r,i,a){for(var o in this._reactName=t,this._targetInst=r,this.type=n,this.nativeEvent=i,this.target=a,this.currentTarget=null,e)e.hasOwnProperty(o)&&(t=e[o],this[o]=t?t(i):i[o]);return this.isDefaultPrevented=(i.defaultPrevented==null?!1===i.returnValue:i.defaultPrevented)?_n:vn,this.isPropagationStopped=vn,this}return m(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var e=this.nativeEvent;e&&(e.preventDefault?e.preventDefault():typeof e.returnValue!=`unknown`&&(e.returnValue=!1),this.isDefaultPrevented=_n)},stopPropagation:function(){var e=this.nativeEvent;e&&(e.stopPropagation?e.stopPropagation():typeof e.cancelBubble!=`unknown`&&(e.cancelBubble=!0),this.isPropagationStopped=_n)},persist:function(){},isPersistent:_n}),t}var bn={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},xn=yn(bn),Sn=m({},bn,{view:0,detail:0}),Cn=yn(Sn),wn,Tn,En,Dn=m({},Sn,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Rn,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return`movementX`in e?e.movementX:(e!==En&&(En&&e.type===`mousemove`?(wn=e.screenX-En.screenX,Tn=e.screenY-En.screenY):Tn=wn=0,En=e),wn)},movementY:function(e){return`movementY`in e?e.movementY:Tn}}),On=yn(Dn),kn=yn(m({},Dn,{dataTransfer:0})),An=yn(m({},Sn,{relatedTarget:0})),jn=yn(m({},bn,{animationName:0,elapsedTime:0,pseudoElement:0})),Mn=yn(m({},bn,{clipboardData:function(e){return`clipboardData`in e?e.clipboardData:window.clipboardData}})),Nn=yn(m({},bn,{data:0})),Pn={Esc:`Escape`,Spacebar:` `,Left:`ArrowLeft`,Up:`ArrowUp`,Right:`ArrowRight`,Down:`ArrowDown`,Del:`Delete`,Win:`OS`,Menu:`ContextMenu`,Apps:`ContextMenu`,Scroll:`ScrollLock`,MozPrintableKey:`Unidentified`},Fn={8:`Backspace`,9:`Tab`,12:`Clear`,13:`Enter`,16:`Shift`,17:`Control`,18:`Alt`,19:`Pause`,20:`CapsLock`,27:`Escape`,32:` `,33:`PageUp`,34:`PageDown`,35:`End`,36:`Home`,37:`ArrowLeft`,38:`ArrowUp`,39:`ArrowRight`,40:`ArrowDown`,45:`Insert`,46:`Delete`,112:`F1`,113:`F2`,114:`F3`,115:`F4`,116:`F5`,117:`F6`,118:`F7`,119:`F8`,120:`F9`,121:`F10`,122:`F11`,123:`F12`,144:`NumLock`,145:`ScrollLock`,224:`Meta`},In={Alt:`altKey`,Control:`ctrlKey`,Meta:`metaKey`,Shift:`shiftKey`};function Ln(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=In[e])?!!t[e]:!1}function Rn(){return Ln}var zn=yn(m({},Sn,{key:function(e){if(e.key){var t=Pn[e.key]||e.key;if(t!==`Unidentified`)return t}return e.type===`keypress`?(e=gn(e),e===13?`Enter`:String.fromCharCode(e)):e.type===`keydown`||e.type===`keyup`?Fn[e.keyCode]||`Unidentified`:``},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Rn,charCode:function(e){return e.type===`keypress`?gn(e):0},keyCode:function(e){return e.type===`keydown`||e.type===`keyup`?e.keyCode:0},which:function(e){return e.type===`keypress`?gn(e):e.type===`keydown`||e.type===`keyup`?e.keyCode:0}})),Bn=yn(m({},Dn,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0})),Vn=yn(m({},Sn,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Rn})),Hn=yn(m({},bn,{propertyName:0,elapsedTime:0,pseudoElement:0})),Un=yn(m({},Dn,{deltaX:function(e){return`deltaX`in e?e.deltaX:`wheelDeltaX`in e?-e.wheelDeltaX:0},deltaY:function(e){return`deltaY`in e?e.deltaY:`wheelDeltaY`in e?-e.wheelDeltaY:`wheelDelta`in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0})),Wn=yn(m({},bn,{newState:0,oldState:0})),Gn=[9,13,27,32],Kn=ln&&`CompositionEvent`in window,qn=null;ln&&`documentMode`in document&&(qn=document.documentMode);var Jn=ln&&`TextEvent`in window&&!qn,Yn=ln&&(!Kn||qn&&8<qn&&11>=qn),Xn=` `,Zn=!1;function Qn(e,t){switch(e){case`keyup`:return Gn.indexOf(t.keyCode)!==-1;case`keydown`:return t.keyCode!==229;case`keypress`:case`mousedown`:case`focusout`:return!0;default:return!1}}function $n(e){return e=e.detail,typeof e==`object`&&`data`in e?e.data:null}var er=!1;function tr(e,t){switch(e){case`compositionend`:return $n(t);case`keypress`:return t.which===32?(Zn=!0,Xn):null;case`textInput`:return e=t.data,e===Xn&&Zn?null:e;default:return null}}function nr(e,t){if(er)return e===`compositionend`||!Kn&&Qn(e,t)?(e=hn(),mn=pn=fn=null,er=!1,e):null;switch(e){case`paste`:return null;case`keypress`:if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case`compositionend`:return Yn&&t.locale!==`ko`?null:t.data;default:return null}}var rr={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function ir(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t===`input`?!!rr[e.type]:t===`textarea`}function ar(e,t,n,r){nn?rn?rn.push(r):rn=[r]:nn=r,t=Ed(t,`onChange`),0<t.length&&(n=new xn(`onChange`,`change`,null,n,r),e.push({event:n,listeners:t}))}var or=null,sr=null;function cr(e){yd(e,0)}function lr(e){if(Ft(_t(e)))return e}function ur(e,t){if(e===`change`)return t}var dr=!1;if(ln){var fr;if(ln){var pr=`oninput`in document;if(!pr){var mr=document.createElement(`div`);mr.setAttribute(`oninput`,`return;`),pr=typeof mr.oninput==`function`}fr=pr}else fr=!1;dr=fr&&(!document.documentMode||9<document.documentMode)}function hr(){or&&(or.detachEvent(`onpropertychange`,gr),sr=or=null)}function gr(e){if(e.propertyName===`value`&&lr(sr)){var t=[];ar(t,sr,e,tn(e)),sn(cr,t)}}function _r(e,t,n){e===`focusin`?(hr(),or=t,sr=n,or.attachEvent(`onpropertychange`,gr)):e===`focusout`&&hr()}function vr(e){if(e===`selectionchange`||e===`keyup`||e===`keydown`)return lr(sr)}function yr(e,t){if(e===`click`)return lr(t)}function br(e,t){if(e===`input`||e===`change`)return lr(t)}function xr(e,t){return e===t&&(e!==0||1/e==1/t)||e!==e&&t!==t}var Sr=typeof Object.is==`function`?Object.is:xr;function Cr(e,t){if(Sr(e,t))return!0;if(typeof e!=`object`||!e||typeof t!=`object`||!t)return!1;var n=Object.keys(e),r=Object.keys(t);if(n.length!==r.length)return!1;for(r=0;r<n.length;r++){var i=n[r];if(!be.call(t,i)||!Sr(e[i],t[i]))return!1}return!0}function wr(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function Tr(e,t){var n=wr(e);e=0;for(var r;n;){if(n.nodeType===3){if(r=e+n.textContent.length,e<=t&&r>=t)return{node:n,offset:t-e};e=r}a:{for(;n;){if(n.nextSibling){n=n.nextSibling;break a}n=n.parentNode}n=void 0}n=wr(n)}}function Er(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Er(e,t.parentNode):`contains`in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function Dr(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=It(e.document);t instanceof e.HTMLIFrameElement;){try{var n=typeof t.contentWindow.location.href==`string`}catch{n=!1}if(n)e=t.contentWindow;else break;t=It(e.document)}return t}function Or(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t===`input`&&(e.type===`text`||e.type===`search`||e.type===`tel`||e.type===`url`||e.type===`password`)||t===`textarea`||e.contentEditable===`true`)}var kr=ln&&`documentMode`in document&&11>=document.documentMode,Ar=null,jr=null,Mr=null,Nr=!1;function Pr(e,t,n){var r=n.window===n?n.document:n.nodeType===9?n:n.ownerDocument;Nr||Ar==null||Ar!==It(r)||(r=Ar,`selectionStart`in r&&Or(r)?r={start:r.selectionStart,end:r.selectionEnd}:(r=(r.ownerDocument&&r.ownerDocument.defaultView||window).getSelection(),r={anchorNode:r.anchorNode,anchorOffset:r.anchorOffset,focusNode:r.focusNode,focusOffset:r.focusOffset}),Mr&&Cr(Mr,r)||(Mr=r,r=Ed(jr,`onSelect`),0<r.length&&(t=new xn(`onSelect`,`select`,null,t,n),e.push({event:t,listeners:r}),t.target=Ar)))}function Fr(e,t){var n={};return n[e.toLowerCase()]=t.toLowerCase(),n[`Webkit`+e]=`webkit`+t,n[`Moz`+e]=`moz`+t,n}var Ir={animationend:Fr(`Animation`,`AnimationEnd`),animationiteration:Fr(`Animation`,`AnimationIteration`),animationstart:Fr(`Animation`,`AnimationStart`),transitionrun:Fr(`Transition`,`TransitionRun`),transitionstart:Fr(`Transition`,`TransitionStart`),transitioncancel:Fr(`Transition`,`TransitionCancel`),transitionend:Fr(`Transition`,`TransitionEnd`)},Lr={},Rr={};ln&&(Rr=document.createElement(`div`).style,`AnimationEvent`in window||(delete Ir.animationend.animation,delete Ir.animationiteration.animation,delete Ir.animationstart.animation),`TransitionEvent`in window||delete Ir.transitionend.transition);function zr(e){if(Lr[e])return Lr[e];if(!Ir[e])return e;var t=Ir[e],n;for(n in t)if(t.hasOwnProperty(n)&&n in Rr)return Lr[e]=t[n];return e}var Br=zr(`animationend`),Vr=zr(`animationiteration`),Hr=zr(`animationstart`),Ur=zr(`transitionrun`),Wr=zr(`transitionstart`),Gr=zr(`transitioncancel`),Kr=zr(`transitionend`),qr=new Map,Jr=`abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(` `);Jr.push(`scrollEnd`);function Yr(e,t){qr.set(e,t),St(t,[e])}var Xr=typeof reportError==`function`?reportError:function(e){if(typeof window==`object`&&typeof window.ErrorEvent==`function`){var t=new window.ErrorEvent(`error`,{bubbles:!0,cancelable:!0,message:typeof e==`object`&&e&&typeof e.message==`string`?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process==`object`&&typeof process.emit==`function`){process.emit(`uncaughtException`,e);return}console.error(e)},Zr=[],Qr=0,$r=0;function ei(){for(var e=Qr,t=$r=Qr=0;t<e;){var n=Zr[t];Zr[t++]=null;var r=Zr[t];Zr[t++]=null;var i=Zr[t];Zr[t++]=null;var a=Zr[t];if(Zr[t++]=null,r!==null&&i!==null){var o=r.pending;o===null?i.next=i:(i.next=o.next,o.next=i),r.pending=i}a!==0&&ii(n,i,a)}}function ti(e,t,n,r){Zr[Qr++]=e,Zr[Qr++]=t,Zr[Qr++]=n,Zr[Qr++]=r,$r|=r,e.lanes|=r,e=e.alternate,e!==null&&(e.lanes|=r)}function ni(e,t,n,r){return ti(e,t,n,r),ai(e)}function ri(e,t){return ti(e,null,null,t),ai(e)}function ii(e,t,n){e.lanes|=n;var r=e.alternate;r!==null&&(r.lanes|=n);for(var i=!1,a=e.return;a!==null;)a.childLanes|=n,r=a.alternate,r!==null&&(r.childLanes|=n),a.tag===22&&(e=a.stateNode,e===null||e._visibility&1||(i=!0)),e=a,a=a.return;return e.tag===3?(a=e.stateNode,i&&t!==null&&(i=31-Le(n),e=a.hiddenUpdates,r=e[i],r===null?e[i]=[t]:r.push(t),t.lane=n|536870912),a):null}function ai(e){if(50<uu)throw uu=0,du=null,Error(a(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var oi={};function si(e,t,n,r){this.tag=e,this.key=n,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=r,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function ci(e,t,n,r){return new si(e,t,n,r)}function li(e){return e=e.prototype,!(!e||!e.isReactComponent)}function ui(e,t){var n=e.alternate;return n===null?(n=ci(e.tag,t,e.key,e.mode),n.elementType=e.elementType,n.type=e.type,n.stateNode=e.stateNode,n.alternate=e,e.alternate=n):(n.pendingProps=t,n.type=e.type,n.flags=0,n.subtreeFlags=0,n.deletions=null),n.flags=e.flags&65011712,n.childLanes=e.childLanes,n.lanes=e.lanes,n.child=e.child,n.memoizedProps=e.memoizedProps,n.memoizedState=e.memoizedState,n.updateQueue=e.updateQueue,t=e.dependencies,n.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},n.sibling=e.sibling,n.index=e.index,n.ref=e.ref,n.refCleanup=e.refCleanup,n}function di(e,t){e.flags&=65011714;var n=e.alternate;return n===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=n.childLanes,e.lanes=n.lanes,e.child=n.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=n.memoizedProps,e.memoizedState=n.memoizedState,e.updateQueue=n.updateQueue,e.type=n.type,t=n.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function fi(e,t,n,r,i,o){var s=0;if(r=e,typeof e==`function`)li(e)&&(s=1);else if(typeof e==`string`)s=$(e,n,oe.current)?26:e===`html`||e===`head`||e===`body`?27:5;else a:switch(e){case O:return e=ci(31,n,t,i),e.elementType=O,e.lanes=o,e;case v:return pi(n.children,i,o,t);case y:s=8,i|=24;break;case b:return e=ci(12,n,t,i|2),e.elementType=b,e.lanes=o,e;case w:return e=ci(13,n,t,i),e.elementType=w,e.lanes=o,e;case T:return e=ci(19,n,t,i),e.elementType=T,e.lanes=o,e;default:if(typeof e==`object`&&e)switch(e.$$typeof){case S:s=10;break a;case x:s=9;break a;case C:s=11;break a;case E:s=14;break a;case D:s=16,r=null;break a}s=29,n=Error(a(130,e===null?`null`:typeof e,``)),r=null}return t=ci(s,n,t,i),t.elementType=e,t.type=r,t.lanes=o,t}function pi(e,t,n,r){return e=ci(7,e,r,t),e.lanes=n,e}function mi(e,t,n){return e=ci(6,e,null,t),e.lanes=n,e}function hi(e){var t=ci(18,null,null,0);return t.stateNode=e,t}function gi(e,t,n){return t=ci(4,e.children===null?[]:e.children,e.key,t),t.lanes=n,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var _i=new WeakMap;function vi(e,t){if(typeof e==`object`&&e){var n=_i.get(e);return n===void 0?(t={value:e,source:t,stack:ye(t)},_i.set(e,t),t):n}return{value:e,source:t,stack:ye(t)}}var yi=[],bi=0,xi=null,Si=0,z=[],Ci=0,wi=null,Ti=1,Ei=``;function Di(e,t){yi[bi++]=Si,yi[bi++]=xi,xi=e,Si=t}function Oi(e,t,n){z[Ci++]=Ti,z[Ci++]=Ei,z[Ci++]=wi,wi=e;var r=Ti;e=Ei;var i=32-Le(r)-1;r&=~(1<<i),n+=1;var a=32-Le(t)+i;if(30<a){var o=i-i%5;a=(r&(1<<o)-1).toString(32),r>>=o,i-=o,Ti=1<<32-Le(t)+i|n<<i|r,Ei=a+e}else Ti=1<<a|n<<i|r,Ei=e}function ki(e){e.return!==null&&(Di(e,1),Oi(e,1,0))}function Ai(e){for(;e===xi;)xi=yi[--bi],yi[bi]=null,Si=yi[--bi],yi[bi]=null;for(;e===wi;)wi=z[--Ci],z[Ci]=null,Ei=z[--Ci],z[Ci]=null,Ti=z[--Ci],z[Ci]=null}function ji(e,t){z[Ci++]=Ti,z[Ci++]=Ei,z[Ci++]=wi,Ti=t.id,Ei=t.overflow,wi=e}var Mi=null,B=null,V=!1,Ni=null,Pi=!1,Fi=Error(a(519));function Ii(e){throw Hi(vi(Error(a(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?`text`:`HTML`,``)),e)),Fi}function Li(e){var t=e.stateNode,n=e.type,r=e.memoizedProps;switch(t[ot]=e,t[st]=r,n){case`dialog`:Q(`cancel`,t),Q(`close`,t);break;case`iframe`:case`object`:case`embed`:Q(`load`,t);break;case`video`:case`audio`:for(n=0;n<_d.length;n++)Q(_d[n],t);break;case`source`:Q(`error`,t);break;case`img`:case`image`:case`link`:Q(`error`,t),Q(`load`,t);break;case`details`:Q(`toggle`,t);break;case`input`:Q(`invalid`,t),Bt(t,r.value,r.defaultValue,r.checked,r.defaultChecked,r.type,r.name,!0);break;case`select`:Q(`invalid`,t);break;case`textarea`:Q(`invalid`,t),Wt(t,r.value,r.defaultValue,r.children)}n=r.children,typeof n!=`string`&&typeof n!=`number`&&typeof n!=`bigint`||t.textContent===``+n||!0===r.suppressHydrationWarning||Md(t.textContent,n)?(r.popover!=null&&(Q(`beforetoggle`,t),Q(`toggle`,t)),r.onScroll!=null&&Q(`scroll`,t),r.onScrollEnd!=null&&Q(`scrollend`,t),r.onClick!=null&&(t.onclick=$t),t=!0):t=!1,t||Ii(e,!0)}function Ri(e){for(Mi=e.return;Mi;)switch(Mi.tag){case 5:case 31:case 13:Pi=!1;return;case 27:case 3:Pi=!0;return;default:Mi=Mi.return}}function zi(e){if(e!==Mi)return!1;if(!V)return Ri(e),V=!0,!1;var t=e.tag,n;if((n=t!==3&&t!==27)&&((n=t===5)&&(n=e.type,n=!(n!==`form`&&n!==`button`)||Wd(e.type,e.memoizedProps)),n=!n),n&&B&&Ii(e),Ri(e),t===13){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));B=df(e)}else if(t===31){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));B=df(e)}else t===27?(t=B,Qd(e.type)?(e=uf,uf=null,B=e):B=t):B=Mi?lf(e.stateNode.nextSibling):null;return!0}function Bi(){B=Mi=null,V=!1}function Vi(){var e=Ni;return e!==null&&(Xl===null?Xl=e:Xl.push.apply(Xl,e),Ni=null),e}function Hi(e){Ni===null?Ni=[e]:Ni.push(e)}var Ui=ae(null),Wi=null,Gi=null;function Ki(e,t,n){L(Ui,t._currentValue),t._currentValue=n}function qi(e){e._currentValue=Ui.current,I(Ui)}function Ji(e,t,n){for(;e!==null;){var r=e.alternate;if((e.childLanes&t)===t?r!==null&&(r.childLanes&t)!==t&&(r.childLanes|=t):(e.childLanes|=t,r!==null&&(r.childLanes|=t)),e===n)break;e=e.return}}function Yi(e,t,n,r){var i=e.child;for(i!==null&&(i.return=e);i!==null;){var o=i.dependencies;if(o!==null){var s=i.child;o=o.firstContext;a:for(;o!==null;){var c=o;o=i;for(var l=0;l<t.length;l++)if(c.context===t[l]){o.lanes|=n,c=o.alternate,c!==null&&(c.lanes|=n),Ji(o.return,n,e),r||(s=null);break a}o=c.next}}else if(i.tag===18){if(s=i.return,s===null)throw Error(a(341));s.lanes|=n,o=s.alternate,o!==null&&(o.lanes|=n),Ji(s,n,e),s=null}else s=i.child;if(s!==null)s.return=i;else for(s=i;s!==null;){if(s===e){s=null;break}if(i=s.sibling,i!==null){i.return=s.return,s=i;break}s=s.return}i=s}}function Xi(e,t,n,r){e=null;for(var i=t,o=!1;i!==null;){if(!o){if(i.flags&524288)o=!0;else if(i.flags&262144)break}if(i.tag===10){var s=i.alternate;if(s===null)throw Error(a(387));if(s=s.memoizedProps,s!==null){var c=i.type;Sr(i.pendingProps.value,s.value)||(e===null?e=[c]:e.push(c))}}else if(i===le.current){if(s=i.alternate,s===null)throw Error(a(387));s.memoizedState.memoizedState!==i.memoizedState.memoizedState&&(e===null?e=[Qf]:e.push(Qf))}i=i.return}e!==null&&Yi(t,e,n,r),t.flags|=262144}function Zi(e){for(e=e.firstContext;e!==null;){if(!Sr(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function Qi(e){Wi=e,Gi=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function $i(e){return ta(Wi,e)}function ea(e,t){return Wi===null&&Qi(e),ta(e,t)}function ta(e,t){var n=t._currentValue;if(t={context:t,memoizedValue:n,next:null},Gi===null){if(e===null)throw Error(a(308));Gi=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else Gi=Gi.next=t;return n}var na=typeof AbortController<`u`?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(t,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(e){return e()})}},ra=t.unstable_scheduleCallback,ia=t.unstable_NormalPriority,aa={$$typeof:S,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function oa(){return{controller:new na,data:new Map,refCount:0}}function sa(e){e.refCount--,e.refCount===0&&ra(ia,function(){e.controller.abort()})}var ca=null,la=0,ua=0,da=null;function fa(e,t){if(ca===null){var n=ca=[];la=0,ua=dd(),da={status:`pending`,value:void 0,then:function(e){n.push(e)}}}return la++,t.then(pa,pa),t}function pa(){if(--la===0&&ca!==null){da!==null&&(da.status=`fulfilled`);var e=ca;ca=null,ua=0,da=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function ma(e,t){var n=[],r={status:`pending`,value:null,reason:null,then:function(e){n.push(e)}};return e.then(function(){r.status=`fulfilled`,r.value=t;for(var e=0;e<n.length;e++)(0,n[e])(t)},function(e){for(r.status=`rejected`,r.reason=e,e=0;e<n.length;e++)(0,n[e])(void 0)}),r}var ha=N.S;N.S=function(e,t){$l=Te(),typeof t==`object`&&t&&typeof t.then==`function`&&fa(e,t),ha!==null&&ha(e,t)};var ga=ae(null);function _a(){var e=ga.current;return e===null?Ll.pooledCache:e}function va(e,t){t===null?L(ga,ga.current):L(ga,t.pool)}function ya(){var e=_a();return e===null?null:{parent:aa._currentValue,pool:e}}var ba=Error(a(460)),xa=Error(a(474)),Sa=Error(a(542)),Ca={then:function(){}};function wa(e){return e=e.status,e===`fulfilled`||e===`rejected`}function Ta(e,t,n){switch(n=e[n],n===void 0?e.push(t):n!==t&&(t.then($t,$t),t=n),t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,ka(e),e;default:if(typeof t.status==`string`)t.then($t,$t);else{if(e=Ll,e!==null&&100<e.shellSuspendCounter)throw Error(a(482));e=t,e.status=`pending`,e.then(function(e){if(t.status===`pending`){var n=t;n.status=`fulfilled`,n.value=e}},function(e){if(t.status===`pending`){var n=t;n.status=`rejected`,n.reason=e}})}switch(t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,ka(e),e}throw Da=t,ba}}function Ea(e){try{var t=e._init;return t(e._payload)}catch(e){throw typeof e==`object`&&e&&typeof e.then==`function`?(Da=e,ba):e}}var Da=null;function Oa(){if(Da===null)throw Error(a(459));var e=Da;return Da=null,e}function ka(e){if(e===ba||e===Sa)throw Error(a(483))}var Aa=null,ja=0;function Ma(e){var t=ja;return ja+=1,Aa===null&&(Aa=[]),Ta(Aa,e,t)}function Na(e,t){t=t.props.ref,e.ref=t===void 0?null:t}function Pa(e,t){throw t.$$typeof===h?Error(a(525)):(e=Object.prototype.toString.call(t),Error(a(31,e===`[object Object]`?`object with keys {`+Object.keys(t).join(`, `)+`}`:e)))}function Fa(e){function t(t,n){if(e){var r=t.deletions;r===null?(t.deletions=[n],t.flags|=16):r.push(n)}}function n(n,r){if(!e)return null;for(;r!==null;)t(n,r),r=r.sibling;return null}function r(e){for(var t=new Map;e!==null;)e.key===null?t.set(e.index,e):t.set(e.key,e),e=e.sibling;return t}function i(e,t){return e=ui(e,t),e.index=0,e.sibling=null,e}function o(t,n,r){return t.index=r,e?(r=t.alternate,r===null?(t.flags|=67108866,n):(r=r.index,r<n?(t.flags|=67108866,n):r)):(t.flags|=1048576,n)}function s(t){return e&&t.alternate===null&&(t.flags|=67108866),t}function c(e,t,n,r){return t===null||t.tag!==6?(t=mi(n,e.mode,r),t.return=e,t):(t=i(t,n),t.return=e,t)}function l(e,t,n,r){var a=n.type;return a===v?d(e,t,n.props.children,r,n.key):t!==null&&(t.elementType===a||typeof a==`object`&&a&&a.$$typeof===D&&Ea(a)===t.type)?(t=i(t,n.props),Na(t,n),t.return=e,t):(t=fi(n.type,n.key,n.props,null,e.mode,r),Na(t,n),t.return=e,t)}function u(e,t,n,r){return t===null||t.tag!==4||t.stateNode.containerInfo!==n.containerInfo||t.stateNode.implementation!==n.implementation?(t=gi(n,e.mode,r),t.return=e,t):(t=i(t,n.children||[]),t.return=e,t)}function d(e,t,n,r,a){return t===null||t.tag!==7?(t=pi(n,e.mode,r,a),t.return=e,t):(t=i(t,n),t.return=e,t)}function f(e,t,n){if(typeof t==`string`&&t!==``||typeof t==`number`||typeof t==`bigint`)return t=mi(``+t,e.mode,n),t.return=e,t;if(typeof t==`object`&&t){switch(t.$$typeof){case g:return n=fi(t.type,t.key,t.props,null,e.mode,n),Na(n,t),n.return=e,n;case _:return t=gi(t,e.mode,n),t.return=e,t;case D:return t=Ea(t),f(e,t,n)}if(ne(t)||ee(t))return t=pi(t,e.mode,n,null),t.return=e,t;if(typeof t.then==`function`)return f(e,Ma(t),n);if(t.$$typeof===S)return f(e,ea(e,t),n);Pa(e,t)}return null}function p(e,t,n,r){var i=t===null?null:t.key;if(typeof n==`string`&&n!==``||typeof n==`number`||typeof n==`bigint`)return i===null?c(e,t,``+n,r):null;if(typeof n==`object`&&n){switch(n.$$typeof){case g:return n.key===i?l(e,t,n,r):null;case _:return n.key===i?u(e,t,n,r):null;case D:return n=Ea(n),p(e,t,n,r)}if(ne(n)||ee(n))return i===null?d(e,t,n,r,null):null;if(typeof n.then==`function`)return p(e,t,Ma(n),r);if(n.$$typeof===S)return p(e,t,ea(e,n),r);Pa(e,n)}return null}function m(e,t,n,r,i){if(typeof r==`string`&&r!==``||typeof r==`number`||typeof r==`bigint`)return e=e.get(n)||null,c(t,e,``+r,i);if(typeof r==`object`&&r){switch(r.$$typeof){case g:return e=e.get(r.key===null?n:r.key)||null,l(t,e,r,i);case _:return e=e.get(r.key===null?n:r.key)||null,u(t,e,r,i);case D:return r=Ea(r),m(e,t,n,r,i)}if(ne(r)||ee(r))return e=e.get(n)||null,d(t,e,r,i,null);if(typeof r.then==`function`)return m(e,t,n,Ma(r),i);if(r.$$typeof===S)return m(e,t,n,ea(t,r),i);Pa(t,r)}return null}function h(i,a,s,c){for(var l=null,u=null,d=a,h=a=0,g=null;d!==null&&h<s.length;h++){d.index>h?(g=d,d=null):g=d.sibling;var _=p(i,d,s[h],c);if(_===null){d===null&&(d=g);break}e&&d&&_.alternate===null&&t(i,d),a=o(_,a,h),u===null?l=_:u.sibling=_,u=_,d=g}if(h===s.length)return n(i,d),V&&Di(i,h),l;if(d===null){for(;h<s.length;h++)d=f(i,s[h],c),d!==null&&(a=o(d,a,h),u===null?l=d:u.sibling=d,u=d);return V&&Di(i,h),l}for(d=r(d);h<s.length;h++)g=m(d,i,h,s[h],c),g!==null&&(e&&g.alternate!==null&&d.delete(g.key===null?h:g.key),a=o(g,a,h),u===null?l=g:u.sibling=g,u=g);return e&&d.forEach(function(e){return t(i,e)}),V&&Di(i,h),l}function y(i,s,c,l){if(c==null)throw Error(a(151));for(var u=null,d=null,h=s,g=s=0,_=null,v=c.next();h!==null&&!v.done;g++,v=c.next()){h.index>g?(_=h,h=null):_=h.sibling;var y=p(i,h,v.value,l);if(y===null){h===null&&(h=_);break}e&&h&&y.alternate===null&&t(i,h),s=o(y,s,g),d===null?u=y:d.sibling=y,d=y,h=_}if(v.done)return n(i,h),V&&Di(i,g),u;if(h===null){for(;!v.done;g++,v=c.next())v=f(i,v.value,l),v!==null&&(s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return V&&Di(i,g),u}for(h=r(h);!v.done;g++,v=c.next())v=m(h,i,g,v.value,l),v!==null&&(e&&v.alternate!==null&&h.delete(v.key===null?g:v.key),s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return e&&h.forEach(function(e){return t(i,e)}),V&&Di(i,g),u}function b(e,r,o,c){if(typeof o==`object`&&o&&o.type===v&&o.key===null&&(o=o.props.children),typeof o==`object`&&o){switch(o.$$typeof){case g:a:{for(var l=o.key;r!==null;){if(r.key===l){if(l=o.type,l===v){if(r.tag===7){n(e,r.sibling),c=i(r,o.props.children),c.return=e,e=c;break a}}else if(r.elementType===l||typeof l==`object`&&l&&l.$$typeof===D&&Ea(l)===r.type){n(e,r.sibling),c=i(r,o.props),Na(c,o),c.return=e,e=c;break a}n(e,r);break}else t(e,r);r=r.sibling}o.type===v?(c=pi(o.props.children,e.mode,c,o.key),c.return=e,e=c):(c=fi(o.type,o.key,o.props,null,e.mode,c),Na(c,o),c.return=e,e=c)}return s(e);case _:a:{for(l=o.key;r!==null;){if(r.key===l)if(r.tag===4&&r.stateNode.containerInfo===o.containerInfo&&r.stateNode.implementation===o.implementation){n(e,r.sibling),c=i(r,o.children||[]),c.return=e,e=c;break a}else{n(e,r);break}else t(e,r);r=r.sibling}c=gi(o,e.mode,c),c.return=e,e=c}return s(e);case D:return o=Ea(o),b(e,r,o,c)}if(ne(o))return h(e,r,o,c);if(ee(o)){if(l=ee(o),typeof l!=`function`)throw Error(a(150));return o=l.call(o),y(e,r,o,c)}if(typeof o.then==`function`)return b(e,r,Ma(o),c);if(o.$$typeof===S)return b(e,r,ea(e,o),c);Pa(e,o)}return typeof o==`string`&&o!==``||typeof o==`number`||typeof o==`bigint`?(o=``+o,r!==null&&r.tag===6?(n(e,r.sibling),c=i(r,o),c.return=e,e=c):(n(e,r),c=mi(o,e.mode,c),c.return=e,e=c),s(e)):n(e,r)}return function(e,t,n,r){try{ja=0;var i=b(e,t,n,r);return Aa=null,i}catch(t){if(t===ba||t===Sa)throw t;var a=ci(29,t,null,e.mode);return a.lanes=r,a.return=e,a}}}var Ia=Fa(!0),La=Fa(!1),Ra=!1;function za(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function Ba(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Va(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function Ha(e,t,n){var r=e.updateQueue;if(r===null)return null;if(r=r.shared,Il&2){var i=r.pending;return i===null?t.next=t:(t.next=i.next,i.next=t),r.pending=t,t=ai(e),ii(e,null,n),t}return ti(e,r,t,n),ai(e)}function Ua(e,t,n){if(t=t.updateQueue,t!==null&&(t=t.shared,n&4194048)){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,$e(e,n)}}function Wa(e,t){var n=e.updateQueue,r=e.alternate;if(r!==null&&(r=r.updateQueue,n===r)){var i=null,a=null;if(n=n.firstBaseUpdate,n!==null){do{var o={lane:n.lane,tag:n.tag,payload:n.payload,callback:null,next:null};a===null?i=a=o:a=a.next=o,n=n.next}while(n!==null);a===null?i=a=t:a=a.next=t}else i=a=t;n={baseState:r.baseState,firstBaseUpdate:i,lastBaseUpdate:a,shared:r.shared,callbacks:r.callbacks},e.updateQueue=n;return}e=n.lastBaseUpdate,e===null?n.firstBaseUpdate=t:e.next=t,n.lastBaseUpdate=t}var Ga=!1;function Ka(){if(Ga){var e=da;if(e!==null)throw e}}function qa(e,t,n,r){Ga=!1;var i=e.updateQueue;Ra=!1;var a=i.firstBaseUpdate,o=i.lastBaseUpdate,s=i.shared.pending;if(s!==null){i.shared.pending=null;var c=s,l=c.next;c.next=null,o===null?a=l:o.next=l,o=c;var u=e.alternate;u!==null&&(u=u.updateQueue,s=u.lastBaseUpdate,s!==o&&(s===null?u.firstBaseUpdate=l:s.next=l,u.lastBaseUpdate=c))}if(a!==null){var d=i.baseState;o=0,u=l=c=null,s=a;do{var f=s.lane&-536870913,p=f!==s.lane;if(p?(X&f)===f:(r&f)===f){f!==0&&f===ua&&(Ga=!0),u!==null&&(u=u.next={lane:0,tag:s.tag,payload:s.payload,callback:null,next:null});a:{var h=e,g=s;f=t;var _=n;switch(g.tag){case 1:if(h=g.payload,typeof h==`function`){d=h.call(_,d,f);break a}d=h;break a;case 3:h.flags=h.flags&-65537|128;case 0:if(h=g.payload,f=typeof h==`function`?h.call(_,d,f):h,f==null)break a;d=m({},d,f);break a;case 2:Ra=!0}}f=s.callback,f!==null&&(e.flags|=64,p&&(e.flags|=8192),p=i.callbacks,p===null?i.callbacks=[f]:p.push(f))}else p={lane:f,tag:s.tag,payload:s.payload,callback:s.callback,next:null},u===null?(l=u=p,c=d):u=u.next=p,o|=f;if(s=s.next,s===null){if(s=i.shared.pending,s===null)break;p=s,s=p.next,p.next=null,i.lastBaseUpdate=p,i.shared.pending=null}}while(1);u===null&&(c=d),i.baseState=c,i.firstBaseUpdate=l,i.lastBaseUpdate=u,a===null&&(i.shared.lanes=0),Wl|=o,e.lanes=o,e.memoizedState=d}}function Ja(e,t){if(typeof e!=`function`)throw Error(a(191,e));e.call(t)}function Ya(e,t){var n=e.callbacks;if(n!==null)for(e.callbacks=null,e=0;e<n.length;e++)Ja(n[e],t)}var Xa=ae(null),Za=ae(0);function Qa(e,t){e=Hl,L(Za,e),L(Xa,t),Hl=e|t.baseLanes}function $a(){L(Za,Hl),L(Xa,Xa.current)}function eo(){Hl=Za.current,I(Xa),I(Za)}var to=ae(null),no=null;function ro(e){var t=e.alternate;L(co,co.current&1),L(to,e),no===null&&(t===null||Xa.current!==null||t.memoizedState!==null)&&(no=e)}function io(e){L(co,co.current),L(to,e),no===null&&(no=e)}function ao(e){e.tag===22?(L(co,co.current),L(to,e),no===null&&(no=e)):oo(e)}function oo(){L(co,co.current),L(to,to.current)}function so(e){I(to),no===e&&(no=null),I(co)}var co=ae(0);function lo(e){for(var t=e;t!==null;){if(t.tag===13){var n=t.memoizedState;if(n!==null&&(n=n.dehydrated,n===null||of(n)||sf(n)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder===`forwards`||t.memoizedProps.revealOrder===`backwards`||t.memoizedProps.revealOrder===`unstable_legacy-backwards`||t.memoizedProps.revealOrder===`together`)){if(t.flags&128)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var uo=0,H=null,fo=null,po=null,mo=!1,ho=!1,go=!1,_o=0,vo=0,yo=null,bo=0;function xo(){throw Error(a(321))}function So(e,t){if(t===null)return!1;for(var n=0;n<t.length&&n<e.length;n++)if(!Sr(e[n],t[n]))return!1;return!0}function Co(e,t,n,r,i,a){return uo=a,H=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,N.H=e===null||e.memoizedState===null?zs:Bs,go=!1,a=n(r,i),go=!1,ho&&(a=To(t,n,r,i)),wo(e),a}function wo(e){N.H=Rs;var t=fo!==null&&fo.next!==null;if(uo=0,po=fo=H=null,mo=!1,vo=0,yo=null,t)throw Error(a(300));e===null||W||(e=e.dependencies,e!==null&&Zi(e)&&(W=!0))}function To(e,t,n,r){H=e;var i=0;do{if(ho&&(yo=null),vo=0,ho=!1,25<=i)throw Error(a(301));if(i+=1,po=fo=null,e.updateQueue!=null){var o=e.updateQueue;o.lastEffect=null,o.events=null,o.stores=null,o.memoCache!=null&&(o.memoCache.index=0)}N.H=Vs,o=t(n,r)}while(ho);return o}function Eo(){var e=N.H,t=e.useState()[0];return t=typeof t.then==`function`?Mo(t):t,e=e.useState()[0],(fo===null?null:fo.memoizedState)!==e&&(H.flags|=1024),t}function Do(){var e=_o!==0;return _o=0,e}function Oo(e,t,n){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~n}function ko(e){if(mo){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}mo=!1}uo=0,po=fo=H=null,ho=!1,vo=_o=0,yo=null}function Ao(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return po===null?H.memoizedState=po=e:po=po.next=e,po}function U(){if(fo===null){var e=H.alternate;e=e===null?null:e.memoizedState}else e=fo.next;var t=po===null?H.memoizedState:po.next;if(t!==null)po=t,fo=e;else{if(e===null)throw H.alternate===null?Error(a(467)):Error(a(310));fo=e,e={memoizedState:fo.memoizedState,baseState:fo.baseState,baseQueue:fo.baseQueue,queue:fo.queue,next:null},po===null?H.memoizedState=po=e:po=po.next=e}return po}function jo(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Mo(e){var t=vo;return vo+=1,yo===null&&(yo=[]),e=Ta(yo,e,t),t=H,(po===null?t.memoizedState:po.next)===null&&(t=t.alternate,N.H=t===null||t.memoizedState===null?zs:Bs),e}function No(e){if(typeof e==`object`&&e){if(typeof e.then==`function`)return Mo(e);if(e.$$typeof===S)return $i(e)}throw Error(a(438,String(e)))}function Po(e){var t=null,n=H.updateQueue;if(n!==null&&(t=n.memoCache),t==null){var r=H.alternate;r!==null&&(r=r.updateQueue,r!==null&&(r=r.memoCache,r!=null&&(t={data:r.data.map(function(e){return e.slice()}),index:0})))}if(t??={data:[],index:0},n===null&&(n=jo(),H.updateQueue=n),n.memoCache=t,n=t.data[t.index],n===void 0)for(n=t.data[t.index]=Array(e),r=0;r<e;r++)n[r]=k;return t.index++,n}function Fo(e,t){return typeof t==`function`?t(e):t}function Io(e){return Lo(U(),fo,e)}function Lo(e,t,n){var r=e.queue;if(r===null)throw Error(a(311));r.lastRenderedReducer=n;var i=e.baseQueue,o=r.pending;if(o!==null){if(i!==null){var s=i.next;i.next=o.next,o.next=s}t.baseQueue=i=o,r.pending=null}if(o=e.baseState,i===null)e.memoizedState=o;else{t=i.next;var c=s=null,l=null,u=t,d=!1;do{var f=u.lane&-536870913;if(f===u.lane?(uo&f)===f:(X&f)===f){var p=u.revertLane;if(p===0)l!==null&&(l=l.next={lane:0,revertLane:0,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null}),f===ua&&(d=!0);else if((uo&p)===p){u=u.next,p===ua&&(d=!0);continue}else f={lane:0,revertLane:u.revertLane,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=f,s=o):l=l.next=f,H.lanes|=p,Wl|=p;f=u.action,go&&n(o,f),o=u.hasEagerState?u.eagerState:n(o,f)}else p={lane:f,revertLane:u.revertLane,gesture:u.gesture,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=p,s=o):l=l.next=p,H.lanes|=f,Wl|=f;u=u.next}while(u!==null&&u!==t);if(l===null?s=o:l.next=c,!Sr(o,e.memoizedState)&&(W=!0,d&&(n=da,n!==null)))throw n;e.memoizedState=o,e.baseState=s,e.baseQueue=l,r.lastRenderedState=o}return i===null&&(r.lanes=0),[e.memoizedState,r.dispatch]}function Ro(e){var t=U(),n=t.queue;if(n===null)throw Error(a(311));n.lastRenderedReducer=e;var r=n.dispatch,i=n.pending,o=t.memoizedState;if(i!==null){n.pending=null;var s=i=i.next;do o=e(o,s.action),s=s.next;while(s!==i);Sr(o,t.memoizedState)||(W=!0),t.memoizedState=o,t.baseQueue===null&&(t.baseState=o),n.lastRenderedState=o}return[o,r]}function zo(e,t,n){var r=H,i=U(),o=V;if(o){if(n===void 0)throw Error(a(407));n=n()}else n=t();var s=!Sr((fo||i).memoizedState,n);if(s&&(i.memoizedState=n,W=!0),i=i.queue,us(Ho.bind(null,r,i,e),[e]),i.getSnapshot!==t||s||po!==null&&po.memoizedState.tag&1){if(r.flags|=2048,as(9,{destroy:void 0},Vo.bind(null,r,i,n,t),null),Ll===null)throw Error(a(349));o||uo&127||Bo(r,t,n)}return n}function Bo(e,t,n){e.flags|=16384,e={getSnapshot:t,value:n},t=H.updateQueue,t===null?(t=jo(),H.updateQueue=t,t.stores=[e]):(n=t.stores,n===null?t.stores=[e]:n.push(e))}function Vo(e,t,n,r){t.value=n,t.getSnapshot=r,Uo(t)&&Wo(e)}function Ho(e,t,n){return n(function(){Uo(t)&&Wo(e)})}function Uo(e){var t=e.getSnapshot;e=e.value;try{var n=t();return!Sr(e,n)}catch{return!0}}function Wo(e){var t=ri(e,2);t!==null&&mu(t,e,2)}function Go(e){var t=Ao();if(typeof e==`function`){var n=e;if(e=n(),go){Ie(!0);try{n()}finally{Ie(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:e},t}function Ko(e,t,n,r){return e.baseState=n,Lo(e,fo,typeof r==`function`?r:Fo)}function qo(e,t,n,r,i){if(Fs(e))throw Error(a(485));if(e=t.action,e!==null){var o={payload:i,action:e,next:null,isTransition:!0,status:`pending`,value:null,reason:null,listeners:[],then:function(e){o.listeners.push(e)}};N.T===null?o.isTransition=!1:n(!0),r(o),n=t.pending,n===null?(o.next=t.pending=o,Jo(t,o)):(o.next=n.next,t.pending=n.next=o)}}function Jo(e,t){var n=t.action,r=t.payload,i=e.state;if(t.isTransition){var a=N.T,o={};N.T=o;try{var s=n(i,r),c=N.S;c!==null&&c(o,s),Yo(e,t,s)}catch(n){Zo(e,t,n)}finally{a!==null&&o.types!==null&&(a.types=o.types),N.T=a}}else try{a=n(i,r),Yo(e,t,a)}catch(n){Zo(e,t,n)}}function Yo(e,t,n){typeof n==`object`&&n&&typeof n.then==`function`?n.then(function(n){Xo(e,t,n)},function(n){return Zo(e,t,n)}):Xo(e,t,n)}function Xo(e,t,n){t.status=`fulfilled`,t.value=n,Qo(t),e.state=n,t=e.pending,t!==null&&(n=t.next,n===t?e.pending=null:(n=n.next,t.next=n,Jo(e,n)))}function Zo(e,t,n){var r=e.pending;if(e.pending=null,r!==null){r=r.next;do t.status=`rejected`,t.reason=n,Qo(t),t=t.next;while(t!==r)}e.action=null}function Qo(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function $o(e,t){return t}function es(e,t){if(V){var n=Ll.formState;if(n!==null){a:{var r=H;if(V){if(B){b:{for(var i=B,a=Pi;i.nodeType!==8;){if(!a){i=null;break b}if(i=lf(i.nextSibling),i===null){i=null;break b}}a=i.data,i=a===`F!`||a===`F`?i:null}if(i){B=lf(i.nextSibling),r=i.data===`F!`;break a}}Ii(r)}r=!1}r&&(t=n[0])}}return n=Ao(),n.memoizedState=n.baseState=t,r={pending:null,lanes:0,dispatch:null,lastRenderedReducer:$o,lastRenderedState:t},n.queue=r,n=Ms.bind(null,H,r),r.dispatch=n,r=Go(!1),a=Ps.bind(null,H,!1,r.queue),r=Ao(),i={state:t,dispatch:null,action:e,pending:null},r.queue=i,n=qo.bind(null,H,i,a,n),i.dispatch=n,r.memoizedState=e,[t,n,!1]}function ts(e){return ns(U(),fo,e)}function ns(e,t,n){if(t=Lo(e,t,$o)[0],e=Io(Fo)[0],typeof t==`object`&&t&&typeof t.then==`function`)try{var r=Mo(t)}catch(e){throw e===ba?Sa:e}else r=t;t=U();var i=t.queue,a=i.dispatch;return n!==t.memoizedState&&(H.flags|=2048,as(9,{destroy:void 0},rs.bind(null,i,n),null)),[r,a,e]}function rs(e,t){e.action=t}function is(e){var t=U(),n=fo;if(n!==null)return ns(t,n,e);U(),t=t.memoizedState,n=U();var r=n.queue.dispatch;return n.memoizedState=e,[t,r,!1]}function as(e,t,n,r){return e={tag:e,create:n,deps:r,inst:t,next:null},t=H.updateQueue,t===null&&(t=jo(),H.updateQueue=t),n=t.lastEffect,n===null?t.lastEffect=e.next=e:(r=n.next,n.next=e,e.next=r,t.lastEffect=e),e}function os(){return U().memoizedState}function ss(e,t,n,r){var i=Ao();H.flags|=e,i.memoizedState=as(1|t,{destroy:void 0},n,r===void 0?null:r)}function cs(e,t,n,r){var i=U();r=r===void 0?null:r;var a=i.memoizedState.inst;fo!==null&&r!==null&&So(r,fo.memoizedState.deps)?i.memoizedState=as(t,a,n,r):(H.flags|=e,i.memoizedState=as(1|t,a,n,r))}function ls(e,t){ss(8390656,8,e,t)}function us(e,t){cs(2048,8,e,t)}function ds(e){H.flags|=4;var t=H.updateQueue;if(t===null)t=jo(),H.updateQueue=t,t.events=[e];else{var n=t.events;n===null?t.events=[e]:n.push(e)}}function fs(e){var t=U().memoizedState;return ds({ref:t,nextImpl:e}),function(){if(Il&2)throw Error(a(440));return t.impl.apply(void 0,arguments)}}function ps(e,t){return cs(4,2,e,t)}function ms(e,t){return cs(4,4,e,t)}function hs(e,t){if(typeof t==`function`){e=e();var n=t(e);return function(){typeof n==`function`?n():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function gs(e,t,n){n=n==null?null:n.concat([e]),cs(4,4,hs.bind(null,t,e),n)}function _s(){}function vs(e,t){var n=U();t=t===void 0?null:t;var r=n.memoizedState;return t!==null&&So(t,r[1])?r[0]:(n.memoizedState=[e,t],e)}function ys(e,t){var n=U();t=t===void 0?null:t;var r=n.memoizedState;if(t!==null&&So(t,r[1]))return r[0];if(r=e(),go){Ie(!0);try{e()}finally{Ie(!1)}}return n.memoizedState=[r,t],r}function bs(e,t,n){return n===void 0||uo&1073741824&&!(X&261930)?e.memoizedState=t:(e.memoizedState=n,e=pu(),H.lanes|=e,Wl|=e,n)}function xs(e,t,n,r){return Sr(n,t)?n:Xa.current===null?!(uo&42)||uo&1073741824&&!(X&261930)?(W=!0,e.memoizedState=n):(e=pu(),H.lanes|=e,Wl|=e,t):(e=bs(e,n,r),Sr(e,t)||(W=!0),e)}function Ss(e,t,n,r,i){var a=P.p;P.p=a!==0&&8>a?a:8;var o=N.T,s={};N.T=s,Ps(e,!1,t,n);try{var c=i(),l=N.S;l!==null&&l(s,c),typeof c==`object`&&c&&typeof c.then==`function`?Ns(e,t,ma(c,r),fu(e)):Ns(e,t,r,fu(e))}catch(n){Ns(e,t,{then:function(){},status:`rejected`,reason:n},fu())}finally{P.p=a,o!==null&&s.types!==null&&(o.types=s.types),N.T=o}}function Cs(){}function ws(e,t,n,r){if(e.tag!==5)throw Error(a(476));var i=Ts(e).queue;Ss(e,i,t,F,n===null?Cs:function(){return Es(e),n(r)})}function Ts(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:F,baseState:F,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:F},next:null};var n={};return t.next={memoizedState:n,baseState:n,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:n},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function Es(e){var t=Ts(e);t.next===null&&(t=e.alternate.memoizedState),Ns(e,t.next.queue,{},fu())}function Ds(){return $i(Qf)}function Os(){return U().memoizedState}function ks(){return U().memoizedState}function As(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var n=fu();e=Va(n);var r=Ha(t,e,n);r!==null&&(mu(r,t,n),Ua(r,t,n)),t={cache:oa()},e.payload=t;return}t=t.return}}function js(e,t,n){var r=fu();n={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},Fs(e)?Is(t,n):(n=ni(e,t,n,r),n!==null&&(mu(n,e,r),Ls(n,t,r)))}function Ms(e,t,n){Ns(e,t,n,fu())}function Ns(e,t,n,r){var i={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null};if(Fs(e))Is(t,i);else{var a=e.alternate;if(e.lanes===0&&(a===null||a.lanes===0)&&(a=t.lastRenderedReducer,a!==null))try{var o=t.lastRenderedState,s=a(o,n);if(i.hasEagerState=!0,i.eagerState=s,Sr(s,o))return ti(e,t,i,0),Ll===null&&ei(),!1}catch{}if(n=ni(e,t,i,r),n!==null)return mu(n,e,r),Ls(n,t,r),!0}return!1}function Ps(e,t,n,r){if(r={lane:2,revertLane:dd(),gesture:null,action:r,hasEagerState:!1,eagerState:null,next:null},Fs(e)){if(t)throw Error(a(479))}else t=ni(e,n,r,2),t!==null&&mu(t,e,2)}function Fs(e){var t=e.alternate;return e===H||t!==null&&t===H}function Is(e,t){ho=mo=!0;var n=e.pending;n===null?t.next=t:(t.next=n.next,n.next=t),e.pending=t}function Ls(e,t,n){if(n&4194048){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,$e(e,n)}}var Rs={readContext:$i,use:No,useCallback:xo,useContext:xo,useEffect:xo,useImperativeHandle:xo,useLayoutEffect:xo,useInsertionEffect:xo,useMemo:xo,useReducer:xo,useRef:xo,useState:xo,useDebugValue:xo,useDeferredValue:xo,useTransition:xo,useSyncExternalStore:xo,useId:xo,useHostTransitionStatus:xo,useFormState:xo,useActionState:xo,useOptimistic:xo,useMemoCache:xo,useCacheRefresh:xo};Rs.useEffectEvent=xo;var zs={readContext:$i,use:No,useCallback:function(e,t){return Ao().memoizedState=[e,t===void 0?null:t],e},useContext:$i,useEffect:ls,useImperativeHandle:function(e,t,n){n=n==null?null:n.concat([e]),ss(4194308,4,hs.bind(null,t,e),n)},useLayoutEffect:function(e,t){return ss(4194308,4,e,t)},useInsertionEffect:function(e,t){ss(4,2,e,t)},useMemo:function(e,t){var n=Ao();t=t===void 0?null:t;var r=e();if(go){Ie(!0);try{e()}finally{Ie(!1)}}return n.memoizedState=[r,t],r},useReducer:function(e,t,n){var r=Ao();if(n!==void 0){var i=n(t);if(go){Ie(!0);try{n(t)}finally{Ie(!1)}}}else i=t;return r.memoizedState=r.baseState=i,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:i},r.queue=e,e=e.dispatch=js.bind(null,H,e),[r.memoizedState,e]},useRef:function(e){var t=Ao();return e={current:e},t.memoizedState=e},useState:function(e){e=Go(e);var t=e.queue,n=Ms.bind(null,H,t);return t.dispatch=n,[e.memoizedState,n]},useDebugValue:_s,useDeferredValue:function(e,t){return bs(Ao(),e,t)},useTransition:function(){var e=Go(!1);return e=Ss.bind(null,H,e.queue,!0,!1),Ao().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,n){var r=H,i=Ao();if(V){if(n===void 0)throw Error(a(407));n=n()}else{if(n=t(),Ll===null)throw Error(a(349));X&127||Bo(r,t,n)}i.memoizedState=n;var o={value:n,getSnapshot:t};return i.queue=o,ls(Ho.bind(null,r,o,e),[e]),r.flags|=2048,as(9,{destroy:void 0},Vo.bind(null,r,o,n,t),null),n},useId:function(){var e=Ao(),t=Ll.identifierPrefix;if(V){var n=Ei,r=Ti;n=(r&~(1<<32-Le(r)-1)).toString(32)+n,t=`_`+t+`R_`+n,n=_o++,0<n&&(t+=`H`+n.toString(32)),t+=`_`}else n=bo++,t=`_`+t+`r_`+n.toString(32)+`_`;return e.memoizedState=t},useHostTransitionStatus:Ds,useFormState:es,useActionState:es,useOptimistic:function(e){var t=Ao();t.memoizedState=t.baseState=e;var n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=n,t=Ps.bind(null,H,!0,n),n.dispatch=t,[e,t]},useMemoCache:Po,useCacheRefresh:function(){return Ao().memoizedState=As.bind(null,H)},useEffectEvent:function(e){var t=Ao(),n={impl:e};return t.memoizedState=n,function(){if(Il&2)throw Error(a(440));return n.impl.apply(void 0,arguments)}}},Bs={readContext:$i,use:No,useCallback:vs,useContext:$i,useEffect:us,useImperativeHandle:gs,useInsertionEffect:ps,useLayoutEffect:ms,useMemo:ys,useReducer:Io,useRef:os,useState:function(){return Io(Fo)},useDebugValue:_s,useDeferredValue:function(e,t){return xs(U(),fo.memoizedState,e,t)},useTransition:function(){var e=Io(Fo)[0],t=U().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:zo,useId:Os,useHostTransitionStatus:Ds,useFormState:ts,useActionState:ts,useOptimistic:function(e,t){return Ko(U(),fo,e,t)},useMemoCache:Po,useCacheRefresh:ks};Bs.useEffectEvent=fs;var Vs={readContext:$i,use:No,useCallback:vs,useContext:$i,useEffect:us,useImperativeHandle:gs,useInsertionEffect:ps,useLayoutEffect:ms,useMemo:ys,useReducer:Ro,useRef:os,useState:function(){return Ro(Fo)},useDebugValue:_s,useDeferredValue:function(e,t){var n=U();return fo===null?bs(n,e,t):xs(n,fo.memoizedState,e,t)},useTransition:function(){var e=Ro(Fo)[0],t=U().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:zo,useId:Os,useHostTransitionStatus:Ds,useFormState:is,useActionState:is,useOptimistic:function(e,t){var n=U();return fo===null?(n.baseState=e,[e,n.queue.dispatch]):Ko(n,fo,e,t)},useMemoCache:Po,useCacheRefresh:ks};Vs.useEffectEvent=fs;function Hs(e,t,n,r){t=e.memoizedState,n=n(r,t),n=n==null?t:m({},t,n),e.memoizedState=n,e.lanes===0&&(e.updateQueue.baseState=n)}var Us={enqueueSetState:function(e,t,n){e=e._reactInternals;var r=fu(),i=Va(r);i.payload=t,n!=null&&(i.callback=n),t=Ha(e,i,r),t!==null&&(mu(t,e,r),Ua(t,e,r))},enqueueReplaceState:function(e,t,n){e=e._reactInternals;var r=fu(),i=Va(r);i.tag=1,i.payload=t,n!=null&&(i.callback=n),t=Ha(e,i,r),t!==null&&(mu(t,e,r),Ua(t,e,r))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var n=fu(),r=Va(n);r.tag=2,t!=null&&(r.callback=t),t=Ha(e,r,n),t!==null&&(mu(t,e,n),Ua(t,e,n))}};function Ws(e,t,n,r,i,a,o){return e=e.stateNode,typeof e.shouldComponentUpdate==`function`?e.shouldComponentUpdate(r,a,o):t.prototype&&t.prototype.isPureReactComponent?!Cr(n,r)||!Cr(i,a):!0}function Gs(e,t,n,r){e=t.state,typeof t.componentWillReceiveProps==`function`&&t.componentWillReceiveProps(n,r),typeof t.UNSAFE_componentWillReceiveProps==`function`&&t.UNSAFE_componentWillReceiveProps(n,r),t.state!==e&&Us.enqueueReplaceState(t,t.state,null)}function Ks(e,t){var n=t;if(`ref`in t)for(var r in n={},t)r!==`ref`&&(n[r]=t[r]);if(e=e.defaultProps)for(var i in n===t&&(n=m({},n)),e)n[i]===void 0&&(n[i]=e[i]);return n}function qs(e){Xr(e)}function Js(e){console.error(e)}function Ys(e){Xr(e)}function Xs(e,t){try{var n=e.onUncaughtError;n(t.value,{componentStack:t.stack})}catch(e){setTimeout(function(){throw e})}}function Zs(e,t,n){try{var r=e.onCaughtError;r(n.value,{componentStack:n.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(e){setTimeout(function(){throw e})}}function Qs(e,t,n){return n=Va(n),n.tag=3,n.payload={element:null},n.callback=function(){Xs(e,t)},n}function $s(e){return e=Va(e),e.tag=3,e}function ec(e,t,n,r){var i=n.type.getDerivedStateFromError;if(typeof i==`function`){var a=r.value;e.payload=function(){return i(a)},e.callback=function(){Zs(t,n,r)}}var o=n.stateNode;o!==null&&typeof o.componentDidCatch==`function`&&(e.callback=function(){Zs(t,n,r),typeof i!=`function`&&(nu===null?nu=new Set([this]):nu.add(this));var e=r.stack;this.componentDidCatch(r.value,{componentStack:e===null?``:e})})}function tc(e,t,n,r,i){if(n.flags|=32768,typeof r==`object`&&r&&typeof r.then==`function`){if(t=n.alternate,t!==null&&Xi(t,n,i,!0),n=to.current,n!==null){switch(n.tag){case 31:case 13:return no===null?Eu():n.alternate===null&&Ul===0&&(Ul=3),n.flags&=-257,n.flags|=65536,n.lanes=i,r===Ca?n.flags|=16384:(t=n.updateQueue,t===null?n.updateQueue=new Set([r]):t.add(r),Gu(e,r,i)),!1;case 22:return n.flags|=65536,r===Ca?n.flags|=16384:(t=n.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([r])},n.updateQueue=t):(n=t.retryQueue,n===null?t.retryQueue=new Set([r]):n.add(r)),Gu(e,r,i)),!1}throw Error(a(435,n.tag))}return Gu(e,r,i),Eu(),!1}if(V)return t=to.current,t===null?(r!==Fi&&(t=Error(a(423),{cause:r}),Hi(vi(t,n))),e=e.current.alternate,e.flags|=65536,i&=-i,e.lanes|=i,r=vi(r,n),i=Qs(e.stateNode,r,i),Wa(e,i),Ul!==4&&(Ul=2)):(!(t.flags&65536)&&(t.flags|=256),t.flags|=65536,t.lanes=i,r!==Fi&&(e=Error(a(422),{cause:r}),Hi(vi(e,n)))),!1;var o=Error(a(520),{cause:r});if(o=vi(o,n),Yl===null?Yl=[o]:Yl.push(o),Ul!==4&&(Ul=2),t===null)return!0;r=vi(r,n),n=t;do{switch(n.tag){case 3:return n.flags|=65536,e=i&-i,n.lanes|=e,e=Qs(n.stateNode,r,e),Wa(n,e),!1;case 1:if(t=n.type,o=n.stateNode,!(n.flags&128)&&(typeof t.getDerivedStateFromError==`function`||o!==null&&typeof o.componentDidCatch==`function`&&(nu===null||!nu.has(o))))return n.flags|=65536,i&=-i,n.lanes|=i,i=$s(i),ec(i,e,n,r),Wa(n,i),!1}n=n.return}while(n!==null);return!1}var nc=Error(a(461)),W=!1;function rc(e,t,n,r){t.child=e===null?La(t,null,n,r):Ia(t,e.child,n,r)}function ic(e,t,n,r,i){n=n.render;var a=t.ref;if(`ref`in r){var o={};for(var s in r)s!==`ref`&&(o[s]=r[s])}else o=r;return Qi(t),r=Co(e,t,n,o,a,i),s=Do(),e!==null&&!W?(Oo(e,t,i),Oc(e,t,i)):(V&&s&&ki(t),t.flags|=1,rc(e,t,r,i),t.child)}function ac(e,t,n,r,i){if(e===null){var a=n.type;return typeof a==`function`&&!li(a)&&a.defaultProps===void 0&&n.compare===null?(t.tag=15,t.type=a,oc(e,t,a,r,i)):(e=fi(n.type,null,r,t,t.mode,i),e.ref=t.ref,e.return=t,t.child=e)}if(a=e.child,!kc(e,i)){var o=a.memoizedProps;if(n=n.compare,n=n===null?Cr:n,n(o,r)&&e.ref===t.ref)return Oc(e,t,i)}return t.flags|=1,e=ui(a,r),e.ref=t.ref,e.return=t,t.child=e}function oc(e,t,n,r,i){if(e!==null){var a=e.memoizedProps;if(Cr(a,r)&&e.ref===t.ref)if(W=!1,t.pendingProps=r=a,kc(e,i))e.flags&131072&&(W=!0);else return t.lanes=e.lanes,Oc(e,t,i)}return mc(e,t,n,r,i)}function sc(e,t,n,r){var i=r.children,a=e===null?null:e.memoizedState;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),r.mode===`hidden`){if(t.flags&128){if(a=a===null?n:a.baseLanes|n,e!==null){for(r=t.child=e.child,i=0;r!==null;)i=i|r.lanes|r.childLanes,r=r.sibling;r=i&~a}else r=0,t.child=null;return lc(e,t,a,n,r)}if(n&536870912)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&va(t,a===null?null:a.cachePool),a===null?$a():Qa(t,a),ao(t);else return r=t.lanes=536870912,lc(e,t,a===null?n:a.baseLanes|n,n,r)}else a===null?(e!==null&&va(t,null),$a(),oo(t)):(va(t,a.cachePool),Qa(t,a),oo(t),t.memoizedState=null);return rc(e,t,i,n),t.child}function cc(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function lc(e,t,n,r,i){var a=_a();return a=a===null?null:{parent:aa._currentValue,pool:a},t.memoizedState={baseLanes:n,cachePool:a},e!==null&&va(t,null),$a(),ao(t),e!==null&&Xi(e,t,r,!0),t.childLanes=i,null}function uc(e,t){return t=Cc({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function dc(e,t,n){return Ia(t,e.child,null,n),e=uc(t,t.pendingProps),e.flags|=2,so(t),t.memoizedState=null,e}function fc(e,t,n){var r=t.pendingProps,i=(t.flags&128)!=0;if(t.flags&=-129,e===null){if(V){if(r.mode===`hidden`)return e=uc(t,r),t.lanes=536870912,cc(null,e);if(io(t),(e=B)?(e=af(e,Pi),e=e!==null&&e.data===`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:wi===null?null:{id:Ti,overflow:Ei},retryLane:536870912,hydrationErrors:null},n=hi(e),n.return=t,t.child=n,Mi=t,B=null)):e=null,e===null)throw Ii(t);return t.lanes=536870912,null}return uc(t,r)}var o=e.memoizedState;if(o!==null){var s=o.dehydrated;if(io(t),i)if(t.flags&256)t.flags&=-257,t=dc(e,t,n);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(a(558));else if(W||Xi(e,t,n,!1),i=(n&e.childLanes)!==0,W||i){if(r=Ll,r!==null&&(s=et(r,n),s!==0&&s!==o.retryLane))throw o.retryLane=s,ri(e,s),mu(r,e,s),nc;Eu(),t=dc(e,t,n)}else e=o.treeContext,B=lf(s.nextSibling),Mi=t,V=!0,Ni=null,Pi=!1,e!==null&&ji(t,e),t=uc(t,r),t.flags|=4096;return t}return e=ui(e.child,{mode:r.mode,children:r.children}),e.ref=t.ref,t.child=e,e.return=t,e}function pc(e,t){var n=t.ref;if(n===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof n!=`function`&&typeof n!=`object`)throw Error(a(284));(e===null||e.ref!==n)&&(t.flags|=4194816)}}function mc(e,t,n,r,i){return Qi(t),n=Co(e,t,n,r,void 0,i),r=Do(),e!==null&&!W?(Oo(e,t,i),Oc(e,t,i)):(V&&r&&ki(t),t.flags|=1,rc(e,t,n,i),t.child)}function hc(e,t,n,r,i,a){return Qi(t),t.updateQueue=null,n=To(t,r,n,i),wo(e),r=Do(),e!==null&&!W?(Oo(e,t,a),Oc(e,t,a)):(V&&r&&ki(t),t.flags|=1,rc(e,t,n,a),t.child)}function gc(e,t,n,r,i){if(Qi(t),t.stateNode===null){var a=oi,o=n.contextType;typeof o==`object`&&o&&(a=$i(o)),a=new n(r,a),t.memoizedState=a.state!==null&&a.state!==void 0?a.state:null,a.updater=Us,t.stateNode=a,a._reactInternals=t,a=t.stateNode,a.props=r,a.state=t.memoizedState,a.refs={},za(t),o=n.contextType,a.context=typeof o==`object`&&o?$i(o):oi,a.state=t.memoizedState,o=n.getDerivedStateFromProps,typeof o==`function`&&(Hs(t,n,o,r),a.state=t.memoizedState),typeof n.getDerivedStateFromProps==`function`||typeof a.getSnapshotBeforeUpdate==`function`||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(o=a.state,typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount(),o!==a.state&&Us.enqueueReplaceState(a,a.state,null),qa(t,r,a,i),Ka(),a.state=t.memoizedState),typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!0}else if(e===null){a=t.stateNode;var s=t.memoizedProps,c=Ks(n,s);a.props=c;var l=a.context,u=n.contextType;o=oi,typeof u==`object`&&u&&(o=$i(u));var d=n.getDerivedStateFromProps;u=typeof d==`function`||typeof a.getSnapshotBeforeUpdate==`function`,s=t.pendingProps!==s,u||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(s||l!==o)&&Gs(t,a,r,o),Ra=!1;var f=t.memoizedState;a.state=f,qa(t,r,a,i),Ka(),l=t.memoizedState,s||f!==l||Ra?(typeof d==`function`&&(Hs(t,n,d,r),l=t.memoizedState),(c=Ra||Ws(t,n,c,r,f,l,o))?(u||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount()),typeof a.componentDidMount==`function`&&(t.flags|=4194308)):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),t.memoizedProps=r,t.memoizedState=l),a.props=r,a.state=l,a.context=o,r=c):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!1)}else{a=t.stateNode,Ba(e,t),o=t.memoizedProps,u=Ks(n,o),a.props=u,d=t.pendingProps,f=a.context,l=n.contextType,c=oi,typeof l==`object`&&l&&(c=$i(l)),s=n.getDerivedStateFromProps,(l=typeof s==`function`||typeof a.getSnapshotBeforeUpdate==`function`)||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(o!==d||f!==c)&&Gs(t,a,r,c),Ra=!1,f=t.memoizedState,a.state=f,qa(t,r,a,i),Ka();var p=t.memoizedState;o!==d||f!==p||Ra||e!==null&&e.dependencies!==null&&Zi(e.dependencies)?(typeof s==`function`&&(Hs(t,n,s,r),p=t.memoizedState),(u=Ra||Ws(t,n,u,r,f,p,c)||e!==null&&e.dependencies!==null&&Zi(e.dependencies))?(l||typeof a.UNSAFE_componentWillUpdate!=`function`&&typeof a.componentWillUpdate!=`function`||(typeof a.componentWillUpdate==`function`&&a.componentWillUpdate(r,p,c),typeof a.UNSAFE_componentWillUpdate==`function`&&a.UNSAFE_componentWillUpdate(r,p,c)),typeof a.componentDidUpdate==`function`&&(t.flags|=4),typeof a.getSnapshotBeforeUpdate==`function`&&(t.flags|=1024)):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),t.memoizedProps=r,t.memoizedState=p),a.props=r,a.state=p,a.context=c,r=u):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),r=!1)}return a=r,pc(e,t),r=(t.flags&128)!=0,a||r?(a=t.stateNode,n=r&&typeof n.getDerivedStateFromError!=`function`?null:a.render(),t.flags|=1,e!==null&&r?(t.child=Ia(t,e.child,null,i),t.child=Ia(t,null,n,i)):rc(e,t,n,i),t.memoizedState=a.state,e=t.child):e=Oc(e,t,i),e}function _c(e,t,n,r){return Bi(),t.flags|=256,rc(e,t,n,r),t.child}var vc={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function yc(e){return{baseLanes:e,cachePool:ya()}}function bc(e,t,n){return e=e===null?0:e.childLanes&~n,t&&(e|=ql),e}function xc(e,t,n){var r=t.pendingProps,i=!1,o=(t.flags&128)!=0,s;if((s=o)||(s=e!==null&&e.memoizedState===null?!1:(co.current&2)!=0),s&&(i=!0,t.flags&=-129),s=(t.flags&32)!=0,t.flags&=-33,e===null){if(V){if(i?ro(t):oo(t),(e=B)?(e=af(e,Pi),e=e!==null&&e.data!==`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:wi===null?null:{id:Ti,overflow:Ei},retryLane:536870912,hydrationErrors:null},n=hi(e),n.return=t,t.child=n,Mi=t,B=null)):e=null,e===null)throw Ii(t);return sf(e)?t.lanes=32:t.lanes=536870912,null}var c=r.children;return r=r.fallback,i?(oo(t),i=t.mode,c=Cc({mode:`hidden`,children:c},i),r=pi(r,i,n,null),c.return=t,r.return=t,c.sibling=r,t.child=c,r=t.child,r.memoizedState=yc(n),r.childLanes=bc(e,s,n),t.memoizedState=vc,cc(null,r)):(ro(t),Sc(t,c))}var l=e.memoizedState;if(l!==null&&(c=l.dehydrated,c!==null)){if(o)t.flags&256?(ro(t),t.flags&=-257,t=wc(e,t,n)):t.memoizedState===null?(oo(t),c=r.fallback,i=t.mode,r=Cc({mode:`visible`,children:r.children},i),c=pi(c,i,n,null),c.flags|=2,r.return=t,c.return=t,r.sibling=c,t.child=r,Ia(t,e.child,null,n),r=t.child,r.memoizedState=yc(n),r.childLanes=bc(e,s,n),t.memoizedState=vc,t=cc(null,r)):(oo(t),t.child=e.child,t.flags|=128,t=null);else if(ro(t),sf(c)){if(s=c.nextSibling&&c.nextSibling.dataset,s)var u=s.dgst;s=u,r=Error(a(419)),r.stack=``,r.digest=s,Hi({value:r,source:null,stack:null}),t=wc(e,t,n)}else if(W||Xi(e,t,n,!1),s=(n&e.childLanes)!==0,W||s){if(s=Ll,s!==null&&(r=et(s,n),r!==0&&r!==l.retryLane))throw l.retryLane=r,ri(e,r),mu(s,e,r),nc;of(c)||Eu(),t=wc(e,t,n)}else of(c)?(t.flags|=192,t.child=e.child,t=null):(e=l.treeContext,B=lf(c.nextSibling),Mi=t,V=!0,Ni=null,Pi=!1,e!==null&&ji(t,e),t=Sc(t,r.children),t.flags|=4096);return t}return i?(oo(t),c=r.fallback,i=t.mode,l=e.child,u=l.sibling,r=ui(l,{mode:`hidden`,children:r.children}),r.subtreeFlags=l.subtreeFlags&65011712,u===null?(c=pi(c,i,n,null),c.flags|=2):c=ui(u,c),c.return=t,r.return=t,r.sibling=c,t.child=r,cc(null,r),r=t.child,c=e.child.memoizedState,c===null?c=yc(n):(i=c.cachePool,i===null?i=ya():(l=aa._currentValue,i=i.parent===l?i:{parent:l,pool:l}),c={baseLanes:c.baseLanes|n,cachePool:i}),r.memoizedState=c,r.childLanes=bc(e,s,n),t.memoizedState=vc,cc(e.child,r)):(ro(t),n=e.child,e=n.sibling,n=ui(n,{mode:`visible`,children:r.children}),n.return=t,n.sibling=null,e!==null&&(s=t.deletions,s===null?(t.deletions=[e],t.flags|=16):s.push(e)),t.child=n,t.memoizedState=null,n)}function Sc(e,t){return t=Cc({mode:`visible`,children:t},e.mode),t.return=e,e.child=t}function Cc(e,t){return e=ci(22,e,null,t),e.lanes=0,e}function wc(e,t,n){return Ia(t,e.child,null,n),e=Sc(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function Tc(e,t,n){e.lanes|=t;var r=e.alternate;r!==null&&(r.lanes|=t),Ji(e.return,t,n)}function Ec(e,t,n,r,i,a){var o=e.memoizedState;o===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:r,tail:n,tailMode:i,treeForkCount:a}:(o.isBackwards=t,o.rendering=null,o.renderingStartTime=0,o.last=r,o.tail=n,o.tailMode=i,o.treeForkCount=a)}function Dc(e,t,n){var r=t.pendingProps,i=r.revealOrder,a=r.tail;r=r.children;var o=co.current,s=(o&2)!=0;if(s?(o=o&1|2,t.flags|=128):o&=1,L(co,o),rc(e,t,r,n),r=V?Si:0,!s&&e!==null&&e.flags&128)a:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&Tc(e,n,t);else if(e.tag===19)Tc(e,n,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break a;for(;e.sibling===null;){if(e.return===null||e.return===t)break a;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(i){case`forwards`:for(n=t.child,i=null;n!==null;)e=n.alternate,e!==null&&lo(e)===null&&(i=n),n=n.sibling;n=i,n===null?(i=t.child,t.child=null):(i=n.sibling,n.sibling=null),Ec(t,!1,i,n,a,r);break;case`backwards`:case`unstable_legacy-backwards`:for(n=null,i=t.child,t.child=null;i!==null;){if(e=i.alternate,e!==null&&lo(e)===null){t.child=i;break}e=i.sibling,i.sibling=n,n=i,i=e}Ec(t,!0,n,null,a,r);break;case`together`:Ec(t,!1,null,null,void 0,r);break;default:t.memoizedState=null}return t.child}function Oc(e,t,n){if(e!==null&&(t.dependencies=e.dependencies),Wl|=t.lanes,(n&t.childLanes)===0)if(e!==null){if(Xi(e,t,n,!1),(n&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(a(153));if(t.child!==null){for(e=t.child,n=ui(e,e.pendingProps),t.child=n,n.return=t;e.sibling!==null;)e=e.sibling,n=n.sibling=ui(e,e.pendingProps),n.return=t;n.sibling=null}return t.child}function kc(e,t){return(e.lanes&t)===0?(e=e.dependencies,!!(e!==null&&Zi(e))):!0}function Ac(e,t,n){switch(t.tag){case 3:ue(t,t.stateNode.containerInfo),Ki(t,aa,e.memoizedState.cache),Bi();break;case 27:case 5:fe(t);break;case 4:ue(t,t.stateNode.containerInfo);break;case 10:Ki(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,io(t),null;break;case 13:var r=t.memoizedState;if(r!==null)return r.dehydrated===null?(n&t.child.childLanes)===0?(ro(t),e=Oc(e,t,n),e===null?null:e.sibling):xc(e,t,n):(ro(t),t.flags|=128,null);ro(t);break;case 19:var i=(e.flags&128)!=0;if(r=(n&t.childLanes)!==0,r||=(Xi(e,t,n,!1),(n&t.childLanes)!==0),i){if(r)return Dc(e,t,n);t.flags|=128}if(i=t.memoizedState,i!==null&&(i.rendering=null,i.tail=null,i.lastEffect=null),L(co,co.current),r)break;return null;case 22:return t.lanes=0,sc(e,t,n,t.pendingProps);case 24:Ki(t,aa,e.memoizedState.cache)}return Oc(e,t,n)}function jc(e,t,n){if(e!==null)if(e.memoizedProps!==t.pendingProps)W=!0;else{if(!kc(e,n)&&!(t.flags&128))return W=!1,Ac(e,t,n);W=!!(e.flags&131072)}else W=!1,V&&t.flags&1048576&&Oi(t,Si,t.index);switch(t.lanes=0,t.tag){case 16:a:{var r=t.pendingProps;if(e=Ea(t.elementType),t.type=e,typeof e==`function`)li(e)?(r=Ks(e,r),t.tag=1,t=gc(null,t,e,r,n)):(t.tag=0,t=mc(null,t,e,r,n));else{if(e!=null){var i=e.$$typeof;if(i===C){t.tag=11,t=ic(null,t,e,r,n);break a}else if(i===E){t.tag=14,t=ac(null,t,e,r,n);break a}}throw t=te(e)||e,Error(a(306,t,``))}}return t;case 0:return mc(e,t,t.type,t.pendingProps,n);case 1:return r=t.type,i=Ks(r,t.pendingProps),gc(e,t,r,i,n);case 3:a:{if(ue(t,t.stateNode.containerInfo),e===null)throw Error(a(387));r=t.pendingProps;var o=t.memoizedState;i=o.element,Ba(e,t),qa(t,r,null,n);var s=t.memoizedState;if(r=s.cache,Ki(t,aa,r),r!==o.cache&&Yi(t,[aa],n,!0),Ka(),r=s.element,o.isDehydrated)if(o={element:r,isDehydrated:!1,cache:s.cache},t.updateQueue.baseState=o,t.memoizedState=o,t.flags&256){t=_c(e,t,r,n);break a}else if(r!==i){i=vi(Error(a(424)),t),Hi(i),t=_c(e,t,r,n);break a}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName===`HTML`?e.ownerDocument.body:e}for(B=lf(e.firstChild),Mi=t,V=!0,Ni=null,Pi=!0,n=La(t,null,r,n),t.child=n;n;)n.flags=n.flags&-3|4096,n=n.sibling}else{if(Bi(),r===i){t=Oc(e,t,n);break a}rc(e,t,r,n)}t=t.child}return t;case 26:return pc(e,t),e===null?(n=Af(t.type,null,t.pendingProps,null))?t.memoizedState=n:V||(n=t.type,e=t.pendingProps,r=Vd(ce.current).createElement(n),r[ot]=t,r[st]=e,Fd(r,n,e),yt(r),t.stateNode=r):t.memoizedState=Af(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return fe(t),e===null&&V&&(r=t.stateNode=pf(t.type,t.pendingProps,ce.current),Mi=t,Pi=!0,i=B,Qd(t.type)?(uf=i,B=lf(r.firstChild)):B=i),rc(e,t,t.pendingProps.children,n),pc(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&V&&((i=r=B)&&(r=nf(r,t.type,t.pendingProps,Pi),r===null?i=!1:(t.stateNode=r,Mi=t,B=lf(r.firstChild),Pi=!1,i=!0)),i||Ii(t)),fe(t),i=t.type,o=t.pendingProps,s=e===null?null:e.memoizedProps,r=o.children,Wd(i,o)?r=null:s!==null&&Wd(i,s)&&(t.flags|=32),t.memoizedState!==null&&(i=Co(e,t,Eo,null,null,n),Qf._currentValue=i),pc(e,t),rc(e,t,r,n),t.child;case 6:return e===null&&V&&((e=n=B)&&(n=rf(n,t.pendingProps,Pi),n===null?e=!1:(t.stateNode=n,Mi=t,B=null,e=!0)),e||Ii(t)),null;case 13:return xc(e,t,n);case 4:return ue(t,t.stateNode.containerInfo),r=t.pendingProps,e===null?t.child=Ia(t,null,r,n):rc(e,t,r,n),t.child;case 11:return ic(e,t,t.type,t.pendingProps,n);case 7:return rc(e,t,t.pendingProps,n),t.child;case 8:return rc(e,t,t.pendingProps.children,n),t.child;case 12:return rc(e,t,t.pendingProps.children,n),t.child;case 10:return r=t.pendingProps,Ki(t,t.type,r.value),rc(e,t,r.children,n),t.child;case 9:return i=t.type._context,r=t.pendingProps.children,Qi(t),i=$i(i),r=r(i),t.flags|=1,rc(e,t,r,n),t.child;case 14:return ac(e,t,t.type,t.pendingProps,n);case 15:return oc(e,t,t.type,t.pendingProps,n);case 19:return Dc(e,t,n);case 31:return fc(e,t,n);case 22:return sc(e,t,n,t.pendingProps);case 24:return Qi(t),r=$i(aa),e===null?(i=_a(),i===null&&(i=Ll,o=oa(),i.pooledCache=o,o.refCount++,o!==null&&(i.pooledCacheLanes|=n),i=o),t.memoizedState={parent:r,cache:i},za(t),Ki(t,aa,i)):((e.lanes&n)!==0&&(Ba(e,t),qa(t,null,null,n),Ka()),i=e.memoizedState,o=t.memoizedState,i.parent===r?(r=o.cache,Ki(t,aa,r),r!==i.cache&&Yi(t,[aa],n,!0)):(i={parent:r,cache:r},t.memoizedState=i,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=i),Ki(t,aa,r))),rc(e,t,t.pendingProps.children,n),t.child;case 29:throw t.pendingProps}throw Error(a(156,t.tag))}function Mc(e){e.flags|=4}function Nc(e,t,n,r,i){if((t=(e.mode&32)!=0)&&(t=!1),t){if(e.flags|=16777216,(i&335544128)===i)if(e.stateNode.complete)e.flags|=8192;else if(Cu())e.flags|=8192;else throw Da=Ca,xa}else e.flags&=-16777217}function Pc(e,t){if(t.type!==`stylesheet`||t.state.loading&4)e.flags&=-16777217;else if(e.flags|=16777216,!Wf(t))if(Cu())e.flags|=8192;else throw Da=Ca,xa}function Fc(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag===22?536870912:Je(),e.lanes|=t,Jl|=t)}function Ic(e,t){if(!V)switch(e.tailMode){case`hidden`:t=e.tail;for(var n=null;t!==null;)t.alternate!==null&&(n=t),t=t.sibling;n===null?e.tail=null:n.sibling=null;break;case`collapsed`:n=e.tail;for(var r=null;n!==null;)n.alternate!==null&&(r=n),n=n.sibling;r===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:r.sibling=null}}function G(e){var t=e.alternate!==null&&e.alternate.child===e.child,n=0,r=0;if(t)for(var i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags&65011712,r|=i.flags&65011712,i.return=e,i=i.sibling;else for(i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags,r|=i.flags,i.return=e,i=i.sibling;return e.subtreeFlags|=r,e.childLanes=n,t}function Lc(e,t,n){var r=t.pendingProps;switch(Ai(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return G(t),null;case 1:return G(t),null;case 3:return n=t.stateNode,r=null,e!==null&&(r=e.memoizedState.cache),t.memoizedState.cache!==r&&(t.flags|=2048),qi(aa),de(),n.pendingContext&&(n.context=n.pendingContext,n.pendingContext=null),(e===null||e.child===null)&&(zi(t)?Mc(t):e===null||e.memoizedState.isDehydrated&&!(t.flags&256)||(t.flags|=1024,Vi())),G(t),null;case 26:var i=t.type,o=t.memoizedState;return e===null?(Mc(t),o===null?(G(t),Nc(t,i,null,r,n)):(G(t),Pc(t,o))):o?o===e.memoizedState?(G(t),t.flags&=-16777217):(Mc(t),G(t),Pc(t,o)):(e=e.memoizedProps,e!==r&&Mc(t),G(t),Nc(t,i,e,r,n)),null;case 27:if(R(t),n=ce.current,i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return G(t),null}e=oe.current,zi(t)?Li(t,e):(e=pf(i,r,n),t.stateNode=e,Mc(t))}return G(t),null;case 5:if(R(t),i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return G(t),null}if(o=oe.current,zi(t))Li(t,o);else{var s=Vd(ce.current);switch(o){case 1:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case 2:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;default:switch(i){case`svg`:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case`math`:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;case`script`:o=s.createElement(`div`),o.innerHTML=`<script><\/script>`,o=o.removeChild(o.firstChild);break;case`select`:o=typeof r.is==`string`?s.createElement(`select`,{is:r.is}):s.createElement(`select`),r.multiple?o.multiple=!0:r.size&&(o.size=r.size);break;default:o=typeof r.is==`string`?s.createElement(i,{is:r.is}):s.createElement(i)}}o[ot]=t,o[st]=r;a:for(s=t.child;s!==null;){if(s.tag===5||s.tag===6)o.appendChild(s.stateNode);else if(s.tag!==4&&s.tag!==27&&s.child!==null){s.child.return=s,s=s.child;continue}if(s===t)break a;for(;s.sibling===null;){if(s.return===null||s.return===t)break a;s=s.return}s.sibling.return=s.return,s=s.sibling}t.stateNode=o;a:switch(Fd(o,i,r),i){case`button`:case`input`:case`select`:case`textarea`:r=!!r.autoFocus;break a;case`img`:r=!0;break a;default:r=!1}r&&Mc(t)}}return G(t),Nc(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,n),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==r&&Mc(t);else{if(typeof r!=`string`&&t.stateNode===null)throw Error(a(166));if(e=ce.current,zi(t)){if(e=t.stateNode,n=t.memoizedProps,r=null,i=Mi,i!==null)switch(i.tag){case 27:case 5:r=i.memoizedProps}e[ot]=t,e=!!(e.nodeValue===n||r!==null&&!0===r.suppressHydrationWarning||Md(e.nodeValue,n)),e||Ii(t,!0)}else e=Vd(e).createTextNode(r),e[ot]=t,t.stateNode=e}return G(t),null;case 31:if(n=t.memoizedState,e===null||e.memoizedState!==null){if(r=zi(t),n!==null){if(e===null){if(!r)throw Error(a(318));if(e=t.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(557));e[ot]=t}else Bi(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;G(t),e=!1}else n=Vi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=n),e=!0;if(!e)return t.flags&256?(so(t),t):(so(t),null);if(t.flags&128)throw Error(a(558))}return G(t),null;case 13:if(r=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(i=zi(t),r!==null&&r.dehydrated!==null){if(e===null){if(!i)throw Error(a(318));if(i=t.memoizedState,i=i===null?null:i.dehydrated,!i)throw Error(a(317));i[ot]=t}else Bi(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;G(t),i=!1}else i=Vi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=i),i=!0;if(!i)return t.flags&256?(so(t),t):(so(t),null)}return so(t),t.flags&128?(t.lanes=n,t):(n=r!==null,e=e!==null&&e.memoizedState!==null,n&&(r=t.child,i=null,r.alternate!==null&&r.alternate.memoizedState!==null&&r.alternate.memoizedState.cachePool!==null&&(i=r.alternate.memoizedState.cachePool.pool),o=null,r.memoizedState!==null&&r.memoizedState.cachePool!==null&&(o=r.memoizedState.cachePool.pool),o!==i&&(r.flags|=2048)),n!==e&&n&&(t.child.flags|=8192),Fc(t,t.updateQueue),G(t),null);case 4:return de(),e===null&&Sd(t.stateNode.containerInfo),G(t),null;case 10:return qi(t.type),G(t),null;case 19:if(I(co),r=t.memoizedState,r===null)return G(t),null;if(i=(t.flags&128)!=0,o=r.rendering,o===null)if(i)Ic(r,!1);else{if(Ul!==0||e!==null&&e.flags&128)for(e=t.child;e!==null;){if(o=lo(e),o!==null){for(t.flags|=128,Ic(r,!1),e=o.updateQueue,t.updateQueue=e,Fc(t,e),t.subtreeFlags=0,e=n,n=t.child;n!==null;)di(n,e),n=n.sibling;return L(co,co.current&1|2),V&&Di(t,r.treeForkCount),t.child}e=e.sibling}r.tail!==null&&Te()>eu&&(t.flags|=128,i=!0,Ic(r,!1),t.lanes=4194304)}else{if(!i)if(e=lo(o),e!==null){if(t.flags|=128,i=!0,e=e.updateQueue,t.updateQueue=e,Fc(t,e),Ic(r,!0),r.tail===null&&r.tailMode===`hidden`&&!o.alternate&&!V)return G(t),null}else 2*Te()-r.renderingStartTime>eu&&n!==536870912&&(t.flags|=128,i=!0,Ic(r,!1),t.lanes=4194304);r.isBackwards?(o.sibling=t.child,t.child=o):(e=r.last,e===null?t.child=o:e.sibling=o,r.last=o)}return r.tail===null?(G(t),null):(e=r.tail,r.rendering=e,r.tail=e.sibling,r.renderingStartTime=Te(),e.sibling=null,n=co.current,L(co,i?n&1|2:n&1),V&&Di(t,r.treeForkCount),e);case 22:case 23:return so(t),eo(),r=t.memoizedState!==null,e===null?r&&(t.flags|=8192):e.memoizedState!==null!==r&&(t.flags|=8192),r?n&536870912&&!(t.flags&128)&&(G(t),t.subtreeFlags&6&&(t.flags|=8192)):G(t),n=t.updateQueue,n!==null&&Fc(t,n.retryQueue),n=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),r=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(r=t.memoizedState.cachePool.pool),r!==n&&(t.flags|=2048),e!==null&&I(ga),null;case 24:return n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),qi(aa),G(t),null;case 25:return null;case 30:return null}throw Error(a(156,t.tag))}function Rc(e,t){switch(Ai(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return qi(aa),de(),e=t.flags,e&65536&&!(e&128)?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return R(t),null;case 31:if(t.memoizedState!==null){if(so(t),t.alternate===null)throw Error(a(340));Bi()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(so(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(a(340));Bi()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return I(co),null;case 4:return de(),null;case 10:return qi(t.type),null;case 22:case 23:return so(t),eo(),e!==null&&I(ga),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return qi(aa),null;case 25:return null;default:return null}}function zc(e,t){switch(Ai(t),t.tag){case 3:qi(aa),de();break;case 26:case 27:case 5:R(t);break;case 4:de();break;case 31:t.memoizedState!==null&&so(t);break;case 13:so(t);break;case 19:I(co);break;case 10:qi(t.type);break;case 22:case 23:so(t),eo(),e!==null&&I(ga);break;case 24:qi(aa)}}function Bc(e,t){try{var n=t.updateQueue,r=n===null?null:n.lastEffect;if(r!==null){var i=r.next;n=i;do{if((n.tag&e)===e){r=void 0;var a=n.create,o=n.inst;r=a(),o.destroy=r}n=n.next}while(n!==i)}}catch(e){Wu(t,t.return,e)}}function Vc(e,t,n){try{var r=t.updateQueue,i=r===null?null:r.lastEffect;if(i!==null){var a=i.next;r=a;do{if((r.tag&e)===e){var o=r.inst,s=o.destroy;if(s!==void 0){o.destroy=void 0,i=t;var c=n,l=s;try{l()}catch(e){Wu(i,c,e)}}}r=r.next}while(r!==a)}}catch(e){Wu(t,t.return,e)}}function Hc(e){var t=e.updateQueue;if(t!==null){var n=e.stateNode;try{Ya(t,n)}catch(t){Wu(e,e.return,t)}}}function Uc(e,t,n){n.props=Ks(e.type,e.memoizedProps),n.state=e.memoizedState;try{n.componentWillUnmount()}catch(n){Wu(e,t,n)}}function Wc(e,t){try{var n=e.ref;if(n!==null){switch(e.tag){case 26:case 27:case 5:var r=e.stateNode;break;case 30:r=e.stateNode;break;default:r=e.stateNode}typeof n==`function`?e.refCleanup=n(r):n.current=r}}catch(n){Wu(e,t,n)}}function Gc(e,t){var n=e.ref,r=e.refCleanup;if(n!==null)if(typeof r==`function`)try{r()}catch(n){Wu(e,t,n)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof n==`function`)try{n(null)}catch(n){Wu(e,t,n)}else n.current=null}function Kc(e){var t=e.type,n=e.memoizedProps,r=e.stateNode;try{a:switch(t){case`button`:case`input`:case`select`:case`textarea`:n.autoFocus&&r.focus();break a;case`img`:n.src?r.src=n.src:n.srcSet&&(r.srcset=n.srcSet)}}catch(t){Wu(e,e.return,t)}}function qc(e,t,n){try{var r=e.stateNode;Id(r,e.type,n,t),r[st]=t}catch(t){Wu(e,e.return,t)}}function Jc(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&Qd(e.type)||e.tag===4}function Yc(e){a:for(;;){for(;e.sibling===null;){if(e.return===null||Jc(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&Qd(e.type)||e.flags&2||e.child===null||e.tag===4)continue a;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function Xc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?(n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n).insertBefore(e,t):(t=n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n,t.appendChild(e),n=n._reactRootContainer,n!=null||t.onclick!==null||(t.onclick=$t));else if(r!==4&&(r===27&&Qd(e.type)&&(n=e.stateNode,t=null),e=e.child,e!==null))for(Xc(e,t,n),e=e.sibling;e!==null;)Xc(e,t,n),e=e.sibling}function Zc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?n.insertBefore(e,t):n.appendChild(e);else if(r!==4&&(r===27&&Qd(e.type)&&(n=e.stateNode),e=e.child,e!==null))for(Zc(e,t,n),e=e.sibling;e!==null;)Zc(e,t,n),e=e.sibling}function Qc(e){var t=e.stateNode,n=e.memoizedProps;try{for(var r=e.type,i=t.attributes;i.length;)t.removeAttributeNode(i[0]);Fd(t,r,n),t[ot]=e,t[st]=n}catch(t){Wu(e,e.return,t)}}var $c=!1,el=!1,tl=!1,nl=typeof WeakSet==`function`?WeakSet:Set,rl=null;function il(e,t){if(e=e.containerInfo,zd=sp,e=Dr(e),Or(e)){if(`selectionStart`in e)var n={start:e.selectionStart,end:e.selectionEnd};else a:{n=(n=e.ownerDocument)&&n.defaultView||window;var r=n.getSelection&&n.getSelection();if(r&&r.rangeCount!==0){n=r.anchorNode;var i=r.anchorOffset,o=r.focusNode;r=r.focusOffset;try{n.nodeType,o.nodeType}catch{n=null;break a}var s=0,c=-1,l=-1,u=0,d=0,f=e,p=null;b:for(;;){for(var m;f!==n||i!==0&&f.nodeType!==3||(c=s+i),f!==o||r!==0&&f.nodeType!==3||(l=s+r),f.nodeType===3&&(s+=f.nodeValue.length),(m=f.firstChild)!==null;)p=f,f=m;for(;;){if(f===e)break b;if(p===n&&++u===i&&(c=s),p===o&&++d===r&&(l=s),(m=f.nextSibling)!==null)break;f=p,p=f.parentNode}f=m}n=c===-1||l===-1?null:{start:c,end:l}}else n=null}n||={start:0,end:0}}else n=null;for(Bd={focusedElem:e,selectionRange:n},sp=!1,rl=t;rl!==null;)if(t=rl,e=t.child,t.subtreeFlags&1028&&e!==null)e.return=t,rl=e;else for(;rl!==null;){switch(t=rl,o=t.alternate,e=t.flags,t.tag){case 0:if(e&4&&(e=t.updateQueue,e=e===null?null:e.events,e!==null))for(n=0;n<e.length;n++)i=e[n],i.ref.impl=i.nextImpl;break;case 11:case 15:break;case 1:if(e&1024&&o!==null){e=void 0,n=t,i=o.memoizedProps,o=o.memoizedState,r=n.stateNode;try{var h=Ks(n.type,i);e=r.getSnapshotBeforeUpdate(h,o),r.__reactInternalSnapshotBeforeUpdate=e}catch(e){Wu(n,n.return,e)}}break;case 3:if(e&1024){if(e=t.stateNode.containerInfo,n=e.nodeType,n===9)tf(e);else if(n===1)switch(e.nodeName){case`HEAD`:case`HTML`:case`BODY`:tf(e);break;default:e.textContent=``}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if(e&1024)throw Error(a(163))}if(e=t.sibling,e!==null){e.return=t.return,rl=e;break}rl=t.return}}function al(e,t,n){var r=n.flags;switch(n.tag){case 0:case 11:case 15:vl(e,n),r&4&&Bc(5,n);break;case 1:if(vl(e,n),r&4)if(e=n.stateNode,t===null)try{e.componentDidMount()}catch(e){Wu(n,n.return,e)}else{var i=Ks(n.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(i,t,e.__reactInternalSnapshotBeforeUpdate)}catch(e){Wu(n,n.return,e)}}r&64&&Hc(n),r&512&&Wc(n,n.return);break;case 3:if(vl(e,n),r&64&&(e=n.updateQueue,e!==null)){if(t=null,n.child!==null)switch(n.child.tag){case 27:case 5:t=n.child.stateNode;break;case 1:t=n.child.stateNode}try{Ya(e,t)}catch(e){Wu(n,n.return,e)}}break;case 27:t===null&&r&4&&Qc(n);case 26:case 5:vl(e,n),t===null&&r&4&&Kc(n),r&512&&Wc(n,n.return);break;case 12:vl(e,n);break;case 31:vl(e,n),r&4&&ul(e,n);break;case 13:vl(e,n),r&4&&q(e,n),r&64&&(e=n.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(n=Ju.bind(null,n),cf(e,n))));break;case 22:if(r=n.memoizedState!==null||$c,!r){t=t!==null&&t.memoizedState!==null||el,i=$c;var a=el;$c=r,(el=t)&&!a?bl(e,n,(n.subtreeFlags&8772)!=0):vl(e,n),$c=i,el=a}break;case 30:break;default:vl(e,n)}}function K(e){var t=e.alternate;t!==null&&(e.alternate=null,K(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&mt(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var ol=null,sl=!1;function cl(e,t,n){for(n=n.child;n!==null;)ll(e,t,n),n=n.sibling}function ll(e,t,n){if(Fe&&typeof Fe.onCommitFiberUnmount==`function`)try{Fe.onCommitFiberUnmount(Pe,n)}catch{}switch(n.tag){case 26:el||Gc(n,t),cl(e,t,n),n.memoizedState?n.memoizedState.count--:n.stateNode&&(n=n.stateNode,n.parentNode.removeChild(n));break;case 27:el||Gc(n,t);var r=ol,i=sl;Qd(n.type)&&(ol=n.stateNode,sl=!1),cl(e,t,n),mf(n.stateNode),ol=r,sl=i;break;case 5:el||Gc(n,t);case 6:if(r=ol,i=sl,ol=null,cl(e,t,n),ol=r,sl=i,ol!==null)if(sl)try{(ol.nodeType===9?ol.body:ol.nodeName===`HTML`?ol.ownerDocument.body:ol).removeChild(n.stateNode)}catch(e){Wu(n,t,e)}else try{ol.removeChild(n.stateNode)}catch(e){Wu(n,t,e)}break;case 18:ol!==null&&(sl?(e=ol,$d(e.nodeType===9?e.body:e.nodeName===`HTML`?e.ownerDocument.body:e,n.stateNode),Np(e)):$d(ol,n.stateNode));break;case 4:r=ol,i=sl,ol=n.stateNode.containerInfo,sl=!0,cl(e,t,n),ol=r,sl=i;break;case 0:case 11:case 14:case 15:Vc(2,n,t),el||Vc(4,n,t),cl(e,t,n);break;case 1:el||(Gc(n,t),r=n.stateNode,typeof r.componentWillUnmount==`function`&&Uc(n,t,r)),cl(e,t,n);break;case 21:cl(e,t,n);break;case 22:el=(r=el)||n.memoizedState!==null,cl(e,t,n),el=r;break;default:cl(e,t,n)}}function ul(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{Np(e)}catch(e){Wu(t,t.return,e)}}}function q(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{Np(e)}catch(e){Wu(t,t.return,e)}}function dl(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new nl),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new nl),t;default:throw Error(a(435,e.tag))}}function fl(e,t){var n=dl(e);t.forEach(function(t){if(!n.has(t)){n.add(t);var r=Yu.bind(null,e,t);t.then(r,r)}})}function pl(e,t){var n=t.deletions;if(n!==null)for(var r=0;r<n.length;r++){var i=n[r],o=e,s=t,c=s;a:for(;c!==null;){switch(c.tag){case 27:if(Qd(c.type)){ol=c.stateNode,sl=!1;break a}break;case 5:ol=c.stateNode,sl=!1;break a;case 3:case 4:ol=c.stateNode.containerInfo,sl=!0;break a}c=c.return}if(ol===null)throw Error(a(160));ll(o,s,i),ol=null,sl=!1,o=i.alternate,o!==null&&(o.return=null),i.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)hl(t,e),t=t.sibling}var ml=null;function hl(e,t){var n=e.alternate,r=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:pl(t,e),gl(e),r&4&&(Vc(3,e,e.return),Bc(3,e),Vc(5,e,e.return));break;case 1:pl(t,e),gl(e),r&512&&(el||n===null||Gc(n,n.return)),r&64&&$c&&(e=e.updateQueue,e!==null&&(r=e.callbacks,r!==null&&(n=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=n===null?r:n.concat(r))));break;case 26:var i=ml;if(pl(t,e),gl(e),r&512&&(el||n===null||Gc(n,n.return)),r&4){var o=n===null?null:n.memoizedState;if(r=e.memoizedState,n===null)if(r===null)if(e.stateNode===null){a:{r=e.type,n=e.memoizedProps,i=i.ownerDocument||i;b:switch(r){case`title`:o=i.getElementsByTagName(`title`)[0],(!o||o[pt]||o[ot]||o.namespaceURI===`http://www.w3.org/2000/svg`||o.hasAttribute(`itemprop`))&&(o=i.createElement(r),i.head.insertBefore(o,i.querySelector(`head > title`))),Fd(o,r,n),o[ot]=e,yt(o),r=o;break a;case`link`:var s=Hf(`link`,`href`,i).get(r+(n.href||``));if(s){for(var c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`href`)===(n.href==null||n.href===``?null:n.href)&&o.getAttribute(`rel`)===(n.rel==null?null:n.rel)&&o.getAttribute(`title`)===(n.title==null?null:n.title)&&o.getAttribute(`crossorigin`)===(n.crossOrigin==null?null:n.crossOrigin)){s.splice(c,1);break b}}o=i.createElement(r),Fd(o,r,n),i.head.appendChild(o);break;case`meta`:if(s=Hf(`meta`,`content`,i).get(r+(n.content||``))){for(c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`content`)===(n.content==null?null:``+n.content)&&o.getAttribute(`name`)===(n.name==null?null:n.name)&&o.getAttribute(`property`)===(n.property==null?null:n.property)&&o.getAttribute(`http-equiv`)===(n.httpEquiv==null?null:n.httpEquiv)&&o.getAttribute(`charset`)===(n.charSet==null?null:n.charSet)){s.splice(c,1);break b}}o=i.createElement(r),Fd(o,r,n),i.head.appendChild(o);break;default:throw Error(a(468,r))}o[ot]=e,yt(o),r=o}e.stateNode=r}else Uf(i,e.type,e.stateNode);else e.stateNode=Lf(i,r,e.memoizedProps);else o===r?r===null&&e.stateNode!==null&&qc(e,e.memoizedProps,n.memoizedProps):(o===null?n.stateNode!==null&&(n=n.stateNode,n.parentNode.removeChild(n)):o.count--,r===null?Uf(i,e.type,e.stateNode):Lf(i,r,e.memoizedProps))}break;case 27:pl(t,e),gl(e),r&512&&(el||n===null||Gc(n,n.return)),n!==null&&r&4&&qc(e,e.memoizedProps,n.memoizedProps);break;case 5:if(pl(t,e),gl(e),r&512&&(el||n===null||Gc(n,n.return)),e.flags&32){i=e.stateNode;try{Gt(i,``)}catch(t){Wu(e,e.return,t)}}r&4&&e.stateNode!=null&&(i=e.memoizedProps,qc(e,i,n===null?i:n.memoizedProps)),r&1024&&(tl=!0);break;case 6:if(pl(t,e),gl(e),r&4){if(e.stateNode===null)throw Error(a(162));r=e.memoizedProps,n=e.stateNode;try{n.nodeValue=r}catch(t){Wu(e,e.return,t)}}break;case 3:if(Vf=null,i=ml,ml=_f(t.containerInfo),pl(t,e),ml=i,gl(e),r&4&&n!==null&&n.memoizedState.isDehydrated)try{Np(t.containerInfo)}catch(t){Wu(e,e.return,t)}tl&&(tl=!1,_l(e));break;case 4:r=ml,ml=_f(e.stateNode.containerInfo),pl(t,e),gl(e),ml=r;break;case 12:pl(t,e),gl(e);break;case 31:pl(t,e),gl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,fl(e,r)));break;case 13:pl(t,e),gl(e),e.child.flags&8192&&e.memoizedState!==null!=(n!==null&&n.memoizedState!==null)&&(Ql=Te()),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,fl(e,r)));break;case 22:i=e.memoizedState!==null;var l=n!==null&&n.memoizedState!==null,u=$c,d=el;if($c=u||i,el=d||l,pl(t,e),el=d,$c=u,gl(e),r&8192)a:for(t=e.stateNode,t._visibility=i?t._visibility&-2:t._visibility|1,i&&(n===null||l||$c||el||yl(e)),n=null,t=e;;){if(t.tag===5||t.tag===26){if(n===null){l=n=t;try{if(o=l.stateNode,i)s=o.style,typeof s.setProperty==`function`?s.setProperty(`display`,`none`,`important`):s.display=`none`;else{c=l.stateNode;var f=l.memoizedProps.style,p=f!=null&&f.hasOwnProperty(`display`)?f.display:null;c.style.display=p==null||typeof p==`boolean`?``:(``+p).trim()}}catch(e){Wu(l,l.return,e)}}}else if(t.tag===6){if(n===null){l=t;try{l.stateNode.nodeValue=i?``:l.memoizedProps}catch(e){Wu(l,l.return,e)}}}else if(t.tag===18){if(n===null){l=t;try{var m=l.stateNode;i?ef(m,!0):ef(l.stateNode,!1)}catch(e){Wu(l,l.return,e)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break a;for(;t.sibling===null;){if(t.return===null||t.return===e)break a;n===t&&(n=null),t=t.return}n===t&&(n=null),t.sibling.return=t.return,t=t.sibling}r&4&&(r=e.updateQueue,r!==null&&(n=r.retryQueue,n!==null&&(r.retryQueue=null,fl(e,n))));break;case 19:pl(t,e),gl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,fl(e,r)));break;case 30:break;case 21:break;default:pl(t,e),gl(e)}}function gl(e){var t=e.flags;if(t&2){try{for(var n,r=e.return;r!==null;){if(Jc(r)){n=r;break}r=r.return}if(n==null)throw Error(a(160));switch(n.tag){case 27:var i=n.stateNode;Zc(e,Yc(e),i);break;case 5:var o=n.stateNode;n.flags&32&&(Gt(o,``),n.flags&=-33),Zc(e,Yc(e),o);break;case 3:case 4:var s=n.stateNode.containerInfo;Xc(e,Yc(e),s);break;default:throw Error(a(161))}}catch(t){Wu(e,e.return,t)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function _l(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;_l(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function vl(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)al(e,t.alternate,t),t=t.sibling}function yl(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:Vc(4,t,t.return),yl(t);break;case 1:Gc(t,t.return);var n=t.stateNode;typeof n.componentWillUnmount==`function`&&Uc(t,t.return,n),yl(t);break;case 27:mf(t.stateNode);case 26:case 5:Gc(t,t.return),yl(t);break;case 22:t.memoizedState===null&&yl(t);break;case 30:yl(t);break;default:yl(t)}e=e.sibling}}function bl(e,t,n){for(n&&=(t.subtreeFlags&8772)!=0,t=t.child;t!==null;){var r=t.alternate,i=e,a=t,o=a.flags;switch(a.tag){case 0:case 11:case 15:bl(i,a,n),Bc(4,a);break;case 1:if(bl(i,a,n),r=a,i=r.stateNode,typeof i.componentDidMount==`function`)try{i.componentDidMount()}catch(e){Wu(r,r.return,e)}if(r=a,i=r.updateQueue,i!==null){var s=r.stateNode;try{var c=i.shared.hiddenCallbacks;if(c!==null)for(i.shared.hiddenCallbacks=null,i=0;i<c.length;i++)Ja(c[i],s)}catch(e){Wu(r,r.return,e)}}n&&o&64&&Hc(a),Wc(a,a.return);break;case 27:Qc(a);case 26:case 5:bl(i,a,n),n&&r===null&&o&4&&Kc(a),Wc(a,a.return);break;case 12:bl(i,a,n);break;case 31:bl(i,a,n),n&&o&4&&ul(i,a);break;case 13:bl(i,a,n),n&&o&4&&q(i,a);break;case 22:a.memoizedState===null&&bl(i,a,n),Wc(a,a.return);break;case 30:break;default:bl(i,a,n)}t=t.sibling}}function xl(e,t){var n=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==n&&(e!=null&&e.refCount++,n!=null&&sa(n))}function Sl(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&sa(e))}function Cl(e,t,n,r){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)J(e,t,n,r),t=t.sibling}function J(e,t,n,r){var i=t.flags;switch(t.tag){case 0:case 11:case 15:Cl(e,t,n,r),i&2048&&Bc(9,t);break;case 1:Cl(e,t,n,r);break;case 3:Cl(e,t,n,r),i&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&sa(e)));break;case 12:if(i&2048){Cl(e,t,n,r),e=t.stateNode;try{var a=t.memoizedProps,o=a.id,s=a.onPostCommit;typeof s==`function`&&s(o,t.alternate===null?`mount`:`update`,e.passiveEffectDuration,-0)}catch(e){Wu(t,t.return,e)}}else Cl(e,t,n,r);break;case 31:Cl(e,t,n,r);break;case 13:Cl(e,t,n,r);break;case 23:break;case 22:a=t.stateNode,o=t.alternate,t.memoizedState===null?a._visibility&2?Cl(e,t,n,r):(a._visibility|=2,wl(e,t,n,r,(t.subtreeFlags&10256)!=0||!1)):a._visibility&2?Cl(e,t,n,r):Tl(e,t),i&2048&&xl(o,t);break;case 24:Cl(e,t,n,r),i&2048&&Sl(t.alternate,t);break;default:Cl(e,t,n,r)}}function wl(e,t,n,r,i){for(i&&=(t.subtreeFlags&10256)!=0||!1,t=t.child;t!==null;){var a=e,o=t,s=n,c=r,l=o.flags;switch(o.tag){case 0:case 11:case 15:wl(a,o,s,c,i),Bc(8,o);break;case 23:break;case 22:var u=o.stateNode;o.memoizedState===null?(u._visibility|=2,wl(a,o,s,c,i)):u._visibility&2?wl(a,o,s,c,i):Tl(a,o),i&&l&2048&&xl(o.alternate,o);break;case 24:wl(a,o,s,c,i),i&&l&2048&&Sl(o.alternate,o);break;default:wl(a,o,s,c,i)}t=t.sibling}}function Tl(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var n=e,r=t,i=r.flags;switch(r.tag){case 22:Tl(n,r),i&2048&&xl(r.alternate,r);break;case 24:Tl(n,r),i&2048&&Sl(r.alternate,r);break;default:Tl(n,r)}t=t.sibling}}var El=8192;function Dl(e,t,n){if(e.subtreeFlags&El)for(e=e.child;e!==null;)Ol(e,t,n),e=e.sibling}function Ol(e,t,n){switch(e.tag){case 26:Dl(e,t,n),e.flags&El&&e.memoizedState!==null&&Gf(n,ml,e.memoizedState,e.memoizedProps);break;case 5:Dl(e,t,n);break;case 3:case 4:var r=ml;ml=_f(e.stateNode.containerInfo),Dl(e,t,n),ml=r;break;case 22:e.memoizedState===null&&(r=e.alternate,r!==null&&r.memoizedState!==null?(r=El,El=16777216,Dl(e,t,n),El=r):Dl(e,t,n));break;default:Dl(e,t,n)}}function kl(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function Al(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Nl(r,e)}kl(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)jl(e),e=e.sibling}function jl(e){switch(e.tag){case 0:case 11:case 15:Al(e),e.flags&2048&&Vc(9,e,e.return);break;case 3:Al(e);break;case 12:Al(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,Ml(e)):Al(e);break;default:Al(e)}}function Ml(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];rl=r,Nl(r,e)}kl(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:Vc(8,t,t.return),Ml(t);break;case 22:n=t.stateNode,n._visibility&2&&(n._visibility&=-3,Ml(t));break;default:Ml(t)}e=e.sibling}}function Nl(e,t){for(;rl!==null;){var n=rl;switch(n.tag){case 0:case 11:case 15:Vc(8,n,t);break;case 23:case 22:if(n.memoizedState!==null&&n.memoizedState.cachePool!==null){var r=n.memoizedState.cachePool.pool;r!=null&&r.refCount++}break;case 24:sa(n.memoizedState.cache)}if(r=n.child,r!==null)r.return=n,rl=r;else a:for(n=e;rl!==null;){r=rl;var i=r.sibling,a=r.return;if(K(r),r===n){rl=null;break a}if(i!==null){i.return=a,rl=i;break a}rl=a}}}var Pl={getCacheForType:function(e){var t=$i(aa),n=t.data.get(e);return n===void 0&&(n=e(),t.data.set(e,n)),n},cacheSignal:function(){return $i(aa).controller.signal}},Fl=typeof WeakMap==`function`?WeakMap:Map,Il=0,Ll=null,Y=null,X=0,Rl=0,zl=null,Bl=!1,Vl=!1,Z=!1,Hl=0,Ul=0,Wl=0,Gl=0,Kl=0,ql=0,Jl=0,Yl=null,Xl=null,Zl=!1,Ql=0,$l=0,eu=1/0,tu=null,nu=null,ru=0,iu=null,au=null,ou=0,su=0,cu=null,lu=null,uu=0,du=null;function fu(){return Il&2&&X!==0?X&-X:N.T===null?rt():dd()}function pu(){if(ql===0)if(!(X&536870912)||V){var e=He;He<<=1,!(He&3932160)&&(He=262144),ql=e}else ql=536870912;return e=to.current,e!==null&&(e.flags|=32),ql}function mu(e,t,n){(e===Ll&&(Rl===2||Rl===9)||e.cancelPendingCommit!==null)&&(xu(e,0),vu(e,X,ql,!1)),Xe(e,n),(!(Il&2)||e!==Ll)&&(e===Ll&&(!(Il&2)&&(Gl|=n),Ul===4&&vu(e,X,ql,!1)),rd(e))}function hu(e,t,n){if(Il&6)throw Error(a(327));var r=!n&&(t&127)==0&&(t&e.expiredLanes)===0||Ke(e,t),i=r?ku(e,t):Du(e,t,!0),o=r;do{if(i===0){Vl&&!r&&vu(e,t,0,!1);break}else{if(n=e.current.alternate,o&&!_u(n)){i=Du(e,t,!1),o=!1;continue}if(i===2){if(o=t,e.errorRecoveryDisabledLanes&o)var s=0;else s=e.pendingLanes&-536870913,s=s===0?s&536870912?536870912:0:s;if(s!==0){t=s;a:{var c=e;i=Yl;var l=c.current.memoizedState.isDehydrated;if(l&&(xu(c,s).flags|=256),s=Du(c,s,!1),s!==2){if(Z&&!l){c.errorRecoveryDisabledLanes|=o,Gl|=o,i=4;break a}o=Xl,Xl=i,o!==null&&(Xl===null?Xl=o:Xl.push.apply(Xl,o))}i=s}if(o=!1,i!==2)continue}}if(i===1){xu(e,0),vu(e,t,0,!0);break}a:{switch(r=e,o=i,o){case 0:case 1:throw Error(a(345));case 4:if((t&4194048)!==t)break;case 6:vu(r,t,ql,!Bl);break a;case 2:Xl=null;break;case 3:case 5:break;default:throw Error(a(329))}if((t&62914560)===t&&(i=Ql+300-Te(),10<i)){if(vu(r,t,ql,!Bl),Ge(r,0,!0)!==0)break a;ou=t,r.timeoutHandle=qd(gu.bind(null,r,n,Xl,tu,Zl,t,ql,Gl,Jl,Bl,o,`Throttled`,-0,0),i);break a}gu(r,n,Xl,tu,Zl,t,ql,Gl,Jl,Bl,o,null,-0,0)}}break}while(1);rd(e)}function gu(e,t,n,r,i,a,o,s,c,l,u,d,f,p){if(e.timeoutHandle=-1,d=t.subtreeFlags,d&8192||(d&16785408)==16785408){d={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:$t},Ol(t,a,d);var m=(a&62914560)===a?Ql-Te():(a&4194048)===a?$l-Te():0;if(m=qf(d,m),m!==null){ou=a,e.cancelPendingCommit=m(Iu.bind(null,e,t,a,n,r,i,o,s,c,u,d,null,f,p)),vu(e,a,o,!l);return}}Iu(e,t,a,n,r,i,o,s,c)}function _u(e){for(var t=e;;){var n=t.tag;if((n===0||n===11||n===15)&&t.flags&16384&&(n=t.updateQueue,n!==null&&(n=n.stores,n!==null)))for(var r=0;r<n.length;r++){var i=n[r],a=i.getSnapshot;i=i.value;try{if(!Sr(a(),i))return!1}catch{return!1}}if(n=t.child,t.subtreeFlags&16384&&n!==null)n.return=t,t=n;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function vu(e,t,n,r){t&=~Kl,t&=~Gl,e.suspendedLanes|=t,e.pingedLanes&=~t,r&&(e.warmLanes|=t),r=e.expirationTimes;for(var i=t;0<i;){var a=31-Le(i),o=1<<a;r[a]=-1,i&=~o}n!==0&&Qe(e,n,t)}function yu(){return Il&6?!0:(id(0,!1),!1)}function bu(){if(Y!==null){if(Rl===0)var e=Y.return;else e=Y,Gi=Wi=null,ko(e),Aa=null,ja=0,e=Y;for(;e!==null;)zc(e.alternate,e),e=e.return;Y=null}}function xu(e,t){var n=e.timeoutHandle;n!==-1&&(e.timeoutHandle=-1,Jd(n)),n=e.cancelPendingCommit,n!==null&&(e.cancelPendingCommit=null,n()),ou=0,bu(),Ll=e,Y=n=ui(e.current,null),X=t,Rl=0,zl=null,Bl=!1,Vl=Ke(e,t),Z=!1,Jl=ql=Kl=Gl=Wl=Ul=0,Xl=Yl=null,Zl=!1,t&8&&(t|=t&32);var r=e.entangledLanes;if(r!==0)for(e=e.entanglements,r&=t;0<r;){var i=31-Le(r),a=1<<i;t|=e[i],r&=~a}return Hl=t,ei(),n}function Su(e,t){H=null,N.H=Rs,t===ba||t===Sa?(t=Oa(),Rl=3):t===xa?(t=Oa(),Rl=4):Rl=t===nc?8:typeof t==`object`&&t&&typeof t.then==`function`?6:1,zl=t,Y===null&&(Ul=1,Xs(e,vi(t,e.current)))}function Cu(){var e=to.current;return e===null?!0:(X&4194048)===X?no===null:(X&62914560)===X||X&536870912?e===no:!1}function wu(){var e=N.H;return N.H=Rs,e===null?Rs:e}function Tu(){var e=N.A;return N.A=Pl,e}function Eu(){Ul=4,Bl||(X&4194048)!==X&&to.current!==null||(Vl=!0),!(Wl&134217727)&&!(Gl&134217727)||Ll===null||vu(Ll,X,ql,!1)}function Du(e,t,n){var r=Il;Il|=2;var i=wu(),a=Tu();(Ll!==e||X!==t)&&(tu=null,xu(e,t)),t=!1;var o=Ul;a:do try{if(Rl!==0&&Y!==null){var s=Y,c=zl;switch(Rl){case 8:bu(),o=6;break a;case 3:case 2:case 9:case 6:to.current===null&&(t=!0);var l=Rl;if(Rl=0,zl=null,Nu(e,s,c,l),n&&Vl){o=0;break a}break;default:l=Rl,Rl=0,zl=null,Nu(e,s,c,l)}}Ou(),o=Ul;break}catch(t){Su(e,t)}while(1);return t&&e.shellSuspendCounter++,Gi=Wi=null,Il=r,N.H=i,N.A=a,Y===null&&(Ll=null,X=0,ei()),o}function Ou(){for(;Y!==null;)ju(Y)}function ku(e,t){var n=Il;Il|=2;var r=wu(),i=Tu();Ll!==e||X!==t?(tu=null,eu=Te()+500,xu(e,t)):Vl=Ke(e,t);a:do try{if(Rl!==0&&Y!==null){t=Y;var o=zl;b:switch(Rl){case 1:Rl=0,zl=null,Nu(e,t,o,1);break;case 2:case 9:if(wa(o)){Rl=0,zl=null,Mu(t);break}t=function(){Rl!==2&&Rl!==9||Ll!==e||(Rl=7),rd(e)},o.then(t,t);break a;case 3:Rl=7;break a;case 4:Rl=5;break a;case 7:wa(o)?(Rl=0,zl=null,Mu(t)):(Rl=0,zl=null,Nu(e,t,o,7));break;case 5:var s=null;switch(Y.tag){case 26:s=Y.memoizedState;case 5:case 27:var c=Y;if(s?Wf(s):c.stateNode.complete){Rl=0,zl=null;var l=c.sibling;if(l!==null)Y=l;else{var u=c.return;u===null?Y=null:(Y=u,Pu(u))}break b}}Rl=0,zl=null,Nu(e,t,o,5);break;case 6:Rl=0,zl=null,Nu(e,t,o,6);break;case 8:bu(),Ul=6;break a;default:throw Error(a(462))}}Au();break}catch(t){Su(e,t)}while(1);return Gi=Wi=null,N.H=r,N.A=i,Il=n,Y===null?(Ll=null,X=0,ei(),Ul):0}function Au(){for(;Y!==null&&!Ce();)ju(Y)}function ju(e){var t=jc(e.alternate,e,Hl);e.memoizedProps=e.pendingProps,t===null?Pu(e):Y=t}function Mu(e){var t=e,n=t.alternate;switch(t.tag){case 15:case 0:t=hc(n,t,t.pendingProps,t.type,void 0,X);break;case 11:t=hc(n,t,t.pendingProps,t.type.render,t.ref,X);break;case 5:ko(t);default:zc(n,t),t=Y=di(t,Hl),t=jc(n,t,Hl)}e.memoizedProps=e.pendingProps,t===null?Pu(e):Y=t}function Nu(e,t,n,r){Gi=Wi=null,ko(t),Aa=null,ja=0;var i=t.return;try{if(tc(e,i,t,n,X)){Ul=1,Xs(e,vi(n,e.current)),Y=null;return}}catch(t){if(i!==null)throw Y=i,t;Ul=1,Xs(e,vi(n,e.current)),Y=null;return}t.flags&32768?(V||r===1?e=!0:Vl||X&536870912?e=!1:(Bl=e=!0,(r===2||r===9||r===3||r===6)&&(r=to.current,r!==null&&r.tag===13&&(r.flags|=16384))),Fu(t,e)):Pu(t)}function Pu(e){var t=e;do{if(t.flags&32768){Fu(t,Bl);return}e=t.return;var n=Lc(t.alternate,t,Hl);if(n!==null){Y=n;return}if(t=t.sibling,t!==null){Y=t;return}Y=t=e}while(t!==null);Ul===0&&(Ul=5)}function Fu(e,t){do{var n=Rc(e.alternate,e);if(n!==null){n.flags&=32767,Y=n;return}if(n=e.return,n!==null&&(n.flags|=32768,n.subtreeFlags=0,n.deletions=null),!t&&(e=e.sibling,e!==null)){Y=e;return}Y=e=n}while(e!==null);Ul=6,Y=null}function Iu(e,t,n,r,i,o,s,c,l){e.cancelPendingCommit=null;do Vu();while(ru!==0);if(Il&6)throw Error(a(327));if(t!==null){if(t===e.current)throw Error(a(177));if(o=t.lanes|t.childLanes,o|=$r,Ze(e,n,o,s,c,l),e===Ll&&(Y=Ll=null,X=0),au=t,iu=e,ou=n,su=o,cu=i,lu=r,t.subtreeFlags&10256||t.flags&10256?(e.callbackNode=null,e.callbackPriority=0,Xu(ke,function(){return Hu(),null})):(e.callbackNode=null,e.callbackPriority=0),r=(t.flags&13878)!=0,t.subtreeFlags&13878||r){r=N.T,N.T=null,i=P.p,P.p=2,s=Il,Il|=4;try{il(e,t,n)}finally{Il=s,P.p=i,N.T=r}}ru=1,Lu(),Ru(),zu()}}function Lu(){if(ru===1){ru=0;var e=iu,t=au,n=(t.flags&13878)!=0;if(t.subtreeFlags&13878||n){n=N.T,N.T=null;var r=P.p;P.p=2;var i=Il;Il|=4;try{hl(t,e);var a=Bd,o=Dr(e.containerInfo),s=a.focusedElem,c=a.selectionRange;if(o!==s&&s&&s.ownerDocument&&Er(s.ownerDocument.documentElement,s)){if(c!==null&&Or(s)){var l=c.start,u=c.end;if(u===void 0&&(u=l),`selectionStart`in s)s.selectionStart=l,s.selectionEnd=Math.min(u,s.value.length);else{var d=s.ownerDocument||document,f=d&&d.defaultView||window;if(f.getSelection){var p=f.getSelection(),m=s.textContent.length,h=Math.min(c.start,m),g=c.end===void 0?h:Math.min(c.end,m);!p.extend&&h>g&&(o=g,g=h,h=o);var _=Tr(s,h),v=Tr(s,g);if(_&&v&&(p.rangeCount!==1||p.anchorNode!==_.node||p.anchorOffset!==_.offset||p.focusNode!==v.node||p.focusOffset!==v.offset)){var y=d.createRange();y.setStart(_.node,_.offset),p.removeAllRanges(),h>g?(p.addRange(y),p.extend(v.node,v.offset)):(y.setEnd(v.node,v.offset),p.addRange(y))}}}}for(d=[],p=s;p=p.parentNode;)p.nodeType===1&&d.push({element:p,left:p.scrollLeft,top:p.scrollTop});for(typeof s.focus==`function`&&s.focus(),s=0;s<d.length;s++){var b=d[s];b.element.scrollLeft=b.left,b.element.scrollTop=b.top}}sp=!!zd,Bd=zd=null}finally{Il=i,P.p=r,N.T=n}}e.current=t,ru=2}}function Ru(){if(ru===2){ru=0;var e=iu,t=au,n=(t.flags&8772)!=0;if(t.subtreeFlags&8772||n){n=N.T,N.T=null;var r=P.p;P.p=2;var i=Il;Il|=4;try{al(e,t.alternate,t)}finally{Il=i,P.p=r,N.T=n}}ru=3}}function zu(){if(ru===4||ru===3){ru=0,we();var e=iu,t=au,n=ou,r=lu;t.subtreeFlags&10256||t.flags&10256?ru=5:(ru=0,au=iu=null,Bu(e,e.pendingLanes));var i=e.pendingLanes;if(i===0&&(nu=null),nt(n),t=t.stateNode,Fe&&typeof Fe.onCommitFiberRoot==`function`)try{Fe.onCommitFiberRoot(Pe,t,void 0,(t.current.flags&128)==128)}catch{}if(r!==null){t=N.T,i=P.p,P.p=2,N.T=null;try{for(var a=e.onRecoverableError,o=0;o<r.length;o++){var s=r[o];a(s.value,{componentStack:s.stack})}}finally{N.T=t,P.p=i}}ou&3&&Vu(),rd(e),i=e.pendingLanes,n&261930&&i&42?e===du?uu++:(uu=0,du=e):uu=0,id(0,!1)}}function Bu(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,sa(t)))}function Vu(){return Lu(),Ru(),zu(),Hu()}function Hu(){if(ru!==5)return!1;var e=iu,t=su;su=0;var n=nt(ou),r=N.T,i=P.p;try{P.p=32>n?32:n,N.T=null,n=cu,cu=null;var o=iu,s=ou;if(ru=0,au=iu=null,ou=0,Il&6)throw Error(a(331));var c=Il;if(Il|=4,jl(o.current),J(o,o.current,s,n),Il=c,id(0,!1),Fe&&typeof Fe.onPostCommitFiberRoot==`function`)try{Fe.onPostCommitFiberRoot(Pe,o)}catch{}return!0}finally{P.p=i,N.T=r,Bu(e,t)}}function Uu(e,t,n){t=vi(n,t),t=Qs(e.stateNode,t,2),e=Ha(e,t,2),e!==null&&(Xe(e,2),rd(e))}function Wu(e,t,n){if(e.tag===3)Uu(e,e,n);else for(;t!==null;){if(t.tag===3){Uu(t,e,n);break}else if(t.tag===1){var r=t.stateNode;if(typeof t.type.getDerivedStateFromError==`function`||typeof r.componentDidCatch==`function`&&(nu===null||!nu.has(r))){e=vi(n,e),n=$s(2),r=Ha(t,n,2),r!==null&&(ec(n,r,t,e),Xe(r,2),rd(r));break}}t=t.return}}function Gu(e,t,n){var r=e.pingCache;if(r===null){r=e.pingCache=new Fl;var i=new Set;r.set(t,i)}else i=r.get(t),i===void 0&&(i=new Set,r.set(t,i));i.has(n)||(Z=!0,i.add(n),e=Ku.bind(null,e,t,n),t.then(e,e))}function Ku(e,t,n){var r=e.pingCache;r!==null&&r.delete(t),e.pingedLanes|=e.suspendedLanes&n,e.warmLanes&=~n,Ll===e&&(X&n)===n&&(Ul===4||Ul===3&&(X&62914560)===X&&300>Te()-Ql?!(Il&2)&&xu(e,0):Kl|=n,Jl===X&&(Jl=0)),rd(e)}function qu(e,t){t===0&&(t=Je()),e=ri(e,t),e!==null&&(Xe(e,t),rd(e))}function Ju(e){var t=e.memoizedState,n=0;t!==null&&(n=t.retryLane),qu(e,n)}function Yu(e,t){var n=0;switch(e.tag){case 31:case 13:var r=e.stateNode,i=e.memoizedState;i!==null&&(n=i.retryLane);break;case 19:r=e.stateNode;break;case 22:r=e.stateNode._retryCache;break;default:throw Error(a(314))}r!==null&&r.delete(t),qu(e,n)}function Xu(e,t){return xe(e,t)}var Zu=null,Qu=null,$u=!1,ed=!1,td=!1,nd=0;function rd(e){e!==Qu&&e.next===null&&(Qu===null?Zu=Qu=e:Qu=Qu.next=e),ed=!0,$u||($u=!0,ud())}function id(e,t){if(!td&&ed){td=!0;do for(var n=!1,r=Zu;r!==null;){if(!t)if(e!==0){var i=r.pendingLanes;if(i===0)var a=0;else{var o=r.suspendedLanes,s=r.pingedLanes;a=(1<<31-Le(42|e)+1)-1,a&=i&~(o&~s),a=a&201326741?a&201326741|1:a?a|2:0}a!==0&&(n=!0,ld(r,a))}else a=X,a=Ge(r,r===Ll?a:0,r.cancelPendingCommit!==null||r.timeoutHandle!==-1),!(a&3)||Ke(r,a)||(n=!0,ld(r,a));r=r.next}while(n);td=!1}}function ad(){od()}function od(){ed=$u=!1;var e=0;nd!==0&&Kd()&&(e=nd);for(var t=Te(),n=null,r=Zu;r!==null;){var i=r.next,a=sd(r,t);a===0?(r.next=null,n===null?Zu=i:n.next=i,i===null&&(Qu=n)):(n=r,(e!==0||a&3)&&(ed=!0)),r=i}ru!==0&&ru!==5||id(e,!1),nd!==0&&(nd=0)}function sd(e,t){for(var n=e.suspendedLanes,r=e.pingedLanes,i=e.expirationTimes,a=e.pendingLanes&-62914561;0<a;){var o=31-Le(a),s=1<<o,c=i[o];c===-1?((s&n)===0||(s&r)!==0)&&(i[o]=qe(s,t)):c<=t&&(e.expiredLanes|=s),a&=~s}if(t=Ll,n=X,n=Ge(e,e===t?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r=e.callbackNode,n===0||e===t&&(Rl===2||Rl===9)||e.cancelPendingCommit!==null)return r!==null&&r!==null&&Se(r),e.callbackNode=null,e.callbackPriority=0;if(!(n&3)||Ke(e,n)){if(t=n&-n,t===e.callbackPriority)return t;switch(r!==null&&Se(r),nt(n)){case 2:case 8:n=Oe;break;case 32:n=ke;break;case 268435456:n=je;break;default:n=ke}return r=cd.bind(null,e),n=xe(n,r),e.callbackPriority=t,e.callbackNode=n,t}return r!==null&&r!==null&&Se(r),e.callbackPriority=2,e.callbackNode=null,2}function cd(e,t){if(ru!==0&&ru!==5)return e.callbackNode=null,e.callbackPriority=0,null;var n=e.callbackNode;if(Vu()&&e.callbackNode!==n)return null;var r=X;return r=Ge(e,e===Ll?r:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r===0?null:(hu(e,r,t),sd(e,Te()),e.callbackNode!=null&&e.callbackNode===n?cd.bind(null,e):null)}function ld(e,t){if(Vu())return null;hu(e,t,!0)}function ud(){Xd(function(){Il&6?xe(De,ad):od()})}function dd(){if(nd===0){var e=ua;e===0&&(e=Ve,Ve<<=1,!(Ve&261888)&&(Ve=256)),nd=e}return nd}function fd(e){return e==null||typeof e==`symbol`||typeof e==`boolean`?null:typeof e==`function`?e:Qt(``+e)}function pd(e,t){var n=t.ownerDocument.createElement(`input`);return n.name=t.name,n.value=t.value,e.id&&n.setAttribute(`form`,e.id),t.parentNode.insertBefore(n,t),e=new FormData(e),n.parentNode.removeChild(n),e}function md(e,t,n,r,i){if(t===`submit`&&n&&n.stateNode===i){var a=fd((i[st]||null).action),o=r.submitter;o&&(t=(t=o[st]||null)?fd(t.formAction):o.getAttribute(`formAction`),t!==null&&(a=t,o=null));var s=new xn(`action`,`action`,null,r,i);e.push({event:s,listeners:[{instance:null,listener:function(){if(r.defaultPrevented){if(nd!==0){var e=o?pd(i,o):new FormData(i);ws(n,{pending:!0,data:e,method:i.method,action:a},null,e)}}else typeof a==`function`&&(s.preventDefault(),e=o?pd(i,o):new FormData(i),ws(n,{pending:!0,data:e,method:i.method,action:a},a,e))},currentTarget:i}]})}}for(var hd=0;hd<Jr.length;hd++){var gd=Jr[hd];Yr(gd.toLowerCase(),`on`+(gd[0].toUpperCase()+gd.slice(1)))}Yr(Br,`onAnimationEnd`),Yr(Vr,`onAnimationIteration`),Yr(Hr,`onAnimationStart`),Yr(`dblclick`,`onDoubleClick`),Yr(`focusin`,`onFocus`),Yr(`focusout`,`onBlur`),Yr(Ur,`onTransitionRun`),Yr(Wr,`onTransitionStart`),Yr(Gr,`onTransitionCancel`),Yr(Kr,`onTransitionEnd`),Ct(`onMouseEnter`,[`mouseout`,`mouseover`]),Ct(`onMouseLeave`,[`mouseout`,`mouseover`]),Ct(`onPointerEnter`,[`pointerout`,`pointerover`]),Ct(`onPointerLeave`,[`pointerout`,`pointerover`]),St(`onChange`,`change click focusin focusout input keydown keyup selectionchange`.split(` `)),St(`onSelect`,`focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(` `)),St(`onBeforeInput`,[`compositionend`,`keypress`,`textInput`,`paste`]),St(`onCompositionEnd`,`compositionend focusout keydown keypress keyup mousedown`.split(` `)),St(`onCompositionStart`,`compositionstart focusout keydown keypress keyup mousedown`.split(` `)),St(`onCompositionUpdate`,`compositionupdate focusout keydown keypress keyup mousedown`.split(` `));var _d=`abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting`.split(` `),vd=new Set(`beforetoggle cancel close invalid load scroll scrollend toggle`.split(` `).concat(_d));function yd(e,t){t=(t&4)!=0;for(var n=0;n<e.length;n++){var r=e[n],i=r.event;r=r.listeners;a:{var a=void 0;if(t)for(var o=r.length-1;0<=o;o--){var s=r[o],c=s.instance,l=s.currentTarget;if(s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){Xr(e)}i.currentTarget=null,a=c}else for(o=0;o<r.length;o++){if(s=r[o],c=s.instance,l=s.currentTarget,s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){Xr(e)}i.currentTarget=null,a=c}}}}function Q(e,t){var n=t[lt];n===void 0&&(n=t[lt]=new Set);var r=e+`__bubble`;n.has(r)||(Cd(t,e,2,!1),n.add(r))}function bd(e,t,n){var r=0;t&&(r|=4),Cd(n,e,r,t)}var xd=`_reactListening`+Math.random().toString(36).slice(2);function Sd(e){if(!e[xd]){e[xd]=!0,bt.forEach(function(t){t!==`selectionchange`&&(vd.has(t)||bd(t,!1,e),bd(t,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[xd]||(t[xd]=!0,bd(`selectionchange`,!1,t))}}function Cd(e,t,n,r){switch(mp(t)){case 2:var i=cp;break;case 8:i=lp;break;default:i=up}n=i.bind(null,t,n,e),i=void 0,!un||t!==`touchstart`&&t!==`touchmove`&&t!==`wheel`||(i=!0),r?i===void 0?e.addEventListener(t,n,!0):e.addEventListener(t,n,{capture:!0,passive:i}):i===void 0?e.addEventListener(t,n,!1):e.addEventListener(t,n,{passive:i})}function wd(e,t,n,r,i){var a=r;if(!(t&1)&&!(t&2)&&r!==null)a:for(;;){if(r===null)return;var o=r.tag;if(o===3||o===4){var c=r.stateNode.containerInfo;if(c===i)break;if(o===4)for(o=r.return;o!==null;){var l=o.tag;if((l===3||l===4)&&o.stateNode.containerInfo===i)return;o=o.return}for(;c!==null;){if(o=ht(c),o===null)return;if(l=o.tag,l===5||l===6||l===26||l===27){r=a=o;continue a}c=c.parentNode}}r=r.return}sn(function(){var r=a,i=tn(n),o=[];a:{var c=qr.get(e);if(c!==void 0){var l=xn,u=e;switch(e){case`keypress`:if(gn(n)===0)break a;case`keydown`:case`keyup`:l=zn;break;case`focusin`:u=`focus`,l=An;break;case`focusout`:u=`blur`,l=An;break;case`beforeblur`:case`afterblur`:l=An;break;case`click`:if(n.button===2)break a;case`auxclick`:case`dblclick`:case`mousedown`:case`mousemove`:case`mouseup`:case`mouseout`:case`mouseover`:case`contextmenu`:l=On;break;case`drag`:case`dragend`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`dragstart`:case`drop`:l=kn;break;case`touchcancel`:case`touchend`:case`touchmove`:case`touchstart`:l=Vn;break;case Br:case Vr:case Hr:l=jn;break;case Kr:l=Hn;break;case`scroll`:case`scrollend`:l=Cn;break;case`wheel`:l=Un;break;case`copy`:case`cut`:case`paste`:l=Mn;break;case`gotpointercapture`:case`lostpointercapture`:case`pointercancel`:case`pointerdown`:case`pointermove`:case`pointerout`:case`pointerover`:case`pointerup`:l=Bn;break;case`toggle`:case`beforetoggle`:l=Wn}var d=(t&4)!=0,f=!d&&(e===`scroll`||e===`scrollend`),p=d?c===null?null:c+`Capture`:c;d=[];for(var m=r,h;m!==null;){var g=m;if(h=g.stateNode,g=g.tag,g!==5&&g!==26&&g!==27||h===null||p===null||(g=cn(m,p),g!=null&&d.push(Td(m,g,h))),f)break;m=m.return}0<d.length&&(c=new l(c,u,null,n,i),o.push({event:c,listeners:d}))}}if(!(t&7)){a:{if(c=e===`mouseover`||e===`pointerover`,l=e===`mouseout`||e===`pointerout`,c&&n!==en&&(u=n.relatedTarget||n.fromElement)&&(ht(u)||u[ct]))break a;if((l||c)&&(c=i.window===i?i:(c=i.ownerDocument)?c.defaultView||c.parentWindow:window,l?(u=n.relatedTarget||n.toElement,l=r,u=u?ht(u):null,u!==null&&(f=s(u),d=u.tag,u!==f||d!==5&&d!==27&&d!==6)&&(u=null)):(l=null,u=r),l!==u)){if(d=On,g=`onMouseLeave`,p=`onMouseEnter`,m=`mouse`,(e===`pointerout`||e===`pointerover`)&&(d=Bn,g=`onPointerLeave`,p=`onPointerEnter`,m=`pointer`),f=l==null?c:_t(l),h=u==null?c:_t(u),c=new d(g,m+`leave`,l,n,i),c.target=f,c.relatedTarget=h,g=null,ht(i)===r&&(d=new d(p,m+`enter`,u,n,i),d.target=h,d.relatedTarget=f,g=d),f=g,l&&u)b:{for(d=Dd,p=l,m=u,h=0,g=p;g;g=d(g))h++;g=0;for(var _=m;_;_=d(_))g++;for(;0<h-g;)p=d(p),h--;for(;0<g-h;)m=d(m),g--;for(;h--;){if(p===m||m!==null&&p===m.alternate){d=p;break b}p=d(p),m=d(m)}d=null}else d=null;l!==null&&Od(o,c,l,d,!1),u!==null&&f!==null&&Od(o,f,u,d,!0)}}a:{if(c=r?_t(r):window,l=c.nodeName&&c.nodeName.toLowerCase(),l===`select`||l===`input`&&c.type===`file`)var v=ur;else if(ir(c))if(dr)v=br;else{v=vr;var y=_r}else l=c.nodeName,!l||l.toLowerCase()!==`input`||c.type!==`checkbox`&&c.type!==`radio`?r&&Yt(r.elementType)&&(v=ur):v=yr;if(v&&=v(e,r)){ar(o,v,n,i);break a}y&&y(e,c,r),e===`focusout`&&r&&c.type===`number`&&r.memoizedProps.value!=null&&Vt(c,`number`,c.value)}switch(y=r?_t(r):window,e){case`focusin`:(ir(y)||y.contentEditable===`true`)&&(Ar=y,jr=r,Mr=null);break;case`focusout`:Mr=jr=Ar=null;break;case`mousedown`:Nr=!0;break;case`contextmenu`:case`mouseup`:case`dragend`:Nr=!1,Pr(o,n,i);break;case`selectionchange`:if(kr)break;case`keydown`:case`keyup`:Pr(o,n,i)}var b;if(Kn)b:{switch(e){case`compositionstart`:var x=`onCompositionStart`;break b;case`compositionend`:x=`onCompositionEnd`;break b;case`compositionupdate`:x=`onCompositionUpdate`;break b}x=void 0}else er?Qn(e,n)&&(x=`onCompositionEnd`):e===`keydown`&&n.keyCode===229&&(x=`onCompositionStart`);x&&(Yn&&n.locale!==`ko`&&(er||x!==`onCompositionStart`?x===`onCompositionEnd`&&er&&(b=hn()):(fn=i,pn=`value`in fn?fn.value:fn.textContent,er=!0)),y=Ed(r,x),0<y.length&&(x=new Nn(x,e,null,n,i),o.push({event:x,listeners:y}),b?x.data=b:(b=$n(n),b!==null&&(x.data=b)))),(b=Jn?tr(e,n):nr(e,n))&&(x=Ed(r,`onBeforeInput`),0<x.length&&(y=new Nn(`onBeforeInput`,`beforeinput`,null,n,i),o.push({event:y,listeners:x}),y.data=b)),md(o,e,r,n,i)}yd(o,t)})}function Td(e,t,n){return{instance:e,listener:t,currentTarget:n}}function Ed(e,t){for(var n=t+`Capture`,r=[];e!==null;){var i=e,a=i.stateNode;if(i=i.tag,i!==5&&i!==26&&i!==27||a===null||(i=cn(e,n),i!=null&&r.unshift(Td(e,i,a)),i=cn(e,t),i!=null&&r.push(Td(e,i,a))),e.tag===3)return r;e=e.return}return[]}function Dd(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function Od(e,t,n,r,i){for(var a=t._reactName,o=[];n!==null&&n!==r;){var s=n,c=s.alternate,l=s.stateNode;if(s=s.tag,c!==null&&c===r)break;s!==5&&s!==26&&s!==27||l===null||(c=l,i?(l=cn(n,a),l!=null&&o.unshift(Td(n,l,c))):i||(l=cn(n,a),l!=null&&o.push(Td(n,l,c)))),n=n.return}o.length!==0&&e.push({event:t,listeners:o})}var kd=/\r\n?/g,Ad=/\u0000|\uFFFD/g;function jd(e){return(typeof e==`string`?e:``+e).replace(kd,`
`).replace(Ad,``)}function Md(e,t){return t=jd(t),jd(e)===t}function Nd(e,t,n,r,i,o){switch(n){case`children`:typeof r==`string`?t===`body`||t===`textarea`&&r===``||Gt(e,r):(typeof r==`number`||typeof r==`bigint`)&&t!==`body`&&Gt(e,``+r);break;case`className`:kt(e,`class`,r);break;case`tabIndex`:kt(e,`tabindex`,r);break;case`dir`:case`role`:case`viewBox`:case`width`:case`height`:kt(e,n,r);break;case`style`:Jt(e,r,o);break;case`data`:if(t!==`object`){kt(e,`data`,r);break}case`src`:case`href`:if(r===``&&(t!==`a`||n!==`href`)){e.removeAttribute(n);break}if(r==null||typeof r==`function`||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=Qt(``+r),e.setAttribute(n,r);break;case`action`:case`formAction`:if(typeof r==`function`){e.setAttribute(n,`javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')`);break}else typeof o==`function`&&(n===`formAction`?(t!==`input`&&Nd(e,t,`name`,i.name,i,null),Nd(e,t,`formEncType`,i.formEncType,i,null),Nd(e,t,`formMethod`,i.formMethod,i,null),Nd(e,t,`formTarget`,i.formTarget,i,null)):(Nd(e,t,`encType`,i.encType,i,null),Nd(e,t,`method`,i.method,i,null),Nd(e,t,`target`,i.target,i,null)));if(r==null||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=Qt(``+r),e.setAttribute(n,r);break;case`onClick`:r!=null&&(e.onclick=$t);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`multiple`:e.multiple=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`muted`:e.muted=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`defaultValue`:case`defaultChecked`:case`innerHTML`:case`ref`:break;case`autoFocus`:break;case`xlinkHref`:if(r==null||typeof r==`function`||typeof r==`boolean`||typeof r==`symbol`){e.removeAttribute(`xlink:href`);break}n=Qt(``+r),e.setAttributeNS(`http://www.w3.org/1999/xlink`,`xlink:href`,n);break;case`contentEditable`:case`spellCheck`:case`draggable`:case`value`:case`autoReverse`:case`externalResourcesRequired`:case`focusable`:case`preserveAlpha`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``+r):e.removeAttribute(n);break;case`inert`:case`allowFullScreen`:case`async`:case`autoPlay`:case`controls`:case`default`:case`defer`:case`disabled`:case`disablePictureInPicture`:case`disableRemotePlayback`:case`formNoValidate`:case`hidden`:case`loop`:case`noModule`:case`noValidate`:case`open`:case`playsInline`:case`readOnly`:case`required`:case`reversed`:case`scoped`:case`seamless`:case`itemScope`:r&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``):e.removeAttribute(n);break;case`capture`:case`download`:!0===r?e.setAttribute(n,``):!1!==r&&r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,r):e.removeAttribute(n);break;case`cols`:case`rows`:case`size`:case`span`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`&&!isNaN(r)&&1<=r?e.setAttribute(n,r):e.removeAttribute(n);break;case`rowSpan`:case`start`:r==null||typeof r==`function`||typeof r==`symbol`||isNaN(r)?e.removeAttribute(n):e.setAttribute(n,r);break;case`popover`:Q(`beforetoggle`,e),Q(`toggle`,e),Ot(e,`popover`,r);break;case`xlinkActuate`:At(e,`http://www.w3.org/1999/xlink`,`xlink:actuate`,r);break;case`xlinkArcrole`:At(e,`http://www.w3.org/1999/xlink`,`xlink:arcrole`,r);break;case`xlinkRole`:At(e,`http://www.w3.org/1999/xlink`,`xlink:role`,r);break;case`xlinkShow`:At(e,`http://www.w3.org/1999/xlink`,`xlink:show`,r);break;case`xlinkTitle`:At(e,`http://www.w3.org/1999/xlink`,`xlink:title`,r);break;case`xlinkType`:At(e,`http://www.w3.org/1999/xlink`,`xlink:type`,r);break;case`xmlBase`:At(e,`http://www.w3.org/XML/1998/namespace`,`xml:base`,r);break;case`xmlLang`:At(e,`http://www.w3.org/XML/1998/namespace`,`xml:lang`,r);break;case`xmlSpace`:At(e,`http://www.w3.org/XML/1998/namespace`,`xml:space`,r);break;case`is`:Ot(e,`is`,r);break;case`innerText`:case`textContent`:break;default:(!(2<n.length)||n[0]!==`o`&&n[0]!==`O`||n[1]!==`n`&&n[1]!==`N`)&&(n=Xt.get(n)||n,Ot(e,n,r))}}function Pd(e,t,n,r,i,o){switch(n){case`style`:Jt(e,r,o);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`children`:typeof r==`string`?Gt(e,r):(typeof r==`number`||typeof r==`bigint`)&&Gt(e,``+r);break;case`onScroll`:r!=null&&Q(`scroll`,e);break;case`onScrollEnd`:r!=null&&Q(`scrollend`,e);break;case`onClick`:r!=null&&(e.onclick=$t);break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`innerHTML`:case`ref`:break;case`innerText`:case`textContent`:break;default:if(!xt.hasOwnProperty(n))a:{if(n[0]===`o`&&n[1]===`n`&&(i=n.endsWith(`Capture`),t=n.slice(2,i?n.length-7:void 0),o=e[st]||null,o=o==null?null:o[n],typeof o==`function`&&e.removeEventListener(t,o,i),typeof r==`function`)){typeof o!=`function`&&o!==null&&(n in e?e[n]=null:e.hasAttribute(n)&&e.removeAttribute(n)),e.addEventListener(t,r,i);break a}n in e?e[n]=r:!0===r?e.setAttribute(n,``):Ot(e,n,r)}}}function Fd(e,t,n){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`img`:Q(`error`,e),Q(`load`,e);var r=!1,i=!1,o;for(o in n)if(n.hasOwnProperty(o)){var s=n[o];if(s!=null)switch(o){case`src`:r=!0;break;case`srcSet`:i=!0;break;case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:Nd(e,t,o,s,n,null)}}i&&Nd(e,t,`srcSet`,n.srcSet,n,null),r&&Nd(e,t,`src`,n.src,n,null);return;case`input`:Q(`invalid`,e);var c=o=s=i=null,l=null,u=null;for(r in n)if(n.hasOwnProperty(r)){var d=n[r];if(d!=null)switch(r){case`name`:i=d;break;case`type`:s=d;break;case`checked`:l=d;break;case`defaultChecked`:u=d;break;case`value`:o=d;break;case`defaultValue`:c=d;break;case`children`:case`dangerouslySetInnerHTML`:if(d!=null)throw Error(a(137,t));break;default:Nd(e,t,r,d,n,null)}}Bt(e,o,c,l,u,s,i,!1);return;case`select`:for(i in Q(`invalid`,e),r=s=o=null,n)if(n.hasOwnProperty(i)&&(c=n[i],c!=null))switch(i){case`value`:o=c;break;case`defaultValue`:s=c;break;case`multiple`:r=c;default:Nd(e,t,i,c,n,null)}t=o,n=s,e.multiple=!!r,t==null?n!=null&&Ht(e,!!r,n,!0):Ht(e,!!r,t,!1);return;case`textarea`:for(s in Q(`invalid`,e),o=i=r=null,n)if(n.hasOwnProperty(s)&&(c=n[s],c!=null))switch(s){case`value`:r=c;break;case`defaultValue`:i=c;break;case`children`:o=c;break;case`dangerouslySetInnerHTML`:if(c!=null)throw Error(a(91));break;default:Nd(e,t,s,c,n,null)}Wt(e,r,i,o);return;case`option`:for(l in n)if(n.hasOwnProperty(l)&&(r=n[l],r!=null))switch(l){case`selected`:e.selected=r&&typeof r!=`function`&&typeof r!=`symbol`;break;default:Nd(e,t,l,r,n,null)}return;case`dialog`:Q(`beforetoggle`,e),Q(`toggle`,e),Q(`cancel`,e),Q(`close`,e);break;case`iframe`:case`object`:Q(`load`,e);break;case`video`:case`audio`:for(r=0;r<_d.length;r++)Q(_d[r],e);break;case`image`:Q(`error`,e),Q(`load`,e);break;case`details`:Q(`toggle`,e);break;case`embed`:case`source`:case`link`:Q(`error`,e),Q(`load`,e);case`area`:case`base`:case`br`:case`col`:case`hr`:case`keygen`:case`meta`:case`param`:case`track`:case`wbr`:case`menuitem`:for(u in n)if(n.hasOwnProperty(u)&&(r=n[u],r!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:Nd(e,t,u,r,n,null)}return;default:if(Yt(t)){for(d in n)n.hasOwnProperty(d)&&(r=n[d],r!==void 0&&Pd(e,t,d,r,n,void 0));return}}for(c in n)n.hasOwnProperty(c)&&(r=n[c],r!=null&&Nd(e,t,c,r,n,null))}function Id(e,t,n,r){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`input`:var i=null,o=null,s=null,c=null,l=null,u=null,d=null;for(m in n){var f=n[m];if(n.hasOwnProperty(m)&&f!=null)switch(m){case`checked`:break;case`value`:break;case`defaultValue`:l=f;default:r.hasOwnProperty(m)||Nd(e,t,m,null,r,f)}}for(var p in r){var m=r[p];if(f=n[p],r.hasOwnProperty(p)&&(m!=null||f!=null))switch(p){case`type`:o=m;break;case`name`:i=m;break;case`checked`:u=m;break;case`defaultChecked`:d=m;break;case`value`:s=m;break;case`defaultValue`:c=m;break;case`children`:case`dangerouslySetInnerHTML`:if(m!=null)throw Error(a(137,t));break;default:m!==f&&Nd(e,t,p,m,r,f)}}zt(e,s,c,l,u,d,o,i);return;case`select`:for(o in m=s=c=p=null,n)if(l=n[o],n.hasOwnProperty(o)&&l!=null)switch(o){case`value`:break;case`multiple`:m=l;default:r.hasOwnProperty(o)||Nd(e,t,o,null,r,l)}for(i in r)if(o=r[i],l=n[i],r.hasOwnProperty(i)&&(o!=null||l!=null))switch(i){case`value`:p=o;break;case`defaultValue`:c=o;break;case`multiple`:s=o;default:o!==l&&Nd(e,t,i,o,r,l)}t=c,n=s,r=m,p==null?!!r!=!!n&&(t==null?Ht(e,!!n,n?[]:``,!1):Ht(e,!!n,t,!0)):Ht(e,!!n,p,!1);return;case`textarea`:for(c in m=p=null,n)if(i=n[c],n.hasOwnProperty(c)&&i!=null&&!r.hasOwnProperty(c))switch(c){case`value`:break;case`children`:break;default:Nd(e,t,c,null,r,i)}for(s in r)if(i=r[s],o=n[s],r.hasOwnProperty(s)&&(i!=null||o!=null))switch(s){case`value`:p=i;break;case`defaultValue`:m=i;break;case`children`:break;case`dangerouslySetInnerHTML`:if(i!=null)throw Error(a(91));break;default:i!==o&&Nd(e,t,s,i,r,o)}Ut(e,p,m);return;case`option`:for(var h in n)if(p=n[h],n.hasOwnProperty(h)&&p!=null&&!r.hasOwnProperty(h))switch(h){case`selected`:e.selected=!1;break;default:Nd(e,t,h,null,r,p)}for(l in r)if(p=r[l],m=n[l],r.hasOwnProperty(l)&&p!==m&&(p!=null||m!=null))switch(l){case`selected`:e.selected=p&&typeof p!=`function`&&typeof p!=`symbol`;break;default:Nd(e,t,l,p,r,m)}return;case`img`:case`link`:case`area`:case`base`:case`br`:case`col`:case`embed`:case`hr`:case`keygen`:case`meta`:case`param`:case`source`:case`track`:case`wbr`:case`menuitem`:for(var g in n)p=n[g],n.hasOwnProperty(g)&&p!=null&&!r.hasOwnProperty(g)&&Nd(e,t,g,null,r,p);for(u in r)if(p=r[u],m=n[u],r.hasOwnProperty(u)&&p!==m&&(p!=null||m!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:if(p!=null)throw Error(a(137,t));break;default:Nd(e,t,u,p,r,m)}return;default:if(Yt(t)){for(var _ in n)p=n[_],n.hasOwnProperty(_)&&p!==void 0&&!r.hasOwnProperty(_)&&Pd(e,t,_,void 0,r,p);for(d in r)p=r[d],m=n[d],!r.hasOwnProperty(d)||p===m||p===void 0&&m===void 0||Pd(e,t,d,p,r,m);return}}for(var v in n)p=n[v],n.hasOwnProperty(v)&&p!=null&&!r.hasOwnProperty(v)&&Nd(e,t,v,null,r,p);for(f in r)p=r[f],m=n[f],!r.hasOwnProperty(f)||p===m||p==null&&m==null||Nd(e,t,f,p,r,m)}function Ld(e){switch(e){case`css`:case`script`:case`font`:case`img`:case`image`:case`input`:case`link`:return!0;default:return!1}}function Rd(){if(typeof performance.getEntriesByType==`function`){for(var e=0,t=0,n=performance.getEntriesByType(`resource`),r=0;r<n.length;r++){var i=n[r],a=i.transferSize,o=i.initiatorType,s=i.duration;if(a&&s&&Ld(o)){for(o=0,s=i.responseEnd,r+=1;r<n.length;r++){var c=n[r],l=c.startTime;if(l>s)break;var u=c.transferSize,d=c.initiatorType;u&&Ld(d)&&(c=c.responseEnd,o+=u*(c<s?1:(s-l)/(c-l)))}if(--r,t+=8*(a+o)/(i.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e==`number`)?e:5}var zd=null,Bd=null;function Vd(e){return e.nodeType===9?e:e.ownerDocument}function Hd(e){switch(e){case`http://www.w3.org/2000/svg`:return 1;case`http://www.w3.org/1998/Math/MathML`:return 2;default:return 0}}function Ud(e,t){if(e===0)switch(t){case`svg`:return 1;case`math`:return 2;default:return 0}return e===1&&t===`foreignObject`?0:e}function Wd(e,t){return e===`textarea`||e===`noscript`||typeof t.children==`string`||typeof t.children==`number`||typeof t.children==`bigint`||typeof t.dangerouslySetInnerHTML==`object`&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var Gd=null;function Kd(){var e=window.event;return e&&e.type===`popstate`?e===Gd?!1:(Gd=e,!0):(Gd=null,!1)}var qd=typeof setTimeout==`function`?setTimeout:void 0,Jd=typeof clearTimeout==`function`?clearTimeout:void 0,Yd=typeof Promise==`function`?Promise:void 0,Xd=typeof queueMicrotask==`function`?queueMicrotask:Yd===void 0?qd:function(e){return Yd.resolve(null).then(e).catch(Zd)};function Zd(e){setTimeout(function(){throw e})}function Qd(e){return e===`head`}function $d(e,t){var n=t,r=0;do{var i=n.nextSibling;if(e.removeChild(n),i&&i.nodeType===8)if(n=i.data,n===`/$`||n===`/&`){if(r===0){e.removeChild(i),Np(t);return}r--}else if(n===`$`||n===`$?`||n===`$~`||n===`$!`||n===`&`)r++;else if(n===`html`)mf(e.ownerDocument.documentElement);else if(n===`head`){n=e.ownerDocument.head,mf(n);for(var a=n.firstChild;a;){var o=a.nextSibling,s=a.nodeName;a[pt]||s===`SCRIPT`||s===`STYLE`||s===`LINK`&&a.rel.toLowerCase()===`stylesheet`||n.removeChild(a),a=o}}else n===`body`&&mf(e.ownerDocument.body);n=i}while(n);Np(t)}function ef(e,t){var n=e;e=0;do{var r=n.nextSibling;if(n.nodeType===1?t?(n._stashedDisplay=n.style.display,n.style.display=`none`):(n.style.display=n._stashedDisplay||``,n.getAttribute(`style`)===``&&n.removeAttribute(`style`)):n.nodeType===3&&(t?(n._stashedText=n.nodeValue,n.nodeValue=``):n.nodeValue=n._stashedText||``),r&&r.nodeType===8)if(n=r.data,n===`/$`){if(e===0)break;e--}else n!==`$`&&n!==`$?`&&n!==`$~`&&n!==`$!`||e++;n=r}while(n)}function tf(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var n=t;switch(t=t.nextSibling,n.nodeName){case`HTML`:case`HEAD`:case`BODY`:tf(n),mt(n);continue;case`SCRIPT`:case`STYLE`:continue;case`LINK`:if(n.rel.toLowerCase()===`stylesheet`)continue}e.removeChild(n)}}function nf(e,t,n,r){for(;e.nodeType===1;){var i=n;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!r&&(e.nodeName!==`INPUT`||e.type!==`hidden`))break}else if(!r)if(t===`input`&&e.type===`hidden`){var a=i.name==null?null:``+i.name;if(i.type===`hidden`&&e.getAttribute(`name`)===a)return e}else return e;else if(!e[pt])switch(t){case`meta`:if(!e.hasAttribute(`itemprop`))break;return e;case`link`:if(a=e.getAttribute(`rel`),a===`stylesheet`&&e.hasAttribute(`data-precedence`)||a!==i.rel||e.getAttribute(`href`)!==(i.href==null||i.href===``?null:i.href)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin)||e.getAttribute(`title`)!==(i.title==null?null:i.title))break;return e;case`style`:if(e.hasAttribute(`data-precedence`))break;return e;case`script`:if(a=e.getAttribute(`src`),(a!==(i.src==null?null:i.src)||e.getAttribute(`type`)!==(i.type==null?null:i.type)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin))&&a&&e.hasAttribute(`async`)&&!e.hasAttribute(`itemprop`))break;return e;default:return e}if(e=lf(e.nextSibling),e===null)break}return null}function rf(e,t,n){if(t===``)return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!n||(e=lf(e.nextSibling),e===null))return null;return e}function af(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!t||(e=lf(e.nextSibling),e===null))return null;return e}function of(e){return e.data===`$?`||e.data===`$~`}function sf(e){return e.data===`$!`||e.data===`$?`&&e.ownerDocument.readyState!==`loading`}function cf(e,t){var n=e.ownerDocument;if(e.data===`$~`)e._reactRetry=t;else if(e.data!==`$?`||n.readyState!==`loading`)t();else{var r=function(){t(),n.removeEventListener(`DOMContentLoaded`,r)};n.addEventListener(`DOMContentLoaded`,r),e._reactRetry=r}}function lf(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t===`$`||t===`$!`||t===`$?`||t===`$~`||t===`&`||t===`F!`||t===`F`)break;if(t===`/$`||t===`/&`)return null}}return e}var uf=null;function df(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`/$`||n===`/&`){if(t===0)return lf(e.nextSibling);t--}else n!==`$`&&n!==`$!`&&n!==`$?`&&n!==`$~`&&n!==`&`||t++}e=e.nextSibling}return null}function ff(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`$`||n===`$!`||n===`$?`||n===`$~`||n===`&`){if(t===0)return e;t--}else n!==`/$`&&n!==`/&`||t++}e=e.previousSibling}return null}function pf(e,t,n){switch(t=Vd(n),e){case`html`:if(e=t.documentElement,!e)throw Error(a(452));return e;case`head`:if(e=t.head,!e)throw Error(a(453));return e;case`body`:if(e=t.body,!e)throw Error(a(454));return e;default:throw Error(a(451))}}function mf(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);mt(e)}var hf=new Map,gf=new Set;function _f(e){return typeof e.getRootNode==`function`?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var vf=P.d;P.d={f:yf,r:bf,D:Cf,C:wf,L:Tf,m:Ef,X:Of,S:Df,M:kf};function yf(){var e=vf.f(),t=yu();return e||t}function bf(e){var t=gt(e);t!==null&&t.tag===5&&t.type===`form`?Es(t):vf.r(e)}var xf=typeof document>`u`?null:document;function Sf(e,t,n){var r=xf;if(r&&typeof t==`string`&&t){var i=Rt(t);i=`link[rel="`+e+`"][href="`+i+`"]`,typeof n==`string`&&(i+=`[crossorigin="`+n+`"]`),gf.has(i)||(gf.add(i),e={rel:e,crossOrigin:n,href:t},r.querySelector(i)===null&&(t=r.createElement(`link`),Fd(t,`link`,e),yt(t),r.head.appendChild(t)))}}function Cf(e){vf.D(e),Sf(`dns-prefetch`,e,null)}function wf(e,t){vf.C(e,t),Sf(`preconnect`,e,t)}function Tf(e,t,n){vf.L(e,t,n);var r=xf;if(r&&e&&t){var i=`link[rel="preload"][as="`+Rt(t)+`"]`;t===`image`&&n&&n.imageSrcSet?(i+=`[imagesrcset="`+Rt(n.imageSrcSet)+`"]`,typeof n.imageSizes==`string`&&(i+=`[imagesizes="`+Rt(n.imageSizes)+`"]`)):i+=`[href="`+Rt(e)+`"]`;var a=i;switch(t){case`style`:a=jf(e);break;case`script`:a=Ff(e)}hf.has(a)||(e=m({rel:`preload`,href:t===`image`&&n&&n.imageSrcSet?void 0:e,as:t},n),hf.set(a,e),r.querySelector(i)!==null||t===`style`&&r.querySelector(Mf(a))||t===`script`&&r.querySelector(If(a))||(t=r.createElement(`link`),Fd(t,`link`,e),yt(t),r.head.appendChild(t)))}}function Ef(e,t){vf.m(e,t);var n=xf;if(n&&e){var r=t&&typeof t.as==`string`?t.as:`script`,i=`link[rel="modulepreload"][as="`+Rt(r)+`"][href="`+Rt(e)+`"]`,a=i;switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:a=Ff(e)}if(!hf.has(a)&&(e=m({rel:`modulepreload`,href:e},t),hf.set(a,e),n.querySelector(i)===null)){switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:if(n.querySelector(If(a)))return}r=n.createElement(`link`),Fd(r,`link`,e),yt(r),n.head.appendChild(r)}}}function Df(e,t,n){vf.S(e,t,n);var r=xf;if(r&&e){var i=vt(r).hoistableStyles,a=jf(e);t||=`default`;var o=i.get(a);if(!o){var s={loading:0,preload:null};if(o=r.querySelector(Mf(a)))s.loading=5;else{e=m({rel:`stylesheet`,href:e,"data-precedence":t},n),(n=hf.get(a))&&zf(e,n);var c=o=r.createElement(`link`);yt(c),Fd(c,`link`,e),c._p=new Promise(function(e,t){c.onload=e,c.onerror=t}),c.addEventListener(`load`,function(){s.loading|=1}),c.addEventListener(`error`,function(){s.loading|=2}),s.loading|=4,Rf(o,t,r)}o={type:`stylesheet`,instance:o,count:1,state:s},i.set(a,o)}}}function Of(e,t){vf.X(e,t);var n=xf;if(n&&e){var r=vt(n).hoistableScripts,i=Ff(e),a=r.get(i);a||(a=n.querySelector(If(i)),a||(e=m({src:e,async:!0},t),(t=hf.get(i))&&Bf(e,t),a=n.createElement(`script`),yt(a),Fd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function kf(e,t){vf.M(e,t);var n=xf;if(n&&e){var r=vt(n).hoistableScripts,i=Ff(e),a=r.get(i);a||(a=n.querySelector(If(i)),a||(e=m({src:e,async:!0,type:`module`},t),(t=hf.get(i))&&Bf(e,t),a=n.createElement(`script`),yt(a),Fd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function Af(e,t,n,r){var i=(i=ce.current)?_f(i):null;if(!i)throw Error(a(446));switch(e){case`meta`:case`title`:return null;case`style`:return typeof n.precedence==`string`&&typeof n.href==`string`?(t=jf(n.href),n=vt(i).hoistableStyles,r=n.get(t),r||(r={type:`style`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};case`link`:if(n.rel===`stylesheet`&&typeof n.href==`string`&&typeof n.precedence==`string`){e=jf(n.href);var o=vt(i).hoistableStyles,s=o.get(e);if(s||(i=i.ownerDocument||i,s={type:`stylesheet`,instance:null,count:0,state:{loading:0,preload:null}},o.set(e,s),(o=i.querySelector(Mf(e)))&&!o._p&&(s.instance=o,s.state.loading=5),hf.has(e)||(n={rel:`preload`,as:`style`,href:n.href,crossOrigin:n.crossOrigin,integrity:n.integrity,media:n.media,hrefLang:n.hrefLang,referrerPolicy:n.referrerPolicy},hf.set(e,n),o||Pf(i,e,n,s.state))),t&&r===null)throw Error(a(528,``));return s}if(t&&r!==null)throw Error(a(529,``));return null;case`script`:return t=n.async,n=n.src,typeof n==`string`&&t&&typeof t!=`function`&&typeof t!=`symbol`?(t=Ff(n),n=vt(i).hoistableScripts,r=n.get(t),r||(r={type:`script`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};default:throw Error(a(444,e))}}function jf(e){return`href="`+Rt(e)+`"`}function Mf(e){return`link[rel="stylesheet"][`+e+`]`}function Nf(e){return m({},e,{"data-precedence":e.precedence,precedence:null})}function Pf(e,t,n,r){e.querySelector(`link[rel="preload"][as="style"][`+t+`]`)?r.loading=1:(t=e.createElement(`link`),r.preload=t,t.addEventListener(`load`,function(){return r.loading|=1}),t.addEventListener(`error`,function(){return r.loading|=2}),Fd(t,`link`,n),yt(t),e.head.appendChild(t))}function Ff(e){return`[src="`+Rt(e)+`"]`}function If(e){return`script[async]`+e}function Lf(e,t,n){if(t.count++,t.instance===null)switch(t.type){case`style`:var r=e.querySelector(`style[data-href~="`+Rt(n.href)+`"]`);if(r)return t.instance=r,yt(r),r;var i=m({},n,{"data-href":n.href,"data-precedence":n.precedence,href:null,precedence:null});return r=(e.ownerDocument||e).createElement(`style`),yt(r),Fd(r,`style`,i),Rf(r,n.precedence,e),t.instance=r;case`stylesheet`:i=jf(n.href);var o=e.querySelector(Mf(i));if(o)return t.state.loading|=4,t.instance=o,yt(o),o;r=Nf(n),(i=hf.get(i))&&zf(r,i),o=(e.ownerDocument||e).createElement(`link`),yt(o);var s=o;return s._p=new Promise(function(e,t){s.onload=e,s.onerror=t}),Fd(o,`link`,r),t.state.loading|=4,Rf(o,n.precedence,e),t.instance=o;case`script`:return o=Ff(n.src),(i=e.querySelector(If(o)))?(t.instance=i,yt(i),i):(r=n,(i=hf.get(o))&&(r=m({},n),Bf(r,i)),e=e.ownerDocument||e,i=e.createElement(`script`),yt(i),Fd(i,`link`,r),e.head.appendChild(i),t.instance=i);case`void`:return null;default:throw Error(a(443,t.type))}else t.type===`stylesheet`&&!(t.state.loading&4)&&(r=t.instance,t.state.loading|=4,Rf(r,n.precedence,e));return t.instance}function Rf(e,t,n){for(var r=n.querySelectorAll(`link[rel="stylesheet"][data-precedence],style[data-precedence]`),i=r.length?r[r.length-1]:null,a=i,o=0;o<r.length;o++){var s=r[o];if(s.dataset.precedence===t)a=s;else if(a!==i)break}a?a.parentNode.insertBefore(e,a.nextSibling):(t=n.nodeType===9?n.head:n,t.insertBefore(e,t.firstChild))}function zf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.title??=t.title}function Bf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.integrity??=t.integrity}var Vf=null;function Hf(e,t,n){if(Vf===null){var r=new Map,i=Vf=new Map;i.set(n,r)}else i=Vf,r=i.get(n),r||(r=new Map,i.set(n,r));if(r.has(e))return r;for(r.set(e,null),n=n.getElementsByTagName(e),i=0;i<n.length;i++){var a=n[i];if(!(a[pt]||a[ot]||e===`link`&&a.getAttribute(`rel`)===`stylesheet`)&&a.namespaceURI!==`http://www.w3.org/2000/svg`){var o=a.getAttribute(t)||``;o=e+o;var s=r.get(o);s?s.push(a):r.set(o,[a])}}return r}function Uf(e,t,n){e=e.ownerDocument||e,e.head.insertBefore(n,t===`title`?e.querySelector(`head > title`):null)}function $(e,t,n){if(n===1||t.itemProp!=null)return!1;switch(e){case`meta`:case`title`:return!0;case`style`:if(typeof t.precedence!=`string`||typeof t.href!=`string`||t.href===``)break;return!0;case`link`:if(typeof t.rel!=`string`||typeof t.href!=`string`||t.href===``||t.onLoad||t.onError)break;switch(t.rel){case`stylesheet`:return e=t.disabled,typeof t.precedence==`string`&&e==null;default:return!0}case`script`:if(t.async&&typeof t.async!=`function`&&typeof t.async!=`symbol`&&!t.onLoad&&!t.onError&&t.src&&typeof t.src==`string`)return!0}return!1}function Wf(e){return!(e.type===`stylesheet`&&!(e.state.loading&3))}function Gf(e,t,n,r){if(n.type===`stylesheet`&&(typeof r.media!=`string`||!1!==matchMedia(r.media).matches)&&!(n.state.loading&4)){if(n.instance===null){var i=jf(r.href),a=t.querySelector(Mf(i));if(a){t=a._p,typeof t==`object`&&t&&typeof t.then==`function`&&(e.count++,e=Jf.bind(e),t.then(e,e)),n.state.loading|=4,n.instance=a,yt(a);return}a=t.ownerDocument||t,r=Nf(r),(i=hf.get(i))&&zf(r,i),a=a.createElement(`link`),yt(a);var o=a;o._p=new Promise(function(e,t){o.onload=e,o.onerror=t}),Fd(a,`link`,r),n.instance=a}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(n,t),(t=n.state.preload)&&!(n.state.loading&3)&&(e.count++,n=Jf.bind(e),t.addEventListener(`load`,n),t.addEventListener(`error`,n))}}var Kf=0;function qf(e,t){return e.stylesheets&&e.count===0&&Xf(e,e.stylesheets),0<e.count||0<e.imgCount?function(n){var r=setTimeout(function(){if(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend){var t=e.unsuspend;e.unsuspend=null,t()}},6e4+t);0<e.imgBytes&&Kf===0&&(Kf=62500*Rd());var i=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend)){var t=e.unsuspend;e.unsuspend=null,t()}},(e.imgBytes>Kf?50:800)+t);return e.unsuspend=n,function(){e.unsuspend=null,clearTimeout(r),clearTimeout(i)}}:null}function Jf(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)Xf(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var Yf=null;function Xf(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,Yf=new Map,t.forEach(Zf,e),Yf=null,Jf.call(e))}function Zf(e,t){if(!(t.state.loading&4)){var n=Yf.get(e);if(n)var r=n.get(null);else{n=new Map,Yf.set(e,n);for(var i=e.querySelectorAll(`link[data-precedence],style[data-precedence]`),a=0;a<i.length;a++){var o=i[a];(o.nodeName===`LINK`||o.getAttribute(`media`)!==`not all`)&&(n.set(o.dataset.precedence,o),r=o)}r&&n.set(null,r)}i=t.instance,o=i.getAttribute(`data-precedence`),a=n.get(o)||r,a===r&&n.set(null,i),n.set(o,i),this.count++,r=Jf.bind(this),i.addEventListener(`load`,r),i.addEventListener(`error`,r),a?a.parentNode.insertBefore(i,a.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(i,e.firstChild)),t.state.loading|=4}}var Qf={$$typeof:S,Provider:null,Consumer:null,_currentValue:F,_currentValue2:F,_threadCount:0};function $f(e,t,n,r,i,a,o,s,c){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=Ye(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=Ye(0),this.hiddenUpdates=Ye(null),this.identifierPrefix=r,this.onUncaughtError=i,this.onCaughtError=a,this.onRecoverableError=o,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=c,this.incompleteTransitions=new Map}function ep(e,t,n,r,i,a,o,s,c,l,u,d){return e=new $f(e,t,n,o,c,l,u,d,s),t=1,!0===a&&(t|=24),a=ci(3,null,null,t),e.current=a,a.stateNode=e,t=oa(),t.refCount++,e.pooledCache=t,t.refCount++,a.memoizedState={element:r,isDehydrated:n,cache:t},za(a),e}function tp(e){return e?(e=oi,e):oi}function np(e,t,n,r,i,a){i=tp(i),r.context===null?r.context=i:r.pendingContext=i,r=Va(t),r.payload={element:n},a=a===void 0?null:a,a!==null&&(r.callback=a),n=Ha(e,r,t),n!==null&&(mu(n,e,t),Ua(n,e,t))}function rp(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var n=e.retryLane;e.retryLane=n!==0&&n<t?n:t}}function ip(e,t){rp(e,t),(e=e.alternate)&&rp(e,t)}function ap(e){if(e.tag===13||e.tag===31){var t=ri(e,67108864);t!==null&&mu(t,e,67108864),ip(e,67108864)}}function op(e){if(e.tag===13||e.tag===31){var t=fu();t=tt(t);var n=ri(e,t);n!==null&&mu(n,e,t),ip(e,t)}}var sp=!0;function cp(e,t,n,r){var i=N.T;N.T=null;var a=P.p;try{P.p=2,up(e,t,n,r)}finally{P.p=a,N.T=i}}function lp(e,t,n,r){var i=N.T;N.T=null;var a=P.p;try{P.p=8,up(e,t,n,r)}finally{P.p=a,N.T=i}}function up(e,t,n,r){if(sp){var i=dp(r);if(i===null)wd(e,t,r,fp,n),Cp(e,r);else if(Tp(i,e,t,n,r))r.stopPropagation();else if(Cp(e,r),t&4&&-1<Sp.indexOf(e)){for(;i!==null;){var a=gt(i);if(a!==null)switch(a.tag){case 3:if(a=a.stateNode,a.current.memoizedState.isDehydrated){var o=We(a.pendingLanes);if(o!==0){var s=a;for(s.pendingLanes|=2,s.entangledLanes|=2;o;){var c=1<<31-Le(o);s.entanglements[1]|=c,o&=~c}rd(a),!(Il&6)&&(eu=Te()+500,id(0,!1))}}break;case 31:case 13:s=ri(a,2),s!==null&&mu(s,a,2),yu(),ip(a,2)}if(a=dp(r),a===null&&wd(e,t,r,fp,n),a===i)break;i=a}i!==null&&r.stopPropagation()}else wd(e,t,r,null,n)}}function dp(e){return e=tn(e),pp(e)}var fp=null;function pp(e){if(fp=null,e=ht(e),e!==null){var t=s(e);if(t===null)e=null;else{var n=t.tag;if(n===13){if(e=c(t),e!==null)return e;e=null}else if(n===31){if(e=l(t),e!==null)return e;e=null}else if(n===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return fp=e,null}function mp(e){switch(e){case`beforetoggle`:case`cancel`:case`click`:case`close`:case`contextmenu`:case`copy`:case`cut`:case`auxclick`:case`dblclick`:case`dragend`:case`dragstart`:case`drop`:case`focusin`:case`focusout`:case`input`:case`invalid`:case`keydown`:case`keypress`:case`keyup`:case`mousedown`:case`mouseup`:case`paste`:case`pause`:case`play`:case`pointercancel`:case`pointerdown`:case`pointerup`:case`ratechange`:case`reset`:case`resize`:case`seeked`:case`submit`:case`toggle`:case`touchcancel`:case`touchend`:case`touchstart`:case`volumechange`:case`change`:case`selectionchange`:case`textInput`:case`compositionstart`:case`compositionend`:case`compositionupdate`:case`beforeblur`:case`afterblur`:case`beforeinput`:case`blur`:case`fullscreenchange`:case`focus`:case`hashchange`:case`popstate`:case`select`:case`selectstart`:return 2;case`drag`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`mousemove`:case`mouseout`:case`mouseover`:case`pointermove`:case`pointerout`:case`pointerover`:case`scroll`:case`touchmove`:case`wheel`:case`mouseenter`:case`mouseleave`:case`pointerenter`:case`pointerleave`:return 8;case`message`:switch(Ee()){case De:return 2;case Oe:return 8;case ke:case Ae:return 32;case je:return 268435456;default:return 32}default:return 32}}var hp=!1,gp=null,_p=null,vp=null,yp=new Map,bp=new Map,xp=[],Sp=`mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset`.split(` `);function Cp(e,t){switch(e){case`focusin`:case`focusout`:gp=null;break;case`dragenter`:case`dragleave`:_p=null;break;case`mouseover`:case`mouseout`:vp=null;break;case`pointerover`:case`pointerout`:yp.delete(t.pointerId);break;case`gotpointercapture`:case`lostpointercapture`:bp.delete(t.pointerId)}}function wp(e,t,n,r,i,a){return e===null||e.nativeEvent!==a?(e={blockedOn:t,domEventName:n,eventSystemFlags:r,nativeEvent:a,targetContainers:[i]},t!==null&&(t=gt(t),t!==null&&ap(t)),e):(e.eventSystemFlags|=r,t=e.targetContainers,i!==null&&t.indexOf(i)===-1&&t.push(i),e)}function Tp(e,t,n,r,i){switch(t){case`focusin`:return gp=wp(gp,e,t,n,r,i),!0;case`dragenter`:return _p=wp(_p,e,t,n,r,i),!0;case`mouseover`:return vp=wp(vp,e,t,n,r,i),!0;case`pointerover`:var a=i.pointerId;return yp.set(a,wp(yp.get(a)||null,e,t,n,r,i)),!0;case`gotpointercapture`:return a=i.pointerId,bp.set(a,wp(bp.get(a)||null,e,t,n,r,i)),!0}return!1}function Ep(e){var t=ht(e.target);if(t!==null){var n=s(t);if(n!==null){if(t=n.tag,t===13){if(t=c(n),t!==null){e.blockedOn=t,it(e.priority,function(){op(n)});return}}else if(t===31){if(t=l(n),t!==null){e.blockedOn=t,it(e.priority,function(){op(n)});return}}else if(t===3&&n.stateNode.current.memoizedState.isDehydrated){e.blockedOn=n.tag===3?n.stateNode.containerInfo:null;return}}}e.blockedOn=null}function Dp(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var n=dp(e.nativeEvent);if(n===null){n=e.nativeEvent;var r=new n.constructor(n.type,n);en=r,n.target.dispatchEvent(r),en=null}else return t=gt(n),t!==null&&ap(t),e.blockedOn=n,!1;t.shift()}return!0}function Op(e,t,n){Dp(e)&&n.delete(t)}function kp(){hp=!1,gp!==null&&Dp(gp)&&(gp=null),_p!==null&&Dp(_p)&&(_p=null),vp!==null&&Dp(vp)&&(vp=null),yp.forEach(Op),bp.forEach(Op)}function Ap(e,n){e.blockedOn===n&&(e.blockedOn=null,hp||(hp=!0,t.unstable_scheduleCallback(t.unstable_NormalPriority,kp)))}var jp=null;function Mp(e){jp!==e&&(jp=e,t.unstable_scheduleCallback(t.unstable_NormalPriority,function(){jp===e&&(jp=null);for(var t=0;t<e.length;t+=3){var n=e[t],r=e[t+1],i=e[t+2];if(typeof r!=`function`){if(pp(r||n)===null)continue;break}var a=gt(n);a!==null&&(e.splice(t,3),t-=3,ws(a,{pending:!0,data:i,method:n.method,action:r},r,i))}}))}function Np(e){function t(t){return Ap(t,e)}gp!==null&&Ap(gp,e),_p!==null&&Ap(_p,e),vp!==null&&Ap(vp,e),yp.forEach(t),bp.forEach(t);for(var n=0;n<xp.length;n++){var r=xp[n];r.blockedOn===e&&(r.blockedOn=null)}for(;0<xp.length&&(n=xp[0],n.blockedOn===null);)Ep(n),n.blockedOn===null&&xp.shift();if(n=(e.ownerDocument||e).$$reactFormReplay,n!=null)for(r=0;r<n.length;r+=3){var i=n[r],a=n[r+1],o=i[st]||null;if(typeof a==`function`)o||Mp(n);else if(o){var s=null;if(a&&a.hasAttribute(`formAction`)){if(i=a,o=a[st]||null)s=o.formAction;else if(pp(i)!==null)continue}else s=o.action;typeof s==`function`?n[r+1]=s:(n.splice(r,3),r-=3),Mp(n)}}}function Pp(){function e(e){e.canIntercept&&e.info===`react-transition`&&e.intercept({handler:function(){return new Promise(function(e){return i=e})},focusReset:`manual`,scroll:`manual`})}function t(){i!==null&&(i(),i=null),r||setTimeout(n,20)}function n(){if(!r&&!navigation.transition){var e=navigation.currentEntry;e&&e.url!=null&&navigation.navigate(e.url,{state:e.getState(),info:`react-transition`,history:`replace`})}}if(typeof navigation==`object`){var r=!1,i=null;return navigation.addEventListener(`navigate`,e),navigation.addEventListener(`navigatesuccess`,t),navigation.addEventListener(`navigateerror`,t),setTimeout(n,100),function(){r=!0,navigation.removeEventListener(`navigate`,e),navigation.removeEventListener(`navigatesuccess`,t),navigation.removeEventListener(`navigateerror`,t),i!==null&&(i(),i=null)}}}function Fp(e){this._internalRoot=e}Ip.prototype.render=Fp.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(a(409));var n=t.current;np(n,fu(),e,t,null,null)},Ip.prototype.unmount=Fp.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;np(e.current,2,null,e,null,null),yu(),t[ct]=null}};function Ip(e){this._internalRoot=e}Ip.prototype.unstable_scheduleHydration=function(e){if(e){var t=rt();e={blockedOn:null,target:e,priority:t};for(var n=0;n<xp.length&&t!==0&&t<xp[n].priority;n++);xp.splice(n,0,e),n===0&&Ep(e)}};var Lp=r.version;if(Lp!==`19.2.8`)throw Error(a(527,Lp,`19.2.8`));P.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render==`function`?Error(a(188)):(e=Object.keys(e).join(`,`),Error(a(268,e)));return e=f(t),e=e===null?null:p(e),e=e===null?null:e.stateNode,e};var Rp={bundleType:0,version:`19.2.8`,rendererPackageName:`react-dom`,currentDispatcherRef:N,reconcilerVersion:`19.2.8`};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<`u`){var zp=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!zp.isDisabled&&zp.supportsFiber)try{Pe=zp.inject(Rp),Fe=zp}catch{}}e.createRoot=function(e,t){if(!o(e))throw Error(a(299));var n=!1,r=``,i=qs,s=Js,c=Ys;return t!=null&&(!0===t.unstable_strictMode&&(n=!0),t.identifierPrefix!==void 0&&(r=t.identifierPrefix),t.onUncaughtError!==void 0&&(i=t.onUncaughtError),t.onCaughtError!==void 0&&(s=t.onCaughtError),t.onRecoverableError!==void 0&&(c=t.onRecoverableError)),t=ep(e,1,!1,null,null,n,r,null,i,s,c,Pp),e[ct]=t.current,Sd(e),new Fp(t)}})),P=t(((e,t)=>{function n(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>`u`||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!=`function`))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n)}catch(e){console.error(e)}}n(),t.exports=N()})),F=e(n(),1),re=P(),ie={wrapper:`_wrapper_1xh50_2`,header:`_header_1xh50_10`,headerActions:`_headerActions_1xh50_21`,title:`_title_1xh50_27`,panelGroup:`_panelGroup_1xh50_36`,panelToggle:`_panelToggle_1xh50_43`,helpToggle:`_helpToggle_1xh50_66`,helpButtonWrapper:`_helpButtonWrapper_1xh50_93`,helpTogglePulsing:`_helpTogglePulsing_1xh50_97`,helpPulse:`_helpPulse_1xh50_1`,helpHint:`_helpHint_1xh50_112`,helpHintFading:`_helpHintFading_1xh50_139`,helpHintKbd:`_helpHintKbd_1xh50_144`,resizeHandle:`_resizeHandle_1xh50_153`},ae=e=>{try{return!new DOMParser().parseFromString(e.trim(),`text/xml`).querySelector(`parsererror`)}catch{return!1}},I=e=>{try{return JSON.parse(e),!0}catch{return!1}},L=e=>e.trim()?I(e)?{valid:!0,error:null,type:`json`}:ae(e)?{valid:!0,error:null,type:`xml`}:{valid:!1,error:`Invalid JSON/XML format`,type:null}:{valid:!0,error:null,type:null},oe=e=>{try{let t=JSON.parse(e);return JSON.stringify(t,null,2)}catch{return e}},se=()=>{let[e,t]=(0,F.useState)([]),n=(0,F.useRef)(0),r=(0,F.useRef)(new Set);return(0,F.useEffect)(()=>()=>{r.current.forEach(clearTimeout)},[]),{toasts:e,addToast:(0,F.useCallback)((e,i=`info`,a)=>{let o=++n.current;t(t=>[...t,{id:o,message:e,type:i,action:a?.action}]);let s=setTimeout(()=>{r.current.delete(s),t(e=>e.filter(e=>e.id!==o))},a?.durationMs??3e3);r.current.add(s)},[]),removeToast:(0,F.useCallback)(e=>{t(t=>t.filter(t=>t.id!==e))},[])}},ce=(e,t)=>{let n=(0,F.useCallback)(()=>{try{let n=window.localStorage.getItem(e);return n?JSON.parse(n):t}catch{return t}},[e]),[r,i]=(0,F.useState)(n);return(0,F.useEffect)(()=>{i(n())},[e]),(0,F.useEffect)(()=>{try{window.localStorage.setItem(e,JSON.stringify(r))}catch(t){console.error(`Error setting localStorage key "${e}":`,t)}},[e,r]),(0,F.useEffect)(()=>{let t=t=>{(t.key===e||t.key===null)&&i(n())};return window.addEventListener(`storage`,t),()=>window.removeEventListener(`storage`,t)},[e,n]),(0,F.useEffect)(()=>{let e=()=>i(n());return window.addEventListener(`focus`,e),document.addEventListener(`visibilitychange`,e),()=>{window.removeEventListener(`focus`,e),document.removeEventListener(`visibilitychange`,e)}},[n]),[r,i]},le=2e4,ue=[{path:`/json-path`,label:`JSON-Path`,title:`JSON-Path Playground`,wsPath:`/ws/json/path`,storageKeyPayload:`jsonpath-last-payload`,storageKeyHistory:`jsonpath-command-history`,storageKeyTab:`jsonpath-right-tab`,storageKeyHelpTopic:`jsonpath-help-topic`,supportsUpload:!0,supportsHelp:!0,helpContentProfile:`json-path`,tabs:[`payload`]},{path:`/`,label:`Minigraph`,title:`Minigraph Playground`,wsPath:`/ws/graph/playground`,storageKeyPayload:`minigraph-last-payload`,storageKeyHistory:`minigraph-command-history`,storageKeyTab:`minigraph-right-tab`,storageKeySavedGraphs:`minigraph-saved-graphs`,storageKeyHelpTopic:`minigraph-help-topic`,supportsClipboard:!0,supportsHelp:!0,helpContentProfile:`minigraph`,supportsAuthoring:!0,supportsGraphRun:!0,supportsSessionCollaboration:!0,tabs:[`graph`,`graph-data`]}],de={json_simple:JSON.stringify({name:`John Doe`,age:30,city:`New York`},null,2),json_nested:JSON.stringify({user:{name:`Jane Smith`,profile:{email:`jane@example.com`,address:{city:`San Francisco`,country:`USA`}}}},null,2),json_array:JSON.stringify([{id:1,name:`Item 1`,status:`active`},{id:2,name:`Item 2`,status:`pending`},{id:3,name:`Item 3`,status:`inactive`}],null,2),xml_simple:`<?xml version="1.0" encoding="UTF-8"?>
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
</items>`};function fe(e){return`ws://${window.location.host}${e}`}var R=k();function pe(e,t,n,r){let i=e[t]??{phase:`idle`,connectionEpoch:null,messages:[]},a=[...i.messages,{id:n,raw:r}];return a.length>200&&a.shift(),{...e,[t]:{...i,messages:a}}}function me(e,t){let n=e[t.path]??{phase:`idle`,connectionEpoch:null,messages:[]};switch(t.type){case`CONNECTING`:return{...e,[t.path]:{...n,phase:`connecting`}};case`CONNECTED`:return pe({...e,[t.path]:{...n,phase:`connected`,connectionEpoch:t.id}},t.path,t.id,t.msg);case`MESSAGE_RECEIVED`:return pe(e,t.path,t.id,t.msg);case`DISCONNECTED`:return pe({...e,[t.path]:{...n,phase:`idle`}},t.path,t.id,t.msg);case`CONNECT_ERROR`:return{...e,[t.path]:{...n,phase:`idle`}};case`CLEAR_MESSAGES`:return{...e,[t.path]:{...n,messages:[]}};default:return e}}var he=(0,F.createContext)(null);function ge({children:e}){let[t,n]=(0,F.useReducer)(me,{}),r=(0,F.useRef)({}),i=(0,F.useRef)({}),a=(0,F.useRef)({});(0,F.useEffect)(()=>()=>{Object.entries(r.current).forEach(([e,t])=>{t?.close();let n=i.current[e];n&&clearInterval(n)})},[]);let o=e=>fe(e),s=e=>(a.current[e]=(a.current[e]??0)+1,a.current[e]),c=()=>{let e=new Date,t=e=>String(e).padStart(2,`0`);return`${t(e.getHours())}:${t(e.getMinutes())}:${t(e.getSeconds())}`},l=(e,t)=>JSON.stringify({type:e,message:t,time:c()}),u=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t){let e=t.type;return e===`ping`||e===`pong`}}catch{}return!1},d=(0,F.useCallback)((e,t)=>{if(!window.WebSocket){t?.(`WebSocket not supported by your browser`,`error`);return}let a=r.current[e];if(a&&(a.readyState===WebSocket.OPEN||a.readyState===WebSocket.CONNECTING)){t?.(`Already connected`,`error`);return}n({type:`CONNECTING`,path:e});let c=new WebSocket(o(e));r.current[e]=c,c.onopen=()=>{n({type:`CONNECTED`,path:e,id:s(e),msg:l(`info`,`connected`)}),t?.(`Connected to WebSocket`,`success`),c.send(JSON.stringify({type:`welcome`})),i.current[e]=setInterval(()=>{c.readyState===WebSocket.OPEN&&c.send(l(`ping`,`keep alive`))},le)},c.onmessage=t=>{u(t.data)||n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t.data})},c.onerror=()=>{n({type:`CONNECT_ERROR`,path:e})},c.onclose=a=>{let o=i.current[e];o&&(clearInterval(o),i.current[e]=null),n({type:`DISCONNECTED`,path:e,id:s(e),msg:l(`info`,`disconnected - (${a.code}) ${a.reason}`)}),t?.(`Disconnected from WebSocket`,`info`),r.current[e]===c&&(r.current[e]=null)}},[]),f=(0,F.useCallback)(e=>{let t=r.current[e];t?t.close():n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:l(`error`,`already disconnected`)})},[]);(0,F.useEffect)(()=>(ue.forEach(e=>{d(e.wsPath)}),()=>{ue.forEach(e=>{let t=r.current[e.wsPath];t&&t.close()})}),[]);let p=(0,F.useCallback)((e,t)=>{let n=r.current[e];return n&&n.readyState===WebSocket.OPEN?(n.send(t),!0):!1},[]),m=(0,F.useCallback)((e,t)=>{n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t})},[]),h=(0,F.useCallback)(e=>{n({type:`CLEAR_MESSAGES`,path:e})},[]),[g,_]=(0,F.useState)({}),v=(0,F.useCallback)((e,t)=>{_(n=>{if(t===null){let t={...n};return delete t[e],t}return{...n,[e]:t}})},[]),y=(0,F.useCallback)(e=>g[e]??null,[g]),b=(0,F.useCallback)(e=>{let t=g[e]??null;return t!==null&&_(t=>{let n={...t};return delete n[e],n}),t},[g]),x=(0,F.useCallback)(e=>t[e]??{phase:`idle`,connectionEpoch:null,messages:[]},[t]),S=(0,F.useMemo)(()=>({getSlot:x,connect:d,disconnect:f,send:p,appendMessage:m,clearMessages:h,setPendingPayload:v,peekPendingPayload:y,takePendingPayload:b}),[x,d,f,p,m,h,v,y,b]);return(0,R.jsx)(he.Provider,{value:S,children:e})}function _e(){let e=(0,F.useContext)(he);if(!e)throw Error(`useWebSocketContext must be used inside <WebSocketProvider>`);return e}var ve=e=>{try{let t=JSON.parse(e);return{type:t.type||`info`,message:t.message||e,time:t.time,raw:e}}catch{return{type:`raw`,message:e,time:null,raw:e}}},ye=e=>({info:`ℹ️`,error:`❌`,ping:`🔄`,welcome:`👋`,raw:``})[e]??`•`,be=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t)return{isJSON:!0,data:t}}catch{}return{isJSON:!1,data:null}};function xe(e){if(!e.includes(`Graph exported to `))return null;let t=we(e);if(!t)return null;let n=t.split(`/`)[4];return n?{graphName:n,apiPath:t}:null}function Se(e){return e.includes(`Invalid filename`)?{reason:`invalid-name`}:e.includes(`Expect root node name`)?{reason:`root-name-conflict`}:null}function Ce(e){let t=be(e);return t.isJSON?(t.data.type,!1):!0}function we(e){let t=e.match(/\/api\/graph\/model\/([^\s'"]+)/);return t?t[0]:null}function Te(e){return Ce(e)?we(e)!==null:!1}function Ee(e){let t=e.match(/\/api\/json\/content\/([\w-]+)/);return t?t[0]:null}function De(e){let t=e.match(/Large payload \((\d+)\)\s*->\s*GET\s+(\/api\/inspect\/[^\s]+)/i);if(!t)return null;let n=parseInt(t[1],10),r=t[2];return{apiPath:r,byteSize:n,filename:`${r.split(`/`).filter(Boolean).pop()??`payload`}.json`}}function Oe(e){let t=e.match(/You may upload .*?->\s*POST\s+(\/api\/mock\/[\w-]+)/i);return t?t[1]:null}function ke(e){if(!e.startsWith(`> `))return!1;let t=e.slice(2).trim().toLowerCase();return t===`help`||t.startsWith(`help `)?!0:t.startsWith(`describe `)?!t.slice(9).trim().startsWith(`graph`):!1}function Ae(e){if(!e.startsWith(`> `)||!e.slice(2).trimStart().toLowerCase().startsWith(`import graph from `))return null;let t=e.slice(2).trimStart().slice(18).trim();return t.length>0?t:null}var je=/^node ([A-Za-z0-9_-]+) created$/i,Me=/^node ([A-Za-z0-9_-]+) already exists$/i,Ne=/^node ([A-Za-z0-9_-]+) updated$/i,Pe=/^node ([A-Za-z0-9_-]+) deleted$/i,Fe=/^node ([A-Za-z0-9_-]+) connected to ([A-Za-z0-9_-]+)$/i,Ie=/^node ([A-Za-z0-9_-]+) not found$/i,Le=/^Source and target nodes must be different$/i,Re=/^Syntax: connect \{node-A\} to \{node-B\} with \{relation\}$/i,ze=/^ERROR: (.+)$/;function Be(e){let t=e.trim();if(t.startsWith(`> `))return null;let n=t.match(je);if(n)return{status:`accepted`,action:`create-node`,alias:n[1],targetAlias:null,message:t};let r=t.match(Me);if(r)return{status:`rejected`,action:`create-node`,alias:r[1],targetAlias:null,message:t};let i=t.match(Ne);if(i)return{status:`accepted`,action:`edit-node`,alias:i[1],targetAlias:null,message:t};let a=t.match(Pe);if(a)return{status:`accepted`,action:`delete-node`,alias:a[1],targetAlias:null,message:t};let o=t.match(Fe);if(o)return{status:`accepted`,action:`create-connection`,alias:o[1],targetAlias:o[2],message:t};let s=t.match(Ie);return s?{status:`rejected`,action:null,alias:s[1],targetAlias:null,message:t}:Le.test(t)||Re.test(t)?{status:`rejected`,action:`create-connection`,alias:null,targetAlias:null,message:t}:t.match(ze)?{status:`error`,action:null,alias:null,targetAlias:null,message:t}:null}function Ve(e){if(!Ce(e)||e.startsWith(`> `)||Te(e))return null;let t=e.toLowerCase();return t.includes(`graph model imported as draft`)?`import-graph`:t.includes(` -> `)&&t.includes(`removed`)||t.startsWith(`node `)&&(t.includes(` created`)||t.includes(` updated`)||t.includes(` deleted`)||t.includes(` connected to `)||t.includes(` imported from `)||t.includes(` overwritten by node from `))?`node-mutation`:null}var He={command:``,historyIndex:-1,draftCommand:``};function Ue(e,t){switch(t.type){case`SET_COMMAND`:return{...e,command:t.value,historyIndex:-1,draftCommand:``};case`CLEAR_COMMAND`:return{...e,command:``,historyIndex:-1,draftCommand:``};case`SET_HISTORY_INDEX`:return{...e,historyIndex:t.index,command:t.command};case`ENTER_HISTORY`:return{...e,historyIndex:0,command:t.command,draftCommand:e.command};case`EXIT_HISTORY`:return{...e,historyIndex:-1,command:e.draftCommand,draftCommand:``};default:return e}}function We({wsPath:e,storageKeyHistory:t,payload:n,addToast:r,bus:i,handleLocalCommand:a}){let o=_e(),{phase:s,connectionEpoch:c,messages:l}=o.getSlot(e),u=s===`connected`,d=s===`connecting`,[f,p]=(0,F.useReducer)(Ue,He),{command:m,historyIndex:h}=f,[g,_]=ce(t,[]),v=(0,F.useRef)(null),y=(0,F.useRef)(!1);(0,F.useEffect)(()=>{v.current&&(v.current.scrollTop=v.current.scrollHeight)},[l]);let b=(0,F.useCallback)(()=>{o.connect(e,r)},[o,e,r]),x=(0,F.useCallback)(()=>{o.disconnect(e)},[o,e]),S=(0,F.useCallback)(()=>{if(s!==`connected`)return;let t=m.trim();if(t.length!==0){if(a?.(t)===!0){g[0]!==t&&_(e=>[t,...e].slice(0,50)),o.appendMessage(e,`> `+t),p({type:`CLEAR_COMMAND`});return}o.send(e,t),g[0]!==t&&_(e=>[t,...e].slice(0,50)),t===`load`&&(n.length===0?o.appendMessage(e,`ERROR: please paste JSON/XML payload in input text area`):o.send(e,n)),p({type:`CLEAR_COMMAND`})}},[o,e,s,m,n,g,_,a]),C=(0,F.useCallback)(e=>{if(e.key===`ArrowUp`){if(e.preventDefault(),g.length===0)return;if(h===-1)p({type:`ENTER_HISTORY`,command:g[0]});else if(h<g.length-1){let e=h+1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}}else if(e.key===`ArrowDown`)if(e.preventDefault(),h<=0)h===0&&p({type:`EXIT_HISTORY`});else{let e=h-1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}},[g,h]);(0,F.useEffect)(()=>{if(i)return i.on(`upload.contentPath`,t=>{if(!y.current)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let i;try{i=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(t.uploadPath,{method:`POST`,headers:{"Content-Type":`application/json`},body:i}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})})},[i,n,e,o,r]),(0,F.useEffect)(()=>{if(i||!y.current||l.length===0)return;let t=l[l.length-1].raw,a=Ee(t);if(!a)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let s;try{s=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(a,{method:`POST`,headers:{"Content-Type":`application/json`},body:s}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})},[i,l,n,e,o,r]);let w=(0,F.useCallback)(()=>{if(s===`connected`){if(n.length===0){r(`Nothing to upload — paste a JSON payload first`,`error`);return}y.current=!0,o.send(e,`upload`)}},[o,e,s,n,r]),T=(0,F.useCallback)(t=>s===`connected`&&o.send(e,t),[o,e,s]),E=(0,F.useCallback)(()=>{navigator.clipboard.writeText(l.map(e=>e.raw).join(`
`)),r(`Console copied to clipboard!`,`success`)},[l,r]),D=(0,F.useCallback)(()=>{o.clearMessages(e),r(`Console cleared`,`info`)},[o,e,r]),O=(0,F.useCallback)(t=>{o.appendMessage(e,t)},[o,e]);return{connected:u,connecting:d,connectionEpoch:c,messages:l,command:m,setCommand:(0,F.useCallback)(e=>p({type:`SET_COMMAND`,value:e}),[]),connect:b,disconnect:x,sendCommand:S,handleKeyDown:C,consoleRef:v,copyMessages:E,clearMessages:D,uploadPayload:w,sendRawText:T,appendMessage:O,history:g}}function Ge(e){let[t,n]=(0,F.useState)(()=>window.matchMedia(e).matches);return(0,F.useEffect)(()=>{let t=window.matchMedia(e),r=e=>n(e.matches);return t.addEventListener(`change`,r),()=>t.removeEventListener(`change`,r)},[e]),t}function Ke(e){if(typeof e!=`object`||!e)return!1;let t=e;return Array.isArray(t.nodes)}function qe(e,t,n){let r=t.includes(n)?n:t[0]??`graph`;return typeof e==`string`&&t.includes(e)?e:r}function Je(e){return Ke(e)&&e.nodes.length>0?e:null}function Ye(e,t,n,r,i){let[a,o]=(0,F.useState)(null),[s,c]=ce(i,n),l=qe(s,r,n),[u,d]=(0,F.useState)(!1),f=(0,F.useCallback)(e=>{c(t=>{let i=qe(t,r,n);return qe(typeof e==`function`?e(i):e,r,n)})},[c,r,n]);(0,F.useEffect)(()=>{s!==l&&c(l)},[s,l,c]);let p=(0,F.useRef)(e);(0,F.useEffect)(()=>{p.current=e},[e]);let m=(0,F.useRef)(!1);(0,F.useEffect)(()=>{m.current=a!==null},[a]);let h=(0,F.useRef)(null);(0,F.useEffect)(()=>{if(!e){o(null);return}let t=new AbortController;return o(null),fetch(e,{signal:t.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{let t=Je(e);t&&(o(t),f(`graph`))}).catch(()=>{}),()=>{t.abort()}},[e]);let g=(0,F.useCallback)(()=>{let e=p.current;if(!e)return;h.current?.abort();let n=new AbortController;h.current=n,d(!0),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{let t=Je(e),n=t!==null&&!m.current;o(t),n&&f(`graph`),d(!1)}).catch(e=>{e.name!==`AbortError`&&(t(`Graph refresh failed: ${e.message}`,`error`),d(!1))})},[]);return(0,F.useEffect)(()=>()=>{h.current?.abort()},[]),{graphData:a,setGraphData:o,rightTab:l,setRightTab:f,isRefreshing:u,refetchGraph:g}}function Xe({bus:e,hasGraph:t,connected:n,refetchGraph:r,clearGraph:i,addToast:a}){let o=(0,F.useRef)(null),s=(0,F.useRef)(t),c=(0,F.useRef)(n),l=(0,F.useRef)(r);(0,F.useEffect)(()=>{s.current=t},[t]),(0,F.useEffect)(()=>{c.current=n},[n]),(0,F.useEffect)(()=>{l.current=r},[r]),(0,F.useEffect)(()=>{!n&&o.current!==null&&(clearTimeout(o.current),o.current=null)},[n]),(0,F.useEffect)(()=>e.on(`graph.mutation`,e=>{if(c.current){if(e.mutationType===`import-graph`){o.current!==null&&(clearTimeout(o.current),o.current=null),l.current(),a(`Graph imported — refreshing view…`,`info`);return}o.current!==null&&clearTimeout(o.current),o.current=setTimeout(()=>{o.current=null,c.current&&(l.current(),a(s.current?`Graph updated — refreshing…`:`Graph updated — opening Graph tab…`,`info`))},300)}}),[e,a]),(0,F.useEffect)(()=>e.on(`session.reset`,()=>{o.current!==null&&(clearTimeout(o.current),o.current=null),i()}),[e,i]),(0,F.useEffect)(()=>()=>{o.current!==null&&clearTimeout(o.current)},[])}function Ze(e){return e.replace(/^help\s*/i,``).trim().toLowerCase()}function Qe(e){let t=e.trim(),n=t.toLowerCase();if(n===`help`||n.startsWith(`help `))return Ze(t);let[r,i,a,...o]=n.split(/\s+/);return r===`describe`&&i===`skill`&&a!==void 0&&o.length===0?a.replace(/\./g,`-`):null}var $e=`Connect two nodes
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
`,et=`Create a node
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
- Node names are matched case-insensitively; use lowercase letters, digits
  and hyphen. The names 'root' and 'end' are reserved: the root node must be
  named 'root' and the end node must be named 'end'.
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
`,tt=`Data Dictionary
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
`,nt=`Delete a node, a connection or the fetch cache
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
`,rt=`Describe graph, node, connection or skill
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
`,it=`Edit a node
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
`,at=`Execute a single node
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
`,ot=`Export a graph model
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
- In the Playground UI, the Download button beside Copy (Graph and Raw views)
  saves the model to your computer as {name}.json after asking for the graph
  id; the root node's name in the file becomes that id, as this command does.
  Browsers with a "save as" dialog let you pick the folder; others save to
  the download folder. Drop the file on the canvas, or use "Import Graph", to
  load it into a session again.
`,st=`Skill: Graph API Fetcher
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
`,ct=`Skill: Graph Data Mapper
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

Decision table lookup
---------------------
A data mapper is also the natural decision node for a static decision table held on a skill-less node
(see "describe skill graph.task" for the table node). The "lookup" simple plugin returns the name of
the first rule whose list contains the value (compared as text, case-insensitively), or the optional
third argument on a miss (null when it is omitted):

\`\`\`
create node select-rule
with type Decision
with properties
skill=graph.data.mapper
mapping[]=f:lookup(state-rules, input.body.state, text(unknown)) -> output.body.rule
\`\`\`

The table's "keys" field lists the rule names in priority order and each rule field lists its values;
each may be a list or a JSON array written as text (keys=[ "a", "b" ]), and the table itself may be
JSON text. One table replaces a ladder of IF-THEN-ELSE and the product owner certifies it on the graph.

When a rule is a band rather than a list of values - a tier by amount, an eligibility threshold, a
time window - f:rangeLookup(table, value, default) returns the first band whose range contains the
value. Each band field is ONE range written as text: min..max runs from min inclusive up to but
excluding max, an omitted end is open, so adjacent bands tile a scale with no gap and no overlap - a
band that ends at a value is written with the next value as its upper bound. A bound is a canonical
number (compared exactly as a decimal; a non-numeric value fails by name), text in single or double quotes
('0020', "2026-11-15" - compared as written, case-sensitively; the quotes keep a zero-padded code as text, and
a quoted bound may hold the other quote character, never its own) or unquoted text (compared without case).
Text orders by Unicode code point. The value must be a number or text - a boolean, a list or a map fails by
name. A table that mixes kinds is refused, and so is a band given as a list. Text compares as written, so a
value must be shaped like the bounds: dates and times in ISO-8601 order and zero-padded, the value carrying the
same parts as the bounds - 9:30 is not 09:30 - and one UTC offset and one precision throughout, UTC by
preference (17:00:00+02:00 sorts after 16:00:00Z; 15:00:00.5Z sorts before 15:00:00Z).

\`\`\`
create node loyalty-tiers
with type DecisionTable
with properties
keys=[ "platinum", "gold", "silver", "standard" ]
platinum=25000..
gold=10000..25000
silver=2500..10000
standard=..2500

mapping[]=f:rangeLookup(loyalty-tiers, input.body.annual_spend, text(unknown)) -> output.body.tier
\`\`\`

The bounds of a band are data too: f:minRange(range, default) and f:maxRange(range, default) read one
range in the same notation and return its lower or upper bound, typed by the notation - a numeric bound
is a long for a number written without a decimal point and the exact decimal text for any other number (the
DECIMAL rule), a text bound is text as written, a quoted one without its quotes - and an open end answers the
optional default, or null when there is none; a null range answers null, never the default. A dynamic key
reads the band the lookup chose:

\`\`\`
mapping[]=f:rangeLookup(loyalty-tiers, input.body.annual_spend, text(unknown)) -> model.tier
mapping[]=f:minRange(loyalty-tiers.{model.tier}, int(0)) -> output.body.tier_from
mapping[]=f:maxRange(loyalty-tiers.{model.tier}) -> output.body.tier_to
\`\`\`

tier_from is 10000 for gold and the default 0 for standard (the lower end of ..2500 is open); tier_to is
25000 for gold and stays unset for platinum (25000.. has no upper bound). On a miss, model.tier is the default
unknown, loyalty-tiers.unknown is nothing, and a null range answers null, never the default, so tier_from and
tier_to stay unset and the run never aborts. A range is refused exactly when rangeLookup would refuse the band -
malformed (a quoted bound holding its own quote character included), a list, mixing kinds (a number and text,
or quoted and unquoted), or empty (3..3 holds nothing, the upper bound being excluded) - and the lookup checks
every band when it reads a table. A decimal bound
stays text so that nothing is rounded; f:double(model.mid_to) in a second mapping converts it when a
floating-point output value is wanted.

Dynamic keys
------------
A {namespace.key} reference inside a source resolves before the source is read, and its value is
inserted verbatim, never quoted. It composes a key segment, a list index, or text in a constant or a
plugin argument:

\`\`\`
mapping[]=census-2020.{model.state} -> model.population
mapping[]=input.body.items[{model.i}] -> output.body.item
mapping[]=text(Hello {input.body.name}!) -> output.body.greeting
\`\`\`

census-2020.{model.state} reads the property that model.state names: a keyed table (one KEY=value
line per key) answers a value per key, where f:lookup answers which rule lists a value.

- The reference may read any namespace (Event Script accepts model.* only), but only on the source
  side: a target is a literal path.
- An unresolved reference renders the text null, so a composed key misses and a constant reads
  "null". A composed key is case-sensitive, unlike f:lookup.
- Nothing is quoted, whatever a text constant contains ("!", "<", ">", "=="). A JSONPath filter is
  the one exception: a text value is quoted there so it reads as a string literal in the query.
- Keep a fixed prefix before a request value (census-2020.{input.body.state}) so the request
  chooses only within that table. A table value is text, and f:long(null) is -1, so refuse an
  unknown key before converting.

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
- A NULL source (a missing key, or a plugin returning null) CLEARS a "model."
  target (removed; set to null when the source key exists or the target is
  indexed such as model.list[1]) and leaves any other target untouched - the
  same rule as Event Script. Defaults for a model variable come from the
  source side: f:defaultValue(input.body.flag, boolean(false)) -> model.flag,
  or a plugin default such as f:lookup(table, value, text(unknown)) - never
  default-then-overlay, which the null overlay would remove.
- The legacy colon-type suffix ("simple type matching") is deprecated - use
  the f:plugin forms instead.
- Inside a graph.math node, MAPPING: statements use exactly this syntax; see
  'help graph-math'.
`,lt=`Skill: Graph Extension
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
`,ut=`Skill: Graph Island
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
`,dt=`Skill: Graph Join
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
`,ft=`Skill: Graph JS (deprecated)
----------------------------
The graph.js skill evaluates inline JavaScript statements for computation and
decision-making. It is DEPRECATED: do not author new graph.js nodes. Use
graph.math for inline computation and IF/THEN/ELSE decisions - its expression
dialect covers the same ground without a script engine (see 'help graph-math') -
and graph.task to invoke a composable function for any logic an inline
expression cannot express (see 'help graph-task').

Status by engine
----------------
- Java engine: still registered so that existing graph models keep running;
  the module is scheduled for removal once field installations have migrated.
  Runtime JavaScript is an injection surface, and an equality comparison with a
  quoted string literal was measured silently evaluating to false - the reasons
  are recorded in the skills reference (graph.js section).
- Rust engine: never registered. A node with skill=graph.js fails at execution
  time with:

\`\`\`
Skill graph.js is retired for security reasons - use graph.math or graph.task instead
\`\`\`

Migrating a graph model
-----------------------
When importing an older graph model that contains graph.js nodes, replace
skill=graph.js with graph.math (compute/branch) or graph.task (custom logic)
before running it. The statement grammar has the same shape - COMPUTE, IF,
MAPPING, EXECUTE, RESET, NEXT, DELAY and for_each with BEGIN/END - so most
nodes migrate by changing the skill and rewriting each expression in the
graph.math dialect (a closed set of operators and functions, listed under
'help graph-math').

Reference for existing graph.js nodes (Java engine only)
--------------------------------------------------------
Route name: "graph.js"

Properties:

\`\`\`
skill=graph.js
statement[]=COMPUTE: variable -> JavaScript statement
statement[]=IF: if-then-else statement
statement[]=MAPPING: source -> target
statement[]=EXECUTE: another-node
statement[]=RESET: node-name
\`\`\`

Optional properties:

\`\`\`
for_each[]={map an array parameter for iterative statement execution}
statement[]=BEGIN
statement[]=END
statement[]=NEXT: {next-node-name}
statement[]=DELAY: {milliseconds}
\`\`\`

- Statements execute in order. A node with only MAPPING statements is rejected -
  use graph.data.mapper for mapping-only work.
- A COMPUTE result is stored in the node's "result" namespace; a later MAPPING
  statement can map it onward.
- An IF statement evaluates a boolean operation and may override the natural
  traversal order by jumping to a named node; when every statement resolves to
  "next", natural traversal is preserved.
- A node executes once per run (the run-once guard). RESET: clears a node's
  "seen" status and its result so conditional traversal can run it again;
  use it with care.
- for_each[] with BEGIN/END iterates a statement block over a runtime array,
  NEXT: jumps to a named node and DELAY: pauses before the next node - the same
  rules as graph.math. Every statement command resolves {dynamic variables}, so
  NEXT:/THEN:/ELSE: targets, RESET: entries and DELAY: values may each be a
  {namespace.key} reference (e.g. NEXT: {error.source} in a generic error
  handler).
- The skill is designed for a simple inline JavaScript statement using the
  standard JavaScript library; complex functions and variables are not
  supported.

COMPUTE statement:

\`\`\`
create node demo-js-runner
with properties
skill=graph.js
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
\`\`\`

The syntax {variable_name} resolves a value from the "input." or "model."
namespace or from a node's properties into the statement. A later statement
can use the result of a prior statement as its parameter.

IF statement - a multi-line command:

\`\`\`
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
\`\`\`

THEN: and ELSE: each name the node to jump to, or the keyword "next". When the
JavaScript statement does not return a boolean, the result is coerced: a
positive number is true and a negative number false; the text values "true",
"yes", "T" and "Y" are true and any other text is false; any other value is
converted to text first. (graph.math never coerces - a boolean is never a
number there.)

MAPPING statement - identical to the data mapper, so no curly braces:

\`\`\`
statement[]=MAPPING: input.body.hr_id -> employee.id
statement[]=MAPPING: input.body.join_date -> employee.join_date
\`\`\`

EXECUTE statement - runs another graph.js node's statements:

\`\`\`
statement[]=EXECUTE: js-3
\`\`\`

The "[]" suffix appends one statement per line to the node's statement list.
`,pt=`Skill: Graph Math
-----------------
Fast inline math and boolean evaluation for computation and decision-making.
A node with this skill runs an ordered list of statement[] lines. This is the
skill for inline compute and branching (graph.js is deprecated - see
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
statement[]=DECIMAL: {var} -> {expression}   (exact decimal arithmetic - see below)
statement[]=CONDITION: {var} -> {boolean expression}
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
- DECIMAL: {var} -> {expression} - the high-precision COMPUTE: the expression
  is evaluated with exact decimal arithmetic and the result is stored in the
  node's result namespace as a canonical decimal string (plain notation, the
  computed scale kept, a zero of any scale written "0"). COMPUTE still computes
  in binary floating point, and a graph that never says DECIMAL keeps its
  arithmetic; two rules reach it all the same: a string that is a canonical
  number compares as a number, and round() is half up, away from zero. See
  "DECIMAL statement" below.
- CONDITION: {var} -> {boolean expression} - the declared boolean statement:
  evaluated as a boolean whatever operators it carries (a bare {model.flag}
  included) and stored as a boolean in THIS node's result namespace; an IF
  may test it directly (IF: {this-node}.result.{var}). A COMPUTE stores a
  boolean only when its expression happens to carry a comparison or boolean
  operator - CONDITION says so in the statement.
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

The dialect is a NARROW JavaScript-like subset evaluated by the engine's own
parser, not a JavaScript runtime - a closed set. It accepts exactly the
following; an operator, function or constant not listed here is rejected by
name ("Unknown function: hypot"), never silently. COMPUTE yields a double, so
an integer result serializes as e.g. 8.0 (numerically exact).

\`\`\`
Literals   : numbers (42, 3.14, .5, 1e-5), strings ('text' or "text"), booleans (true, false)
Variables  : {namespace.key} substitution only - e.g. {input.body.qty}, {model.total}, {book.price};
             an unresolved selector fails by name before evaluation
Operators  : **  exponent, right-associative; a unary operand needs parentheses: -(2 ** 2), never -2 ** 2
             unary + - !          * / % (remainder)          + - (+ concatenates when either side is a string)
             < <= > >= (two numbers, or two strings compared lexically)
             == != (same type on both sides)          && || (short-circuit)          test ? a : b          ( )
Functions  : sin, cos, tan, asin, acos, atan, sqrt, abs, floor, ceil, round, log, log10, exp   (one argument)
             pow(x, y)          min(a, b, ...)          max(a, b, ...)          random()
             every function is also available as Math.name, e.g. Math.pow(2, 3)
Constants  : PI, E (also Math.PI, Math.E)
Not in the dialect: bitwise and shift operators (& | ^ ~ <<), assignment (=), user-defined variables
             and functions, arrays, objects, string methods - use a graph.task function instead
\`\`\`

Precedence, tightest first: ** > unary > * / % > + - > relational > equality > && > || > ?:

Numbers and booleans - each rule is enforced by a named failure, never a
silent value:
- A boolean is not a number. A boolean where arithmetic, a < or > comparison
  or a function argument needs a number fails naming the selector, e.g.
  "Boolean operand: model.flag (true) in '{model.flag} + 1' - a boolean is
  not a number; store a boolean with CONDITION or assert the type with
  f:validate". A JSON true in a numeric slot never computes as 1. Equality
  (==, !=) type-checks its two sides; a string that is a canonical number
  counts as a number, so '200' == 200, 200 == '200' and '200' == '200' are the
  same comparison and '9.5' < '10.25' compares 9.5 with 10.25.
- A misspelled or unsupported function fails by name ("Unknown function: mn").
- Arithmetic is IEEE double precision. An overflow to infinity, a division by
  zero and a NaN each fail naming the operator ("Arithmetic overflow in '*'
  (result Infinity)", "Division by zero or arithmetic overflow in '/'");
  integers beyond 2^53 lose precision; round() is half up, away from zero
  (round(-2.5) is -3), the same as f:round. Money that needs exact decimal
  arithmetic or a stated rounding mode belongs in a DECIMAL statement, the
  high-precision COMPUTE; COMPUTE stays floating point.

DECIMAL statement
-----------------
\`\`\`
statement[]=DECIMAL: fee -> {input.body.amount} * {input.body.rate}
statement[]=DECIMAL: rounded -> round({price.result.fee}, 2, HALF_UP)
\`\`\`

Numbers or strings - a conscious decision: a decimal may arrive as a string
("0.0375") or as a JSON number (0.0375), and both give the same answer. A JSON
number is a double, and DECIMAL converts it through the shortest decimal text
it prints as, at its minimal scale (5.0E-4 becomes 0.0005, 100.0 becomes 100).
That is exact over the text received, but a double that was already computed in
floating point is only as exact as that computation: COMPUTE: 1.005 * 100 is
100.49999999999999, and rounding that in a DECIMAL statement gives 100 where
the exact 100.5 gives 101; and a JSON number longer than a double holds was
rounded by the parser. So send money as strings and keep a COMPUTE result out
of a DECIMAL statement; to insist on strings, assert the type:
f:validate(input.body.rate, text(rate; String; required)). Whole numbers are
exact either way. The result is a string on purpose: graph.suspend saves the
state machine and graph.resume restores it, and a string is the same after as
before.

Arithmetic: + - * are exact; / never truncates (the exact quotient when it
terminates, otherwise 34 significant digits, half-even); % is the remainder;
** and pow(x, n) take a whole exponent from -999 to 999; abs, floor, ceil, min
and max are exact. Rounding is always explicit: round(x, scale, mode) with mode
HALF_UP, HALF_EVEN, HALF_DOWN, UP, DOWN, CEILING or FLOOR. What cannot be exact
is refused by name: sqrt, log, log10, exp, trigonometry, random(), PI and E -
keep that step in a COMPUTE or a graph.task function. A DECIMAL statement
computes a number; a comparison may appear only inside a ternary test. A COMPUTE
on a decimal string computes in binary floating point, so use DECIMAL for money.

A zero of any scale is stored as "0" (round(0.004, 2, HALF_UP), 1.50 - 1.50 and
0.00 all give "0"), and "0" + "1.50" is "1.50": the next scaled addend restores
the scale. The remainder (%) follows the sign of the dividend (-7 % 3 is "-1").
The decimal plugins (f:decimalAdd, f:decimalRound ...) work in a MAPPING
statement.

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
so seed the variable before relying on it. A COMPUTE: or IF: expression over an unresolved variable
fails before evaluation and names it ("Unknown identifier: model.backoff (unresolved variable in
'{model.backoff} * 2')") - every unresolved variable when there are several ("Unknown identifier:
model.threshold or model.factor") - so the node that failed to set it can be found. See tutorial 12
for the full generic retry handler.

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
`,mt=`Skill: Graph Resume
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
`,ht=`Skill: Graph Suspend
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
record. When the graph runs as one iteration of a parent's for_each fan-out, the iteration
index is appended as a third segment ("graph:{graph_id}:{cid}:{index}") so concurrent
iterations do not collide. The store must acknowledge with a 2xx reply before the graph completes - a failed
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
`,gt=`Skill: Graph Task
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

Static decision table
---------------------
A lookup table that changes with legislation rather than with each request belongs on a node, not
in the function. It is more readable - the product owner certifies the rules on the graph, in the
business vocabulary - and it replaces a ladder of IF-THEN-ELSE (a chain of graph.math decision nodes,
or conditionals inside a composable function) with one table that grows a row per rule instead of a
branch per rule. Do not hard-code the table in graph.math statements or in a function. Every node's
properties are copied into the state machine when the graph is instantiated, so a skill-less node's
alias is a mapping source and ONE input entry hands the whole table to a generic function:

\`\`\`
create node state-rules
with type DecisionTable
with properties
keys=[ "community-property", "separate-property" ]
community-property=[ "CA", "TX" ]
separate-property=[ "NY" ]
\`\`\`

\`\`\`
input[]=state-rules -> table
input[]=input.body.state -> key
\`\`\`

"keys" names the rules in priority order and each rule lists the values that select it. Every value
is a JSON array written as text, so the node reads as a table in the Playground and the function
reconstructs the lists (SimpleMapper in Java, serde_json in Rust) and reads the rule names from
"table.keys". For the common case no function is needed at all: a graph.data.mapper decision node
resolves the rule in one entry with the "lookup" simple plugin, its optional third argument being the
default for a miss - see "describe skill graph.data.mapper". Variations: "key[]=member" lines build a real list property,
and a nested table may be ONE JSON text property (a multi-line '''...''' value) that f:json parses
at mapping time: input[]=f:json(state-rules.table) -> table. The product owner certifies the table on
the graph and a new table ships as a new graph version, never as a code change. Wire the table node
under the graph's island so that no node is left unconnected.

If the function is declared as a TypedLambdaFunction with a PoJo input class, the request body map
is automatically converted to the PoJo at the function boundary.

Result set
----------
Upon successful execution, the function's response body is stored in the "result" parameter, the
response status in "status" and the response headers in "header" in the properties of the node.
The optional output data mapping can copy them to the 'model.' or 'output.' namespace.
The source of an output entry is a constant, a simple-plugin call (f:...), 'result' or 'result.{key}',
'model.{key}' or this node's own namespace - 'input.*' is valid only on the input side. To echo a request value, stage it at a
graph.data.mapper node (input.body.id -> model.id) and map model.id out; any other source fails the
node with an error that names the entry.

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
`,_t=`Import a graph model or a node
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
- In the Playground UI, the Graph view's "Import Graph" button (also on the
  empty canvas) and a graph JSON file dropped on the canvas import a model
  from your computer as your draft (POST /api/graph/import/{session-id}).
  The file must be a JSON object with a "nodes" section; "connections" is
  optional (a work in progress may have none); any other top-level section
  is refused by name, and a node without alias or types is refused too.
  CompileGraph remains the quality gate when the model is deployed.
- An import replaces the draft of every member of a shared session, like a
  command, and clears a graph instance; a corrupt model reports "Graph model
  not imported" with the reason. The UI asks before replacing a loaded graph.
`,vt=`Inspect the state machine
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
`,yt=`Instantiate a graph instance
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
`,bt=`List nodes, connections, graphs or flows
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
`,xt=`Graph set packaging
-------------------
Pack graph models into one graph set - a <set>.pack file checked by the
deployment gate as it is packed - inspect a set, and deploy it all or none.
This topic explains the Playground panel, the command line and the manifest;
there is no console command behind it.

What a graph set is
-------------------
- One file, <set>.pack, holds one or more graph models. The file name
  without the extension is the set name (letters, digits, '_' and '-').
- Each graph is an entry named <graph-id>.json, and the root node's "name"
  must equal the graph id - the rule of 'export graph as'.
- The manifest carries text fields: 'set' (the set name), 'version',
  'description', 'author', the optional 'graph_id' (the set's entry-point
  graph) and anything else you add; 'format' and 'format_version' are the
  packager's. Nothing comes from the clock: the same graphs and fields
  always give the same bytes, so one file can be signed (a detached
  <set>.pack.sig beside it) and promoted.
- A set of one graph is valid: packing a graph alone is how one graph is
  signed. Only an empty set is refused.

The panel
---------
Open it from the Tools menu: "Graph set packaging". It takes the console's
place; Esc or Cancel gives it back, and the list survives a close.

- Drop several <graph-id>.json files on the panel, or browse for them, and
  "Add current graph" adds the graph in the Graph view named after its id.
  Each entry shows its node and connection counts, and what the engine
  would refuse is flagged in place - an id that breaks the file-name rule,
  a root node named differently from its id, a duplicate id - so the list
  is fixed before anything is packed.
- Name the set (required) and fill the manifest fields you want; a blank
  value is left out, and 'graph_id' must name one of the entries.
- "Pack and download" has the engine pack the set (POST /api/graph-set/pack):
  every graph passes the deployment gate first, and a set that breaks a
  rule is refused with every reason. The file is saved through the
  browser's "save as" dialog, or into its download folder.
- Drop a .pack file on the panel to inspect it (POST /api/graph-set/unpack):
  the manifest and the graphs appear; "Import as draft" loads one graph as
  your session's draft (the UI asks before replacing a loaded graph), and
  "Edit as new set" loads the whole set into the editor to pack it again.

The command line
----------------
The graph packager does the same from a build pipeline - a Java jar
(helpers/graph-packager) or a Rust binary (tools/graph-packager):

\`\`\`
graph-packager pack    --set {name} [--manifest key=value]... [--out {dir}] {graph.json}... | {folder}
graph-packager pack    --set {name} --from-manifest graphs.yaml [--manifest key=value]... [--out {dir}]
graph-packager unpack  {file.pack} --out {dir}
graph-packager inspect {file.pack} [--json]
\`\`\`

'pack' prints the SHA-256 for a signer; exit code 1 is a refused input and
2 an I/O or format error. Both engines, and the panel, pack the same graphs
and fields to the same bytes.

Deploying a set
---------------
List the set in the deployment manifest beside the loose graphs, and name a
file:/ folder the application can write, where the set is unpacked before
the gate reads it. The folder must be outside the Playground's temp folder
(location.graph.temp, /tmp/graph by default):

\`\`\`yaml
location: 'classpath:/graph'      # where <set>.pack is read from
sets:
  - 'my-set'
unpack: 'file:/tmp/unpack'
\`\`\`

- A set registers all of its graphs or none; the ERROR in the startup log
  names every graph the gate refused.
- A duplicate id that involves a set is logged as an ERROR, and the later
  copy in compile order wins (a manifest's sets follow its loose graphs).
- 'list graphs' shows a set's graphs with their set and version, and
  'import graph from {id}' finds the unpacked copy.
- A deployment is still a restart: the sets are re-read at every start, and
  the loader's generated manifest in the unpack folder records what it
  deployed and which files it wrote.

See also 'help export', 'help import' and 'help tutorial 2', and the guides:
the canonical package format and the configuration reference.
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
  "Graph traversal aborted: <reason>" - every abort names its reason (a node's
  error, a pre-run gate rule, the deadline).
- 'run' may be repeated on the same instance: each run clears the visited
  set and the output namespace, but model values persist across runs -
  instantiate again for a completely fresh state.
- Use 'seen' to list the nodes visited by the last run, and 'inspect' to
  read the results (e.g. 'inspect output.body').
`,Ct=`Graph schema
------------
Declare the graph's request and response as a 'schema' property on the root
and end nodes, in a closed subset of OpenAPI 3.0. The engine validates every
request at the root, answers an OpenAPI 3.0 document on demand, and the
Schema panel fills the declaration in from discovery and from the last run.
This topic explains the property, the panel and the endpoints; there is no
console command behind it.

The declaration
---------------
- The root node's 'schema' describes the request: 'schema.body' for
  input.body and 'schema.header' for input.header. The end node's 'schema'
  describes the response (output.body, output.header) and is documentary -
  it is never enforced.
- Both parts are ordinary nested properties, written with the composite keys
  of 'create node' and 'update node':

\`\`\`
update node root
with type Root
with properties
schema.body.type=object
schema.body.required[]=amount
schema.body.properties.amount.type=number
schema.body.properties.amount.minimum=0
schema.body.properties.currency.type=string
schema.body.properties.currency.enum[]=USD
schema.body.properties.currency.enum[]=EUR
schema.header.required[]=X-Tenant
schema.header.properties.X-Tenant.type=string
\`\`\`

- The vocabulary is closed: type (object, array, string, number, integer,
  boolean), properties, required, items, enum, minimum, maximum,
  exclusiveMinimum, exclusiveMaximum, minLength, maxLength, pattern,
  minItems, maxItems, nullable and additionalProperties; title, description,
  example and format are documentary. The deployment gate - and the
  pre-run check before 'run' - refuses any other keyword, a keyword that
  does not apply to the declared type, and a malformed value, naming the
  location: "node root - schema.body.properties.amount: unknown keyword
  'min' - ...".
- Types are strict JSON types: a numeric string is not a number; money is
  a string with a pattern. A header is text, so an integer header validates
  the parsed text, and header names match case-insensitively.

Validation at the root
----------------------
- When the root carries a 'schema', every run validates input.body and
  input.header against it first - before the root's own skill or anything
  else - through the built-in function graph.schema.validator. The step is
  assumed: nothing is written into the node for it.
- Success prints "Input validated by graph.schema.validator in N ms".
  Failure aborts the run with every violation in one message, at most ten:
  "Graph traversal aborted: Input validation failed - input.body.amount:
  expected number, got string; input.header.X-Tenant: required (node root)".
  A deployed graph answers HTTP 400 with the same message; a root
  'exception=' handler takes over with error.message, error.source and
  error.code.
- A dry run validates exactly as a deployed run does: 'instantiate graph',
  Upload the mock body and the Headers rows, then 'run'.

The panel
---------
Open it from the Tools menu, "Graph schema", or from the root or end node
editor. It takes the console's place; Esc or Cancel gives it back, and
unsaved rows survive a close.

- Two tabs: Input (node root) and Output (node end). Each has a Body
  section, one row per path (items[] for a list), and a Headers section,
  one row per header name: path or name, type, req, description, example,
  and a chip - declared, discovered (the tooltip names the referencing
  nodes), from last run, or new. A second chip summarizes the keywords the
  panel does not edit (enum, minimum, pattern, ...); they are kept as
  declared and edited in the node editor.
- The rows are pre-filled from the engine's view of the draft: what the
  model references, merged with what the nodes declare. The engine's
  declaration-versus-model issues are listed above the rows.
- "Fill from last run" types the rows from the instance's actual input and
  output after a dry run - the only way to type a task's or a fetcher's
  result. "Reload" discards unsaved rows.
- "Save root schema" / "Save end schema" writes the declaration through one
  'update node', so every member of the session sees it; what the gate
  would refuse is flagged in place first.
- "Download YAML" saves the draft's OpenAPI document.

The endpoints
-------------
Dev mode only:

\`\`\`
GET /api/openapi/{graph-id}                    # a deployed graph, YAML attachment
GET /api/openapi/{graph-id}?format=json        # the same as JSON
GET /api/openapi/{graph-id}?view=contract      # the derived contract with its evidence
GET /api/openapi/session/{sessionId}           # the same views for this session's draft
\`\`\`

The document holds POST /api/graph/{graph-id} with the request body, the
header parameters, the 200 response with its body and headers, every staged
output.status, and the error shape; it is derived from the model each time
and never stored. Paste the URL into Swagger UI's explore bar to try the
graph from there. 'describe graph {graph-id}' prints the same surface as
text, with [declared] on a path only the declaration knows.

See also 'help describe', 'help upload', 'help run' and the guide
"The graph contract".
`,wt=`Display nodes that have been 'seen'
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
`,Tt=`Session commands
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
  run in both sessions, keeping the graphs in sync. A mock-data upload
  (POST /api/mock/{id}, see 'help upload') travels the same way: it loads
  into every member's graph instance. On subscribe the graphs
  are aligned - if the primary session is empty, your draft is pushed to it;
  otherwise its graph replaces your draft.
- You can subscribe only to a primary session (one that has not itself
  subscribed to another), and never to yourself. If you are already
  subscribed, do 'session reset' before subscribing to another session.
- 'session unsubscribe' decouples your session from the one you subscribed
  to; your graph is retained so you can continue editing. A primary session
  gets "Nothing to unsubscribe".
- 'session reset' restarts your session with an empty draft graph. As a
  primary session it disconnects all subscribers (they keep their own
  graphs); as a subscriber it unsubscribes first. Export first if you want
  to keep your draft (see 'help export').
- The companion REST endpoints reject 'session subscribe', 'session
  unsubscribe' and 'session reset': a companion is an assistant to a
  session, not a session of its own. Only the read-only 'session' status
  query works there - session topology is managed from a
  WebSocket-connected session (the browser console) only.
`,Et=`Tutorial 1
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
`,Dt=`Tutorial 10
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
`,Ot=`Tutorial 11
-----------
In this tutorial, you will create a graph model that uses an "event flow" as an extension.

Pre-requisite
-------------
You would need some working knowledge of Event Script. For more details, see the
Event Script guide: https://accenture.github.io/mercury-composable/guides/event-script/

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
`,kt=`Tutorial 12
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

The dev-mode mock endpoint (mock.mdm.profile) answers with a 401 "simulated
exception" when the request header x-exception is true. In the Java example it
reads like this (the Rust example's mock endpoint does the same):

\`\`\`java
@Override
public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
    if ("true".equals(input.getHeader("x-exception"))) {
        throw new AppException(401, "simulated exception");
    }
    // for simplicity, business logic not shown here
}
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
`,At=`Tutorial 13
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
`,jt=`Tutorial 14
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
('run' may repeat on one instance and model values persist across runs -
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
`,Mt=`Tutorial 2
----------
In this tutorial, you will deploy the 'hello world' graph model that you created in
tutorial 1, then enhance it into an echo application.

Exercise
--------
To deploy the graph model from tutorial 1, copy the 'tutorial-1.json' file that was
exported earlier into your application's resources/graph folder
(src/main/resources/graph in a Java project, resources/graph in a Rust project).

\`\`\`
cp /tmp/graph/tutorial-1.json ~/sandbox/{your_project}/src/main/resources/graph
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

The telemetry entries look like this (abridged; the Rust engine logs the same trace
records in its own JSON log format):

\`\`\`
2026-03-31 15:19:08.052 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=http.flow.adapter, success=true, from=http.request, exec_time=0.12, status=200}}
2026-03-31 15:19:08.055 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=graph.data.mapper, success=true, from=graph.executor, exec_time=0.074, status=200},
    annotations={node=end}}
2026-03-31 15:19:08.056 INFO  GraphHousekeeper:44 - Graph instance 2c1a00d63f7d4ec2b657db4a75021068
    for model 'tutorial-1' cleared
2026-03-31 15:19:08.056 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=task.executor, success=true, from=event.script.manager, exec_time=4.0, status=200},
    annotations={execution=Run 1 task in 4 ms, flow=graph-executor}}
2026-03-31 15:19:08.056 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=async.http.response, success=true, from=task.executor, exec_time=0.224, status=200}}
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
`,Nt=`Tutorial 3
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
`,Pt=`Tutorial 4
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
`,Ft=`Tutorial 5
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
GET http://127.0.0.1:8085/api/mdm/profile/100, with [person_id], ttl=30000
GET http://127.0.0.1:8085/api/mdm/profile/200, with [person_id], ttl=30000
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
`,It=`Tutorial 6
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
`,Lt=`Tutorial 7
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
enter "help graph-data-mapper"; the full syntax is in the Event Script syntax guide:
https://accenture.github.io/mercury-composable/guides/event-script/syntax/

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
mapping plugins are available — see the simple-plugin catalog in the Event Script
syntax guide: https://accenture.github.io/mercury-composable/guides/event-script/syntax/

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
`,Rt=`Tutorial 8
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
`,zt=`Tutorial 9
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
`,Bt=`Update a node
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
task=no.op
input[]=input.body -> *
output[]=result -> output.body
\`\`\`

Notes
-----
- Node names are matched case-insensitively; use lowercase letters, digits
  and hyphen ('root' and 'end' are reserved for the root and end nodes).
- Types are descriptive labels, conventionally Capitalized; the type and
  properties are validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional; a key[]=entry line
  appends one entry to the list "key"; wrap a multi-line value in triple
  single quotes ('''). Values may use the Event Script constant syntax.
- Tip: 'edit node {name}' prints an existing node as a ready-to-edit
  'update node' command (see 'help edit').
`,Vt=`Upload mock data
----------------
Print the URLs for uploading mock input to the current graph instance - a
JSON payload as 'input.body', optional request headers as 'input.header' and
optional model variables - convenient when the mock input is too large to
seed line by line, or when the graph reads a header or a model variable that
a request's edge would set.

Syntax
------
\`\`\`
upload mock data
\`\`\`

Example
-------
\`\`\`
> upload mock data
You may upload JSON payload -> POST /api/mock/{name} (mock headers, a JSON object of text values -> POST /api/mock/{name}?namespace=header; model variables, a JSON object -> POST /api/mock/{name}?namespace=model)
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload (a map or a list) to the first URL replaces
  the instance's 'input.body'; the console confirms with "Mock data loaded
  into 'input.body' namespace".
- An HTTP POST of a JSON object of text values to the same URL with
  '?namespace=header' replaces the instance's 'input.header'; the console
  confirms with "Mock data loaded into 'input.header' namespace". The headers
  are processed as a real request's are: every name is stored in lower case
  (two names that differ only by case are refused), CR and LF are removed
  from a value, the business correlation-ID header (the property
  'http.correlation.id.header', X-Correlation-Id by default) sets
  'model.cid' - without one, the header carries the instance's 'model.cid',
  as a request always does - and an 'x-iteration-index' header sets
  'model.iteration_index', as the deployed walker lifts it for a subgraph's
  for_each iteration. The console names each value it derives, for example
  "model.cid = order-1001 (from input.header.x-correlation-id)".
- An HTTP POST of a JSON object to the same URL with '?namespace=model' sets
  each key as a model variable ('tier' becomes 'model.tier'; a dotted or
  indexed key such as 'limits.daily' or 'items[0]' sets a nested value) and
  leaves the other model variables in place; the console confirms with "Mock
  data loaded into 'model' namespace". A request does not set model
  variables - the edge and the graph do - so a dry run mocks them here. A
  key with an empty path segment, or a null value, is refused.
- Any other namespace, or a payload of the wrong shape, is refused (HTTP
  400).
- The 'instantiate graph' command (see 'help instantiate') seeds the same
  namespaces line by line: its '{value} -> input.body.{key}',
  '{value} -> input.header.{name}' and '{value} -> model.{key}' lines write
  the state machine directly, without the request processing above.
- In a collaborative session (see 'help session') an uploaded payload loads
  into every member's graph instance - the primary and all its subscribers -
  and each member's console confirms it. In the Playground UI the toolbar's
  Upload button opens the upload form for your own session only, with the
  JSON body, optional input header rows and optional model variable rows
  (each with a type: text, number, boolean or JSON); the form posts the body,
  then the headers, then the model variables, so a model variable it names
  wins over one a header derives. Uploading is optional (a graph that reads
  no input runs without it).
`,Ht=`MiniGraph
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
- help package (graph sets: pack, inspect and deploy)
- help schema (the graph contract: declare, validate, OpenAPI document, the Schema panel)
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
3. graph.js - deprecated (not registered in the Rust engine); use graph.math or graph.task instead
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
`,Ut=[`# JSON-Path Playground Overview`,``,`Use the JSON-Path Playground to load a JSON or XML document and evaluate expressions against it.`,`This starter Overview is intentionally the only section for now and can grow into a fuller guide later.`,``,`## Quick start`,``,`1. Click **Start** to connect to the JSON-Path playground.`,`2. Paste a document into the Payload Editor, or choose a Quick load sample.`,"3. Enter `load` to send the document to the active session.","4. Enter a JSONPath expression beginning with `$`, such as `$.response.user.name`.",``,`## Example expressions`,``,`| Expression | Purpose |`,`| --- | --- |`,"| `$.response` | Read the loaded response object |","| `$.response.items[*]` | Select every item in an array |","| `$.response.items[0].name` | Read a field from the first item |",``,`## Help controls`,``,"- `Ctrl + backtick` - Toggle the Help panel",`- Use the maximize button to expand Help and the close button to return to the editor`].join(`
`),Wt=Object.assign({"../../../src/main/resources/help/help connect.md":$e,"../../../src/main/resources/help/help create.md":et,"../../../src/main/resources/help/help data-dictionary.md":tt,"../../../src/main/resources/help/help delete.md":nt,"../../../src/main/resources/help/help describe.md":rt,"../../../src/main/resources/help/help edit.md":it,"../../../src/main/resources/help/help execute.md":at,"../../../src/main/resources/help/help export.md":ot,"../../../src/main/resources/help/help graph-api-fetcher.md":st,"../../../src/main/resources/help/help graph-data-mapper.md":ct,"../../../src/main/resources/help/help graph-extension.md":lt,"../../../src/main/resources/help/help graph-island.md":ut,"../../../src/main/resources/help/help graph-join.md":dt,"../../../src/main/resources/help/help graph-js.md":ft,"../../../src/main/resources/help/help graph-math.md":pt,"../../../src/main/resources/help/help graph-resume.md":mt,"../../../src/main/resources/help/help graph-suspend.md":ht,"../../../src/main/resources/help/help graph-task.md":gt,"../../../src/main/resources/help/help import.md":_t,"../../../src/main/resources/help/help inspect.md":vt,"../../../src/main/resources/help/help instantiate.md":yt,"../../../src/main/resources/help/help list.md":bt,"../../../src/main/resources/help/help package.md":xt,"../../../src/main/resources/help/help run.md":St,"../../../src/main/resources/help/help schema.md":Ct,"../../../src/main/resources/help/help seen.md":wt,"../../../src/main/resources/help/help session.md":Tt,"../../../src/main/resources/help/help tutorial 1.md":Et,"../../../src/main/resources/help/help tutorial 10.md":Dt,"../../../src/main/resources/help/help tutorial 11.md":Ot,"../../../src/main/resources/help/help tutorial 12.md":kt,"../../../src/main/resources/help/help tutorial 13.md":At,"../../../src/main/resources/help/help tutorial 14.md":jt,"../../../src/main/resources/help/help tutorial 2.md":Mt,"../../../src/main/resources/help/help tutorial 3.md":Nt,"../../../src/main/resources/help/help tutorial 4.md":Pt,"../../../src/main/resources/help/help tutorial 5.md":Ft,"../../../src/main/resources/help/help tutorial 6.md":It,"../../../src/main/resources/help/help tutorial 7.md":Lt,"../../../src/main/resources/help/help tutorial 8.md":Rt,"../../../src/main/resources/help/help tutorial 9.md":zt,"../../../src/main/resources/help/help update.md":Bt,"../../../src/main/resources/help/help upload.md":Vt,"../../../src/main/resources/help/help.md":Ht});function Gt(e){let t=e.split(`/`);return(t[t.length-1]??e).replace(/\.md$/,``)}var Kt=Object.fromEntries(Object.entries(Wt).map(([e,t])=>[Gt(e),t])),qt={help:Ut};function Jt(e){return e===`json-path`?qt:Kt}function Yt(e,t=`minigraph`){let n=e===``?`help`:`help ${e}`;return Jt(t)[n]??null}var Xt=Object.keys(Kt).filter(e=>e!==`help`).map(e=>e.replace(/^help\s+/,``)).sort(),Zt=[{id:`overview`,label:`Overview`},{id:`graph-model`,label:`Graph Model`},{id:`graph-skills`,label:`Graph Skills`},{id:`instance-model`,label:`Instance Model`},{id:`tutorials`,label:`Tutorials`,chipStripLabel:`Chapters`}],Qt=[{id:`overview`,label:`Overview`}];function $t(e=`minigraph`){return e===`json-path`?Qt:Zt}var en=new Set([`execute`,`inspect`,`instantiate`,`run`,`seen`,`upload`]);function tn(e,t=`minigraph`){return t===`json-path`||e===``?`overview`:e.startsWith(`tutorial `)?`tutorials`:e.startsWith(`graph-`)?`graph-skills`:en.has(e)?`instance-model`:`graph-model`}function nn(e,t=`minigraph`){if(e===`overview`)return[``];if(t===`json-path`)return[];let n=Xt.filter(n=>tn(n,t)===e);return e===`tutorials`?[...n].sort((e,t)=>parseInt(e.replace(/^tutorial\s+/,``),10)-parseInt(t.replace(/^tutorial\s+/,``),10)):n}function rn(e,t){return e===``?`Overview`:t===`tutorials`?e.replace(/^tutorial\s+/,``):e}function an(e=`minigraph`){return $t(e).flatMap(t=>nn(t.id,e))}an();function on(e,t,n=`minigraph`){if(!t)return null;let r=Qe(e);return r===null||Yt(r,n)===null?null:r}function sn({bus:e,setHelpTopic:t,onTabSwitch:n,enabled:r=!0,contentProfile:i=`minigraph`}){let a=(0,F.useRef)(n);(0,F.useEffect)(()=>{a.current=n}),(0,F.useEffect)(()=>{if(r)return e.on(`command.helpOrDescribe`,e=>{let n=on(e.commandText,!0,i);n!==null&&(t(n),a.current())})},[e,t,r,i])}function cn({ctx:e,navigate:t,addToast:n,wsPath:r}){let i=ue.find(e=>e.tabs.includes(`payload`)&&e.supportsUpload),a=(0,F.useRef)(null),o=i?.wsPath;(0,F.useEffect)(()=>{if(!(!o||!a.current)&&e.getSlot(o).phase===`connected`){let{wsPath:r,json:o}=a.current;a.current=null,e.setPendingPayload(r,o),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)}},[o,e,t,n,i]);let s=(0,F.useCallback)(r=>{if(!i)return;let o=e.getSlot(i.wsPath);o.phase===`connected`?(e.setPendingPayload(i.wsPath,r),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)):o.phase===`connecting`?(a.current={wsPath:i.wsPath,json:r},n(`Updated pending JSON transfer — latest payload will open when connected`,`info`)):(a.current={wsPath:i.wsPath,json:r},e.connect(i.wsPath,n),n(`Connecting to JSON-Path Playground…`,`info`))},[e,t,n,i]);return{handleSendToJsonPath:i&&r!==i.wsPath?s:void 0}}function ln({addToast:e}){let[t,n]=(0,F.useState)(null),r=(0,F.useRef)(null),i=(0,F.useRef)([]),a=(0,F.useRef)(null),[o,s]=(0,F.useState)(new Set),c=(0,F.useCallback)(e=>{let t=r.current;if(t!==null){t!==e&&!i.current.includes(e)&&i.current.push(e);return}a.current=document.activeElement,r.current=e,n(e)},[]),l=(0,F.useCallback)(()=>{let e=i.current.shift()??null;r.current=e,n(e),e===null&&setTimeout(()=>a.current?.focus(),0)},[]);return{uploadPanelPath:t,successfulUploadPaths:o,handleOpenUploadPanel:c,handleCloseUploadPanel:(0,F.useCallback)(()=>{l()},[l]),handleCloseUploadPath:(0,F.useCallback)(e=>{if(r.current===e)return l(),!0;let t=i.current.indexOf(e);return t===-1?!1:(i.current.splice(t,1),!0)},[l]),handleUploadSuccess:(0,F.useCallback)(t=>{let n=r.current;n&&s(e=>new Set([...e,n])),l(),e(`Mock data uploaded successfully ✓`,`success`)},[e,l]),handleUploadError:(0,F.useCallback)(t=>{e(`Upload failed: ${t}`,`error`)},[e]),resetSuccessfulPaths:(0,F.useCallback)(()=>{s(new Set)},[])}}function un({bus:e,connected:t,appendMessage:n,addToast:r}){let i=(0,F.useRef)(null),a=(0,F.useRef)(!1),o=(0,F.useRef)(n);(0,F.useEffect)(()=>{o.current=n},[n]);let s=(0,F.useRef)(r);(0,F.useEffect)(()=>{s.current=r},[r]),(0,F.useEffect)(()=>{t||(i.current?.abort(),i.current=null,a.current=!1)},[t]),(0,F.useEffect)(()=>()=>{i.current?.abort()},[]),(0,F.useEffect)(()=>e.on(`payload.large`,e=>{if(a.current)return;let{apiPath:t,byteSize:n}=e;i.current?.abort();let r=new AbortController;i.current=r;let c=(n/(1024*1024)).toFixed(2);s.current(`Fetching large payload (${c} MB)…`,`info`),a.current=!0,fetch(t,{signal:r.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.text()}).then(e=>{if(!e.trim())throw Error(`empty response body`);let t=e;try{t=JSON.stringify(JSON.parse(e),null,2)}catch{}o.current(t),a.current=!1,i.current=null}).catch(e=>{e.name!==`AbortError`&&(a.current=!1,i.current=null,o.current(`ERROR: payload fetch failed — ${e.message}`),s.current(`Payload fetch failed: ${e.message}`,`error`))})}),[e])}function dn(e){let[t,n]=ce(e,{}),r=(0,F.useCallback)(e=>{n(t=>({...t,[e]:{name:e,savedAt:new Date().toISOString()}}))},[n]),i=(0,F.useCallback)(e=>{n(t=>{let n={...t};return delete n[e],n})},[n]),a=(0,F.useCallback)(e=>Object.prototype.hasOwnProperty.call(t,e),[t]);return{savedGraphs:(0,F.useMemo)(()=>Object.values(t).sort((e,t)=>new Date(t.savedAt).getTime()-new Date(e.savedAt).getTime()),[t]),saveGraph:r,deleteGraph:i,hasGraph:a}}var fn={importedName:null,lastSavedName:null,isSaved:!1,untitledSlotConsumed:!1};function pn(e,t){switch(t.type){case`imported`:return{...e,importedName:t.name,lastSavedName:null,isSaved:!1};case`exported`:return{...e,lastSavedName:t.name,isSaved:!0,untitledSlotConsumed:e.untitledSlotConsumed||t.consumesUntitled};case`dirty`:return{...e,isSaved:!1};case`reset`:return{...fn}}}var mn=new Map;function hn(e,t,n){return t&&n!==null&&e===n}function gn(e,t){let n=e.on(`command.importGraph`,e=>{t.onImported(e.graphName)}),r=e.on(`graph.exported`,e=>{t.onExported(e.graphName)}),i=e.on(`graph.mutation`,()=>{t.onDirty()}),a=e.on(`session.reset`,()=>{t.onReset()});return()=>{n(),r(),i(),a()}}function _n(e,t,n,r){let[i,a]=ce(e,1),[o,s]=(0,F.useState)(()=>{let t=mn.get(e);return t&&hn(t.connectionEpoch,n,r)?t.state:{...fn}}),c=(0,F.useRef)(o),l=(0,F.useCallback)(t=>{let n=pn(c.current,t);c.current=n,t.type===`reset`?mn.delete(e):r!==null&&mn.set(e,{connectionEpoch:r,state:n}),s(n)},[r,e]),u=(0,F.useCallback)(e=>{l({type:`exported`,name:e,consumesUntitled:e===`untitled-${i}`})},[i,l]),d=(0,F.useCallback)(()=>{let t=mn.get(e)?.state;(c.current.untitledSlotConsumed||t?.untitledSlotConsumed)&&a(e=>e+1),l({type:`reset`})},[a,e,l]),f=(0,F.useRef)(r);return(0,F.useEffect)(()=>{let t=mn.get(e)?.connectionEpoch;(!n||f.current!==r||t!==void 0&&t!==r)&&d(),f.current=r},[n,r,d,e]),(0,F.useEffect)(()=>gn(t,{onImported:e=>l({type:`imported`,name:e}),onExported:u,onDirty:()=>l({type:`dirty`}),onReset:d}),[t,d,u,l]),{defaultName:o.lastSavedName??o.importedName??`untitled-${i}`,savedName:o.isSaved?o.lastSavedName:null,resetName:d}}var vn=new Set([`description`,`question`,`purpose`]),yn=`input.body`,bn=`input.header`;function xn(e){return/[A-Za-z0-9_.]/.test(e)}function Sn(e){return/[A-Za-z0-9_.*\[\]-]/.test(e)}function Cn(e,t,n){let r=0;for(;r<e.length;){let i=e.indexOf(t,r);if(i===-1)return;let a=i>0?e[i-1]:``,o=i>1?e[i-2]:``,s=e[i+t.length]??``;if(a&&xn(a)&&!(a===`.`&&o===`$`)||s&&/[A-Za-z0-9_]/.test(s)){r=i+t.length;continue}let c=i+t.length;for(;c<e.length&&Sn(e[c]);)c+=1;let l=e.slice(i,c);for(;l.endsWith(`.`)||l.endsWith(`-`)||l.endsWith(`[`);)l=l.slice(0,-1);for(;l.endsWith(`]`)&&!l.includes(`[`);)l=l.slice(0,-1);n.add(l),r=Math.max(c,i+t.length)}}function wn(e,t,n,r){if(!(r&&vn.has(r.toLowerCase()))){if(typeof e==`string`){Cn(e,t,n);return}if(Array.isArray(e)){for(let r of e)wn(r,t,n);return}if(typeof e==`object`&&e)for(let[r,i]of Object.entries(e))wn(i,t,n,r)}}function Tn(e,t){if(!e)return[];let n=new Set;for(let r of e.nodes)wn(r.properties,t,n);return Array.from(n).sort()}function En(e){return Tn(e,yn)}function Dn(e){return Tn(e,bn).map(e=>e===bn?`*`:e.slice(13))}var On={instantiate:`instantiate graph`,run:`run`},kn=/^Graph instance created\. Loaded (\d+) mock (?:entry|entries), model\.ttl = (\d+) ms$/,An=/^Graph traversal completed in (\d+) ms$/,jn=/^ERROR:\s*(.+)$/;function Mn(e){let t=e.match(kn);return t?{mockEntries:Number.parseInt(t[1],10),ttlMs:Number.parseInt(t[2],10)}:null}function Nn(e){return e===`Graph instance cleared`}function Pn(e){let t=e.match(An);return t?{status:`completed`,elapsedMs:Number.parseInt(t[1],10)}:e.startsWith(`Graph traversal aborted`)?{status:`aborted`,elapsedMs:null}:null}function Fn(e){return e.match(jn)?.[1]?.trim()||null}function In(e){let t=e.trim().toLowerCase();return t===On.instantiate||t===`${On.instantiate}...`}function Ln(e){return e.trim().toLowerCase()===On.run}var Rn=1e4,zn={phase:`idle`,pendingSignal:null,ownAction:!1,invalidated:!1};function Bn({enabled:e,bus:t,connected:n,connectionEpoch:r,graphData:i,graphIdentity:a,isPrimary:o,sendRawText:s,addToast:c}){let[l,u]=(0,F.useState)(zn),d=(0,F.useRef)(zn),f=(0,F.useRef)(s),p=(0,F.useRef)(c),m=(0,F.useMemo)(()=>En(i),[i]),h=(0,F.useMemo)(()=>Dn(i),[i]),g=m.length>0||h.length>0,_=(0,F.useRef)(m),v=(0,F.useRef)(g);(0,F.useEffect)(()=>{f.current=s},[s]),(0,F.useEffect)(()=>{p.current=c},[c]),(0,F.useEffect)(()=>{_.current=m},[m]),(0,F.useEffect)(()=>{v.current=g},[g]);let y=(0,F.useCallback)(e=>{d.current=e,u(e)},[]),b=(0,F.useCallback)(()=>{y({...zn})},[y]),x=(0,F.useCallback)(()=>{let e=d.current;if(e.pendingSignal!==null){y({...e,phase:`outcome-uncertain`,invalidated:!0});return}b()},[b,y]),S=(0,F.useCallback)((e,t,n)=>(y(t),f.current(e)?!0:(b(),p.current(n,`error`),!1)),[b,y]),C=e&&n&&i!==null,w=C&&(l.phase===`idle`||l.phase===`ready`),T=C&&l.phase===`ready`,E=C&&l.phase===`ready`,D=e?i?n?``:`Connect first to run the graph`:`Load a graph first`:`Graph run controls are unavailable`,O=(0,F.useCallback)(()=>!E||d.current.phase!==`ready`?!1:S(On.run,{phase:`running`,pendingSignal:`run-terminal`,ownAction:!0,invalidated:!1},`Could not run graph because the WebSocket is not open.`),[E,S]),k=(0,F.useCallback)(()=>{if(!w)return!1;let e=d.current;return e.phase!==`idle`&&e.phase!==`ready`?!1:S(On.instantiate,{phase:`instantiating`,pendingSignal:`instance-created`,ownAction:!0,invalidated:!1},`Could not instantiate graph because the WebSocket is not open.`)},[w,S]);(0,F.useEffect)(()=>{let e=t.on(`graph.instance.created`,()=>{let e=d.current;if(e.pendingSignal===`instance-created`){if(e.invalidated){b();return}y({phase:`ready`,pendingSignal:null,ownAction:!1,invalidated:!1}),e.ownAction&&p.current(v.current?`Graph instantiated. Upload mock input if the run needs it, then run.`:`Graph instantiated and ready to run.`,`success`)}}),n=t.on(`graph.instance.cleared`,b),r=t.on(`graph.mutation`,x),i=t.on(`session.reset`,b),a=t.on(`graph.exported`,x),o=t.on(`graph.run.terminal`,e=>{let t=d.current;if(t.pendingSignal!==`run-terminal`)return;let n=!t.invalidated&&e.status===`aborted`;b(),n&&p.current(`Graph run aborted. See the console for details.`,`error`)}),s=t.on(`command.error`,e=>{let t=d.current;if(t.pendingSignal!==`instance-created`)return;let n=!t.invalidated;b(),n&&p.current(`Could not instantiate graph: ${e.message}`,`error`)}),c=t.on(`command.echo`,e=>{let t=d.current;t.phase!==`idle`&&t.phase!==`ready`||(In(e.commandText)?y({phase:`instantiating`,pendingSignal:`instance-created`,ownAction:!1,invalidated:!1}):Ln(e.commandText)&&y({phase:`running`,pendingSignal:`run-terminal`,ownAction:!1,invalidated:!1}))});return()=>{e(),n(),r(),i(),a(),o(),s(),c()}},[t,x,b,y]),(0,F.useEffect)(()=>{if(l.phase!==`instantiating`)return;let e=setTimeout(()=>{let e=d.current;e.phase===`instantiating`&&(y({...e,phase:`outcome-uncertain`}),p.current(`Graph setup is taking longer than expected. Waiting for the backend outcome…`,`info`))},Rn);return()=>clearTimeout(e)},[l.phase,y]);let A=(0,F.useRef)(a);(0,F.useEffect)(()=>{A.current!==a&&x(),A.current=a},[a,x]);let ee=(0,F.useRef)(r);(0,F.useEffect)(()=>{ee.current!==r&&b(),ee.current=r},[r,b]);let j=(0,F.useRef)(o);(0,F.useEffect)(()=>{j.current!==o&&b(),j.current=o},[o,b]),(0,F.useEffect)(()=>{!e||!n?b():i||x()},[e,n,i,b,x]);let te=l.phase!==`idle`&&l.phase!==`ready`;return{phase:l.phase,ready:l.phase===`ready`,busy:te,canInteract:C,canInstantiate:w,canUpload:T,canRun:E,disabledReason:D,inputBodyPaths:m,inputHeaderNames:h,runGraph:O,instantiateGraph:k}}function Vn(e,t){return e.on(`graph.exported`,e=>t(e.graphName))}function Hn({bus:e,connected:t,sendRawText:n,saveGraph:r,addToast:i}){let a=(0,F.useRef)(null),o=(0,F.useCallback)(e=>{if(!t){i(`Save failed: connection required to export graph`,`error`);return}let r=setTimeout(()=>{a.current!==null&&(a.current=null,i(`Save failed: export confirmation timed out`,`error`))},1e4);a.current={graphName:e,timeoutId:r},n(`export graph as ${e}`)},[t,n,i]);return(0,F.useEffect)(()=>{if(r!==null)return Vn(e,r)},[e,r]),(0,F.useEffect)(()=>e.on(`graph.exported`,e=>{if(a.current===null||e.graphName!==a.current.graphName)return;clearTimeout(a.current.timeoutId);let t=a.current.graphName;a.current=null,i(`Graph saved as "${t}"`,`success`)}),[e,i]),(0,F.useEffect)(()=>e.on(`graph.export.failed`,e=>{a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,e.reason===`invalid-name`?i(`Save failed: invalid filename (a–z, A–Z, 0–9, hyphen only)`,`error`):i(`Save failed: root node name does not match existing graph`,`error`))}),[e,i]),(0,F.useEffect)(()=>{!t&&a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,i(`Save failed: connection closed before export confirmation`,`error`))},[t,i]),(0,F.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current.timeoutId)},[]),{handleSaveGraph:o,handleLoadGraph:(0,F.useCallback)(e=>{t&&(n(`import graph from ${e}`),i(`Importing graph "${e}"…`,`info`))},[t,n,i])}}var Un=/^[A-Za-z0-9_-]+$/,Wn=/^[A-Za-z0-9_-]+(?:\[(?:0|[1-9]\d*)?\])*$/,Gn=new Set([`input`,`output`,`model`,`response`,`result`,`parameter`,`none`,`next`,`api`,`error`]);function Kn(e,t){return`properties.${e}.${t}`}function qn(e){return e.split(`.`).every(e=>Wn.test(e))}function Jn(e){return qn(e)}function Yn(e,t={}){let n={},r=t.mode??`create`,i=e.alias.trim(),a=t.originalAlias?.trim()??``,o=e.nodeType.trim();r===`edit`?a?Un.test(a)||(n.alias=`Use only letters, numbers, underscore, and hyphen.`):n.alias=`Original alias is required.`:i?Un.test(i)?Gn.has(i.toLowerCase())?n.alias=`"${i}" is reserved.`:t.graphData?.nodes.some(e=>e.alias.toLowerCase()===i.toLowerCase())&&(n.alias=`Node "${i}" already exists in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,o&&!Un.test(o)&&(n.nodeType=`Use only letters, numbers, underscore, and hyphen.`);for(let t of e.properties){let e=t.key.trim(),r=t.value.trim();!e&&!r||(!e&&r?n[Kn(t.id,`key`)]=`Property key is required when value is present.`:Jn(e)||(n[Kn(t.id,`key`)]=`Use a property name or dot/bracket path, for example mapping[] or config.value.`),r.includes(`'''`)&&(n[Kn(t.id,`value`)]=`Property value cannot contain '''.`))}return{valid:Object.keys(n).length===0,errors:n}}function Xn(e,t={}){let n={},r=e.trim();return r?Un.test(r)?t.graphData&&!t.graphData.nodes.some(e=>e.alias.toLowerCase()===r.toLowerCase())&&(n.alias=`Node "${r}" is no longer available in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,{valid:Object.keys(n).length===0,errors:n}}function Zn(e,t){let n={},r=e.trim(),i=t.trim();return(!r||!Un.test(r))&&(n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`),(!i||!Un.test(i))&&(n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`),!n.sourceAlias&&!n.targetAlias&&r===i&&(n.targetAlias=`Source and target must be different nodes.`),{valid:Object.keys(n).length===0,errors:n}}function Qn(e,t){return!!e?.nodes.some(e=>e.alias.toLowerCase()===t.toLowerCase())}function $n(e,t={}){let n={},r=e.sourceAlias.trim(),i=e.targetAlias.trim(),a=e.relation.trim();return t.connected===!1&&(n.command=`Connection disconnected. Reconnect before creating a connection.`),r?Un.test(r)?t.graphData&&!Qn(t.graphData,r)&&(n.sourceAlias=`Node "${r}" is no longer available in the current graph.`):n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`:n.sourceAlias=`Source is required.`,i?Un.test(i)?t.graphData&&!Qn(t.graphData,i)&&(n.targetAlias=`Node "${i}" is no longer available in the current graph.`):n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`:n.targetAlias=`Target is required.`,r&&i&&r.toLowerCase()===i.toLowerCase()&&(n.targetAlias=`Source and target nodes must be different.`),a?Un.test(a)||(n.relation=`Use only letters, numbers, underscore, and hyphen.`):n.relation=`Relation is required.`,{valid:Object.keys(n).length===0,errors:n}}function er(e){return e.length<=63488?{valid:!0,errors:{}}:{valid:!1,errors:{command:`The node command is too large. Shorten property values before submitting.`}}}var tr=0;function nr(e=``,t=``){return tr+=1,{id:`property-row-${tr}`,key:e,value:t}}function rr(e){return{alias:e===`empty-graph`?`root`:``,nodeType:e===`empty-graph`?`Root`:``,properties:[nr()],source:e}}function ir(e){let t=e.indexOf(`[`);if(t===-1)return null;let n=e.indexOf(`]`,t+1);return n===-1||e.indexOf(`[`,n+1)!==-1?null:{open:t,close:n}}function ar(e){let t=ir(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+n.padStart(3,`0`)+e.slice(t.close):e}function or(e){let t=ir(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+e.slice(t.close):e}function sr(e){return e.slice().sort((e,t)=>{let n=ar(e.key.trim()),r=ar(t.key.trim());return!n&&!r?0:n?!r||n<r?-1:+(n>r):1})}var cr=`This node contains data that cannot be safely represented in the edit form. Use the console edit command for this node.`;function lr(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function ur(e){return e===null?`null`:String(e)}function dr(e,t,n){if(!qn(e))return!1;if(Array.isArray(t))return t.length!==0&&t.every((t,r)=>dr(`${e}[${r}]`,t,n));if(lr(t)){let r=Object.entries(t);return r.length!==0&&r.every(([t,r])=>dr(`${e}.${t}`,r,n))}let r=ur(t);return r.includes(`'''`)?!1:(n.push(nr(e,r)),!0)}function fr(e){if(!Un.test(e.alias)||e.types.length>1)return{valid:!1,formState:null,message:cr};let t=Object.entries(e.properties),n=[];for(let[e,r]of t)if(!dr(e,r,n))return{valid:!1,formState:null,message:cr};let r=sr(n).map(e=>({...e,key:or(e.key)}));return{valid:!0,formState:{alias:e.alias,nodeType:e.types[0]??``,properties:r.length>0?r:[nr()],source:`edit-node`},message:null}}function pr(e,t=!1){return e.properties.map(e=>({key:e.key.trim(),value:t?e.value.replace(/\r\n/g,`
`).replace(/\r/g,`
`):e.value.trim()})).filter(e=>e.key||e.value.trim())}function mr(e){let t=er(e);if(!t.valid)throw Error(t.errors.command)}function hr(e,t,n){if(n.includes(`
`)){e.push(`${t}='''`),e.push(n),e.push(`'''`);return}e.push(`${t}=${n}`)}function gr(e){let t=Yn(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid node form state.`);let n=e.alias.trim(),r=e.nodeType.trim(),i=pr(e,!0),a=[`create node ${n}`];if(r&&a.push(`with type ${r}`),i.length>0){a.push(`with properties`);for(let e of i)hr(a,e.key,e.value)}let o=a.join(`
`);return mr(o),o}function _r(e,t){let n=t.trim(),r=Yn(e,{mode:`edit`,originalAlias:n});if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node form state.`);let i=e.nodeType.trim(),a=pr(e,!0),o=[`update node ${n}`];if(i&&o.push(`with type ${i}`),a.length>0){o.push(`with properties`);for(let e of a)hr(o,e.key,e.value)}let s=o.join(`
`);return mr(s),s}function vr(e,t={}){let n=e.trim(),r=Xn(n,t);if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node alias.`);let i=`delete node ${n}`;return mr(i),i}function yr(e,t){let n=e.trim(),r=t.trim(),i=Zn(n,r);if(!i.valid)throw Error(Object.values(i.errors)[0]??`Invalid connection endpoints.`);let a=`delete connection ${n} and ${r}`;return mr(a),a}function br(e){let t=$n(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid connection form state.`);let n=`connect ${e.sourceAlias.trim()} to ${e.targetAlias.trim()} with ${e.relation.trim()}`;return mr(n),n}function xr(e,t){let n=fr(t);if(!n.valid||n.formState===null)throw Error(`Node "${t.alias}" cannot be pasted: its data cannot be written as a ${e} node command (an empty list or map, a value containing ''', or more than one type).`);return e===`update`?_r(n.formState,t.alias):gr(n.formState)}function Sr(e,t){let n=t?.nodes.some(t=>t.alias===e.node.alias)?`update`:`create`;return{verb:n,command:xr(n,e.node)}}function Cr(e){return`${e} ${e===1?`node`:`nodes`} clipped to workspace`}function wr(e){let{added:t,duplicates:n,failed:r}=e;if(t===0&&n===0&&r===0)return{message:`No selected nodes are available to clip.`,type:`info`};if(t===0&&n>0&&r===0)return{message:`All selected nodes already exist in workspace.`,type:`info`};if(t===0&&n===0&&r>0)return{message:`Failed to clip selected nodes to workspace.`,type:`error`};let i=Cr(t);return n>0&&(i+=`. ${n} already existed.`),r>0&&(i+=`. ${r} failed.`),{message:i,type:r>0?`error`:`success`}}function Tr(e){return{execute(t){return e(t)}}}function Er(e,t,n){let r=[];for(let i of e.connections??[])if(!(i.source!==t||i.target!==n))for(let e of i.relations)r.push(e.type);return r}function Dr(e,t){try{let n=new Map,r=(t,r)=>{let i=`${t}\t${r}`,a=n.get(i);return a||(a=Er(e,t,r),n.set(i,a)),a},i=[],a=[],o=new Set;for(let{source:e,target:n,relation:s}of t){let t=r(e,n);if(s===void 0){if(t.length===0)continue;for(let r of t)i.push({source:e,target:n,relation:r});t.length=0}else{let r=t.indexOf(s);if(r===-1)continue;t.splice(r,1),i.push({source:e,target:n,relation:s})}let c=[e,n].sort().join(`	`);o.has(c)||(o.add(c),a.push({a:e,b:n}))}if(i.length===0)return null;let s=[];for(let{a:e,b:t}of a){s.push(yr(e,t));for(let n of r(e,t))s.push(br({sourceAlias:e,targetAlias:t,relation:n}));for(let n of r(t,e))s.push(br({sourceAlias:t,targetAlias:e,relation:n}))}return{commands:s,removed:i}}catch{return null}}function Or(e){let t=e[0];return e.length===1?`relation '${t.relation}' (${t.source} → ${t.target})`:e.every(e=>e.source===t.source&&e.target===t.target)?`connection ${t.source} → ${t.target}`:`${e.length} relations`}function kr(e,t,n){let r=[];for(let i of e.connections??[]){let e=i.source===t&&i.target===n,a=i.source===n&&i.target===t;if(!(!e&&!a))for(let e of i.relations)r.push(br({sourceAlias:i.source,targetAlias:i.target,relation:e.type}))}return r}function Ar(e){if(e.length===0)return null;try{return{label:`delete ${Or(e)}`,inverseCommands:e.map(({source:e,target:t,relation:n})=>br({sourceAlias:e,targetAlias:t,relation:n}))}}catch{return null}}function jr(e){let t=fr(e);if(!t.valid||t.formState===null)return null;try{return{label:`edit node ${e.alias}`,inverseCommands:[_r(t.formState,e.alias)]}}catch{return null}}function Mr(e){try{return{label:`create node ${e}`,inverseCommands:[vr(e)]}}catch{return null}}function Nr(e,t,n){try{let r=kr(e,t,n);return{label:`create connection ${t} → ${n}`,inverseCommands:[yr(t,n),...r]}}catch{return null}}function Pr(e,t){let n=fr(t);if(!n.valid||n.formState===null)return null;try{let r=[gr(n.formState)];for(let n of e.connections??[])if(!(n.source!==t.alias&&n.target!==t.alias))for(let e of n.relations)r.push(br({sourceAlias:n.source,targetAlias:n.target,relation:e.type}));return{label:`delete node ${t.alias}`,inverseCommands:r}}catch{return null}}var Fr=2500;function Ir({bus:e,connected:t,sendRawText:n,addToast:r}){let i=(0,F.useRef)([]),a=(0,F.useRef)(0),o=(0,F.useCallback)(()=>{i.current=[]},[]);(0,F.useEffect)(()=>{t||o()},[o,t]),(0,F.useEffect)(()=>e.on(`graph.mutation`,e=>{e.mutationType===`import-graph`&&o()}),[e,o]);let s=(0,F.useCallback)(e=>{if(e===null||e.inverseCommands.length===0)return null;let t=++a.current;return i.current=[...i.current.slice(-19),{...e,id:t}],t},[]),c=(0,F.useRef)(!1),l=(0,F.useCallback)(()=>new Promise(t=>{let n=!1,r=()=>{n||(n=!0,i(),clearTimeout(a),t())},i=e.on(`graph.mutation`,r),a=setTimeout(r,Fr)}),[e]),u=(0,F.useRef)([]),d=(0,F.useCallback)(e=>{e.length!==0&&(u.current.push(e),!c.current&&(c.current=!0,(async()=>{try{for(;;){let e=u.current.shift();if(!e)break;for(let t of e){if(!n(t)){r(`Could not send the command because the WebSocket is not open.`,`error`),u.current=[];return}await l()}}}finally{c.current=!1}})()))},[r,n,l]),f=(0,F.useCallback)(e=>{if(c.current){r(`A graph edit is already in progress.`,`info`);return}let t=i.current,a=t[t.length-1];if(!a){r(`Nothing to undo.`,`info`);return}if(e!==void 0&&a.id!==e){r(`Newer changes exist — press Ctrl+Z to undo them in order.`,`info`);return}i.current=t.slice(0,-1),c.current=!0,(async()=>{try{for(let e of a.inverseCommands){if(!n(e)){r(`Could not send the undo command because the WebSocket is not open.`,`error`),o();return}await l()}r(`Undo: ${a.label}`,`success`)}finally{c.current=!1}})()},[r,o,n,l]);return{push:s,undoLast:(0,F.useCallback)(()=>f(void 0),[f]),undoEntry:f,runCommands:d,hasEntries:(0,F.useCallback)(()=>i.current.length>0,[]),clear:o}}function Lr(e){return e.trim().toLowerCase()}function Rr(e,t){if(!t.alias||t.action!==null&&t.action!==`delete-node`)return null;let n=e.aliases.find(n=>Lr(n)===Lr(t.alias)&&e.results[n]===void 0);return n?{...e,results:{...e.results,[n]:t.status===`accepted`?`success`:`error`}}:null}function zr(e){return e.aliases.every(t=>e.results[t]!==void 0)}function Br(e){let t=Object.values(e.results).filter(e=>e===`success`).length,n=Object.values(e.results).filter(e=>e===`error`).length,r=`${t} selected ${t===1?`node`:`nodes`} deleted.`;return t===0&&n>0?{message:`Failed to delete selected nodes.`,type:`error`}:n>0?{message:`${r} ${n} failed.`,type:`error`}:{message:r,type:`success`}}var Vr={toastContainer:`_toastContainer_1rn7u_1`,toast:`_toast_1rn7u_1`,slideIn:`_slideIn_1rn7u_1`,success:`_success_1rn7u_36`,error:`_error_1rn7u_40`,info:`_info_1rn7u_44`,toastIcon:`_toastIcon_1rn7u_48`,toastMessage:`_toastMessage_1rn7u_53`,toastAction:`_toastAction_1rn7u_59`},Hr=({toasts:e,onRemove:t})=>e.length===0?null:(0,R.jsx)(`div`,{className:Vr.toastContainer,children:e.map(e=>(0,R.jsxs)(`div`,{className:`${Vr.toast} ${Vr[e.type]}`,onClick:()=>t(e.id),children:[(0,R.jsxs)(`span`,{className:Vr.toastIcon,children:[e.type===`success`&&`✅`,e.type===`error`&&`❌`,e.type===`info`&&`ℹ️`]}),(0,R.jsx)(`span`,{className:Vr.toastMessage,children:e.message}),e.action&&(0,R.jsx)(`button`,{type:`button`,className:Vr.toastAction,onClick:n=>{n.stopPropagation(),e.action?.onClick(),t(e.id)},children:e.action.label})]},e.id))}),Ur={container:`_container_9dbh2_3`,trigger:`_trigger_9dbh2_7`,chevron:`_chevron_9dbh2_37`,chevronOpen:`_chevronOpen_9dbh2_43`,dot:`_dot_9dbh2_49`,dotIdle:`_dotIdle_9dbh2_56`,dotConnecting:`_dotConnecting_9dbh2_57`,pulse:`_pulse_9dbh2_1`,dotConnected:`_dotConnected_9dbh2_58`,dotPartial:`_dotPartial_9dbh2_59`,dropdown:`_dropdown_9dbh2_65`,fadeIn:`_fadeIn_9dbh2_1`};function Wr({label:e,dotStatus:t,children:n}){let[r,i]=(0,F.useState)(!1),a=(0,F.useRef)(null),o=(0,F.useCallback)(()=>i(!1),[]);(0,F.useEffect)(()=>{if(!r)return;let e=e=>{a.current&&!a.current.contains(e.target)&&i(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[r]);let s=e=>{e.key===`Escape`&&(i(!1),a.current?.querySelector(`button[aria-haspopup]`)?.focus())},c=t===`connected`?Ur.dotConnected:t===`connecting`?Ur.dotConnecting:t===`partial`?Ur.dotPartial:t===`idle`?Ur.dotIdle:void 0;return(0,R.jsxs)(`div`,{className:Ur.container,ref:a,onKeyDown:s,children:[(0,R.jsxs)(`button`,{className:Ur.trigger,onClick:()=>i(e=>!e),"aria-haspopup":`true`,"aria-expanded":r,children:[t!==void 0&&(0,R.jsx)(`span`,{className:`${Ur.dot} ${c??``}`,"aria-hidden":`true`}),(0,R.jsx)(`span`,{children:e}),(0,R.jsx)(`span`,{className:`${Ur.chevron} ${r?Ur.chevronOpen:``}`,"aria-hidden":`true`,children:`▾`})]}),r&&(0,R.jsx)(`div`,{className:Ur.dropdown,role:`menu`,children:typeof n==`function`?n(o):n})]})}var Gr={panel:`_panel_1ws0d_1`,section:`_section_1ws0d_7`,sectionHeader:`_sectionHeader_1ws0d_20`,sessionHeaderRow:`_sessionHeaderRow_1ws0d_29`,iconButton:`_iconButton_1ws0d_36`,copyButton:`_copyButton_1ws0d_63`,copyButtonCopied:`_copyButtonCopied_1ws0d_95`,relationshipRow:`_relationshipRow_1ws0d_102`,subscriberRow:`_subscriberRow_1ws0d_103`,sessionId:`_sessionId_1ws0d_114`,statusDot:`_statusDot_1ws0d_127`,metaText:`_metaText_1ws0d_136`,emptyMessage:`_emptyMessage_1ws0d_137`,subscriberList:`_subscriberList_1ws0d_148`,subscribeForm:`_subscribeForm_1ws0d_157`,subscribeInput:`_subscribeInput_1ws0d_164`,subscribeButton:`_subscribeButton_1ws0d_189`,resetButton:`_resetButton_1ws0d_190`,unsubscribeButton:`_unsubscribeButton_1ws0d_218`,actionsRow:`_actionsRow_1ws0d_241`,errorMessage:`_errorMessage_1ws0d_261`,infoMessage:`_infoMessage_1ws0d_262`};function Kr({copied:e}){return e?(0,R.jsx)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:(0,R.jsx)(`path`,{d:`M6.2 11.4 2.9 8.1l1.1-1.1 2.2 2.2 5.8-5.8 1.1 1.1-6.9 6.9Z`})}):(0,R.jsxs)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:[(0,R.jsx)(`path`,{d:`M5 2.5A1.5 1.5 0 0 1 6.5 1h6A1.5 1.5 0 0 1 14 2.5v6A1.5 1.5 0 0 1 12.5 10H11V8.7h1.5a.2.2 0 0 0 .2-.2v-6a.2.2 0 0 0-.2-.2h-6a.2.2 0 0 0-.2.2V4H5V2.5Z`}),(0,R.jsx)(`path`,{d:`M2 6.5A1.5 1.5 0 0 1 3.5 5h6A1.5 1.5 0 0 1 11 6.5v7A1.5 1.5 0 0 1 9.5 15h-6A1.5 1.5 0 0 1 2 13.5v-7Zm1.5-.2a.2.2 0 0 0-.2.2v7a.2.2 0 0 0 .2.2h6a.2.2 0 0 0 .2-.2v-7a.2.2 0 0 0-.2-.2h-6Z`})]})}function qr(e){return e.connected?e.state.pendingCommand===`refresh`||e.state.loading?`connecting`:e.state.sessionId?`connected`:`partial`:`idle`}function Jr({controller:e}){let{state:t}=e,[n,r]=(0,F.useState)(!1),[i,a]=(0,F.useState)(``),[o,s]=(0,F.useState)(null),[c,l]=(0,F.useState)(null),u=(0,F.useRef)(null),d=e.canReset;(0,F.useEffect)(()=>()=>{u.current!==null&&window.clearTimeout(u.current)},[]),(0,F.useEffect)(()=>{t.subscribedTo&&t.pendingCommand!==`subscribe`&&(r(!1),a(``))},[t.pendingCommand,t.subscribedTo]);let f=t=>{t.preventDefault(),e.subscribeToSession(i)},p=async e=>{try{await navigator.clipboard.writeText(e),s(null),l(e),u.current!==null&&window.clearTimeout(u.current),u.current=window.setTimeout(()=>{l(null),u.current=null},1400)}catch{l(null),s(`Could not copy the session ID. Please copy it manually.`)}};return e.connected?(0,R.jsxs)(`div`,{className:Gr.panel,children:[(0,R.jsxs)(`section`,{className:Gr.section,children:[(0,R.jsx)(`div`,{className:Gr.sectionHeader,children:`This session`}),(0,R.jsxs)(`div`,{className:Gr.sessionHeaderRow,children:[t.sessionId?(0,R.jsx)(`code`,{className:Gr.sessionId,title:t.sessionId,children:t.sessionId}):(0,R.jsx)(`p`,{className:Gr.emptyMessage,children:`Session details are not loaded yet.`}),t.sessionId&&(0,R.jsx)(`button`,{type:`button`,className:`${Gr.copyButton} ${c===t.sessionId?Gr.copyButtonCopied:``}`,onClick:()=>void p(t.sessionId),"aria-label":`Copy session ID`,title:c===t.sessionId?`Copied`:`Copy session ID`,children:(0,R.jsx)(Kr,{copied:c===t.sessionId})}),e.canSubscribe&&(0,R.jsx)(`button`,{type:`button`,className:Gr.iconButton,onClick:()=>{e.clearMessage(),r(e=>!e)},"aria-expanded":n,"aria-label":n?`Close subscribe form`:`Subscribe to another session`,title:n?`Close subscribe form`:`Subscribe to another session`,children:n?`×`:`+`})]}),t.startedSince&&(0,R.jsxs)(`div`,{className:Gr.metaText,children:[`Started since `,t.startedSince]}),n&&(0,R.jsxs)(`form`,{className:Gr.subscribeForm,onSubmit:f,children:[(0,R.jsx)(`input`,{className:Gr.subscribeInput,value:i,onChange:t=>{a(t.target.value),e.clearMessage()},placeholder:`ws-123456-1`,autoComplete:`off`,disabled:!e.canSubscribe}),(0,R.jsx)(`button`,{type:`submit`,className:Gr.subscribeButton,disabled:!e.canSubscribe||i.trim().length===0,children:t.pendingCommand===`subscribe`?`...`:`Subscribe`})]})]}),t.subscribedTo&&(0,R.jsxs)(`section`,{className:Gr.section,children:[(0,R.jsx)(`div`,{className:Gr.sectionHeader,children:`Subscribed to`}),(0,R.jsxs)(`div`,{className:Gr.relationshipRow,children:[(0,R.jsx)(`span`,{className:Gr.statusDot,"aria-hidden":`true`}),(0,R.jsx)(`code`,{className:Gr.sessionId,title:t.subscribedTo,children:t.subscribedTo}),(0,R.jsx)(`button`,{type:`button`,className:Gr.unsubscribeButton,onClick:e.unsubscribe,disabled:!e.canUnsubscribe,children:t.pendingCommand===`unsubscribe`?`...`:`Unsubscribe`})]})]}),t.subscribers.length>0&&(0,R.jsxs)(`section`,{className:Gr.section,children:[(0,R.jsx)(`div`,{className:Gr.sectionHeader,children:`Subscribers`}),(0,R.jsx)(`ul`,{className:Gr.subscriberList,children:t.subscribers.map(e=>(0,R.jsxs)(`li`,{className:Gr.subscriberRow,children:[(0,R.jsx)(`span`,{className:Gr.statusDot,"aria-hidden":`true`}),(0,R.jsx)(`code`,{className:Gr.sessionId,title:e,children:e})]},e))})]}),d&&(0,R.jsx)(`div`,{className:Gr.actionsRow,children:(0,R.jsx)(`button`,{type:`button`,className:Gr.resetButton,onClick:e.resetSession,disabled:!e.canReset,children:t.pendingCommand===`reset`?`Resetting...`:`Reset Session`})}),t.error&&(0,R.jsx)(`div`,{className:Gr.errorMessage,role:`alert`,children:t.error}),o&&(0,R.jsx)(`div`,{className:Gr.errorMessage,role:`alert`,children:o})]}):(0,R.jsx)(`div`,{className:Gr.panel,children:(0,R.jsx)(`p`,{className:Gr.emptyMessage,children:`Connect Minigraph to view session details.`})})}function Yr({controller:e}){return(0,R.jsx)(Wr,{label:`Session`,dotStatus:qr(e),children:(0,R.jsx)(Jr,{controller:e})})}var Xr={nav:`_nav_7x22x_3`,menuList:`_menuList_7x22x_11`,menuItem:`_menuItem_7x22x_19`,menuButton:`_menuButton_7x22x_47`,menuSection:`_menuSection_7x22x_57`,toolRow:`_toolRow_7x22x_72`,toolLink:`_toolLink_7x22x_83`,toolLinkActive:`_toolLinkActive_7x22x_108`,toolDot:`_toolDot_7x22x_115`,toolDotIdle:`_toolDotIdle_7x22x_122`,toolDotConnecting:`_toolDotConnecting_7x22x_123`,pulse:`_pulse_7x22x_1`,toolDotConnected:`_toolDotConnected_7x22x_124`,connectAllRow:`_connectAllRow_7x22x_128`,connectAllBtn:`_connectAllBtn_7x22x_134`,connectAllBtnStop:`_connectAllBtnStop_7x22x_158`,toolConnectBtn:`_toolConnectBtn_7x22x_170`,toolConnectBtnStop:`_toolConnectBtnStop_7x22x_196`,externalIcon:`_externalIcon_7x22x_208`};function Zr(e){return e.every(e=>e===`connected`)?`connected`:e.every(e=>e===`idle`)?`idle`:e.some(e=>e===`connecting`)?`connecting`:`partial`}function Qr(e){return e===`connected`?`connected`:e===`connecting`?`connecting`:`idle`}var $r=[{href:`/info`,label:`Info`},{href:`/info/lib`,label:`Libraries`},{href:`/info/routes`,label:`Services`},{href:`/health`,label:`Health`},{href:`/env`,label:`Environment`},{href:`http://localhost:8085/api/ws/json`,label:`Legacy JSON`},{href:`http://localhost:8085/api/ws/graph`,label:`Legacy Graph`}];function ei({addToast:e,sessionCollaboration:t,onPackageGraphs:n,onEditSchema:r}){let i=_e(),a=ue.map(e=>i.getSlot(e.wsPath).phase),o=Zr(a),s=a.every(e=>e===`connected`),c=a.some(e=>e===`connecting`);function l(){ue.forEach(t=>{i.getSlot(t.wsPath).phase===`idle`&&i.connect(t.wsPath,e)})}function u(){ue.forEach(e=>{let{phase:t}=i.getSlot(e.wsPath);(t===`connected`||t===`connecting`)&&i.disconnect(e.wsPath)})}return(0,R.jsxs)(`nav`,{className:Xr.nav,"aria-label":`Main navigation`,children:[t&&(0,R.jsx)(Yr,{controller:t}),(0,R.jsx)(Wr,{label:`Tools`,dotStatus:o,children:t=>(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`div`,{className:Xr.connectAllRow,children:(0,R.jsx)(`button`,{className:`${Xr.connectAllBtn} ${s?Xr.connectAllBtnStop:``}`,onClick:s?u:l,disabled:c,"aria-label":c?`Connecting…`:s?`Disconnect all WebSockets`:`Connect all WebSockets`,children:c?`Connecting…`:s?`Disconnect All`:`Connect All`})}),(0,R.jsx)(`ul`,{className:Xr.menuList,role:`none`,children:ue.map(t=>{let{phase:n}=i.getSlot(t.wsPath),r=Qr(n),a=n===`connected`,o=n===`connecting`,s=r===`connected`?Xr.toolDotConnected:r===`connecting`?Xr.toolDotConnecting:Xr.toolDotIdle;return(0,R.jsxs)(`li`,{role:`none`,className:Xr.toolRow,children:[(0,R.jsxs)(w,{to:t.path,role:`menuitem`,className:({isActive:e})=>`${Xr.toolLink} ${e?Xr.toolLinkActive:``}`,children:[(0,R.jsx)(`span`,{className:`${Xr.toolDot} ${s}`,"aria-hidden":`true`}),(0,R.jsx)(`span`,{className:Xr.toolLabel,children:t.label})]}),(0,R.jsx)(`button`,{className:`${Xr.toolConnectBtn} ${a?Xr.toolConnectBtnStop:``}`,onClick:()=>a||o?i.disconnect(t.wsPath):i.connect(t.wsPath,e),disabled:o,"aria-label":o?`Connecting…`:a?`Disconnect ${t.label}`:`Connect ${t.label}`,title:o?`Connecting…`:fe(t.wsPath),children:o?`…`:a?`Stop`:`Start`})]},t.path)})}),(r||n)&&(0,R.jsxs)(`ul`,{className:`${Xr.menuList} ${Xr.menuSection}`,role:`none`,children:[r&&(0,R.jsx)(`li`,{role:`none`,children:(0,R.jsx)(`button`,{type:`button`,role:`menuitem`,className:`${Xr.menuItem} ${Xr.menuButton}`,onClick:()=>{t(),r()},title:`Declare the graph's request and response contract - the schema on the root and end nodes`,children:`📐 Graph schema`})}),n&&(0,R.jsx)(`li`,{role:`none`,children:(0,R.jsx)(`button`,{type:`button`,role:`menuitem`,className:`${Xr.menuItem} ${Xr.menuButton}`,onClick:()=>{t(),n()},title:`Pack graph models into a graph set (<set>.pack), or inspect one`,children:`📦 Graph set packaging`})})]})]})}),(0,R.jsx)(Wr,{label:`Quick Links`,children:(0,R.jsx)(`ul`,{className:Xr.menuList,role:`none`,children:$r.map(e=>(0,R.jsx)(`li`,{role:`none`,children:(0,R.jsxs)(`a`,{href:e.href,role:`menuitem`,className:Xr.menuItem,target:`_blank`,rel:`noopener noreferrer`,children:[e.label,(0,R.jsx)(`span`,{className:Xr.externalIcon,"aria-hidden":`true`,children:`↗`})]})},e.href))})})]})}var ti={saveBtn:`_saveBtn_ek34s_3`,saveBtnSaved:`_saveBtnSaved_ek34s_25`,saveBtnLabel:`_saveBtnLabel_ek34s_35`,saveForm:`_saveForm_ek34s_50`,saveInput:`_saveInput_ek34s_56`,saveInputWarn:`_saveInputWarn_ek34s_72`,saveWarnLabel:`_saveWarnLabel_ek34s_76`,saveActionBtn:`_saveActionBtn_ek34s_82`};function ni(e){return e?`Saved: ${e}`:`Save Graph`}function ri({disabled:e,defaultName:t,savedName:n=null,onSave:r,nameExists:i,connected:a=!1}){let[o,s]=(0,F.useState)(!1),[c,l]=(0,F.useState)(``),u=(0,F.useRef)(null),d=(0,F.useRef)(null),f=(0,F.useRef)(!1),p=(0,F.useCallback)(()=>{l(t),s(!0)},[t]),m=(0,F.useCallback)(()=>{f.current=!0,s(!1),l(``)},[]),h=(0,F.useCallback)(()=>{let e=c.trim();e&&(r(e),f.current=!0,s(!1),l(``))},[c,r]),g=(0,F.useCallback)(e=>{e.key===`Enter`&&(e.preventDefault(),h()),e.key===`Escape`&&(e.preventDefault(),m())},[h,m]);return(0,F.useEffect)(()=>{o?u.current?.focus():f.current&&(f.current=!1,d.current?.focus())},[o]),o?(0,R.jsxs)(`div`,{className:ti.saveForm,children:[(0,R.jsx)(`input`,{ref:u,className:`${ti.saveInput}${i?.(c.trim())?` ${ti.saveInputWarn}`:``}`,type:`text`,value:c,onChange:e=>l(e.target.value),onKeyDown:g,placeholder:`Enter a name…`,"aria-label":`Graph save name`,maxLength:80}),i?.(c.trim())&&(0,R.jsx)(`span`,{className:ti.saveWarnLabel,role:`status`,children:`Overwrite?`}),(0,R.jsx)(`button`,{className:ti.saveActionBtn,onClick:h,disabled:!c.trim(),"aria-label":`Confirm save`,children:`✅`}),(0,R.jsx)(`button`,{className:ti.saveActionBtn,onClick:m,"aria-label":`Cancel save`,children:`❌`})]}):(0,R.jsxs)(`button`,{ref:d,className:`${ti.saveBtn}${n?` ${ti.saveBtnSaved}`:``}`,onClick:p,disabled:e||!a,title:e?`No graph loaded`:a?n?`Graph saved as ${n}. Click to save again`:`Export graph snapshot to server and save bookmark`:`Connect first to save`,"aria-label":n?`Graph saved as ${n}. Save again`:`Save graph snapshot`,children:[(0,R.jsx)(`span`,{"aria-hidden":`true`,children:n?`✅`:`💾`}),(0,R.jsx)(`span`,{className:ti.saveBtnLabel,children:ni(n)})]})}var ii={empty:`_empty_tpeii_3`,hint:`_hint_tpeii_12`,list:`_list_tpeii_21`,row:`_row_tpeii_31`,rowInfo:`_rowInfo_tpeii_50`,rowName:`_rowName_tpeii_58`,rowMeta:`_rowMeta_tpeii_67`,rowActions:`_rowActions_tpeii_78`,loadBtn:`_loadBtn_tpeii_84`,deleteBtn:`_deleteBtn_tpeii_85`};function ai({savedGraphs:e,onLoad:t,onDelete:n,connected:r}){return(0,R.jsx)(Wr,{label:e.length>0?`Load Graph (${e.length})`:`Load Graph`,children:e.length===0?(0,R.jsx)(`p`,{className:ii.empty,children:`No saved graphs yet.`}):(0,R.jsxs)(R.Fragment,{children:[!r&&(0,R.jsx)(`p`,{className:ii.hint,children:`Connect to load a graph`}),(0,R.jsx)(`ul`,{className:ii.list,role:`list`,children:e.map(e=>(0,R.jsxs)(`li`,{className:ii.row,children:[(0,R.jsxs)(`div`,{className:ii.rowInfo,children:[(0,R.jsx)(`span`,{className:ii.rowName,title:e.name,children:e.name}),(0,R.jsx)(`span`,{className:ii.rowMeta,children:new Date(e.savedAt).toLocaleString()})]}),(0,R.jsxs)(`div`,{className:ii.rowActions,children:[(0,R.jsx)(`button`,{className:ii.loadBtn,onClick:()=>t(e.name),disabled:!r,title:r?`Run: import graph from ${e.name}`:`Connect to the playground first`,"aria-label":`Load graph ${e.name}`,children:`Load`}),(0,R.jsx)(`button`,{className:ii.deleteBtn,onClick:()=>n(e.name),title:`Remove "${e.name}" from local storage`,"aria-label":`Delete saved graph ${e.name}`,children:`Delete`})]})]},e.name))})]})})}var oi={payloadRoot:`_payloadRoot_6u47x_2`,labelRow:`_labelRow_6u47x_10`,label:`_label_6u47x_10`,payloadControls:`_payloadControls_6u47x_26`,charCounter:`_charCounter_6u47x_32`,typeIndicator:`_typeIndicator_6u47x_38`,validationIcon:`_validationIcon_6u47x_49`,formatButton:`_formatButton_6u47x_53`,uploadButton:`_uploadButton_6u47x_67`,textarea:`_textarea_6u47x_82`,textareaError:`_textareaError_6u47x_107`,errorMessage:`_errorMessage_6u47x_109`,sampleButtonsRow:`_sampleButtonsRow_6u47x_117`,sampleButtons:`_sampleButtons_6u47x_117`,sampleLabel:`_sampleLabel_6u47x_130`,sampleGroup:`_sampleGroup_6u47x_136`,sampleGroupLabel:`_sampleGroupLabel_6u47x_143`,sampleButton:`_sampleButton_6u47x_117`};function si({onLoad:e}){let t=Object.keys(de).filter(e=>e.startsWith(`json_`)),n=Object.keys(de).filter(e=>e.startsWith(`xml_`)),r=e=>e.replace(/^(json|xml)_/,``).replace(/_/g,` `);return(0,R.jsxs)(`div`,{className:oi.sampleButtons,children:[(0,R.jsx)(`span`,{className:oi.sampleLabel,children:`Quick load:`}),(0,R.jsxs)(`div`,{className:oi.sampleGroup,children:[(0,R.jsx)(`span`,{className:oi.sampleGroupLabel,children:`JSON:`}),t.map(t=>(0,R.jsx)(`button`,{className:oi.sampleButton,onClick:()=>e(de[t]),children:r(t)},t))]}),(0,R.jsxs)(`div`,{className:oi.sampleGroup,children:[(0,R.jsx)(`span`,{className:oi.sampleGroupLabel,children:`XML:`}),n.map(t=>(0,R.jsx)(`button`,{className:oi.sampleButton,onClick:()=>e(de[t]),children:r(t)},t))]})]})}function ci({payload:e,onChange:t,validation:n,onFormat:r,onUpload:i}){return(0,R.jsxs)(`div`,{className:oi.payloadRoot,children:[(0,R.jsxs)(`div`,{className:oi.labelRow,children:[(0,R.jsx)(`label`,{htmlFor:`payload`,className:oi.label,children:`JSON/XML Payload`}),(0,R.jsxs)(`div`,{className:oi.payloadControls,children:[(0,R.jsxs)(`span`,{className:oi.charCounter,children:[`size: `,e.length]}),e&&n.type&&(0,R.jsx)(`span`,{className:oi.typeIndicator,children:n.type.toUpperCase()}),e&&(0,R.jsx)(`span`,{className:oi.validationIcon,children:n.valid?`✅`:`❌`}),(0,R.jsx)(`button`,{className:oi.formatButton,onClick:r,disabled:!e||n.type!==`json`,title:n.type===`xml`?`Format only available for JSON`:`Format JSON`,children:`Format`}),i!==void 0&&(0,R.jsx)(`button`,{className:oi.uploadButton,onClick:i,disabled:!e||!n.valid||n.type!==`json`,title:`Upload JSON payload to current session via REST`,children:`Upload`})]})]}),(0,R.jsx)(`textarea`,{id:`payload`,className:`${oi.textarea} ${n.valid?``:oi.textareaError}`,placeholder:`Paste your JSON/XML payload here`,value:e,onChange:e=>t(e.target.value)}),!n.valid&&(0,R.jsx)(`div`,{className:oi.errorMessage,children:n.error}),(0,R.jsx)(`div`,{className:oi.sampleButtonsRow,children:(0,R.jsx)(si,{onLoad:t})})]})}var li=d(),ui={Root:{icon:`🚀`,label:`Root`},End:{icon:`🏁`,label:`End`},Fetcher:{icon:`🌐`,label:`Fetcher`},mapper:{icon:`🗺️`,label:`Mapper`},Math:{icon:`🔢`,label:`Math`},JavaScript:{icon:`📜`,label:`JavaScript`},Provider:{icon:`🔌`,label:`Provider`},Dictionary:{icon:`📖`,label:`Dictionary`},Join:{icon:`🔀`,label:`Join`},Extension:{icon:`🧩`,label:`Extension`},Island:{icon:`🏝️`,label:`Island`},Decision:{icon:`❓`,label:`Decision`},Suspend:{icon:`⏸️`,label:`Suspend`},Resume:{icon:`▶️`,label:`Resume`},Suspensible:{icon:`⏯️`,label:`Suspensible`}},di={boxSizing:`border-box`,borderRadius:`8px`,borderWidth:`1.5px`,borderStyle:`solid`,background:`var(--bg-secondary, #1e1e2e)`,color:`var(--text-primary, #cdd6f4)`,fontSize:`0.75rem`,boxShadow:`0 2px 8px rgba(0,0,0,0.45)`,overflow:`visible`,padding:0},fi={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`,Suspend:`#0d9488`,Resume:`#0284c7`,Suspensible:`#c026d3`},pi=`#6c7086`;function mi(e){return ui[e]??{icon:`📦`,label:e}}function hi(e){return fi[e]??pi}function gi(e){let t=fi[e]??pi;return{...di,borderColor:t,"--node-accent":t}}var _i={content:`_content_1g9w1_8`,header:`_header_1g9w1_24`,icon:`_icon_1g9w1_44`,alias:`_alias_1g9w1_49`,badge:`_badge_1g9w1_55`,body:`_body_1g9w1_67`,bodyPeek:`_bodyPeek_1g9w1_77`,row:`_row_1g9w1_85`,label:`_label_1g9w1_98`,value:`_value_1g9w1_104`,edgeHandle:`_edgeHandle_1g9w1_118`,authoringRing:`_authoringRing_1g9w1_134`,authoringTargetOverlay:`_authoringTargetOverlay_1g9w1_197`,authoringTargetOverlayActive:`_authoringTargetOverlayActive_1g9w1_213`};function vi({label:e,value:t}){return(0,R.jsxs)(`div`,{className:_i.row,children:[(0,R.jsx)(`span`,{className:_i.label,children:e}),(0,R.jsx)(`span`,{className:_i.value,title:t,children:t})]})}function yi({properties:e}){let t=Object.entries(e).filter(([,e])=>e!=null);return t.length===0?null:(0,R.jsx)(R.Fragment,{children:t.map(([e,t])=>Array.isArray(t)?t.map((t,n)=>{let r=typeof t==`string`?t:JSON.stringify(t);return(0,R.jsx)(vi,{label:n===0?e:``,value:r},`${e}-${n}`)}):(0,R.jsx)(vi,{label:e,value:typeof t==`string`?t:JSON.stringify(t)},e))})}function bi({alias:e,nodeType:t,properties:n,compact:r=!1}){let i=mi(t);return(0,R.jsx)(F.Fragment,{children:(0,R.jsxs)(`div`,{className:_i.content,children:[(0,R.jsxs)(`div`,{className:_i.header,children:[(0,R.jsx)(`span`,{className:_i.icon,children:i.icon}),(0,R.jsx)(`span`,{className:_i.alias,children:e}),(0,R.jsx)(`span`,{className:_i.badge,children:i.label})]}),(0,R.jsx)(`div`,{className:r?`${_i.body} ${_i.bodyPeek}`:_i.body,children:(0,R.jsx)(yi,{properties:n})})]})})}function xi({id:e,data:t,isConnectable:n,selected:r}){let[i,a]=(0,F.useState)(!1),o=t.supportsConnectionAuthoring&&!i,c=p(),l=o&&c.inProgress&&c.fromNode?.id!==e;return(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(y,{minWidth:180,minHeight:t.minHeight,isVisible:r,onResizeStart:()=>a(!0),onResizeEnd:()=>a(!1)}),t.targetHandles.map(({id:e,offset:t})=>(0,R.jsx)(s,{id:e,type:`target`,position:_.Left,isConnectable:n,className:_i.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backSourceHandles.map(({id:e,offset:t})=>(0,R.jsx)(s,{id:e,type:`source`,position:_.Left,isConnectable:n,className:_i.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),(0,R.jsx)(bi,{alias:t.alias,nodeType:t.nodeType,properties:t.properties,compact:t.compact}),o&&(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(s,{id:`authoring-source`,type:`source`,position:_.Right,isConnectable:n,isConnectableEnd:!1,className:_i.authoringRing}),(0,R.jsx)(s,{id:`authoring-target`,type:`target`,position:_.Left,isConnectable:n,isConnectableStart:!1,className:l?`${_i.authoringTargetOverlay} ${_i.authoringTargetOverlayActive}`:_i.authoringTargetOverlay})]}),t.sourceHandles.map(({id:e,offset:t})=>(0,R.jsx)(s,{id:e,type:`source`,position:_.Right,isConnectable:n,className:_i.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backTargetHandles.map(({id:e,offset:t})=>(0,R.jsx)(s,{id:e,type:`target`,position:_.Right,isConnectable:n,className:_i.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e))]})}var Si={Root:xi,End:xi,Fetcher:xi,mapper:xi,Math:xi,JavaScript:xi,Provider:xi,Dictionary:xi,Join:xi,Extension:xi,Island:xi,Decision:xi,default:xi},z={graphWrapper:`_graphWrapper_ped3d_15`,graphSurface:`_graphSurface_ped3d_24`,empty:`_empty_ped3d_30`,emptyIcon:`_emptyIcon_ped3d_43`,emptyCreateButton:`_emptyCreateButton_ped3d_48`,emptyHint:`_emptyHint_ped3d_70`,emptyActions:`_emptyActions_ped3d_75`,emptySecondaryButton:`_emptySecondaryButton_ped3d_87`,refreshingOverlay:`_refreshingOverlay_ped3d_177`,clipboardDropOverlay:`_clipboardDropOverlay_ped3d_189`,fileDropOverlay:`_fileDropOverlay_ped3d_203`,fileDropMessage:`_fileDropMessage_ped3d_216`,clipboardDropMessage:`_clipboardDropMessage_ped3d_227`,refreshingSpinner:`_refreshingSpinner_ped3d_242`,graphRefreshSpin:`_graphRefreshSpin_ped3d_1`,connectBanner:`_connectBanner_ped3d_256`,connectBannerCancel:`_connectBannerCancel_ped3d_276`,detailToggleIcon:`_detailToggleIcon_ped3d_316`,detailToggleHeader:`_detailToggleHeader_ped3d_327`,detailToggleLine:`_detailToggleLine_ped3d_334`},Ci=class extends F.Component{constructor(...e){super(...e),this.state={caughtError:null}}static getDerivedStateFromError(e){return{caughtError:e instanceof Error?e.message:String(e)}}componentDidCatch(e,t){let n=e instanceof Error?e.message:String(e);console.error(`[GraphView] Render error:`,n,t.componentStack),this.props.onRenderError?.(`Graph render failed: ${n}`)}render(){return this.state.caughtError?(0,R.jsxs)(`div`,{className:z.empty,children:[(0,R.jsx)(`span`,{className:z.emptyIcon,children:`⚠️`}),(0,R.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,R.jsx)(`span`,{children:this.state.caughtError})]}):this.props.children}},wi=[`fetch`,`details`,`ext-call`,`mapping`,`compute`,`calculate`,`evaluate`,`fork`,`join`,`one`,`two`,`three`,`more`,`done`,`complete`,`finish`,`positive`,`negative`],Ti={fetch:`#0369a1`,details:`#0369a1`,"ext-call":`#0369a1`,mapping:`#b45309`,compute:`#b45309`,calculate:`#b45309`,evaluate:`#b45309`,fork:`#7e22ce`,join:`#7e22ce`,one:`#7e22ce`,two:`#6d28d9`,three:`#5b21b6`,more:`#4c1d95`,done:`#15803d`,complete:`#15803d`,finish:`#15803d`,positive:`#15803d`,negative:`#b91c1c`},Ei=[`#0369a1`,`#15803d`,`#b45309`,`#7e22ce`,`#b91c1c`,`#0f766e`,`#c2410c`,`#a16207`];function Di(e){let t=0;for(let n=0;n<e.length;n++)t=(t<<5)-t+e.charCodeAt(n),t|=0;return Math.abs(t)}function Oi(e,t){if(e.length===0)return t;let n=e[0].trim().toLowerCase();return Ti[n]||Ei[Di(n)%Ei.length]}var ki=240,Ai=480,ji=40,Mi=1.5,B=100,V=100,Ni=60,Pi=120,Fi=360,Ii=120,Li=80,Ri=4,zi=1e4,Bi=2e3,Vi=64,Hi=256,Ui=256,Wi=512,Gi=.001,Ki=8,qi=16,Ji=`rgba(148, 163, 184, 0.42)`,Yi=`var(--bg-secondary)`,Xi=24,Zi=32;function Qi(e){return Oi(e,Ji)}function $i(e){return`source-${e}`}function ea(e){return`target-${e}`}function ta(e){return`back-source-${e}`}function na(e){return`back-target-${e}`}function ra(e,t){return t<=1?0:t===2?e===0?-24:Xi:(e-(t-1)/2)*Xi}function ia(e,t=B){return e<=1?t:Math.max(t,(e-1)*Xi+Zi*2)}var aa=40,oa=9,sa=18,ca=86,la=7;function ua(e){return Math.floor((e-ca)/la)}function da(e){let t=[];for(let n of Object.values(e.properties??{})){if(n==null)continue;let e=Array.isArray(n)?n:[n];for(let n of e)t.push(typeof n==`string`?n:JSON.stringify(n)??``)}return t}function fa(e,t){return Math.max(1,Math.ceil(e.length/t))}function pa(e,t){let n=ua(t),r=aa;for(let t of da(e))r+=oa+fa(t,n)*sa;return Math.max(B,r)}function ma(e){let t=da(e),n=t.length*Mi;for(let e=ki;e<Ai;e+=ji){let r=ua(e);if(t.reduce((e,t)=>e+fa(t,r),0)<=n)return e}return Ai}var ha=new Set([`graph.math`,`graph.js`]),ga=[`Dictionary`,`Provider`,`Module`,`Entity`],_a={ROOT_TREE:0,DEFAULT_TREE:1,END_TREE:2};function va(e){return e.alias.toLowerCase()===`root`||e.types.includes(`Root`)||e.types.includes(`entry_point`)}function ya(e){return e.alias.toLowerCase()===`end`||e.types.includes(`End`)}function ba(e){return e.hasRoot?_a.ROOT_TREE:e.hasEnd?_a.END_TREE:_a.DEFAULT_TREE}function xa(e,t){let n=ba(e)-ba(t);return n===0?e.sortKey.localeCompare(t.sortKey):n}function Sa(e){return`real:${e}`}function Ca(e){return new Map([...e].map(([e,t])=>[e,t.slice()]))}function wa(e){return new Map(e.map((e,t)=>[e.id,t]))}function Ta(e,t,n){let r=(t.get(e)??[]).map(e=>n.get(e)).filter(e=>e!==void 0);if(r.length!==0)return r.reduce((e,t)=>e+t,0)/r.length}function Ea(e,t,n){let r=wa(e);return e.slice().sort((e,i)=>{let a=Ta(e.id,t,n),o=Ta(i.id,t,n);if(a!==void 0&&o!==void 0){let e=a-o;if(Math.abs(e)>2**-52)return e}let s=r.get(e.id)-r.get(i.id);return s===0?e.stableKey.localeCompare(i.stableKey):s})}function Da(e,t){let n=Array(t+1).fill(0),r=e=>{let t=0;for(let r=e;r>0;r-=r&-r)t+=n[r];return t},i=e=>{for(let t=e;t<n.length;t+=t&-t)n[t]+=1},a=0;for(let t=e.length-1;t>=0;t--){let n=e[t]+1;a+=r(n-1),i(n)}return a}function Oa(e,t){let n=new Map([...e].map(([e,t])=>[e,wa(t)])),r=new Map;for(let e of t)r.has(e.level)||r.set(e.level,[]),r.get(e.level).push(e);let i=0;for(let[e,t]of r){let r=n.get(e),a=n.get(e+1);if(!r||!a)continue;let o=t.map(e=>({sourceId:e.sourceId,targetId:e.targetId,sourcePosition:r.get(e.sourceId),targetPosition:a.get(e.targetId)})).sort((e,t)=>e.sourcePosition-t.sourcePosition||e.targetPosition-t.targetPosition||e.sourceId.localeCompare(t.sourceId)||e.targetId.localeCompare(t.targetId)).map(e=>e.targetPosition);i+=Da(o,a.size)}return i}function ka(e,t,n){e.has(t)||e.set(t,[]),e.get(t).push(n)}function Aa(e){return[e.source,e.target,...e.relations.map(e=>e.type)].join(`	`)}function ja(e,t){return e.nodeIntrusions<t.nodeIntrusions||e.nodeIntrusions===t.nodeIntrusions&&e.edgeCrossings<t.edgeCrossings}function Ma(e){let t=new Map,n=e.size>0?Math.max(...e.keys()):-1,r=0;for(let i=0;i<=n;i++){let n=(e.get(i)??[]).reduce((e,t)=>Math.max(e,t.width),0)||ki;t.set(i,{x:r,width:n}),r+=n+Pi}return t}function Na(e){let t=new Map,n=Ma(e);for(let[r,i]of e){let e=-(i.reduce((e,t)=>e+t.height,0)+Math.max(0,i.length-1)*Ni)/2;for(let a of i)a.alias&&t.set(a.alias,{alias:a.alias,level:r,x:n.get(r).x,y:e,width:a.width,height:a.height}),e+=a.height+Ni}return t}function Pa(e,t,n,r,i){let a=1-i;return a*a*a*e+3*a*a*i*t+3*a*i*i*n+i*i*i*r}function Fa(e,t,n,r){let i=0,a=1;for(let o=0;o<32;o++){let o=(i+a)/2;Pa(e,t,t,n,o)<r?i=o:a=o}return(i+a)/2}function Ia(e,t,n,r,i){let a=e.x+e.width,o=t.x,s=i.x+Gi,c=i.x+i.width-Gi;if(s>=o||c<=a)return null;let l=a+(o-a)/2,u=Fa(a,l,o,s),d=Fa(a,l,o,c),f=e.y+e.height/2+n,p=t.y+t.height/2+r,m=Pa(f,f,p,p,u),h=Pa(f,f,p,p,d);return{yLow:Math.min(m,h),yHigh:Math.max(m,h)}}function La(e,t,n,r,i){let a=Ia(e,t,n,r,i);if(!a)return!1;let o=i.y+Gi,s=i.y+i.height-Gi;return a.yHigh>o&&a.yLow<s}function Ra(e,t){let n=new Map;for(let[e,r]of t)n.has(r)||n.set(r,[]),n.get(r).push(e);for(let e of n.values())e.sort();let r=[];for(let[i,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(!(e===void 0||o===void 0||o-e<=1)){for(let t=e+1;t<o;t++)for(let e of n.get(t)??[])if(r.push({connectionIndex:i,nodeAlias:e}),r.length>Bi)return null}}return r}function za(e,t,n){let r=new Map,i=new Map;for(let e of t.keys())r.set(e,[]),i.set(e,[]);for(let[n,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(e===void 0||o===void 0)continue;let s=e>=o,c=Aa(a);s?(i.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),r.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c})):(r.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),i.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c}))}let a=(e,t)=>n(e.peerAlias)-n(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex;for(let e of r.values())e.sort(a);for(let e of i.values())e.sort(a);let o=new Map,s=new Map;for(let e of r.values())for(let[t,n]of e.entries()){let r=ra(t,e.length);n.isBack?s.set(n.connectionIndex,r):o.set(n.connectionIndex,r)}for(let e of i.values())for(let[t,n]of e.entries()){let r=ra(t,e.length);n.isBack?o.set(n.connectionIndex,r):s.set(n.connectionIndex,r)}return{sourceOffsets:o,targetOffsets:s}}function Ba(e,t,n,r){let i=Na(e),{sourceOffsets:a,targetOffsets:o}=za(n,t,e=>i.get(e)?.y??0),s=0;for(let{connectionIndex:e,nodeAlias:t}of r){let r=n[e],c=i.get(r.source),l=i.get(r.target),u=i.get(t);!c||!l||!u||La(c,l,a.get(e)??0,o.get(e)??0,u)&&(s+=1)}return s}function Va(e,t,n,r){let i=e=>t.map(t=>e.get(t).map(e=>e.id).join(`	`)).join(`
`),a=Ca(e),o=1,s=a,c=n(a),l=[{candidate:a,depth:0}],u=new Set([i(a)]),d=0;for(;d<l.length&&o<r;){let{candidate:e,depth:a}=l[d++];if(!(a>=2))for(let d of t){let t=e.get(d);for(let f=0;f<t.length;f++){for(let p=0;p<t.length;p++){if(f===p)continue;let t=Ca(e),m=t.get(d),[h]=m.splice(f,1);m.splice(p,0,h);let g=i(t);if(u.has(g))continue;u.add(g);let _=n(t);if(o+=1,ja(_,c)&&(s=t,c=_),a+1<2&&l.push({candidate:t,depth:a+1}),o>=r)break}if(o>=r)break}if(o>=r)break}}for(let n of t)e.set(n,s.get(n).slice());return o}function Ha(e,t,n,r,i){let a=new Map;for(let[t,n]of e)a.has(n)||a.set(n,[]),a.get(n).push({id:Sa(t),alias:t,height:r.get(t)??B,width:i.get(t)??ki,stableKey:`real:${t}`});for(let e of a.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let o=Ca(a),s=Ca(a),c=t.map((t,n)=>({connectionIndex:n,source:t.source,target:t.target,sourceLevel:e.get(t.source),targetLevel:e.get(t.target),stableKey:Aa(t)})).filter(e=>e.sourceLevel!==void 0&&e.targetLevel!==void 0&&!n.has(`${e.source}\t${e.target}`)&&e.sourceLevel<e.targetLevel).sort((e,t)=>e.source.localeCompare(t.source)||e.target.localeCompare(t.target)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex),l=c.reduce((e,t)=>e+(t.targetLevel-t.sourceLevel),0)<=zi,u=new Map,d=new Map,f=[];for(let[e,t]of c.entries()){let n=t.sourceLevel,r=t.targetLevel;if(!l&&r-n>1)continue;let i=Sa(t.source),a=n;for(let o=n+1;o<r;o++){let n=`dummy:${t.source}\t${t.target}\t${e}\t${o}`;s.has(o)||s.set(o,[]),s.get(o).push({id:n,height:B,width:0,stableKey:n}),f.push({sourceId:i,targetId:n,level:a}),ka(d,i,n),ka(u,n,i),i=n,a=o}let o=Sa(t.target);f.push({sourceId:i,targetId:o,level:a}),ka(d,i,o),ka(u,o,i)}for(let e of s.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let p=[...s.keys()].sort((e,t)=>e-t);if(p.length<=1||f.length===0)return o;let m=Ca(s),h=Ca(s),g=Ra(t,e),_=g?.length??2001,v=g!==null,y=n=>({nodeIntrusions:v?Ba(n,e,t,g??[]):0,edgeCrossings:Oa(n,f)}),b=y(h),x=()=>{let e=y(m);ja(e,b)&&(b=e,h=Ca(m))},S=[...s.values()].reduce((e,t)=>e+t.length,0)<=Vi&&f.length<=Hi&&_<=Ui,C=0,w=()=>{let e=Wi-C;!S||e<=1||(C+=Va(m,p,y,e),x())};for(let e=0;e<Ri;e++){for(let e=1;e<p.length;e++){let t=p[e],n=p[e-1];m.set(t,Ea(m.get(t),u,wa(m.get(n))))}x(),w();for(let e=p.length-2;e>=0;e--){let t=p[e],n=p[e+1];m.set(t,Ea(m.get(t),d,wa(m.get(n))))}x(),w()}return v&&Ba(o,e,t,g??[])<b.nodeIntrusions?o:h}function Ua(e,t,n,r,i){let a=n.slice().sort((e,n)=>Aa(t[e.connectionIndex]).localeCompare(Aa(t[n.connectionIndex]))||e.nodeAlias.localeCompare(n.nodeAlias)||e.connectionIndex-n.connectionIndex);for(let{connectionIndex:n,nodeAlias:o}of a){let a=t[n],s=e.get(a.source),c=e.get(a.target),l=e.get(o);if(!s||!c||!l)continue;let u=Ia(s,c,r.get(n)??0,i.get(n)??0,l);if(!u)continue;let d=l.y+Gi,f=l.y+l.height-Gi;if(u.yHigh<=d||u.yLow>=f)continue;let p=u.yHigh+qi-l.y,m=l.y+l.height-(u.yLow-qi);return p<=m?{x:l.x,y:l.y,delta:p,direction:1}:{x:l.x,y:l.y,delta:m,direction:-1}}return null}function Wa(e,t,n,r,i){let a=Ra(n,t);if(!(a===null||a.length===0))for(let o=0;o<Ki;o++){let o=new Map;for(let[n,a]of e){let e=t.get(n);e!==void 0&&o.set(n,{alias:n,level:e,x:a.x,y:a.y,width:i.get(n)??ki,height:r.get(n)??B})}let{sourceOffsets:s,targetOffsets:c}=za(n,t,e=>o.get(e)?.y??0),l=Ua(o,n,a,s,c);if(!l)return;for(let[t,n]of e)n.x===l.x&&(l.direction===1?n.y>=l.y:n.y<=l.y)&&e.set(t,{x:n.x,y:n.y+l.delta*l.direction})}}function Ga(e,t){if(t.has(e.alias))return`flow`;let n=e.types[0]??``,r=typeof e.properties.skill==`string`?e.properties.skill:void 0;return n===`Dictionary`?`Dictionary`:n===`Provider`?`Provider`:r&&ha.has(r)?`Module`:r?`__unknown__`:`Entity`}function Ka(e,t,n,r){let i=new Set;for(let e of t??[])i.add(e.source),i.add(e.target);let a=[],o=[],s=new Map;for(let t of e){let e=Ga(t,i);s.set(t.alias,e),e===`flow`?a.push(t):o.push(t)}let c=new Set(a.map(e=>e.alias)),l=new Map(a.map(e=>[e.alias,e])),u=new Map,d=new Map,f=new Map;for(let e of a)u.set(e.alias,[]),d.set(e.alias,new Set),f.set(e.alias,0);for(let e of t??[])!c.has(e.source)||!c.has(e.target)||(u.get(e.source)?.push(e.target),d.get(e.source)?.add(e.target),d.get(e.target)?.add(e.source),f.set(e.target,(f.get(e.target)??0)+1));for(let e of u.values())e.sort();let p=a.filter(e=>f.get(e.alias)===0||e.types.includes(`entry_point`)||va(e)).map(e=>e.alias).sort(),m=new Set;{let e=new Map;for(let t of a)e.set(t.alias,0);function t(t){if(e.get(t)!==0)return;e.set(t,1);let n=[{node:t,childIdx:0}];for(;n.length>0;){let t=n[n.length-1],r=u.get(t.node)??[];if(t.childIdx>=r.length){e.set(t.node,2),n.pop();continue}let i=r[t.childIdx++],a=e.get(i);a===1?m.add(`${t.node}\t${i}`):a===0&&(e.set(i,1),n.push({node:i,childIdx:0}))}}for(let e of p)t(e);for(let e of[...c].sort())t(e)}let h=[],g=new Set;for(let e of Array.from(c).sort()){if(g.has(e))continue;let t=[],n=[e];for(g.add(e);n.length>0;){let e=n.pop();t.push(e);for(let t of d.get(e)??[])g.has(t)||(g.add(t),n.push(t))}t.sort();let r=t.map(e=>l.get(e)).filter(e=>!!e);h.push({aliases:t,nodes:r,hasRoot:r.some(va),hasEnd:r.some(ya),sortKey:t[0]??``})}h.sort(xa);let _=new Map;h.forEach((e,t)=>{e.aliases.forEach(e=>_.set(e,t))});let v=h.map(()=>[]);for(let e of t){let t=_.get(e.source),n=_.get(e.target);t!==void 0&&t===n&&v[t].push(e)}let y=new Map,b=new Map,x=new Map(r),S=0,C=0;for(let[e,t]of h.entries()){let i=new Set(t.aliases),a=t.nodes.filter(e=>f.get(e.alias)===0||e.types.includes(`entry_point`)||va(e)).map(e=>e.alias).sort();a.length===0&&t.aliases.length>0&&a.push(t.aliases[0]);let o=new Map,s=[...a];a.forEach(e=>o.set(e,0));let c=0;for(;c<s.length;){let e=s[c++],t=o.get(e)??0;for(let n of u.get(e)??[])i.has(n)&&(m.has(`${e}\t${n}`)||(!o.has(n)||o.get(n)<=t)&&(o.set(n,t+1),s.push(n)))}let l=o.size>0?Math.max(...o.values()):0;for(let e of t.aliases)o.has(e)||o.set(e,l+1);let d=new Map;for(let[e,t]of o)d.set(t,Math.max(d.get(t)??ki,r.get(e)??ki));for(let[e,t]of o)x.set(e,d.get(t));let p=Ha(o,v[e],m,n,x),h=Ma(p),g=C+ki;for(let[e,t]of[...p].sort(([e],[t])=>e-t)){let n=-(t.reduce((e,t)=>e+t.height,0)+Math.max(0,t.length-1)*Ni)/2,r=S+e,i=h.get(e),a=C+i.x;g=Math.max(g,a+i.width);for(let e of t)e.alias&&(y.set(e.alias,r),b.set(e.alias,{x:a,y:n})),n+=e.height+Ni}let _=o.size>0?Math.max(...o.values()):0;S+=_+1,C=g+Fi}Wa(b,y,t,n,x);let w=0;for(let[e,t]of b)w=Math.max(w,t.y+(n.get(e)??B));let T=w+(b.size>0?Ii:0),E=new Map;for(let e of ga)E.set(e,[]);E.set(`__unknown__`,[]);for(let e of o){let t=s.get(e.alias);E.get(t).push(e.alias)}for(let e of[...ga,`__unknown__`]){let t=(E.get(e)??[]).slice().sort();if(t.length===0)continue;let r=0,i=t.reduce((e,t)=>Math.max(e,n.get(t)??B),0);for(let e of t)b.set(e,{x:r,y:T}),r+=(x.get(e)??ki)+Pi;T+=i+Li}return{positions:b,levelOf:y,widths:x}}function qa(e,t){return new Map(e.map(e=>[e.alias,t?ki:ma(e)]))}function Ja(e,t,n,r){let i=new Map,a=new Map;for(let e of t)i.set(e.source,(i.get(e.source)??0)+1),a.set(e.target,(a.get(e.target)??0)+1);return new Map(e.map(e=>{let t=ia(Math.max(i.get(e.alias)??0,a.get(e.alias)??0),n?V:B);return[e.alias,n?t:Math.max(t,pa(e,r.get(e.alias)??ki))]}))}function Ya(e,t,n={}){let r=e.connections??[],i=n.compactNodes===!0,a=qa(e.nodes,i),o=Ja(e.nodes,r,i,a);for(let[e,n]of t)o.has(e)&&Number.isFinite(n)&&n>0&&o.set(e,n);return Ka(e.nodes,r,o,a).positions}function Xa(e,t={}){let n=e.connections??[],r=t.supportsConnectionAuthoring===!0,i=t.compactNodes===!0,a=qa(e.nodes,i),o=Ja(e.nodes,n,i,a),{positions:s,levelOf:c,widths:l}=Ka(e.nodes,n,o,a),d=new Set;for(let[e,t]of n.entries()){let n=c.get(t.source),r=c.get(t.target);n!==void 0&&r!==void 0&&n>=r&&d.add(e)}let f=new Map,p=new Map;for(let t of e.nodes)f.set(t.alias,[]),p.set(t.alias,[]);for(let[e,t]of n.entries()){let n=Aa(t);d.has(e)?(p.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!0,stableKey:n}),f.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!0,stableKey:n})):(f.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!1,stableKey:n}),p.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!1,stableKey:n}))}let m=e=>s.get(e)?.y??0,h=(e,t)=>m(e.peerAlias)-m(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connIndex-t.connIndex;for(let e of f.values())e.sort(h);for(let e of p.values())e.sort(h);let g=new Map,_=new Map,v=e.nodes.map(e=>{let t=f.get(e.alias)??[],n=p.get(e.alias)??[],a=ia(Math.max(t.length,n.length),i?V:B),c=Math.max(a,o.get(e.alias)??B),u=[],d=[],m=0,h=0;for(let e=0;e<t.length;e++){let n=t[e],r=ra(e,t.length);if(n.isBack){let e=na(h++);d.push({id:e,offset:r}),_.set(n.connIndex,e)}else{let e=$i(m++);u.push({id:e,offset:r}),g.set(n.connIndex,e)}}let v=[],y=[],b=0,x=0;for(let e=0;e<n.length;e++){let t=n[e],r=ra(e,n.length);if(t.isBack){let e=ta(x++);y.push({id:e,offset:r}),g.set(t.connIndex,e)}else{let e=ea(b++);v.push({id:e,offset:r}),_.set(t.connIndex,e)}}return{id:e.alias,type:e.types[0]??`default`,className:`nokey`,position:s.get(e.alias)??{x:0,y:0},width:l.get(e.alias)??ki,...i?{height:c}:{initialHeight:c},style:{...gi(e.types[0]??`unknown`),minHeight:a},data:{alias:e.alias,nodeType:e.types[0]??`unknown`,properties:e.properties,sourceHandles:u,targetHandles:v,backSourceHandles:y,backTargetHandles:d,supportsConnectionAuthoring:r,compact:i,minHeight:a}}}),y=[];for(let[e,t]of n.entries()){let n=t.relations.map(e=>e.type),r=`${t.source}__${t.target}__${e}`,i=Qi(n);y.push({id:r,source:t.source,target:t.target,sourceHandle:g.get(e),targetHandle:_.get(e),label:n.join(`, `),type:`bezier`,markerEnd:{type:u.ArrowClosed,width:16,height:16,color:Ji},style:{stroke:Ji,strokeWidth:2},labelStyle:{fill:i,fontSize:10,fontWeight:700},labelBgStyle:{fill:Yi,fillOpacity:.94,stroke:`rgba(15, 23, 42, 0.16)`,strokeWidth:1},labelBgPadding:[5,2],labelBgBorderRadius:6,data:{relationTypes:n}})}return{nodes:v,edges:y}}var Za=/^[A-Za-z0-9_-]+$/,Qa=`letters, digits, hyphen or underscore`;function $a(e){return Za.test(e)}function eo(e){return(e??``).trim().replace(/[^A-Za-z0-9_-]+/g,`-`).replace(/^-+|-+$/g,``)||`untitled`}function to(e){return`${e}.json`}var no=new Set([`nodes`,`connections`]);function ro(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function io(e){if(!ro(e))return{ok:!1,error:`The file is not a JSON object with a 'nodes' section.`};let t=Object.keys(e).filter(e=>!no.has(e)).sort();if(t.length>0)return{ok:!1,error:`Unexpected top-level section${t.length>1?`s`:``}: ${t.join(`, `)}. A graph model has only 'nodes' and 'connections'.`};let n=e.nodes;if(!Array.isArray(n))return{ok:!1,error:`The 'nodes' section is mandatory and must be a list.`};if(n.length===0)return{ok:!1,error:`The 'nodes' section is empty - there is nothing to import.`};let r=null;for(let[e,t]of n.entries()){let n=`Node entry ${e+1}`;if(!ro(t))return{ok:!1,error:`${n} is not an object.`};if(typeof t.alias!=`string`||t.alias.trim()===``)return{ok:!1,error:`${n} has no alias.`};let i=t.types;if(!Array.isArray(i)||i.length===0||!i.every(e=>typeof e==`string`))return{ok:!1,error:`${n} ('${t.alias}') has no types.`};if(t.properties!==void 0&&!ro(t.properties))return{ok:!1,error:`${n} ('${t.alias}') has properties that are not an object.`};i.includes(`Root`)&&ro(t.properties)&&typeof t.properties.name==`string`&&(r=t.properties.name)}if(e.connections!==void 0&&!Array.isArray(e.connections))return{ok:!1,error:`The 'connections' section must be a list.`};let i=Array.isArray(e.connections)?e.connections:[];for(let[e,t]of i.entries())if(!ro(t)||typeof t.source!=`string`||typeof t.target!=`string`)return{ok:!1,error:`Connection entry ${e+1} needs a 'source' and a 'target'.`};return{ok:!0,model:e,name:r,nodeCount:n.length,connectionCount:i.length}}function ao(e,t){let n=JSON.parse(JSON.stringify(e)),r=n.nodes.find(e=>Array.isArray(e.types)&&e.types.includes(`Root`));return r&&(r.properties={...r.properties??{},name:t}),`${JSON.stringify(n,null,2)}\n`}function oo(){return typeof window<`u`&&typeof window.showSaveFilePicker==`function`}var so={description:`MiniGraph model (JSON)`,mimeType:`application/json`,extension:`.json`},co={description:`MiniGraph graph set`,mimeType:`application/octet-stream`,extension:`.pack`},lo={description:`OpenAPI document (YAML)`,mimeType:`application/yaml`,extension:`.yaml`},uo={description:`PNG image`,mimeType:`image/png`,extension:`.png`};function H(e,t){return go(e,t,so)}function fo(e,t){return go(e,t,lo)}function po(e,t){return go(e,t,co)}function mo(e,t){return go(e,t,uo)}async function ho(e){if(!(typeof e.getFile!=`function`||typeof e.remove!=`function`))try{(await e.getFile()).size===0&&await e.remove()}catch{}}async function go(e,t,n){let r=typeof e==`function`?e:()=>Promise.resolve(e),i=window.showSaveFilePicker;if(typeof i==`function`){let e;try{e=await i.call(window,{suggestedName:t,excludeAcceptAllOption:!0,types:[{description:n.description,accept:{[n.mimeType]:[n.extension]}}]})}catch(e){if(e.name===`AbortError`)return{saved:!1,cancelled:!0};throw e}try{let t=await r(),n=await e.createWritable();try{await n.write(t),await n.close()}catch(e){throw await n.abort().catch(()=>void 0),e}}catch(t){throw await ho(e),t}return{saved:!0,fileName:e.name}}let a=new Blob([await r()],{type:n.mimeType}),o=URL.createObjectURL(a),s=document.createElement(`a`);return s.href=o,s.download=t,s.rel=`noopener`,document.body.appendChild(s),s.click(),s.remove(),setTimeout(()=>URL.revokeObjectURL(o),0),{saved:!0,fileName:t}}function _o(e,t){if(e.match(/^[a-z]+:\/\//i))return e;if(e.match(/^\/\//))return window.location.protocol+e;if(e.match(/^[a-z]+:/i))return e;let n=document.implementation.createHTMLDocument(),r=n.createElement(`base`),i=n.createElement(`a`);return n.head.appendChild(r),n.body.appendChild(i),t&&(r.href=t),i.href=e,i.href}var vo=(()=>{let e=0,t=()=>`0000${(Math.random()*36**4<<0).toString(36)}`.slice(-4);return()=>(e+=1,`u${t()}${e}`)})();function yo(e){let t=[];for(let n=0,r=e.length;n<r;n++)t.push(e[n]);return t}function bo(e,t){let n=(e.ownerDocument.defaultView||window).getComputedStyle(e).getPropertyValue(t);return n?parseFloat(n.replace(`px`,``)):0}function xo(e){let t=bo(e,`border-left-width`),n=bo(e,`border-right-width`);return e.clientWidth+t+n}function So(e){let t=bo(e,`border-top-width`),n=bo(e,`border-bottom-width`);return e.clientHeight+t+n}function Co(e,t={}){return{width:t.width||xo(e),height:t.height||So(e)}}function wo(){let e,t;try{t=process}catch{}let n=t&&t.env?t.env.devicePixelRatio:null;return n&&(e=parseInt(n,10),Number.isNaN(e)&&(e=1)),e||window.devicePixelRatio||1}var To=16384;function Eo(e){(e.width>To||e.height>To)&&(e.width>To&&e.height>To?e.width>e.height?(e.height*=To/e.width,e.width=To):(e.width*=To/e.height,e.height=To):e.width>To?(e.height*=To/e.width,e.width=To):(e.width*=To/e.height,e.height=To))}function Do(e,t={}){return e.toBlob?new Promise(n=>{e.toBlob(n,t.type?t.type:`image/png`,t.quality?t.quality:1)}):new Promise(n=>{let r=window.atob(e.toDataURL(t.type?t.type:void 0,t.quality?t.quality:void 0).split(`,`)[1]),i=r.length,a=new Uint8Array(i);for(let e=0;e<i;e+=1)a[e]=r.charCodeAt(e);n(new Blob([a],{type:t.type?t.type:`image/png`}))})}function Oo(e){return new Promise((t,n)=>{let r=new Image;r.decode=()=>t(r),r.onload=()=>t(r),r.onerror=n,r.crossOrigin=`anonymous`,r.decoding=`async`,r.src=e})}async function ko(e){return Promise.resolve().then(()=>new XMLSerializer().serializeToString(e)).then(encodeURIComponent).then(e=>`data:image/svg+xml;charset=utf-8,${e}`)}async function Ao(e,t,n){let r=`http://www.w3.org/2000/svg`,i=document.createElementNS(r,`svg`),a=document.createElementNS(r,`foreignObject`);return i.setAttribute(`width`,`${t}`),i.setAttribute(`height`,`${n}`),i.setAttribute(`viewBox`,`0 0 ${t} ${n}`),a.setAttribute(`width`,`100%`),a.setAttribute(`height`,`100%`),a.setAttribute(`x`,`0`),a.setAttribute(`y`,`0`),a.setAttribute(`externalResourcesRequired`,`true`),i.appendChild(a),a.appendChild(e),ko(i)}var U=(e,t)=>{if(e instanceof t)return!0;let n=Object.getPrototypeOf(e);return n===null?!1:n.constructor.name===t.name||U(n,t)};function jo(e){let t=e.getPropertyValue(`content`);return`${e.cssText} content: '${t.replace(/'|"/g,``)}';`}function Mo(e){return yo(e).map(t=>`${t}: ${e.getPropertyValue(t)}${e.getPropertyPriority(t)?` !important`:``};`).join(` `)}function No(e,t,n){let r=`.${e}:${t}`,i=n.cssText?jo(n):Mo(n);return document.createTextNode(`${r}{${i}}`)}function Po(e,t,n){let r=window.getComputedStyle(e,n),i=r.getPropertyValue(`content`);if(i===``||i===`none`)return;let a=vo();try{t.className=`${t.className} ${a}`}catch{return}let o=document.createElement(`style`);o.appendChild(No(a,n,r)),t.appendChild(o)}function Fo(e,t){Po(e,t,`:before`),Po(e,t,`:after`)}var Io=`application/font-woff`,Lo=`image/jpeg`,Ro={woff:Io,woff2:Io,ttf:`application/font-truetype`,eot:`application/vnd.ms-fontobject`,png:`image/png`,jpg:Lo,jpeg:Lo,gif:`image/gif`,tiff:`image/tiff`,svg:`image/svg+xml`,webp:`image/webp`};function zo(e){let t=/\.([^./]*?)$/g.exec(e);return t?t[1]:``}function Bo(e){return Ro[zo(e).toLowerCase()]||``}function Vo(e){return e.split(/,/)[1]}function Ho(e){return e.search(/^(data:)/)!==-1}function Uo(e,t){return`data:${t};base64,${e}`}async function Wo(e,t,n){let r=await fetch(e,t);if(r.status===404)throw Error(`Resource "${r.url}" not found`);let i=await r.blob();return new Promise((e,t)=>{let a=new FileReader;a.onerror=t,a.onloadend=()=>{try{e(n({res:r,result:a.result}))}catch(e){t(e)}},a.readAsDataURL(i)})}var Go={};function Ko(e,t,n){let r=e.replace(/\?.*/,``);return n&&(r=e),/ttf|otf|eot|woff2?/i.test(r)&&(r=r.replace(/.*\//,``)),t?`[${t}]${r}`:r}async function qo(e,t,n){let r=Ko(e,t,n.includeQueryParams);if(Go[r]!=null)return Go[r];n.cacheBust&&(e+=(/\?/.test(e)?`&`:`?`)+new Date().getTime());let i;try{i=Uo(await Wo(e,n.fetchRequestInit,({res:e,result:n})=>(t||=e.headers.get(`Content-Type`)||``,Vo(n))),t)}catch(t){i=n.imagePlaceholder||``;let r=`Failed to fetch resource: ${e}`;t&&(r=typeof t==`string`?t:t.message),r&&console.warn(r)}return Go[r]=i,i}async function Jo(e){let t=e.toDataURL();return t===`data:,`?e.cloneNode(!1):Oo(t)}async function Yo(e,t){if(e.currentSrc){let t=document.createElement(`canvas`),n=t.getContext(`2d`);return t.width=e.clientWidth,t.height=e.clientHeight,n?.drawImage(e,0,0,t.width,t.height),Oo(t.toDataURL())}let n=e.poster;return Oo(await qo(n,Bo(n),t))}async function Xo(e){try{if(e?.contentDocument?.body)return await as(e.contentDocument.body,{},!0)}catch{}return e.cloneNode(!1)}async function Zo(e,t){return U(e,HTMLCanvasElement)?Jo(e):U(e,HTMLVideoElement)?Yo(e,t):U(e,HTMLIFrameElement)?Xo(e):e.cloneNode(!1)}var Qo=e=>e.tagName!=null&&e.tagName.toUpperCase()===`SLOT`;async function $o(e,t,n){let r=[];return r=Qo(e)&&e.assignedNodes?yo(e.assignedNodes()):U(e,HTMLIFrameElement)&&e.contentDocument?.body?yo(e.contentDocument.body.childNodes):yo((e.shadowRoot??e).childNodes),r.length===0||U(e,HTMLVideoElement)||await r.reduce((e,r)=>e.then(()=>as(r,n)).then(e=>{e&&t.appendChild(e)}),Promise.resolve()),t}function es(e,t){let n=t.style;if(!n)return;let r=window.getComputedStyle(e);r.cssText?(n.cssText=r.cssText,n.transformOrigin=r.transformOrigin):yo(r).forEach(i=>{let a=r.getPropertyValue(i);i===`font-size`&&a.endsWith(`px`)&&(a=`${Math.floor(parseFloat(a.substring(0,a.length-2)))-.1}px`),U(e,HTMLIFrameElement)&&i===`display`&&a===`inline`&&(a=`block`),i===`d`&&t.getAttribute(`d`)&&(a=`path(${t.getAttribute(`d`)})`),n.setProperty(i,a,r.getPropertyPriority(i))})}function ts(e,t){U(e,HTMLTextAreaElement)&&(t.innerHTML=e.value),U(e,HTMLInputElement)&&t.setAttribute(`value`,e.value)}function ns(e,t){if(U(e,HTMLSelectElement)){let n=t,r=Array.from(n.children).find(t=>e.value===t.getAttribute(`value`));r&&r.setAttribute(`selected`,``)}}function rs(e,t){return U(t,Element)&&(es(e,t),Fo(e,t),ts(e,t),ns(e,t)),t}async function is(e,t){let n=e.querySelectorAll?e.querySelectorAll(`use`):[];if(n.length===0)return e;let r={};for(let i=0;i<n.length;i++){let a=n[i].getAttribute(`xlink:href`);if(a){let n=e.querySelector(a),i=document.querySelector(a);!n&&i&&!r[a]&&(r[a]=await as(i,t,!0))}}let i=Object.values(r);if(i.length){let t=`http://www.w3.org/1999/xhtml`,n=document.createElementNS(t,`svg`);n.setAttribute(`xmlns`,t),n.style.position=`absolute`,n.style.width=`0`,n.style.height=`0`,n.style.overflow=`hidden`,n.style.display=`none`;let r=document.createElementNS(t,`defs`);n.appendChild(r);for(let e=0;e<i.length;e++)r.appendChild(i[e]);e.appendChild(n)}return e}async function as(e,t,n){return!n&&t.filter&&!t.filter(e)?null:Promise.resolve(e).then(e=>Zo(e,t)).then(n=>$o(e,n,t)).then(t=>rs(e,t)).then(e=>is(e,t))}var os=/url\((['"]?)([^'"]+?)\1\)/g,ss=/url\([^)]+\)\s*format\((["']?)([^"']+)\1\)/g,cs=/src:\s*(?:url\([^)]+\)\s*format\([^)]+\)[,;]\s*)+/g;function ls(e){let t=e.replace(/([.*+?^${}()|\[\]\/\\])/g,`\\$1`);return RegExp(`(url\\(['"]?)(${t})(['"]?\\))`,`g`)}function us(e){let t=[];return e.replace(os,(e,n,r)=>(t.push(r),e)),t.filter(e=>!Ho(e))}async function ds(e,t,n,r,i){try{let a=n?_o(t,n):t,o=Bo(t),s;return s=i?Uo(await i(a),o):await qo(a,o,r),e.replace(ls(t),`$1${s}$3`)}catch{}return e}function fs(e,{preferredFontFormat:t}){return t?e.replace(cs,e=>{for(;;){let[n,,r]=ss.exec(e)||[];if(!r)return``;if(r===t)return`src: ${n};`}}):e}function ps(e){return e.search(os)!==-1}async function ms(e,t,n){if(!ps(e))return e;let r=fs(e,n);return us(r).reduce((e,r)=>e.then(e=>ds(e,r,t,n)),Promise.resolve(r))}async function hs(e,t,n){let r=t.style?.getPropertyValue(e);if(r){let i=await ms(r,null,n);return t.style.setProperty(e,i,t.style.getPropertyPriority(e)),!0}return!1}async function gs(e,t){await hs(`background`,e,t)||await hs(`background-image`,e,t),await hs(`mask`,e,t)||await hs(`mask-image`,e,t)}async function _s(e,t){let n=U(e,HTMLImageElement);if(!(n&&!Ho(e.src))&&!(U(e,SVGImageElement)&&!Ho(e.href.baseVal)))return;let r=n?e.src:e.href.baseVal,i=await qo(r,Bo(r),t);await new Promise((t,r)=>{e.onload=t,e.onerror=r;let a=e;a.decode&&=t,a.loading===`lazy`&&(a.loading=`eager`),n?(e.srcset=``,e.src=i):e.href.baseVal=i})}async function vs(e,t){let n=yo(e.childNodes).map(e=>ys(e,t));await Promise.all(n).then(()=>e)}async function ys(e,t){U(e,Element)&&(await gs(e,t),await _s(e,t),await vs(e,t))}function bs(e,t){let{style:n}=e;t.backgroundColor&&(n.backgroundColor=t.backgroundColor),t.width&&(n.width=`${t.width}px`),t.height&&(n.height=`${t.height}px`);let r=t.style;return r!=null&&Object.keys(r).forEach(e=>{n[e]=r[e]}),e}var xs={};async function Ss(e){let t=xs[e];return t??(t={url:e,cssText:await(await fetch(e)).text()},xs[e]=t,t)}async function Cs(e,t){let n=e.cssText,r=/url\(["']?([^"')]+)["']?\)/g,i=(n.match(/url\([^)]+\)/g)||[]).map(async i=>{let a=i.replace(r,`$1`);return a.startsWith(`https://`)||(a=new URL(a,e.url).href),Wo(a,t.fetchRequestInit,({result:e})=>(n=n.replace(i,`url(${e})`),[i,e]))});return Promise.all(i).then(()=>n)}function ws(e){if(e==null)return[];let t=[],n=e.replace(/(\/\*[\s\S]*?\*\/)/gi,``),r=RegExp(`((@.*?keyframes [\\s\\S]*?){([\\s\\S]*?}\\s*?)})`,`gi`);for(;;){let e=r.exec(n);if(e===null)break;t.push(e[0])}n=n.replace(r,``);let i=/@import[\s\S]*?url\([^)]*\)[\s\S]*?;/gi,a=RegExp(`((\\s*?(?:\\/\\*[\\s\\S]*?\\*\\/)?\\s*?@media[\\s\\S]*?){([\\s\\S]*?)}\\s*?})|(([\\s\\S]*?){([\\s\\S]*?)})`,`gi`);for(;;){let e=i.exec(n);if(e===null){if(e=a.exec(n),e===null)break;i.lastIndex=a.lastIndex}else a.lastIndex=i.lastIndex;t.push(e[0])}return t}async function Ts(e,t){let n=[],r=[];return e.forEach(n=>{if(`cssRules`in n)try{yo(n.cssRules||[]).forEach((e,i)=>{if(e.type===CSSRule.IMPORT_RULE){let a=i+1,o=e.href,s=Ss(o).then(e=>Cs(e,t)).then(e=>ws(e).forEach(e=>{try{n.insertRule(e,e.startsWith(`@import`)?a+=1:n.cssRules.length)}catch(t){console.error(`Error inserting rule from remote css`,{rule:e,error:t})}})).catch(e=>{console.error(`Error loading remote css`,e.toString())});r.push(s)}})}catch(i){let a=e.find(e=>e.href==null)||document.styleSheets[0];n.href!=null&&r.push(Ss(n.href).then(e=>Cs(e,t)).then(e=>ws(e).forEach(e=>{a.insertRule(e,n.cssRules.length)})).catch(e=>{console.error(`Error loading remote stylesheet`,e)})),console.error(`Error inlining remote css file`,i)}}),Promise.all(r).then(()=>(e.forEach(e=>{if(`cssRules`in e)try{yo(e.cssRules||[]).forEach(e=>{n.push(e)})}catch(t){console.error(`Error while reading CSS rules from ${e.href}`,t)}}),n))}function Es(e){return e.filter(e=>e.type===CSSRule.FONT_FACE_RULE).filter(e=>ps(e.style.getPropertyValue(`src`)))}async function Ds(e,t){if(e.ownerDocument==null)throw Error(`Provided element is not within a Document`);return Es(await Ts(yo(e.ownerDocument.styleSheets),t))}async function Os(e,t){let n=await Ds(e,t);return(await Promise.all(n.map(e=>{let n=e.parentStyleSheet?e.parentStyleSheet.href:null;return ms(e.cssText,n,t)}))).join(`
`)}async function ks(e,t){let n=t.fontEmbedCSS==null?t.skipFonts?null:await Os(e,t):t.fontEmbedCSS;if(n){let t=document.createElement(`style`),r=document.createTextNode(n);t.appendChild(r),e.firstChild?e.insertBefore(t,e.firstChild):e.appendChild(t)}}async function As(e,t={}){let{width:n,height:r}=Co(e,t),i=await as(e,t,!0);return await ks(i,t),await ys(i,t),bs(i,t),await Ao(i,n,r)}async function js(e,t={}){let{width:n,height:r}=Co(e,t),i=await Oo(await As(e,t)),a=document.createElement(`canvas`),o=a.getContext(`2d`),s=t.pixelRatio||wo(),c=t.canvasWidth||n,l=t.canvasHeight||r;return a.width=c*s,a.height=l*s,t.skipAutoScale||Eo(a),a.style.width=`${c}`,a.style.height=`${l}`,t.backgroundColor&&(o.fillStyle=t.backgroundColor,o.fillRect(0,0,a.width,a.height)),o.drawImage(i,0,0,a.width,a.height),a}async function Ms(e,t={}){return await Do(await js(e,t))}var Ns=`#ffffff`,Ps=[`react-flow__handle`,`react-flow__resize-control`,`react-flow__nodesselection`];function Fs(e){return`${eo(e)}.png`}function Is(e){if(e.length===0)return null;let t=1/0,n=1/0,r=-1/0,i=-1/0;for(let a of e)t=Math.min(t,a.x),n=Math.min(n,a.y),r=Math.max(r,a.x+a.width),i=Math.max(i,a.y+a.height);return{x:t,y:n,width:r-t,height:i-n}}function Ls(e,t=40){return{width:Math.ceil(e.width+2*t),height:Math.ceil(e.height+2*t),transform:`translate(${t-e.x}px, ${t-e.y}px) scale(1)`}}function Rs(e){let t=e.classList;return!t||!Ps.some(e=>t.contains(e))}function zs(e){let t=[];for(let n of Array.from(e.querySelectorAll(`.react-flow__edge`))){let e=n;if(typeof e.getBBox!=`function`)continue;let r=e.getBBox();(r.width>0||r.height>0)&&t.push({x:r.x,y:r.y,width:r.width,height:r.height})}return t}function Bs(e){return e===``||e===`transparent`||/^rgba\(.*,\s*0\)$/.test(e)}function Vs(e){for(let t=e;t;t=t.parentElement){let e=getComputedStyle(t).backgroundColor;if(!Bs(e))return e}return Ns}async function Hs(e,t,n){let r=Ls(t),i=await Ms(e,{backgroundColor:n,width:r.width,height:r.height,pixelRatio:2,style:{width:`${r.width}px`,height:`${r.height}px`,transform:r.transform},filter:Rs});if(!i)throw Error(`The browser could not draw the image.`);return i}var Us=`application/x-minigraph-clipboard-item`;function Ws(e){return e.includes(Us)}function Gs(e,t){e.effectAllowed=`copy`,e.setData(Us,t)}function Ks(e){let t=e?.getData(`application/x-minigraph-clipboard-item`)??``;return t.trim()?t:null}function qs(e,t){return e.nodes.find(e=>e.alias===t)}function Js(e,t){return(e.connections??[]).filter(e=>e.source!==e.target&&(e.source===t||e.target===t))}var Ys={viewBox:`0 0 16 16`,width:16,height:16,fill:`none`,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,focusable:`false`};function Xs(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`path`,{d:`M8 1.75 13.25 4.75v6.5L8 14.25l-5.25-3v-6.5L8 1.75Z`}),(0,R.jsx)(`path`,{d:`m2.95 4.9 5.05 2.9 5.05-2.9M8 7.8v6.15`})]})}function Zs(e){return(0,R.jsx)(`svg`,{...Ys,...e,children:(0,R.jsx)(`path`,{d:`m5.25 3 6.5 5-6.5 5V3Z`})})}function Qs(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`rect`,{x:`5`,y:`2`,width:`8.5`,height:`9`,rx:`1.25`}),(0,R.jsx)(`path`,{d:`M10.75 14h-7A1.25 1.25 0 0 1 2.5 12.75V6A1.25 1.25 0 0 1 3.75 4.75H5`})]})}function $s(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`path`,{d:`M8 10.5V2.75M4.75 6 8 2.75 11.25 6`}),(0,R.jsx)(`path`,{d:`M2.75 10.75v1.5a1 1 0 0 0 1 1h8.5a1 1 0 0 0 1-1v-1.5`})]})}function ec(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`path`,{d:`M8 2.75v7.75M4.75 7.25 8 10.5l3.25-3.25`}),(0,R.jsx)(`path`,{d:`M2.75 10.75v1.5a1 1 0 0 0 1 1h8.5a1 1 0 0 0 1-1v-1.5`})]})}function tc(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`path`,{d:`M2.75 8h7M7 5.25 9.75 8 7 10.75`}),(0,R.jsx)(`path`,{d:`M8.75 2.75h3.5a1 1 0 0 1 1 1v8.5a1 1 0 0 1-1 1h-3.5`})]})}function nc(e){return(0,R.jsxs)(`svg`,{...Ys,...e,children:[(0,R.jsx)(`rect`,{x:`2.25`,y:`2.75`,width:`11.5`,height:`10.5`,rx:`1.25`}),(0,R.jsx)(`circle`,{cx:`5.75`,cy:`6.25`,r:`1.25`}),(0,R.jsx)(`path`,{d:`m2.75 12 3.75-3.75 2.5 2.5 1.75-1.75 2.5 2.5`})]})}var W={toolbar:`_toolbar_stok4_2`,nameGroup:`_nameGroup_stok4_17`,graphName:`_graphName_stok4_24`,stats:`_stats_stok4_33`,toolbarActions:`_toolbarActions_stok4_53`,runControls:`_runControls_stok4_59`,tooltipAnchor:`_tooltipAnchor_stok4_65`,actionTooltip:`_actionTooltip_stok4_76`,toolbarButton:`_toolbarButton_stok4_116`,toolbarIconButton:`_toolbarIconButton_stok4_134`,toolbarIcon:`_toolbarIcon_stok4_134`};function rc({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i,onImport:a,importDisabledReason:o=null,onDownload:s,onExportImage:c,exportingImage:l=!1}){let u=(0,F.useCallback)(()=>{e&&navigator.clipboard.writeText(JSON.stringify(e,null,2)).then(()=>n?.()).catch(()=>r?.())},[e,n,r]),d=eo(t),f=e?.nodes.length??0,p=(e?.connections??[]).length;return(0,R.jsxs)(`div`,{className:W.toolbar,children:[(0,R.jsxs)(`div`,{className:W.nameGroup,children:[(0,R.jsx)(`span`,{className:W.graphName,children:t??`Untitled`}),(0,R.jsxs)(`span`,{className:W.stats,children:[f,` node`,f===1?``:`s`,` · `,p,` connection`,p===1?``:`s`]})]}),(0,R.jsxs)(`div`,{className:W.toolbarActions,children:[i,a&&(0,R.jsx)(`button`,{type:`button`,className:`${W.toolbarButton} ${W.toolbarIconButton}`,onClick:a,disabled:o!==null,title:o??`Import a graph model from a JSON file (replaces the current draft)`,"aria-label":`Import graph model from a JSON file`,children:(0,R.jsx)(tc,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`})}),s&&(0,R.jsx)(`button`,{type:`button`,className:`${W.toolbarButton} ${W.toolbarIconButton}`,onClick:s,disabled:!e,title:e?`Download the graph as ${to(d)}`:`No graph to download`,"aria-label":`Download graph as a JSON file`,children:(0,R.jsx)(ec,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`})}),c&&(0,R.jsx)(`button`,{type:`button`,className:`${W.toolbarButton} ${W.toolbarIconButton}`,onClick:c,disabled:!e||l,title:l?`Exporting the graph image…`:e?`Export the whole graph as ${d}.png`:`No graph to export`,"aria-label":`Export graph as a PNG image`,children:(0,R.jsx)(nc,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`})}),(0,R.jsx)(`button`,{type:`button`,className:`${W.toolbarButton} ${W.toolbarIconButton}`,onClick:u,title:`Copy raw graph JSON to clipboard`,"aria-label":`Copy raw graph JSON to clipboard`,children:(0,R.jsx)(Qs,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`})})]})]})}var ic=new Set([`instantiating`,`outcome-uncertain`]);function ac(e){switch(e){case`instantiating`:return`Instantiating…`;case`outcome-uncertain`:return`Waiting…`;default:return`Instantiate`}}function oc(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return`Instantiate graph`}}function sc(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`running`:return`Graph is running`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return``}}function cc(e,t){return t?`${e} ${/[.!?]$/.test(t)?t:`${t}.`}`:e}function lc(e,t){let n=[];return e.length>0&&n.push(`${e.length} input.body ${e.length===1?`path`:`paths`}`),t.length>0&&n.push(`${t.length} input ${t.length===1?`header`:`headers`}`),n.length===0?`This graph reads neither input.body nor input.header, so uploading is optional`:`This graph reads ${n.join(` and `)}`}function uc({id:e,text:t,keyboardFallback:n,children:r}){let[i,a]=(0,F.useState)(!1),[o,s]=(0,F.useState)(!1);return(0,R.jsxs)(`span`,{className:W.tooltipAnchor,onMouseEnter:()=>a(!0),onMouseLeave:()=>a(!1),onFocusCapture:()=>s(!0),onBlurCapture:e=>{e.currentTarget.contains(e.relatedTarget)||s(!1)},tabIndex:n?0:void 0,"aria-describedby":n?e:void 0,children:[r,(0,R.jsx)(`span`,{id:e,className:W.actionTooltip,role:`tooltip`,"data-state":i||o?`open`:`closed`,children:t})]})}function dc({phase:e,canInstantiate:t,canUpload:n,canRun:r,disabledReason:i,inputBodyPaths:a=[],inputHeaderNames:o=[],onInstantiate:s,onUpload:c,onRun:l}){let u=(0,F.useId)(),d=`${u}-instantiate`,f=`${u}-upload`,p=`${u}-run`,m=ic.has(e),h=sc(e),g=i||h||(e===`ready`?`Instantiating again starts from a fresh instance`:``),_=i||h||(n?lc(a,o):`Instantiate the graph first`),v=i||h||(r?``:`Instantiate the graph first`),y=cc(`Create a runnable instance of the current graph.`,g),b=cc(`Upload a JSON payload as the mock input.body of the instance, optional mock headers as its input.header and optional mock model variables. Only you see the form.`,_),x=cc(`Run the instantiated graph, with the uploaded mock input if any.`,v);return(0,R.jsxs)(`div`,{className:W.runControls,role:`group`,"aria-label":`Graph run controls`,children:[(0,R.jsx)(uc,{id:d,text:y,keyboardFallback:!t,children:(0,R.jsxs)(`button`,{type:`button`,className:W.toolbarButton,onClick:s,disabled:!t,"aria-label":oc(e),"aria-describedby":d,"aria-busy":m,children:[(0,R.jsx)(Xs,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,R.jsx)(`span`,{children:ac(e)})]})}),(0,R.jsx)(uc,{id:f,text:b,keyboardFallback:!n,children:(0,R.jsxs)(`button`,{type:`button`,className:W.toolbarButton,onClick:c,disabled:!n,"aria-label":`Upload mock input`,"aria-describedby":f,children:[(0,R.jsx)($s,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,R.jsx)(`span`,{children:`Upload`})]})}),(0,R.jsx)(uc,{id:p,text:x,keyboardFallback:!r,children:(0,R.jsxs)(`button`,{type:`button`,className:W.toolbarButton,onClick:l,disabled:!r,"aria-label":e===`ready`?`Run instantiated graph`:e===`running`?`Graph is running`:`Run graph`,"aria-describedby":p,"aria-busy":e===`running`,children:[(0,R.jsx)(Zs,{className:W.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,R.jsx)(`span`,{children:e===`running`?`Running…`:`Run`})]})})]})}var fc={menu:`_menu_13qxg_1`,menuItem:`_menuItem_13qxg_12`};function pc({open:e,x:t,y:n,canCreateNode:r,onCreateNode:i,onClose:a}){let o=(0,F.useRef)(null),s=(0,F.useRef)(null);return(0,F.useEffect)(()=>{if(!e)return;s.current?.focus();let t=e=>{o.current&&!o.current.contains(e.target)&&a()},n=e=>{e.key===`Escape`&&(e.preventDefault(),a())};return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n)}},[e,a]),e?(0,R.jsx)(`div`,{ref:o,className:fc.menu,style:{left:t,top:n},role:`menu`,"aria-label":`Graph actions`,children:(0,R.jsx)(`button`,{ref:s,role:`menuitem`,type:`button`,className:fc.menuItem,disabled:!r,onClick:()=>{r&&(i(),a())},children:`Create Node`})}):null}var mc={menu:`_menu_1trgd_1`,menuItem:`_menuItem_1trgd_12`,dangerItem:`_dangerItem_1trgd_38`,confirmation:`_confirmation_1trgd_51`,confirmationText:`_confirmationText_1trgd_57`,confirmationActions:`_confirmationActions_1trgd_65`},hc=8;function gc(e){let{open:t,x:n,y:r,onClose:i}=e,[a,o]=(0,F.useState)(!1),[s,c]=(0,F.useState)({left:n,top:r}),l=(0,F.useRef)(null),u=(0,F.useRef)(null),d=(0,F.useRef)(null),f=e.mode===`multi-node`?e.selectedCount:null,p=f!==null&&f>1,m=e.mode===`multi-node`?p&&e.canClipSelectedNodes:e.canClipNode,h=e.mode===`single-node`&&e.canConnectNode,g=e.mode===`single-node`&&e.canEditNode,_=e.mode===`multi-node`?p&&e.canDeleteSelectedNodes:e.canDeleteNode,v=m||h||g||_,y=p?`${f} selected nodes`:e.mode===`single-node`?e.nodeAlias:``;return(0,F.useLayoutEffect)(()=>{t&&o(!1)},[t,y,n,r]),(0,F.useLayoutEffect)(()=>{if(!t)return;let e=l.current;if(!e){c({left:n,top:r});return}let i=e.getBoundingClientRect(),a=Math.max(hc,window.innerWidth-i.width-hc),o=Math.max(hc,window.innerHeight-i.height-hc);c({left:Math.min(Math.max(n,hc),a),top:Math.min(Math.max(r,hc),o)})},[m,_,g,a,t,y,n,r]),(0,F.useEffect)(()=>{if(!t){o(!1);return}a?d.current?.focus():u.current?.focus()},[a,t]),(0,F.useEffect)(()=>{if(!t)return;let e=e=>{l.current&&!l.current.contains(e.target)&&i()},n=e=>{e.key===`Escape`&&(e.preventDefault(),i())},r=()=>i();return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[i,t]),!t||!v?null:(0,R.jsx)(`div`,{ref:l,className:mc.menu,style:{left:s.left,top:s.top},role:`menu`,"aria-label":p?`Actions for ${f} selected nodes`:`Node actions for ${y}`,children:a?(0,R.jsxs)(`div`,{className:mc.confirmation,role:`group`,"aria-label":`Confirm delete ${y}`,children:[(0,R.jsx)(`div`,{className:mc.confirmationText,children:p?`Delete ${f} selected nodes?`:`Delete "${y}"?`}),(0,R.jsxs)(`div`,{className:mc.confirmationActions,children:[(0,R.jsx)(`button`,{ref:d,type:`button`,className:`${mc.menuItem} ${mc.dangerItem}`,onClick:()=>{e.mode===`multi-node`?e.onDeleteSelectedNodes():e.onDeleteNode(),i()},children:`Delete`}),(0,R.jsx)(`button`,{type:`button`,className:mc.menuItem,onClick:()=>o(!1),children:`Cancel`})]})]}):(0,R.jsxs)(R.Fragment,{children:[m&&(0,R.jsx)(`button`,{ref:u,role:`menuitem`,type:`button`,className:mc.menuItem,onClick:()=>{e.mode===`multi-node`?e.onClipSelectedNodes():e.onClipNode(),i()},children:p?`Clip ${f} selected nodes to Workspace`:`Clip to Workspace`}),h&&e.mode===`single-node`&&(0,R.jsx)(`button`,{ref:m?void 0:u,role:`menuitem`,type:`button`,className:mc.menuItem,onClick:()=>{e.onConnectNode(),i()},children:`Connect to…`}),g&&e.mode===`single-node`&&(0,R.jsx)(`button`,{ref:m||h?void 0:u,role:`menuitem`,type:`button`,className:mc.menuItem,onClick:()=>{e.onEditNode(),i()},children:`Edit Node`}),_&&(0,R.jsx)(`button`,{ref:!m&&!h&&!g?u:void 0,role:`menuitem`,type:`button`,className:`${mc.menuItem} ${mc.dangerItem}`,onClick:()=>o(!0),children:p?`Delete ${f} selected nodes`:`Delete Node`})]})})}var _c=8;function vc({open:e,x:t,y:n,sourceAlias:r,targetAlias:i,relations:a,onDeleteRelation:o,onClose:s}){let[c,l]=(0,F.useState)({left:t,top:n}),u=(0,F.useRef)(null),d=(0,F.useRef)(null);return(0,F.useLayoutEffect)(()=>{if(!e)return;let r=u.current;if(!r){l({left:t,top:n});return}let i=r.getBoundingClientRect(),a=Math.max(_c,window.innerWidth-i.width-_c),o=Math.max(_c,window.innerHeight-i.height-_c);l({left:Math.min(Math.max(t,_c),a),top:Math.min(Math.max(n,_c),o)})},[e,a,t,n]),(0,F.useEffect)(()=>{e&&d.current?.focus()},[e]),(0,F.useEffect)(()=>{if(!e)return;let t=e=>{u.current&&!u.current.contains(e.target)&&s()},n=e=>{e.key===`Escape`&&(e.preventDefault(),s())},r=()=>s();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[s,e]),!e||a.length===0?null:(0,R.jsxs)(`div`,{ref:u,className:mc.menu,style:{left:c.left,top:c.top},role:`menu`,"aria-label":`Connection actions for ${r} → ${i}`,children:[a.map((e,t)=>(0,R.jsxs)(`button`,{ref:t===0?d:void 0,role:`menuitem`,type:`button`,className:`${mc.menuItem} ${mc.dangerItem}`,onClick:()=>{o(e),s()},children:[`Delete '`,e,`'`]},`${e}-${t}`)),a.length>1&&(0,R.jsxs)(`button`,{role:`menuitem`,type:`button`,className:`${mc.menuItem} ${mc.dangerItem}`,onClick:()=>{o(void 0),s()},children:[`Delete all (`,a.length,`)`]})]})}var yc={island:`_island_had6p_6`,islandDragging:`_islandDragging_had6p_18`,islandGrip:`_islandGrip_had6p_22`,islandGripDots:`_islandGripDots_had6p_46`,islandGripLabel:`_islandGripLabel_had6p_51`,minimap:`_minimap_had6p_55`,controls:`_controls_had6p_59`,toggleButton:`_toggleButton_had6p_63`,toggleIcon:`_toggleIcon_had6p_68`},bc={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`},xc={left:50,bottom:15},Sc=8,Cc=`graph-minimap-position`;function wc(e){return bc[e.type??``]??`#6c7086`}function Tc(e){return e instanceof Element&&e.closest(`input, textarea, select, [contenteditable]:not([contenteditable="false"])`)!==null}function Ec(e,t,n){if(t.clientWidth<=0||t.clientHeight<=0||n.offsetWidth<=0||n.offsetHeight<=0)return e;let r=Math.max(t.clientWidth-n.offsetWidth-Sc,Sc),i=Math.max(t.clientHeight-n.offsetHeight-Sc,Sc);return{left:Math.min(Math.max(e.left,Sc),r),bottom:Math.min(Math.max(e.bottom,Sc),i)}}function Dc({open:e,onOpenChange:t,hotkeyEnabled:n,children:r}){let i=e?`Hide minimap`:`Show minimap`,a=(0,F.useCallback)(()=>{t(!e)},[t,e]);(0,F.useEffect)(()=>{if(!n)return;let e=e=>{e.defaultPrevented||e.repeat||!e.ctrlKey||e.metaKey||e.altKey||e.shiftKey||e.code!==`KeyM`||Tc(e.target)||(e.preventDefault(),a())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[n,a]);let[o,s]=ce(Cc,xc),c=(0,F.useRef)(null),[l,u]=(0,F.useState)(null),d=(0,F.useCallback)(e=>{e.pointerType===`mouse`&&e.button!==0||(e.preventDefault(),u({pointerId:e.pointerId,startX:e.clientX,startY:e.clientY,origin:o}))},[o]);return(0,F.useEffect)(()=>{if(l===null)return;let e=e=>{if(e.pointerId!==l.pointerId)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null,r={left:l.origin.left+(e.clientX-l.startX),bottom:l.origin.bottom-(e.clientY-l.startY)};s(t&&n?Ec(r,n,t):r)},t=e=>{e.pointerId===l.pointerId&&u(null)};return window.addEventListener(`pointermove`,e),window.addEventListener(`pointerup`,t),window.addEventListener(`pointercancel`,t),()=>{window.removeEventListener(`pointermove`,e),window.removeEventListener(`pointerup`,t),window.removeEventListener(`pointercancel`,t)}},[l,s]),(0,F.useEffect)(()=>{if(!e)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null;if(!t||!n)return;let r=()=>{s(e=>{let r=Ec(e,n,t);return r.left===e.left&&r.bottom===e.bottom?e:r})};if(r(),typeof ResizeObserver>`u`)return;let i=new ResizeObserver(r);return i.observe(n),()=>i.disconnect()},[e,s]),(0,R.jsxs)(R.Fragment,{children:[e&&(0,R.jsxs)(`div`,{ref:c,className:l===null?yc.island:`${yc.island} ${yc.islandDragging}`,style:{left:o.left,bottom:o.bottom},role:`group`,"aria-label":`Graph minimap`,children:[(0,R.jsxs)(`div`,{className:yc.islandGrip,role:`button`,"aria-label":`Move minimap`,title:`Drag to move the minimap`,onPointerDown:d,children:[(0,R.jsx)(`span`,{className:yc.islandGripDots,"aria-hidden":`true`,children:`⠿`}),(0,R.jsx)(`span`,{className:yc.islandGripLabel,children:`Minimap`})]}),(0,R.jsx)(g,{className:yc.minimap,nodeColor:wc,maskColor:`rgba(0,0,0,0.3)`,pannable:!0,style:{position:`relative`,margin:0,background:`#fff`}})]}),(0,R.jsxs)(f,{position:`bottom-left`,showInteractive:!1,className:yc.controls,children:[r,(0,R.jsx)(v,{className:`${yc.toggleButton} nodrag nopan`,"aria-label":i,"aria-keyshortcuts":`Control+M`,"aria-pressed":e,title:`${i} (Ctrl + M)`,onClick:a,children:(0,R.jsxs)(`svg`,{className:yc.toggleIcon,viewBox:`1.5 2 17 16`,fill:`none`,"aria-hidden":`true`,focusable:`false`,children:[(0,R.jsx)(`rect`,{x:`2.5`,y:`3`,width:`15`,height:`14`,rx:`1.5`}),(0,R.jsx)(`path`,{d:`M6 12.5 9 8l2.5 2 2.5-3`}),(0,R.jsx)(`circle`,{cx:`6`,cy:`12.5`,r:`1`}),(0,R.jsx)(`circle`,{cx:`9`,cy:`8`,r:`1`}),(0,R.jsx)(`circle`,{cx:`11.5`,cy:`10`,r:`1`}),(0,R.jsx)(`circle`,{cx:`14`,cy:`7`,r:`1`})]})})]})]})}function Oc(e){return e.trim().toLowerCase()}function kc(e){let t=new Set;return e.filter(e=>{let n=Oc(e);return t.has(n)?!1:(t.add(n),!0)})}function Ac(e,t){let n=kc(t),r=Oc(e);return n.length>1&&n.some(e=>Oc(e)===r)?{kind:`multi-node`,aliases:n}:{kind:`single-node`,alias:e}}function jc(e,t){let n=new Map(t.nodes.map(e=>[Oc(e.alias),e]));return kc(e).map(e=>n.get(Oc(e))).filter(e=>e!==void 0)}var Mc=[],Nc=[],Pc=[`Shift`,`Control`,`Meta`];function Fc(e,t){return e.length===t.length&&e.every((e,n)=>e===t[n])}function Ic({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,graphRunControls:i,onRenderError:a,isRefreshing:o=!1,onClipNode:s,onClipNodes:u,onClipboardDrop:d,onImportFiles:f,onImportGraph:p,onDownload:g,onExportImageSaved:_,onExportImageError:y,isActive:S,isConnected:C,supportsAuthoring:w=!1,onCreateNode:T,onCreateConnection:E,onEditNode:D,onDeleteNode:O,onDeleteNodes:k,onDeleteConnections:A,panelLayoutKey:ee}){let[j,te]=(0,F.useState)(null),[ne,M]=(0,F.useState)(null),[N,P]=(0,F.useState)(null),[re,ie]=(0,F.useState)([]),[ae,I]=(0,F.useState)(!1),[L,oe]=(0,F.useState)(!1),[se,le]=(0,F.useState)(!1),ue=(0,F.useRef)(0),de=(0,F.useRef)(0),fe=!!(w&&T&&C),pe=!!(w&&E&&C),me=!!s,he=!!u,ge=!!(w&&D&&C),_e=!!(w&&O&&C),ve=!!(w&&k&&C),ye=me||ge||_e||pe,be=he||ve,xe=ye||be,Se=!!(d&&C),Ce=!!f,we=(0,F.useCallback)(()=>{ue.current=0,I(!1),de.current=0,oe(!1)},[]);(0,F.useEffect)(()=>{if(!ne)return;let e=e=>{e.key===`Escape`&&M(null)},t=()=>M(null);return document.addEventListener(`keydown`,e),window.addEventListener(`scroll`,t,!0),window.addEventListener(`resize`,t),()=>{document.removeEventListener(`keydown`,e),window.removeEventListener(`scroll`,t,!0),window.removeEventListener(`resize`,t)}},[ne]),(0,F.useEffect)(()=>{let e=()=>we();return window.addEventListener(`dragend`,e),window.addEventListener(`drop`,e),()=>{window.removeEventListener(`dragend`,e),window.removeEventListener(`drop`,e),we()}},[we]);let Te=(0,F.useRef)(a);(0,F.useEffect)(()=>{Te.current=a},[a]);let[Ee,De]=ce(`graph-nodes-compact`,!1),{nodes:Oe,edges:ke,transformError:Ae}=(0,F.useMemo)(()=>{if(!e)return{nodes:Mc,edges:Nc,transformError:null};try{return{...Xa(e,{supportsConnectionAuthoring:pe,compactNodes:Ee}),transformError:null}}catch(e){return{nodes:Mc,edges:Nc,transformError:e instanceof Error?e.message:String(e)}}},[pe,Ee,e]);(0,F.useEffect)(()=>{Ae&&Te.current?.(`Graph render failed: ${Ae}`)},[Ae]);let je=(0,F.useMemo)(()=>e?JSON.stringify(e.nodes.map(e=>e.alias)):`empty`,[e]),[Me,Ne,Pe]=l(Oe),[Fe,Ie,Le]=x(ke),Re=(0,F.useRef)(null),ze=!!(e&&e.nodes.length>0),Be=(0,F.useCallback)(({nodes:e})=>{let t=e.map(e=>e.data.alias);ie(e=>Fc(e,t)?e:t)},[]);(0,F.useEffect)(()=>{Ne(Oe),Ie(ke),ie([]),te(null),P(null)},[Oe,ke,Ne,Ie]);let Ve=(0,F.useRef)(null),He=(0,F.useRef)(null);(0,F.useEffect)(()=>{if(!e||e.nodes.length===0)return;let t=He.current;if(t&&t.graph===e&&t.compact===Ee)return;let n=new Map(e.nodes.map(e=>[e.alias,e.properties]));if(!(Me.length===n.size&&Me.every(e=>n.get(e.id)===e.data.properties&&e.data.compact===Ee)))return;let r=new Map;for(let e of Me){let t=e.measured?.height;if(typeof t!=`number`)return;r.set(e.id,t)}He.current={graph:e,compact:Ee};let i=Ya(e,r,{compactNodes:Ee});Me.some(e=>{let t=i.get(e.id);return t!==void 0&&(t.x!==e.position.x||t.y!==e.position.y)})&&Ne(e=>e.map(e=>{let t=i.get(e.id);return t?{...e,position:t}:e})),requestAnimationFrame(()=>{Ve.current?.fitView({padding:.1})})},[Ee,e,Me,Ne]);let Ue=e=>{Se&&Ws(Array.from(e.dataTransfer.types))&&(e.preventDefault(),ue.current+=1,I(!0))},We=e=>{Se&&Ws(Array.from(e.dataTransfer.types))&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,I(!0))},Ge=e=>{Ws(Array.from(e.dataTransfer.types))&&(ue.current=Math.max(0,ue.current-1),ue.current===0&&I(!1))},Ke=e=>{if(!Se||!Ws(Array.from(e.dataTransfer.types)))return;e.preventDefault();let t=Ks(e.dataTransfer);we(),t&&d?.(t)},qe=e=>Array.from(e.types).includes(`Files`),Je=e=>{if(!qe(e.dataTransfer)){Ue(e);return}Ce&&(e.preventDefault(),de.current+=1,oe(!0))},Ye=e=>{if(!qe(e.dataTransfer)){We(e);return}Ce&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,oe(!0))},Xe=e=>{if(!qe(e.dataTransfer)){Ge(e);return}de.current=Math.max(0,de.current-1),de.current===0&&oe(!1)},Ze=e=>{if(!qe(e.dataTransfer)){Ke(e);return}if(!Ce)return;e.preventDefault();let t=e.dataTransfer.files;we(),t.length>0&&f?.(t)},Qe=(0,F.useMemo)(()=>new Set(e?.nodes.map(e=>e.alias)??[]),[e]),$e=j?.target.kind===`single-node`&&e?qs(e,j.target.alias):null,et=j?.target.kind===`multi-node`?j.target.aliases:[],tt=e?jc(et,e):[],nt=(0,F.useCallback)(e=>!pe||!e.source||!e.target||e.source===e.target||!Qe.has(e.source)||!Qe.has(e.target)?!1:e.sourceHandle===`authoring-source`&&e.targetHandle===`authoring-target`,[pe,Qe]),rt=(0,F.useRef)(null);(0,F.useEffect)(()=>{let e=e=>{rt.current={x:e.clientX,y:e.clientY}};return document.addEventListener(`pointerup`,e,!0),()=>document.removeEventListener(`pointerup`,e,!0)},[]);let it=(0,F.useCallback)(e=>{nt(e)&&(!e.source||!e.target||E?.(e.source,e.target,rt.current??void 0))},[nt,E]),[at,ot]=(0,F.useState)(null);(0,F.useEffect)(()=>{ot(null)},[e,pe]),(0,F.useEffect)(()=>{if(at===null)return;let e=e=>{e.key===`Escape`&&(e.preventDefault(),ot(null))};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[at]);let st=(0,F.useCallback)((e,t)=>{t.handleId!==`authoring-source`||t.handleType!==`source`||(Re.current=t.nodeId,te(null),M(null),P(null))},[]),ct=(0,F.useCallback)(()=>{Re.current=null},[]),lt=(0,F.useRef)(ee);(0,F.useEffect)(()=>{if(lt.current===ee||(lt.current=ee,!ze))return;let e=null,t=requestAnimationFrame(()=>{e=requestAnimationFrame(()=>{Ve.current?.fitView({padding:.1})})});return()=>{cancelAnimationFrame(t),e!==null&&cancelAnimationFrame(e)}},[ze,ee]);let ut=!!(w&&A&&C),dt=(0,F.useCallback)(async({edges:e})=>{if(ut&&e.length>0){let t=new Set,n=[];for(let r of e){let e=`${r.source}\t${r.target}`;t.has(e)||(t.add(e),n.push({source:r.source,target:r.target}))}A?.(n)}return!1},[ut,A]),ft=(0,F.useRef)(null),[pt,mt]=(0,F.useState)(!1),ht=(0,F.useCallback)(()=>{let e=Ve.current,n=ft.current?.querySelector(`.react-flow__viewport`);if(!e||!n){y?.(`The graph is not on screen yet.`);return}mo(async()=>{(0,li.flushSync)(()=>mt(!0));try{let t=Is([e.getNodesBounds(e.getNodes()),...zs(n)]);if(!t)throw Error(`The graph has nothing to draw.`);return await Hs(n,t,Vs(n))}finally{mt(!1)}},Fs(t)).then(e=>{e.saved&&_?.(e.fileName)}).catch(e=>{y?.(e instanceof Error?e.message:String(e))})},[t,y,_]);return Ae?(0,R.jsxs)(`div`,{className:z.empty,children:[(0,R.jsx)(`span`,{className:z.emptyIcon,children:`⚠️`}),(0,R.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,R.jsx)(`span`,{children:Ae})]}):(0,R.jsx)(Ci,{onRenderError:a,children:(0,R.jsxs)(`div`,{className:z.graphWrapper,"aria-busy":o,children:[ze&&e&&(0,R.jsx)(rc,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i?(0,R.jsx)(dc,{...i}):void 0,onImport:p,importDisabledReason:C?null:`Connect first to import a graph`,onDownload:g,onExportImage:ht,exportingImage:pt}),(0,R.jsxs)(`div`,{ref:ft,className:z.graphSurface,"data-connect-picking":at!==null||void 0,"data-exporting":pt||void 0,onDragEnter:Je,onDragOver:Ye,onDragLeave:Xe,onDrop:Ze,children:[ze?(0,R.jsxs)(c,{nodes:Me,edges:Fe,onInit:e=>{Ve.current=e},onNodesChange:Pe,onEdgesChange:Le,nodesConnectable:pe,edgesReconnectable:!1,connectOnClick:!1,isValidConnection:nt,onConnect:it,onConnectStart:st,onConnectEnd:ct,deleteKeyCode:[`Delete`,`Backspace`],onBeforeDelete:dt,nodeTypes:Si,fitView:!0,fitViewOptions:{padding:.1},minZoom:.2,maxZoom:4,zoomOnScroll:!0,zoomOnPinch:!0,zoomOnDoubleClick:!0,panOnScroll:!1,selectionKeyCode:`Shift`,multiSelectionKeyCode:Pc,selectionOnDrag:!1,selectionMode:m.Partial,proOptions:{hideAttribution:!1},onSelectionChange:Be,onNodeContextMenu:(e,t)=>{if(e.preventDefault(),e.stopPropagation(),M(null),P(null),!xe)return;let n=Ac(t.data.alias,re);n.kind===`single-node`&&re.length>1&&(Ne(e=>e.map(e=>({...e,selected:e.data.alias===t.data.alias}))),ie([t.data.alias])),te({x:e.clientX,y:e.clientY,target:n})},onEdgeContextMenu:(e,t)=>{e.preventDefault(),e.stopPropagation(),te(null),M(null),ut&&P({x:e.clientX,y:e.clientY,source:t.source,target:t.target,relations:t.data?.relationTypes??[]})},onPaneContextMenu:e=>{e.preventDefault(),te(null),P(null),fe&&M({x:e.clientX,y:e.clientY})},onPaneClick:()=>{te(null),M(null),P(null),ot(null)},onNodeClick:(e,t)=>{if(at===null)return;e.preventDefault(),e.stopPropagation();let n=t.data.alias;n!==at&&E?.(at,n,{x:e.clientX,y:e.clientY}),ot(null)},children:[(0,R.jsx)(b,{variant:h.Dots,gap:18,size:1,color:`rgba(255,255,255,0.07)`}),(0,R.jsx)(Dc,{open:se,onOpenChange:le,hotkeyEnabled:S,children:(0,R.jsx)(v,{onClick:()=>De(e=>!e),title:Ee?`Show node details`:`Show thumbnail nodes`,"aria-label":Ee?`Show node details`:`Show thumbnail nodes`,"aria-pressed":Ee,children:(0,R.jsxs)(`span`,{className:z.detailToggleIcon,"aria-hidden":`true`,children:[(0,R.jsx)(`span`,{className:z.detailToggleHeader}),Ee&&(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:z.detailToggleLine}),(0,R.jsx)(`span`,{className:z.detailToggleLine}),(0,R.jsx)(`span`,{className:z.detailToggleLine})]})]})})})]}):(0,R.jsxs)(`div`,{className:z.empty,children:[(0,R.jsx)(`span`,{className:z.emptyIcon,children:`🕸️`}),(0,R.jsx)(`span`,{children:`No graph data yet.`}),(0,R.jsxs)(`span`,{children:[`Run `,(0,R.jsx)(`strong`,{children:`create node`}),` or `,(0,R.jsx)(`strong`,{children:`import graph from`}),` in the playground`,f?`, or drop a graph JSON file here`:``,` — the view follows the live session graph.`]}),(w&&T||p)&&(0,R.jsxs)(R.Fragment,{children:[(0,R.jsxs)(`div`,{className:z.emptyActions,children:[w&&T&&(0,R.jsx)(`button`,{type:`button`,className:z.emptyCreateButton,disabled:!C,onClick:()=>T(`empty-graph`),children:`Create Node`}),p&&(0,R.jsx)(`button`,{type:`button`,className:z.emptySecondaryButton,disabled:!C,onClick:p,children:`Import Graph`})]}),!C&&(0,R.jsx)(`span`,{className:z.emptyHint,children:`Connect WebSocket to create or import a graph.`})]})]}),at!==null&&(0,R.jsxs)(`div`,{className:z.connectBanner,role:`status`,children:[(0,R.jsxs)(`span`,{children:[`Connecting from `,(0,R.jsx)(`strong`,{children:at}),` — click a target node`]}),(0,R.jsx)(`button`,{type:`button`,className:z.connectBannerCancel,onClick:()=>ot(null),children:`Cancel (Esc)`})]}),o&&(0,R.jsx)(`div`,{className:z.refreshingOverlay,children:(0,R.jsx)(`div`,{className:z.refreshingSpinner,role:`status`,"aria-label":`Graph refreshing`})}),ae&&(0,R.jsx)(`div`,{className:z.clipboardDropOverlay,children:(0,R.jsx)(`div`,{className:z.clipboardDropMessage,children:`Drop to paste workspace node`})}),L&&(0,R.jsx)(`div`,{className:z.fileDropOverlay,children:(0,R.jsx)(`div`,{className:z.fileDropMessage,children:C?`Drop to import the graph file as the draft`:`Connect first to import a graph file`})}),(0,R.jsx)(pc,{open:ne!==null,x:ne?.x??0,y:ne?.y??0,canCreateNode:fe,onCreateNode:()=>T?.(`pane-context-menu`),onClose:()=>M(null)}),j?.target.kind===`multi-node`?(0,R.jsx)(gc,{mode:`multi-node`,open:tt.length>1&&be,x:j.x,y:j.y,selectedCount:et.length,canClipSelectedNodes:he,canDeleteSelectedNodes:ve,onClipSelectedNodes:()=>{if(!e){u?.([]);return}let t=tt.map(t=>({node:t,connections:Js(e,t.alias)}));u?.(t)},onDeleteSelectedNodes:()=>{let e=tt.length===et.length;k?.(e?tt:[])},onClose:()=>te(null)}):(0,R.jsx)(gc,{mode:`single-node`,open:j!==null&&$e!==null&&ye,x:j?.x??0,y:j?.y??0,nodeAlias:j?.target.kind===`single-node`?j.target.alias:``,canClipNode:me&&$e!==null,canConnectNode:pe&&$e!==null,canEditNode:ge&&$e!==null,canDeleteNode:_e&&$e!==null,onConnectNode:()=>{$e&&ot($e.alias)},onClipNode:()=>{if(!$e||!e)return;let t=Js(e,$e.alias);s?.($e,t)},onEditNode:()=>{$e&&D?.($e)},onDeleteNode:()=>{$e&&O?.($e)},onClose:()=>te(null)}),(0,R.jsx)(vc,{open:N!==null&&ut,x:N?.x??0,y:N?.y??0,sourceAlias:N?.source??``,targetAlias:N?.target??``,relations:N?.relations??[],onDeleteRelation:e=>{N&&A?.([{source:N.source,target:N.target,relation:e}])},onClose:()=>P(null)})]})]})},je)}var G={root:`_root_1yhjs_2`,empty:`_empty_1yhjs_10`,emptyIcon:`_emptyIcon_1yhjs_23`,toolbarButton:`_toolbarButton_1yhjs_29 _toolbarButton_stok4_116`,scrollBody:`_scrollBody_1yhjs_34`,jsonContainer:`_jsonContainer_1yhjs_45`,jsonLabel:`_jsonLabel_1yhjs_46`,jsonString:`_jsonString_1yhjs_47`,jsonNumber:`_jsonNumber_1yhjs_48`,jsonBoolean:`_jsonBoolean_1yhjs_49`,jsonNull:`_jsonNull_1yhjs_50`},Lc={default:e=>e<3,all:i,none:a};function Rc({graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,onDownload:a}){let[s,c]=(0,F.useState)(`all`);return e?(0,R.jsxs)(`div`,{className:G.root,children:[(0,R.jsx)(rc,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,onDownload:a,extraActions:(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`button`,{className:G.toolbarButton,onClick:()=>c(`all`),title:`Expand all nodes`,"aria-label":`Expand all JSON nodes`,"aria-pressed":s===`all`,children:`➖`}),(0,R.jsx)(`button`,{className:G.toolbarButton,onClick:()=>c(`none`),title:`Collapse all nodes`,"aria-label":`Collapse all JSON nodes`,"aria-pressed":s===`none`,children:`➕`})]})}),(0,R.jsx)(`div`,{className:G.scrollBody,children:(0,R.jsx)(o,{data:e,shouldExpandNode:Lc[s],style:{...r,container:`${r.container} ${G.jsonContainer}`,label:G.jsonLabel,stringValue:G.jsonString,numberValue:G.jsonNumber,booleanValue:G.jsonBoolean,nullValue:G.jsonNull}})})]}):(0,R.jsx)(`div`,{className:G.root,children:(0,R.jsxs)(`div`,{className:G.empty,children:[(0,R.jsx)(`span`,{className:G.emptyIcon,children:`🕸️`}),(0,R.jsx)(`span`,{children:`No graph data yet.`}),(0,R.jsx)(`span`,{children:`The raw JSON follows the live session graph.`})]})})}var zc={rightPanel:`_rightPanel_xa3j1_2`,tabStrip:`_tabStrip_xa3j1_10`,tab:`_tab_xa3j1_10`,tabActive:`_tabActive_xa3j1_41`,tabBadge:`_tabBadge_xa3j1_45`,tabBody:`_tabBody_xa3j1_51`,tabBodyHidden:`_tabBodyHidden_xa3j1_64`,graphContent:`_graphContent_xa3j1_68`,rightPanelGroup:`_rightPanelGroup_xa3j1_75`,verticalResizeHandle:`_verticalResizeHandle_xa3j1_83`},Bc=`help-split-percent`,Vc=`help-split-maximized`,Hc=45,Uc=98;function Wc({tabs:e,payload:t,onChange:n,validation:r,onFormat:i,onUpload:a,graphData:o,graphName:s,activeTab:c,onTabChange:l,onGraphRenderError:u,onGraphDataCopySuccess:d,onGraphDataCopyError:f,graphRunControls:p,isGraphRefreshing:m,onClipNode:h,onClipNodes:g,onClipboardDrop:_,onImportFiles:v,onImportGraph:y,onDownloadGraph:b,onGraphImageSaved:x,onGraphImageError:S,isConnected:C,supportsAuthoring:w,onCreateNode:T,onCreateConnection:E,onEditNode:D,onDeleteNode:O,onDeleteNodes:k,onDeleteConnections:A,panelLayoutKey:ne,helpPanel:M}){let N=(0,F.useId)(),P=`${N}-tab-payload`,re=`${N}-tab-graph`,ie=`${N}-tab-graph-data`,ae=!!M,I=(0,F.useRef)(Number(sessionStorage.getItem(Bc))||Hc),L=(0,F.useRef)(null),oe=(0,F.useRef)(null),[se,ce]=(0,F.useState)(()=>sessionStorage.getItem(Vc)===`1`),le=(0,F.useRef)(se),ue=(0,R.jsxs)(`div`,{className:zc.rightPanel,children:[e.length>1&&(0,R.jsxs)(`div`,{className:zc.tabStrip,role:`tablist`,"aria-label":`Right panel tabs`,children:[e.includes(`payload`)&&(0,R.jsx)(`button`,{role:`tab`,"aria-selected":c===`payload`,"aria-controls":P,className:`${zc.tab}${c===`payload`?` ${zc.tabActive}`:``}`,onClick:()=>l(`payload`),children:`Payload Editor`}),e.includes(`graph`)&&(0,R.jsxs)(`button`,{role:`tab`,"aria-selected":c===`graph`,"aria-controls":re,className:`${zc.tab}${c===`graph`?` ${zc.tabActive}`:``}`,onClick:()=>l(`graph`),children:[`Graph`,o!==null&&(0,R.jsx)(`span`,{className:zc.tabBadge,"aria-label":`Graph data available`,children:`🕸️`})]}),e.includes(`graph-data`)&&(0,R.jsx)(`button`,{role:`tab`,"aria-selected":c===`graph-data`,"aria-controls":ie,className:`${zc.tab}${c===`graph-data`?` ${zc.tabActive}`:``}`,onClick:()=>l(`graph-data`),children:`Raw`})]}),e.includes(`payload`)&&(0,R.jsx)(`div`,{role:`tabpanel`,id:P,tabIndex:c===`payload`?0:-1,className:`${zc.tabBody}${c===`payload`?``:` ${zc.tabBodyHidden}`}`,children:(0,R.jsx)(ci,{payload:t,onChange:n,validation:r,onFormat:i,onUpload:a})}),e.includes(`graph`)&&(0,R.jsx)(`div`,{role:`tabpanel`,id:re,tabIndex:c===`graph`?0:-1,className:`${zc.tabBody}${c===`graph`?``:` ${zc.tabBodyHidden}`}`,children:(0,R.jsx)(`div`,{className:zc.graphContent,children:(0,R.jsx)(Ic,{graphData:o,graphName:s,onRenderError:u,isRefreshing:m,onCopySuccess:d,onCopyError:f,graphRunControls:p,onClipNode:h,onClipNodes:g,onClipboardDrop:_,onImportFiles:v,onImportGraph:y,onDownload:b,onExportImageSaved:x,onExportImageError:S,isActive:c===`graph`,isConnected:C,supportsAuthoring:w,onCreateNode:T,onCreateConnection:E,onEditNode:D,onDeleteNode:O,onDeleteNodes:k,onDeleteConnections:A,panelLayoutKey:ne})})}),e.includes(`graph-data`)&&(0,R.jsx)(`div`,{role:`tabpanel`,id:ie,tabIndex:c===`graph-data`?0:-1,className:`${zc.tabBody}${c===`graph-data`?``:` ${zc.tabBodyHidden}`}`,children:(0,R.jsx)(Rc,{graphData:o,graphName:s,onCopySuccess:d,onCopyError:f,onDownload:b})})]}),de=(0,F.useCallback)(e=>{let t=e[`help-split-help`];if(t===void 0)return;let n=t>=Uc;n!==le.current&&(le.current=n,ce(n),sessionStorage.setItem(Vc,n?`1`:`0`)),n||(I.current=t,sessionStorage.setItem(Bc,String(t)))},[]),fe=(0,F.useCallback)(()=>{let e=!le.current;if(le.current=e,ce(e),sessionStorage.setItem(Vc,e?`1`:`0`),e)oe.current?.resize(`0%`),L.current?.resize(`100%`);else{let e=I.current;L.current?.resize(`${e}%`),oe.current?.resize(`${100-e}%`)}},[]);if((0,F.useEffect)(()=>{ae&&le.current&&requestAnimationFrame(()=>{oe.current?.resize(`0%`),L.current?.resize(`100%`)})},[ae]),!M)return ue;let pe=typeof M==`function`?M(fe,se):M,me=le.current?100:I.current,he=100-me;return(0,R.jsxs)(te,{orientation:`vertical`,className:zc.rightPanelGroup,onLayoutChanged:de,children:[(0,R.jsx)(ee,{panelRef:oe,defaultSize:`${he}%`,minSize:`0%`,children:ue}),(0,R.jsx)(j,{className:zc.verticalResizeHandle,"aria-label":`Resize help panel`}),(0,R.jsx)(ee,{id:`help-split-help`,panelRef:L,defaultSize:`${me}%`,minSize:`15%`,children:pe})]})}var Gc=class extends F.Component{constructor(...e){super(...e),this.state={hasError:!1}}static getDerivedStateFromError(){return{hasError:!0}}componentDidCatch(e,t){console.error(`[ConsoleErrorBoundary] Failed to render message:`,e,t.componentStack)}render(){return this.state.hasError?(0,R.jsx)(`span`,{children:this.props.fallback}):this.props.children}},Kc=2e3,qc=(e={})=>{let{onSuccess:t,onError:n}=e,[r,i]=(0,F.useState)(!1),a=(0,F.useRef)(null);return(0,F.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current)},[]),{copy:(0,F.useCallback)(async e=>{if(!navigator.clipboard)return console.warn(`useCopyToClipboard: Clipboard API not available in this browser.`),n?.(),!1;try{return await navigator.clipboard.writeText(e),i(!0),a.current!==null&&clearTimeout(a.current),a.current=setTimeout(()=>{a.current=null,i(!1)},Kc),t?.(),!0}catch(e){return console.error(`useCopyToClipboard: Failed to write to clipboard.`,e),n?.(),!1}},[t,n]),copied:r}},Jc={consoleRoot:`_consoleRoot_1lgp1_2`,consoleHeader:`_consoleHeader_1lgp1_10`,consoleTitle:`_consoleTitle_1lgp1_20`,consoleControls:`_consoleControls_1lgp1_25`,controlButton:`_controlButton_1lgp1_30`,console:`_console_1lgp1_2`,emptyConsole:`_emptyConsole_1lgp1_67`,consoleMessage:`_consoleMessage_1lgp1_80`,consoleMessageActivatable:`_consoleMessageActivatable_1lgp1_94`,consoleMessageGraphLink:`_consoleMessageGraphLink_1lgp1_104`,consoleMessageLargePayload:`_consoleMessageLargePayload_1lgp1_115`,consoleMessageMockUpload:`_consoleMessageMockUpload_1lgp1_122`,uploadMockButton:`_uploadMockButton_1lgp1_131`,copyButton:`_copyButton_1lgp1_172`,copyButtonCopied:`_copyButtonCopied_1lgp1_225`,sendToJsonPathButton:`_sendToJsonPathButton_1lgp1_234`,messageIcon:`_messageIcon_1lgp1_268`,messageContent:`_messageContent_1lgp1_272`,messageText:`_messageText_1lgp1_278`,messageTime:`_messageTime_1lgp1_283`,"messageType-error":`_messageType-error_1lgp1_290`,"messageType-info":`_messageType-info_1lgp1_291`,"messageType-welcome":`_messageType-welcome_1lgp1_292`,jsonViewWrapper:`_jsonViewWrapper_1lgp1_295`,jsonContainer:`_jsonContainer_1lgp1_301`,jsonLabel:`_jsonLabel_1lgp1_302`,jsonString:`_jsonString_1lgp1_303`,jsonNumber:`_jsonNumber_1lgp1_304`,jsonBoolean:`_jsonBoolean_1lgp1_305`,jsonNull:`_jsonNull_1lgp1_306`};function Yc({message:e,msgId:t,classificationMap:n,onGraphLink:i,onCopyMessage:a,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){let u=ve(e),d=ye(u.type),f=be(u.message),p=(t===void 0?void 0:n?.get(t))??[],m=p.some(e=>e.kind===`graph.link`),h=p.some(e=>e.kind===`payload.large`),g=p.some(e=>e.kind===`upload.invitation`),_=p.find(e=>e.kind===`upload.invitation`)?.uploadPath??null,v=!!c&&g&&_!==null,y=v&&!!l?.has(_),b=!!i&&m&&!g&&!h,x=!!s&&f.isJSON,{copy:S,copied:C}=qc({onSuccess:a}),w=t=>{t.stopPropagation(),S(e)},T=t=>{(t.key===`Enter`||t.key===` `)&&(t.preventDefault(),t.stopPropagation(),S(e))},E=e=>{e.stopPropagation(),!(!s||!f.isJSON)&&s(JSON.stringify(f.data,null,2))},D=e=>{e.stopPropagation(),!(!c||!_)&&c(_)};return(0,R.jsxs)(`div`,{className:[Jc.consoleMessage,Jc[`messageType-${u.type}`],b?Jc.consoleMessageActivatable:``,m?Jc.consoleMessageGraphLink:``,h?Jc.consoleMessageLargePayload:``,g?Jc.consoleMessageMockUpload:``].filter(Boolean).join(` `),onClick:b?()=>i():void 0,title:b?`Click to load graph in Graph View`:void 0,role:b?`button`:void 0,tabIndex:b?0:void 0,onKeyDown:b?e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),i())}:void 0,"aria-label":b?`Load graph in Graph View`:void 0,children:[(0,R.jsx)(`span`,{className:Jc.messageIcon,children:g?`⬆️`:h?`⬇️`:m?`🕸️`:d}),(0,R.jsx)(`div`,{className:Jc.messageContent,children:f.isJSON?(0,R.jsx)(`div`,{className:Jc.jsonViewWrapper,children:(0,R.jsx)(o,{data:f.data,shouldExpandNode:e=>e<1,style:{...r,container:`${r.container} ${Jc.jsonContainer}`,label:Jc.jsonLabel,stringValue:Jc.jsonString,numberValue:Jc.jsonNumber,booleanValue:Jc.jsonBoolean,nullValue:Jc.jsonNull}})}):(0,R.jsxs)(`span`,{className:Jc.messageText,children:[u.message,y&&(0,R.jsx)(`span`,{title:`Upload succeeded`,children:` ✅`})]})}),(0,R.jsx)(`button`,{className:`${Jc.copyButton} ${C?Jc.copyButtonCopied:``}`,onClick:w,onKeyDown:T,title:C?`Copied!`:`Copy message`,"aria-label":C?`Copied to clipboard`:`Copy message to clipboard`,tabIndex:0,children:C?`✅`:`📄`}),x&&(0,R.jsx)(`button`,{className:Jc.sendToJsonPathButton,onClick:E,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&E(e)},title:`Open in JSON-Path Playground`,"aria-label":`Open this JSON in the JSON-Path Playground`,tabIndex:0,children:`➡️`}),v&&(0,R.jsx)(`button`,{className:Jc.uploadMockButton,onClick:D,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&D(e)},title:`Re-open upload dialog`,"aria-label":`Re-open mock data upload dialog`,tabIndex:0,children:`⬆️ Upload JSON…`}),u.time&&(0,R.jsx)(`span`,{className:Jc.messageTime,children:u.time})]})}function Xc({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){return(0,R.jsxs)(`div`,{className:Jc.consoleRoot,children:[(0,R.jsxs)(`div`,{className:Jc.consoleHeader,children:[(0,R.jsx)(`span`,{className:Jc.consoleTitle,children:`Console Output`}),(0,R.jsxs)(`div`,{className:Jc.consoleControls,children:[(0,R.jsx)(`button`,{className:Jc.controlButton,onClick:n,title:`Copy console output`,"aria-label":`Copy console output to clipboard`,children:`📑`}),(0,R.jsx)(`button`,{className:Jc.controlButton,onClick:r,title:`Clear console`,"aria-label":`Clear console`,children:`🗑️`})]})]}),(0,R.jsxs)(`div`,{className:Jc.console,ref:i,role:`log`,"aria-live":`polite`,children:[e.map(e=>(0,R.jsx)(Gc,{fallback:e.raw,children:(0,R.jsx)(Yc,{message:e.raw,msgId:e.id,classificationMap:t,onGraphLink:a?()=>a(e):void 0,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l})},e.id)),e.length===0&&(0,R.jsxs)(`div`,{className:Jc.emptyConsole,children:[`No messages yet. Use the `,(0,R.jsx)(`strong`,{children:`Start`}),` button in the header to connect.`]})]})]})}var Zc={commandInput:`_commandInput_188pe_2`,labelRow:`_labelRow_188pe_8`,labelGroup:`_labelGroup_188pe_16`,label:`_label_188pe_8`,infoWrapper:`_infoWrapper_188pe_28`,paletteToggle:`_paletteToggle_188pe_34`,paletteToggleActive:`_paletteToggleActive_188pe_66`,popover:`_popover_188pe_73`,popoverOpen:`_popoverOpen_188pe_95`,popoverTitle:`_popoverTitle_188pe_121`,popoverRow:`_popoverRow_188pe_135`,popoverKeyword:`_popoverKeyword_188pe_156`,popoverDesc:`_popoverDesc_188pe_168`,popoverAlias:`_popoverAlias_188pe_174`,inputRow:`_inputRow_188pe_181`,inputWrapper:`_inputWrapper_188pe_187`,textarea:`_textarea_188pe_197`,sendButton:`_sendButton_188pe_236`,hint:`_hint_188pe_253`,dropup:`_dropup_188pe_261`,dropupHeader:`_dropupHeader_188pe_276`,dropupItem:`_dropupItem_188pe_292`,dropupItemText:`_dropupItemText_188pe_315`,matchHighlight:`_matchHighlight_188pe_323`,multilineIndicator:`_multilineIndicator_188pe_329`},Qc=[`graph.data.mapper`,`graph.math`,`graph.js`,`graph.api.fetcher`,`graph.extension`,`graph.island`,`graph.join`],$c=[{keyword:`help`,description:`List all help topics, or get help for a specific command`,template:`help`},{keyword:`create`,description:`Create a new graph node`,template:`create node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`update`,description:`Update an existing node`,template:`update node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`edit`,description:`Print raw node data ready for editing and re-submitting`,template:`edit node {name}`},{keyword:`delete node`,description:`Delete a node by name`,alias:`clear node`,template:`delete node {name}`},{keyword:`delete connection`,description:`Delete connection(s) between two nodes`,alias:`clear connection`,template:`delete connection {nodeA} and {nodeB}`},{keyword:`delete cache`,description:`Clear cached API fetcher results`,alias:`clear cache`,template:`delete cache`},{keyword:`connect`,description:`Connect two nodes with a named relation`,template:`connect {node-A} to {node-B} with {relation}`},{keyword:`list nodes`,description:`List all nodes in the current graph`,template:`list nodes`},{keyword:`list connections`,description:`List all connections in the current graph`,template:`list connections`},{keyword:`describe graph`,description:`Describe the current graph model`,template:`describe graph`},{keyword:`describe node`,description:`Describe a specific node and its connections`,template:`describe node {name}`},{keyword:`describe connection`,description:`Describe connection(s) between two nodes`,template:`describe connection {nodeA} and {nodeB}`},{keyword:`describe skill`,description:`Show documentation for a skill by route name`,template:`describe skill {skill.route}`},{keyword:`export`,description:`Export the graph model to a JSON file`,template:`export graph as {name}`},{keyword:`import graph`,description:`Import a graph model from a saved file`,template:`import graph from {name}`},{keyword:`import node`,description:`Import a single node from another saved graph`,template:`import node {node-name} from {graph-name}`},{keyword:`instantiate`,description:`Create a runnable graph instance with mock input`,alias:`start`,template:`instantiate graph
{constant} -> input.body.{key}`,multiline:!0},{keyword:`upload mock data`,description:`Print the URL to POST a JSON payload as mock input.body`,template:`upload mock data`},{keyword:`execute`,description:`Execute a single node skill in isolation`,template:`execute node {name}`},{keyword:`inspect`,description:`Inspect a state-machine variable`,template:`inspect {variable_name}`},{keyword:`run`,description:`Run the graph instance from root to end`,template:`run`}];[...Qc.map(e=>({tokens:[`describe`,`skill`,e],template:`describe skill ${e}`,hint:`Describe built-in skill: ${e}`}))];function el(e,t){let[n,r]=(0,F.useState)(!1),[i,a]=(0,F.useState)(-1),o=(0,F.useMemo)(()=>{let n=t.trimStart();if(n.length===0)return[];let r=n.toLowerCase(),i=e.filter(e=>e.toLowerCase().startsWith(r)),a=new Set;return i.filter(e=>a.has(e)?!1:(a.add(e),!0)).slice(0,8)},[e,t]),s=()=>{r(!0),a(-1)},c=e=>{let t=o.length;t!==0&&a(n=>e===1?n<0?0:(n+1)%t:n<=0?t-1:n-1)},l=(e,t)=>{e>=0&&e<o.length&&t(o[e]),r(!1),a(-1)};return{suggestions:o,isOpen:n,activeIndex:i,onCommandChange:s,navigate:c,accept:l,onTab:e=>{!n||o.length===0||l(i>=0?i:0,e)},dismiss:()=>{r(!1),a(-1)}}}var tl=e=>(0,R.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:14,height:14,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,R.jsx)(`polyline`,{points:`2,4 6,8 2,12`}),(0,R.jsx)(`line`,{x1:7,y1:12,x2:14,y2:12})]});function nl({command:e,onChange:t,onKeyDown:n,onSend:r,sendDisabled:i,disabled:a,history:o}){let s=(0,F.useRef)(null),c=(0,F.useRef)(null),l=(0,F.useRef)(null),[u,d]=(0,F.useState)(!1);(0,F.useEffect)(()=>{if(!u)return;let e=e=>{c.current&&!c.current.contains(e.target)&&d(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[u]);let f=el(o,e),p=(0,F.useCallback)(()=>{let e=s.current;e&&(e.style.height=`auto`,e.style.height=`${e.scrollHeight}px`)},[]);(0,F.useEffect)(()=>{p()},[e,p]),(0,F.useEffect)(()=>{let e=s.current?.parentElement;if(!e||typeof ResizeObserver>`u`)return;let t=null,n=new ResizeObserver(()=>{t===null&&(t=requestAnimationFrame(()=>{t=null,p()}))});return n.observe(e),()=>{n.disconnect(),t!==null&&cancelAnimationFrame(t)}},[p]);let m=a?`Not connected`:`Enter command`,h=a?`Enter your test message once it is connected`:`⏎ to send · ⇧⏎ for new line · ↑↓ for history`;return(0,R.jsxs)(`div`,{className:Zc.commandInput,children:[(0,R.jsx)(`div`,{className:Zc.labelRow,children:(0,R.jsxs)(`div`,{className:Zc.labelGroup,children:[(0,R.jsx)(`label`,{htmlFor:`command`,className:Zc.label,children:`Command`}),(0,R.jsxs)(`span`,{ref:c,className:Zc.infoWrapper,children:[(0,R.jsx)(`button`,{type:`button`,className:`${Zc.paletteToggle}${u?` ${Zc.paletteToggleActive}`:``}`,"aria-label":`Toggle command palette`,"aria-expanded":u,"aria-controls":`command-palette`,onClick:()=>d(e=>!e),onKeyDown:e=>{e.key===`ArrowDown`&&u&&(e.preventDefault(),(l.current?.querySelector(`[role="option"]`))?.focus())},title:`Command palette`,children:(0,R.jsx)(tl,{"aria-hidden":`true`,focusable:`false`})}),(0,R.jsxs)(`div`,{id:`command-palette`,ref:l,className:`${Zc.popover}${u?` ${Zc.popoverOpen}`:``}`,role:`listbox`,"aria-label":`Command palette`,onKeyDown:e=>{if(e.key===`ArrowDown`||e.key===`ArrowUp`){e.preventDefault();let t=l.current?.querySelectorAll(`[role="option"]`);if(!t||t.length===0)return;let n=Array.from(t).indexOf(document.activeElement);e.key===`ArrowDown`?t[n<0?0:(n+1)%t.length].focus():t[n<=0?t.length-1:n-1].focus()}else e.key===`Escape`&&(e.preventDefault(),d(!1),s.current?.focus())},children:[(0,R.jsx)(`p`,{className:Zc.popoverTitle,children:`Command palette — click to insert`}),$c.map(({keyword:e,alias:n,description:r,template:i})=>(0,R.jsxs)(`div`,{className:Zc.popoverRow,role:`option`,"aria-selected":!1,tabIndex:u?0:-1,onMouseDown:e=>e.preventDefault(),onClick:()=>{t(i),d(!1),s.current?.focus()},onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),t(i),d(!1),s.current?.focus())},children:[(0,R.jsx)(`span`,{className:Zc.popoverKeyword,children:e}),(0,R.jsxs)(`span`,{className:Zc.popoverDesc,children:[r,n&&(0,R.jsxs)(`span`,{className:Zc.popoverAlias,children:[` · alias: `,n]})]})]},e))]})]})]})}),(0,R.jsxs)(`div`,{className:Zc.inputRow,children:[(0,R.jsxs)(`div`,{className:Zc.inputWrapper,children:[(0,R.jsxs)(`div`,{id:`history-dropup`,role:`listbox`,"aria-label":`Command history suggestions`,className:Zc.dropup,hidden:!(f.isOpen&&f.suggestions.length>0),children:[(0,R.jsx)(`div`,{className:Zc.dropupHeader,"aria-hidden":`true`,children:`Recent Commands`}),f.isOpen&&f.suggestions.length>0&&f.suggestions.map((n,r)=>{let i=n.split(`
`)[0],a=n.includes(`
`),o=e.trimStart().split(`
`)[0],c=Math.min(o.length,i.length),l=i.slice(0,c),u=i.slice(c);return(0,R.jsxs)(`div`,{id:`history-option-${r}`,role:`option`,"aria-selected":r===f.activeIndex,className:Zc.dropupItem,onMouseDown:e=>e.preventDefault(),onClick:()=>{f.accept(r,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)})},children:[(0,R.jsxs)(`span`,{className:Zc.dropupItemText,children:[c>0&&(0,R.jsx)(`strong`,{className:Zc.matchHighlight,children:l}),u,a?`…`:``]}),a&&(0,R.jsx)(`span`,{className:Zc.multilineIndicator,"aria-label":`multi-line command`,children:`↵`})]},n)})]}),(0,R.jsx)(`textarea`,{ref:s,id:`command`,role:`combobox`,"aria-expanded":f.isOpen&&f.suggestions.length>0,"aria-haspopup":`listbox`,"aria-controls":`history-dropup`,"aria-activedescendant":f.isOpen&&f.suggestions.length>0&&f.activeIndex>=0?`history-option-${f.activeIndex}`:void 0,"aria-autocomplete":`list`,className:Zc.textarea,rows:1,placeholder:m,value:e,disabled:a,onChange:e=>{t(e.target.value),f.onCommandChange()},onKeyDown:e=>{if(e.key===`Tab`){e.preventDefault(),f.isOpen&&f.suggestions.length>0&&(f.onTab(e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}));return}if(e.key===`Enter`){if(e.shiftKey)return;if(e.preventDefault(),f.isOpen&&f.activeIndex>=0){f.accept(f.activeIndex,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}),s.current?.focus();return}r(),s.current?.focus();return}if(e.key===`Escape`){if(f.isOpen){f.dismiss(),e.preventDefault();return}return}if(e.key===`ArrowUp`||e.key===`ArrowDown`){if(f.isOpen&&f.suggestions.length>0){e.preventDefault(),f.navigate(e.key===`ArrowDown`?1:-1);return}let t=s.current;if(t){let{selectionStart:n,value:r}=t,i=!r.slice(0,n).includes(`
`),a=!r.slice(n).includes(`
`);if(!(e.key===`ArrowUp`&&i||e.key===`ArrowDown`&&a))return}n(e),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)});return}n(e)},onBlur:()=>f.dismiss(),autoComplete:`off`,autoCorrect:`off`,spellCheck:!1})]}),(0,R.jsx)(`button`,{className:Zc.sendButton,onClick:()=>{r(),s.current?.focus()},disabled:i,"aria-label":`Send command`,children:`Send`})]}),h&&(0,R.jsx)(`p`,{className:Zc.hint,children:h})]})}var rl={root:`_root_1ac49_1`};function il({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l,command:u,onCommandChange:d,onCommandKeyDown:f,onSend:p,sendDisabled:m,inputDisabled:h,commandHistory:g}){return(0,R.jsxs)(`div`,{className:rl.root,children:[(0,R.jsx)(Xc,{messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}),(0,R.jsx)(nl,{command:u,onChange:d,onKeyDown:f,onSend:p,disabled:h,sendDisabled:m,history:g})]})}var al=e=>(0,R.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:16,height:16,stroke:`currentColor`,strokeWidth:1.8,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,R.jsx)(`line`,{x1:4.75,y1:4.75,x2:11.25,y2:11.25}),(0,R.jsx)(`line`,{x1:11.25,y1:4.75,x2:4.75,y2:11.25})]}),K={root:`_root_1f65i_9`,card:`_card_1f65i_19`,ribbon:`_ribbon_1f65i_32`,ribbonIcon:`_ribbonIcon_1f65i_45`,ribbonAlias:`_ribbonAlias_1f65i_50`,ribbonAliasInput:`_ribbonAliasInput_1f65i_57`,ribbonBadge:`_ribbonBadge_1f65i_82`,ribbonClose:`_ribbonClose_1f65i_93`,ribbonCloseIcon:`_ribbonCloseIcon_1f65i_114`,body:`_body_1f65i_121`,row:`_row_1f65i_128`,rowDropTarget:`_rowDropTarget_1f65i_138`,dragGrip:`_dragGrip_1f65i_142`,rowLabel:`_rowLabel_1f65i_166`,rowKey:`_rowKey_1f65i_167`,rowValue:`_rowValue_1f65i_178`,rowSpacer:`_rowSpacer_1f65i_185`,keyInput:`_keyInput_1f65i_189`,valueInput:`_valueInput_1f65i_190`,removeButton:`_removeButton_1f65i_228`,removeIcon:`_removeIcon_1f65i_251`,addRow:`_addRow_1f65i_257`,addButton:`_addButton_1f65i_263`,message:`_message_1f65i_282`,errorMessage:`_errorMessage_1f65i_283`,warningMessage:`_warningMessage_1f65i_284`,errorText:`_errorText_1f65i_306`,footer:`_footer_1f65i_312`,secondaryButton:`_secondaryButton_1f65i_321`,primaryButton:`_primaryButton_1f65i_322`,schemaLink:`_schemaLink_1f65i_359`},ol=1,sl=10,cl=36;function ll(e){let t=e.split(`
`).reduce((e,t)=>e+Math.max(1,Math.ceil(t.length/cl)),0);return Math.min(Math.max(t,ol),sl)}function ul({mode:e,formState:t,phase:n,lockReason:r,serverMessage:i,validationErrors:a,onFormStateChange:o,onSubmit:s,onClose:c,onOpenSchemaPanel:l}){let u=(0,F.useRef)(null),d=(0,F.useRef)(null),f=(0,F.useRef)(new Map),p=(0,F.useRef)(null),m=e===`create`,h=n===`sending`,g=r===`disconnected`,_=h||g,v=m?`Create Node`:`Save Changes`,y=m?`Creating...`:`Saving...`,b=m?`Connection disconnected. Refresh the page and create the node again after the app reconnects.`:`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,x=mi(t.nodeType),S=hi(t.nodeType);(0,F.useEffect)(()=>{m?u.current?.focus():d.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),h||c())};return document.addEventListener(`keydown`,e),()=>{document.removeEventListener(`keydown`,e)}},[m,c,h]),(0,F.useEffect)(()=>{let e=p.current;if(!e)return;let t=f.current.get(e);t&&(t.focus(),p.current=null)},[t.properties]);let C=(0,F.useCallback)(e=>{e.preventDefault(),!_&&s()},[_,s]),w=(0,F.useCallback)(e=>{o({...t,...e})},[t,o]),T=(0,F.useCallback)((e,n)=>{o({...t,properties:t.properties.map(t=>t.id===e?{...t,...n}:t)})},[t,o]),E=(0,F.useCallback)(()=>{let e=nr();p.current=e.id,o({...t,properties:[...t.properties,e]})},[t,o]),D=(0,F.useCallback)(e=>{let n=t.properties.filter(t=>t.id!==e);o({...t,properties:n.length>0?n:[nr()]})},[t,o]),O=(0,F.useRef)(null),[k,A]=(0,F.useState)(null),ee=(0,F.useCallback)(()=>{O.current=null,A(null)},[]),j=(0,F.useCallback)(e=>t=>{if(O.current=e,t.dataTransfer){t.dataTransfer.effectAllowed=`move`,t.dataTransfer.setData(`text/plain`,e);let n=t.currentTarget.closest(`[data-row-id]`);n instanceof HTMLElement&&typeof t.dataTransfer.setDragImage==`function`&&t.dataTransfer.setDragImage(n,16,16)}},[]),te=(0,F.useCallback)(e=>t=>{O.current!==null&&(t.preventDefault(),t.dataTransfer&&(t.dataTransfer.dropEffect=`move`),A(t=>t===e?t:e))},[]),ne=(0,F.useCallback)(e=>{let n=O.current;if(ee(),n===null||n===e)return;let r=t.properties.find(e=>e.id===n);if(!r)return;let i=t.properties.filter(e=>e.id!==n),a=i.length;if(e!==null){let t=i.findIndex(t=>t.id===e);t!==-1&&(a=t)}let s=[...i.slice(0,a),r,...i.slice(a)];o({...t,properties:sr(s)})},[ee,t,o]),M=(0,F.useCallback)(e=>t=>{O.current!==null&&(t.preventDefault(),ne(e))},[ne]);return(0,R.jsx)(`div`,{className:K.root,children:(0,R.jsxs)(`form`,{className:K.card,style:{borderColor:S,"--node-accent":S},"aria-label":m?`Create node`:`Edit node ${t.alias}`,onSubmit:C,children:[(0,R.jsxs)(`header`,{className:K.ribbon,children:[(0,R.jsx)(`span`,{className:K.ribbonIcon,"aria-hidden":`true`,children:x.icon}),m?(0,R.jsx)(`input`,{ref:u,className:K.ribbonAliasInput,value:t.alias,placeholder:`node-alias`,"aria-label":`Node alias`,disabled:_,"aria-invalid":!!a.alias,onChange:e=>w({alias:e.target.value})}):(0,R.jsx)(`span`,{className:K.ribbonAlias,children:t.alias}),(0,R.jsx)(`span`,{className:K.ribbonBadge,children:x.label}),(0,R.jsx)(`button`,{type:`button`,className:K.ribbonClose,"aria-label":`Close node editor`,title:`Close (Esc)`,onClick:c,disabled:h,children:(0,R.jsx)(al,{className:K.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,R.jsxs)(`div`,{className:K.body,children:[i&&!g&&(0,R.jsx)(`div`,{className:K.message,role:`status`,children:i}),a.command&&(0,R.jsx)(`div`,{className:K.errorMessage,role:`alert`,children:a.command}),a.alias&&(0,R.jsx)(`div`,{className:K.errorMessage,role:`alert`,children:a.alias}),g&&(0,R.jsx)(`div`,{className:K.warningMessage,role:`status`,children:i??b}),l&&(0,R.jsxs)(`div`,{className:K.message,role:`note`,children:[`This node carries the graph's `,t.alias.trim()===`root`?`request`:`response`,` contract (`,(0,R.jsx)(`code`,{children:`schema`}),`).`,` `,(0,R.jsx)(`button`,{type:`button`,className:K.schemaLink,onClick:l,disabled:h,title:`Edit the contract as rows in the Schema panel; this editor closes`,children:`Open the Schema panel…`})]}),(0,R.jsxs)(`div`,{className:K.row,children:[(0,R.jsx)(`span`,{"aria-hidden":`true`}),(0,R.jsx)(`label`,{className:K.rowLabel,htmlFor:`node-edit-type`,children:`type`}),(0,R.jsxs)(`div`,{className:K.rowValue,children:[(0,R.jsx)(`input`,{id:`node-edit-type`,ref:d,className:K.valueInput,value:t.nodeType,disabled:_,"aria-invalid":!!a.nodeType,onChange:e=>w({nodeType:e.target.value})}),a.nodeType&&(0,R.jsx)(`span`,{className:K.errorText,children:a.nodeType})]}),(0,R.jsx)(`span`,{className:K.rowSpacer,"aria-hidden":`true`})]}),t.properties.map(e=>{let t=a[Kn(e.id,`key`)],n=a[Kn(e.id,`value`)];return(0,R.jsxs)(`div`,{"data-row-id":e.id,className:k===e.id?`${K.row} ${K.rowDropTarget}`:K.row,onDragOver:te(e.id),onDrop:M(e.id),children:[(0,R.jsx)(`span`,{className:K.dragGrip,role:`button`,"aria-label":`Reorder property ${e.key.trim()||`(empty)`}`,title:`Drag to reorder — same keys append as [0], [1], … in row order`,draggable:!_,onDragStart:j(e.id),onDragEnd:ee,children:`⠿`}),(0,R.jsxs)(`div`,{className:K.rowKey,children:[(0,R.jsx)(`input`,{ref:t=>{t?f.current.set(e.id,t):f.current.delete(e.id)},className:K.keyInput,value:e.key,placeholder:`key`,"aria-label":`Property key`,disabled:_,"aria-invalid":!!t,onChange:t=>T(e.id,{key:t.target.value})}),t&&(0,R.jsx)(`span`,{className:K.errorText,children:t})]}),(0,R.jsxs)(`div`,{className:K.rowValue,children:[(0,R.jsx)(`textarea`,{className:K.valueInput,value:e.value,placeholder:`value`,"aria-label":`Property value`,disabled:_,rows:ll(e.value),"aria-invalid":!!n,onChange:t=>T(e.id,{value:t.target.value})}),n&&(0,R.jsx)(`span`,{className:K.errorText,children:n})]}),(0,R.jsx)(`button`,{type:`button`,className:K.removeButton,"aria-label":`Remove property`,disabled:_,onClick:()=>D(e.id),children:(0,R.jsx)(al,{className:K.removeIcon,"aria-hidden":`true`,focusable:`false`})})]},e.id)}),(0,R.jsx)(`div`,{className:k===`end`?`${K.addRow} ${K.rowDropTarget}`:K.addRow,onDragOver:te(`end`),onDrop:M(null),children:(0,R.jsxs)(`button`,{type:`button`,className:K.addButton,disabled:_,onClick:E,children:[(0,R.jsx)(`span`,{"aria-hidden":`true`,children:`+`}),(0,R.jsx)(`span`,{children:`Add Property`})]})})]}),(0,R.jsxs)(`footer`,{className:K.footer,children:[(0,R.jsx)(`button`,{type:`button`,className:K.secondaryButton,onClick:c,disabled:h,children:`Cancel`}),(0,R.jsx)(`button`,{type:`submit`,className:K.primaryButton,disabled:_,children:h?y:v})]})]})})}var q={root:`_root_15u4d_7`,card:`_card_15u4d_17`,ribbon:`_ribbon_15u4d_30`,titleGroup:`_titleGroup_15u4d_41`,title:`_title_15u4d_41`,path:`_path_15u4d_54`,ribbonClose:`_ribbonClose_15u4d_61`,ribbonCloseIcon:`_ribbonCloseIcon_15u4d_90`,body:`_body_15u4d_96`,description:`_description_15u4d_105`,inputHints:`_inputHints_15u4d_112`,inputHintsLabel:`_inputHintsLabel_15u4d_122`,inputHintList:`_inputHintList_15u4d_128`,moreHints:`_moreHints_15u4d_135`,inputHintsNote:`_inputHintsNote_15u4d_147`,dropZone:`_dropZone_15u4d_153`,dropZoneActive:`_dropZoneActive_15u4d_175`,dropZoneIcon:`_dropZoneIcon_15u4d_181`,dropZoneText:`_dropZoneText_15u4d_187`,dropZoneOr:`_dropZoneOr_15u4d_200`,browseButton:`_browseButton_15u4d_207`,fileInputHidden:`_fileInputHidden_15u4d_236`,fileError:`_fileError_15u4d_241`,textareaLabel:`_textareaLabel_15u4d_246`,textarea:`_textarea_15u4d_246`,validationError:`_validationError_15u4d_274`,keyboardHint:`_keyboardHint_15u4d_279`,errorBanner:`_errorBanner_15u4d_284`,footer:`_footer_15u4d_295`,footerActions:`_footerActions_15u4d_305`,formatButton:`_formatButton_15u4d_311`,cancelButton:`_cancelButton_15u4d_312`,uploadButton:`_uploadButton_15u4d_313`,spinner:`_spinner_15u4d_380`,spin:`_spin_15u4d_380`,rows:`_rows_15u4d_396`,rowBlock:`_rowBlock_15u4d_402`,row:`_row_15u4d_396`,rowName:`_rowName_15u4d_414`,rowValue:`_rowValue_15u4d_418`,input:`_input_15u4d_112`,inputInvalid:`_inputInvalid_15u4d_435`,rowType:`_rowType_15u4d_440`,entryRemove:`_entryRemove_15u4d_445`,entryIssue:`_entryIssue_15u4d_450`,linkButton:`_linkButton_15u4d_455`,headers:`_headers_15u4d_480`};function dl({uploadPath:e,json:t,headers:n,model:r,onSuccess:i,onError:a}){let[o,s]=(0,F.useState)(!1),c=(0,F.useRef)(null),l=(0,F.useCallback)(()=>{c.current?.abort(),c.current=null,s(!1)},[]);return{isUploading:o,upload:(0,F.useCallback)(async()=>{c.current?.abort();let o=new AbortController;c.current=o,s(!0);let l=Object.entries(n??{}),u=Object.entries(r??{}),d=t.trim()!==``;try{let n=``;if(d){let r=await fetch(e,{method:`POST`,headers:{"Content-Type":`application/json`},body:t,signal:o.signal});if(n=await r.text(),!r.ok){s(!1),a(`HTTP ${r.status} — ${n}`);return}}let r=[[`header`,l,`mock headers`],[`model`,u,`mock model variables`]];for(let[t,i,c]of r){if(i.length===0)continue;let r=await fetch(`${e}?namespace=${t}`,{method:`POST`,headers:{"Content-Type":`application/json`},body:JSON.stringify(Object.fromEntries(i)),signal:o.signal});if(n=await r.text(),!r.ok){s(!1),a(`HTTP ${r.status} — ${n} (${c})`);return}}s(!1),i(n)}catch(e){if(e.name===`AbortError`){s(!1);return}s(!1),a(e.message??`Network error`)}},[e,t,n,r,i,a]),cancel:l}}function fl(e){return new Promise((t,n)=>{let r=new FileReader;r.onload=()=>t(r.result),r.onerror=()=>n(Error(`Could not read file "${e.name}"`)),r.readAsText(e,`utf-8`)})}function pl(e){let t=e.name.toLowerCase().endsWith(`.json`),n=e.type===`application/json`||e.type===`text/plain`;return!t&&!n?`"${e.name}" does not appear to be a JSON file. Only .json files are accepted.`:null}var ml=0;function hl(){return ml+=1,{key:`h${ml}`,name:``,value:``}}function gl(e){let t=new Map,n=new Set;for(let r of e){let e=r.name.trim();if(e===``){r.value.trim()!==``&&t.set(r.key,`A header value needs a name`);continue}let i=e.toLowerCase();n.has(i)?t.set(r.key,`Duplicate header name '${e}'`):n.add(i)}return t}var _l=0;function vl(){return _l+=1,{key:`m${_l}`,name:``,type:`text`,value:``}}var yl=/^-?(0|[1-9]\d*)(\.\d+)?([eE][+-]?\d+)?$/;function bl(e){switch(e.type){case`number`:return yl.test(e.value.trim())?{value:Number(e.value.trim())}:{issue:`The value is not a number`};case`boolean`:return{value:e.value===`true`};case`json`:try{let t=JSON.parse(e.value);return t===null?{issue:`A JSON value cannot be null`}:{value:t}}catch{return{issue:`The value is not valid JSON`}}default:return{value:e.value}}}function xl(e){let t=new Map,n=new Set;for(let r of e){let e=r.name.trim();if(e===``){r.type!==`boolean`&&r.value.trim()!==``&&t.set(r.key,`A model variable needs a name`);continue}if(e.startsWith(`.`)||e.endsWith(`.`)||e.includes(`..`))t.set(r.key,`Empty path segment in '${e}'`);else if(n.has(e))t.set(r.key,`Duplicate model variable '${e}'`);else{n.add(e);let i=bl(r).issue;i&&t.set(r.key,i)}}return t}var Sl=(navigator.userAgentData?.platform??navigator.platform).toLowerCase().includes(`mac`);function Cl({uploadPath:e,onSuccess:t,onClose:n,onError:r,title:i=`⬆️ Upload Mock Data`,description:a,inputPathHints:o=[],inputHeaderHints:s=[],submitLabel:c=`Upload`}){let[l,u]=(0,F.useState)(``),[d,f]=(0,F.useState)([]),[p,m]=(0,F.useState)([]),[h,g]=(0,F.useState)(null),[_,v]=(0,F.useState)(null),[y,b]=(0,F.useState)(!1),x=(0,F.useRef)(null),S=(0,F.useRef)(null),C=be(l).isJSON,w=l.trim()!==``,T=(0,F.useMemo)(()=>gl(d),[d]),E=(0,F.useMemo)(()=>{let e={};for(let t of d){let n=t.name.trim();n!==``&&!T.has(t.key)&&(e[n]=t.value)}return e},[d,T]),D=Object.keys(E).length,O=(0,F.useMemo)(()=>xl(p),[p]),k=(0,F.useMemo)(()=>{let e={};for(let t of p){let n=t.name.trim();n!==``&&!O.has(t.key)&&(e[n]=bl(t).value)}return e},[p,O]),A=Object.keys(k).length,ee=(w?C:D+A>0)&&T.size===0&&O.size===0,{isUploading:j,upload:te,cancel:ne}=dl({uploadPath:e,json:l,headers:E,model:k,onSuccess:n=>t(n,e),onError:e=>{g(e),r(e)}}),M=(0,F.useCallback)(()=>{ne(),n(e)},[ne,n,e]),N=(0,F.useRef)(j);(0,F.useEffect)(()=>{N.current=j},[j]),(0,F.useEffect)(()=>{x.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),N.current||M())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[M]);let P=(0,F.useCallback)(()=>{g(null),te()},[te]),re=(0,F.useCallback)(e=>{e.key===`Enter`&&(e.ctrlKey||e.metaKey)&&(e.preventDefault(),ee&&!j&&P())},[ee,j,P]),ie=(0,F.useCallback)(()=>f(e=>[...e,hl()]),[]),ae=(0,F.useCallback)((e,t)=>{f(n=>n.map(n=>n.key===e?{...n,...t}:n))},[]),I=(0,F.useCallback)(e=>{f(t=>t.filter(t=>t.key!==e))},[]),L=(0,F.useCallback)(()=>m(e=>[...e,vl()]),[]),se=(0,F.useCallback)((e,t)=>{m(n=>n.map(n=>{if(n.key!==e)return n;let r={...n,...t};return r.type===`boolean`&&r.value!==`true`&&r.value!==`false`&&(r.value=`true`),r}))},[]),ce=(0,F.useCallback)(e=>{m(t=>t.filter(t=>t.key!==e))},[]),le=(0,F.useCallback)(()=>{C&&u(oe(l))},[C,l]),ue=(0,F.useCallback)(async e=>{v(null),g(null);let t=pl(e);if(t){v(t);return}try{let t=await fl(e);if(!be(t).isJSON){v(`"${e.name}" contains invalid JSON.`);return}u(oe(t)),x.current?.focus()}catch(e){v(e.message)}},[]),de=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),y||b(!0)},[y]),fe=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&b(!1)},[]),pe=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),b(!1);let t=e.dataTransfer.files[0];t&&ue(t)},[ue]),me=(0,F.useCallback)(e=>{let t=e.target.files?.[0];t&&(ue(t),e.target.value=``)},[ue]),he=!C&&w;return(0,R.jsx)(`div`,{className:q.root,children:(0,R.jsxs)(`section`,{className:q.card,"aria-label":i,children:[(0,R.jsxs)(`header`,{className:q.ribbon,children:[(0,R.jsxs)(`div`,{className:q.titleGroup,children:[(0,R.jsx)(`span`,{className:q.title,children:i}),(0,R.jsx)(`span`,{className:q.path,children:e})]}),(0,R.jsx)(`button`,{className:q.ribbonClose,onClick:M,"aria-label":`Close upload panel`,title:`Close (Esc)`,disabled:j,children:(0,R.jsx)(al,{className:q.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,R.jsxs)(`div`,{className:q.body,children:[a&&(0,R.jsx)(`p`,{className:q.description,children:a}),o.length>0&&(0,R.jsxs)(`div`,{className:q.inputHints,"aria-label":`Referenced graph input paths`,children:[(0,R.jsx)(`span`,{className:q.inputHintsLabel,children:`Referenced input paths`}),(0,R.jsxs)(`div`,{className:q.inputHintList,children:[o.slice(0,6).map(e=>(0,R.jsx)(`code`,{children:e},e)),o.length>6&&(0,R.jsxs)(`span`,{className:q.moreHints,children:[`+`,o.length-6,` more`]})]}),(0,R.jsx)(`span`,{className:q.inputHintsNote,children:`Hints are derived from graph references.`})]}),s.length>0&&(0,R.jsxs)(`div`,{className:q.inputHints,"aria-label":`Referenced input headers`,children:[(0,R.jsx)(`span`,{className:q.inputHintsLabel,children:`Referenced input headers`}),(0,R.jsxs)(`div`,{className:q.inputHintList,children:[s.slice(0,6).map(e=>(0,R.jsx)(`code`,{children:e===`*`?`input.header (all)`:e},e)),s.length>6&&(0,R.jsxs)(`span`,{className:q.moreHints,children:[`+`,s.length-6,` more`]})]}),(0,R.jsx)(`span`,{className:q.inputHintsNote,children:`Header names are case-insensitive to the graph.`})]}),(0,R.jsxs)(`div`,{className:`${q.dropZone} ${y?q.dropZoneActive:``}`,onDragOver:de,onDragLeave:fe,onDrop:pe,"aria-label":`Drop a JSON file here`,children:[(0,R.jsx)(`span`,{className:q.dropZoneIcon,children:`📂`}),(0,R.jsxs)(`span`,{className:q.dropZoneText,children:[`Drop a `,(0,R.jsx)(`code`,{children:`.json`}),` file here`]}),(0,R.jsx)(`span`,{className:q.dropZoneOr,children:`— or —`}),(0,R.jsx)(`input`,{ref:S,type:`file`,accept:`.json,application/json`,className:q.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:me}),(0,R.jsx)(`button`,{type:`button`,className:q.browseButton,onClick:()=>S.current?.click(),disabled:j,"aria-label":`Browse for a JSON file`,children:`Browse file…`})]}),_&&(0,R.jsxs)(`span`,{className:q.fileError,role:`alert`,children:[`⚠️ `,_]}),(0,R.jsx)(`label`,{htmlFor:`mock-upload-textarea`,className:q.textareaLabel,children:`JSON payload (input body)`}),(0,R.jsx)(`textarea`,{id:`mock-upload-textarea`,ref:x,className:q.textarea,value:l,onChange:e=>{u(e.target.value),v(null)},onKeyDown:re,placeholder:`Paste JSON here, or drop / browse a .json file above`,rows:10,spellCheck:!1,"aria-describedby":he?`mock-upload-validation`:void 0}),he&&(0,R.jsx)(`span`,{id:`mock-upload-validation`,className:q.validationError,role:`status`,children:`⚠️ Invalid JSON — check syntax`}),(0,R.jsx)(`span`,{className:q.keyboardHint,children:Sl?`⌘+Enter to ${c.toLowerCase()}`:`Ctrl+Enter to ${c.toLowerCase()}`}),(0,R.jsxs)(`div`,{className:q.headers,children:[(0,R.jsx)(`span`,{className:q.textareaLabel,children:`Input header (optional)`}),(0,R.jsxs)(`div`,{className:q.rows,role:`group`,"aria-label":`Mock headers`,children:[d.map(e=>{let t=T.get(e.key);return(0,R.jsxs)(`div`,{className:q.rowBlock,children:[(0,R.jsxs)(`div`,{className:q.row,children:[(0,R.jsx)(`input`,{className:`${q.input} ${q.rowName}${t?` ${q.inputInvalid}`:``}`,type:`text`,value:e.name,onChange:t=>ae(e.key,{name:t.target.value}),placeholder:`name`,"aria-label":`Header name`,autoComplete:`off`,spellCheck:!1,disabled:j}),(0,R.jsx)(`input`,{className:`${q.input} ${q.rowValue}`,type:`text`,value:e.value,onChange:t=>ae(e.key,{value:t.target.value}),placeholder:`value`,"aria-label":`Value of header ${e.name||`(unnamed)`}`,autoComplete:`off`,disabled:j}),(0,R.jsx)(`button`,{type:`button`,className:q.entryRemove,onClick:()=>I(e.key),disabled:j,"aria-label":`Remove header ${e.name||`(unnamed)`}`,title:`Remove header`,children:`×`})]}),t&&(0,R.jsxs)(`span`,{className:q.entryIssue,role:`alert`,children:[`⚠️ `,t]})]},e.key)}),(0,R.jsx)(`button`,{type:`button`,className:q.linkButton,onClick:ie,disabled:j,children:`+ Add header`})]}),(0,R.jsx)(`span`,{className:q.inputHintsNote,children:`The headers become the instance's input.header as a request delivers them: the names in lower case, and the correlation-ID header (X-Correlation-Id by default) sets model.cid - without one, the header carries the instance's model.cid.`})]}),(0,R.jsxs)(`div`,{className:q.headers,children:[(0,R.jsx)(`span`,{className:q.textareaLabel,children:`Model variables (optional)`}),(0,R.jsxs)(`div`,{className:q.rows,role:`group`,"aria-label":`Mock model variables`,children:[p.map(e=>{let t=O.get(e.key),n=e.name||`(unnamed)`;return(0,R.jsxs)(`div`,{className:q.rowBlock,children:[(0,R.jsxs)(`div`,{className:q.row,children:[(0,R.jsx)(`input`,{className:`${q.input} ${q.rowName}${t?` ${q.inputInvalid}`:``}`,type:`text`,value:e.name,onChange:t=>se(e.key,{name:t.target.value}),placeholder:`name`,"aria-label":`Model variable name`,autoComplete:`off`,spellCheck:!1,disabled:j}),(0,R.jsxs)(`select`,{className:`${q.input} ${q.rowType}`,value:e.type,onChange:t=>se(e.key,{type:t.target.value}),"aria-label":`Type of model variable ${n}`,disabled:j,children:[(0,R.jsx)(`option`,{value:`text`,children:`text`}),(0,R.jsx)(`option`,{value:`number`,children:`number`}),(0,R.jsx)(`option`,{value:`boolean`,children:`boolean`}),(0,R.jsx)(`option`,{value:`json`,children:`JSON`})]}),e.type===`boolean`?(0,R.jsxs)(`select`,{className:`${q.input} ${q.rowValue}`,value:e.value,onChange:t=>se(e.key,{value:t.target.value}),"aria-label":`Value of model variable ${n}`,disabled:j,children:[(0,R.jsx)(`option`,{value:`true`,children:`true`}),(0,R.jsx)(`option`,{value:`false`,children:`false`})]}):(0,R.jsx)(`input`,{className:`${q.input} ${q.rowValue}`,type:`text`,value:e.value,onChange:t=>se(e.key,{value:t.target.value}),placeholder:e.type===`json`?`{"a": 1} or [1, 2]`:`value`,"aria-label":`Value of model variable ${n}`,autoComplete:`off`,spellCheck:!1,disabled:j}),(0,R.jsx)(`button`,{type:`button`,className:q.entryRemove,onClick:()=>ce(e.key),disabled:j,"aria-label":`Remove model variable ${n}`,title:`Remove model variable`,children:`×`})]}),t&&(0,R.jsxs)(`span`,{className:q.entryIssue,role:`alert`,children:[`⚠️ `,t]})]},e.key)}),(0,R.jsx)(`button`,{type:`button`,className:q.linkButton,onClick:L,disabled:j,children:`+ Add model variable`})]}),(0,R.jsxs)(`span`,{className:q.inputHintsNote,children:[`Each variable is set as model.`,`{name}`,` after the input header, and the other model variables stay - the state a request's edge would set, which a dry run mocks here. A dotted or indexed name (limits.daily, items[0]) sets a nested value.`]})]}),h&&(0,R.jsxs)(`div`,{className:q.errorBanner,role:`alert`,children:[`❌ Upload failed: `,h]})]}),(0,R.jsxs)(`footer`,{className:q.footer,children:[(0,R.jsx)(`button`,{className:q.formatButton,onClick:le,disabled:!C||j,title:`Format JSON`,"aria-label":`Format JSON`,children:`Format`}),(0,R.jsxs)(`div`,{className:q.footerActions,children:[(0,R.jsx)(`button`,{className:q.cancelButton,onClick:M,disabled:j,children:`Cancel`}),(0,R.jsx)(`button`,{className:q.uploadButton,onClick:P,disabled:!ee||j,"aria-busy":j,children:j?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:q.spinner,"aria-hidden":`true`}),` Uploading…`]}):c===`Upload`?`Upload ▶`:c})]})]})]})})}var J={root:`_root_iz4rh_7`,card:`_card_iz4rh_17`,ribbon:`_ribbon_iz4rh_30`,titleGroup:`_titleGroup_iz4rh_41`,title:`_title_iz4rh_41`,subtitle:`_subtitle_iz4rh_54`,ribbonClose:`_ribbonClose_iz4rh_61`,ribbonCloseIcon:`_ribbonCloseIcon_iz4rh_90`,body:`_body_iz4rh_96`,description:`_description_iz4rh_105`,empty:`_empty_iz4rh_106`,hint:`_hint_iz4rh_115`,sectionTitle:`_sectionTitle_iz4rh_123`,dropZone:`_dropZone_iz4rh_133`,dropZoneActive:`_dropZoneActive_iz4rh_153`,dropZoneIcon:`_dropZoneIcon_iz4rh_159`,dropZoneText:`_dropZoneText_iz4rh_165`,dropZoneOr:`_dropZoneOr_iz4rh_178`,browseButton:`_browseButton_iz4rh_184`,fileInputHidden:`_fileInputHidden_iz4rh_212`,addRow:`_addRow_iz4rh_216`,fileErrors:`_fileErrors_iz4rh_223`,entryList:`_entryList_iz4rh_235`,entry:`_entry_iz4rh_235`,entryFlagged:`_entryFlagged_iz4rh_255`,entryMain:`_entryMain_iz4rh_260`,entryId:`_entryId_iz4rh_267`,entryCounts:`_entryCounts_iz4rh_274`,entryIssue:`_entryIssue_iz4rh_280`,entryRemove:`_entryRemove_iz4rh_285`,entryAction:`_entryAction_iz4rh_286`,manifestList:`_manifestList_iz4rh_323`,manifestRow:`_manifestRow_iz4rh_330`,manifestKey:`_manifestKey_iz4rh_337`,manifestValue:`_manifestValue_iz4rh_343`,fieldLabel:`_fieldLabel_iz4rh_349`,inputRow:`_inputRow_iz4rh_355`,input:`_input_iz4rh_355`,inputInvalid:`_inputInvalid_iz4rh_383`,extension:`_extension_iz4rh_387`,hintError:`_hintError_iz4rh_400`,rows:`_rows_iz4rh_404`,rowBlock:`_rowBlock_iz4rh_410`,row:`_row_iz4rh_404`,rowName:`_rowName_iz4rh_422`,rowValue:`_rowValue_iz4rh_426`,linkButton:`_linkButton_iz4rh_430`,errorBanner:`_errorBanner_iz4rh_455`,footer:`_footer_iz4rh_466`,footerNote:`_footerNote_iz4rh_478`,footerActions:`_footerActions_iz4rh_485`,secondaryButton:`_secondaryButton_iz4rh_492`,primaryButton:`_primaryButton_iz4rh_493`,spinner:`_spinner_iz4rh_543`,spin:`_spin_iz4rh_543`},wl=`/api/graph-set/pack`,Tl=`/api/graph-set/unpack`,El=`.pack`,Dl=`.json`,Ol=`graph_id`,kl=new Set([`format`,`format_version`]),Al=[`version`,`description`];function jl(e){let t=e.trim();return t.toLowerCase().endsWith(Dl)?t.slice(0,t.length-5):``}function Ml(e){return e.name.toLowerCase().endsWith(El)}function Nl(e){return`${e}${El}`}function Pl(e){let t=e.trim();return t===``?`A set name is required - it names the set and its file.`:$a(t)?null:`A set name uses ${Qa}.`}function Fl(e){let t=new Map,n=new Map;for(let t of e)n.set(t.id,(n.get(t.id)??0)+1);for(let r of e)$a(r.id)?r.name!==null&&r.name!==r.id?t.set(r.key,`The root node's name "${r.name}" differs from the graph id.`):(n.get(r.id)??0)>1&&t.set(r.key,`Duplicate graph id "${r.id}" - remove one.`):t.set(r.key,`The graph id uses ${Qa}; name the file <graph-id>.json.`);return t}function Il(e,t){let n=new Map,r=new Map;for(let t of e){let e=t.name.trim();e!==``&&r.set(e,(r.get(e)??0)+1)}for(let i of e){let e=i.name.trim(),a=i.value.trim();e===``&&a===``||(e===``?n.set(i.key,`A value needs a field name.`):e===`set`?n.set(i.key,`"set" is written from the set name.`):kl.has(e)?n.set(i.key,`"${e}" is written by the packager.`):(r.get(e)??0)>1?n.set(i.key,`Duplicate field "${e}".`):e===`graph_id`&&a!==``&&!t.has(a)&&n.set(i.key,`"${a}" is not a graph of the set.`))}return n}function Ll(e,t,n){let r={set:e.trim()};for(let e of t){let t=e.name.trim(),n=e.value.trim();t!==``&&n!==``&&(r[t]=n)}let i={};for(let e of n)i[e.id]=e.model;return{manifest:r,graphs:i}}function Y(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function X(e){if(!Y(e)||!Y(e.manifest)||!Y(e.graphs))throw Error(`The answer is not a graph set: expected 'manifest' and 'graphs'.`);let t={};for(let[n,r]of Object.entries(e.manifest)){if(typeof r!=`string`)throw Error(`Manifest field '${n}' is not text.`);t[n]=r}let n=[];for(let[t,r]of Object.entries(e.graphs)){let e=io(r);if(!e.ok)throw Error(`Graph '${t}': ${e.error}`);n.push({id:t,model:e.model,name:e.name,nodeCount:e.nodeCount,connectionCount:e.connectionCount})}return{manifest:t,graphs:n}}function Rl(e,t){return Object.entries(e).filter(([e])=>e!==`set`&&!kl.has(e)).map(([e,n])=>({key:t(),name:e,value:n}))}var zl=`📦 Graph set packaging`;function Bl(e,t){return`${e} ${t}${e===1?``:`s`}`}function Vl({controller:e,supportsFolderPicker:t}){let{mode:n,busy:r,close:i,leaveInspect:a,entries:o,entryIssues:s,addFiles:c,fileErrors:l,canAddCurrentGraph:u,addCurrentGraph:d,removeEntry:f,clearEntries:p,setName:m,setSetName:h,setNameError:g,rows:_,rowIssues:v,updateRow:y,addRow:b,removeRow:x,canPack:S,pack:C,isPacking:w,packError:T,inspected:E,isReading:D,importInspectedGraph:O,editInspectedSet:k}=e,[A,ee]=(0,F.useState)(!1),[j,te]=(0,F.useState)(!1),ne=(0,F.useRef)(null),M=(0,F.useRef)(null),N=(0,F.useId)(),P=(0,F.useId)(),re=(0,F.useRef)(()=>void 0);re.current=()=>{r||(n===`inspect`?a():i())},(0,F.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),re.current())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[]),(0,F.useEffect)(()=>{n===`assemble`&&ne.current?.focus()},[n]);let ie=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),A||ee(!0)},[A]),ae=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&ee(!1)},[]),I=(0,F.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),ee(!1),!r&&c(e.dataTransfer.files)},[r,c]),L=(0,F.useCallback)(e=>{let t=e.target.files;t&&t.length>0&&c(t),e.target.value=``},[c]),oe=m.trim(),se=j&&g!==null,ce=Nl(oe||`set-name`),le=(0,R.jsxs)(`div`,{className:`${J.dropZone} ${A?J.dropZoneActive:``}`,onDragOver:ie,onDragLeave:ae,onDrop:I,"aria-label":`Drop graph files or a graph set here`,children:[(0,R.jsx)(`span`,{className:J.dropZoneIcon,children:`📂`}),(0,R.jsxs)(`span`,{className:J.dropZoneText,children:[`Drop `,(0,R.jsx)(`code`,{children:`.json`}),` graph files here, or a `,(0,R.jsx)(`code`,{children:El}),` to inspect`]}),(0,R.jsx)(`span`,{className:J.dropZoneOr,children:`— or —`}),(0,R.jsx)(`input`,{ref:M,type:`file`,multiple:!0,accept:`.json,${El},application/json,application/octet-stream`,className:J.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:L}),(0,R.jsx)(`button`,{type:`button`,className:J.browseButton,onClick:()=>M.current?.click(),disabled:r,"aria-label":`Browse for graph files`,children:`Browse files…`})]});if(n===`inspect`&&E){let{fileName:e,set:t}=E,n=Object.entries(t.manifest);return(0,R.jsx)(`div`,{className:J.root,children:(0,R.jsxs)(`section`,{className:J.card,"aria-label":`📦 Graph set`,children:[(0,R.jsxs)(`header`,{className:J.ribbon,children:[(0,R.jsxs)(`div`,{className:J.titleGroup,children:[(0,R.jsx)(`span`,{className:J.title,children:`📦 Graph set`}),(0,R.jsx)(`span`,{className:J.subtitle,children:e})]}),(0,R.jsx)(`button`,{type:`button`,className:J.ribbonClose,onClick:i,"aria-label":`Close graph-set panel`,title:`Close`,disabled:r,children:(0,R.jsx)(al,{className:J.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,R.jsxs)(`div`,{className:J.body,children:[(0,R.jsx)(`p`,{className:J.description,children:`What the engine read from the file: its manifest and its graphs. Import a graph as the session's draft to review it, or edit the whole set and pack it again.`}),(0,R.jsx)(`h3`,{className:J.sectionTitle,children:`Manifest`}),(0,R.jsx)(`dl`,{className:J.manifestList,"aria-label":`Manifest`,children:n.map(([e,t])=>(0,R.jsxs)(`div`,{className:J.manifestRow,children:[(0,R.jsx)(`dt`,{className:J.manifestKey,children:e}),(0,R.jsx)(`dd`,{className:J.manifestValue,children:t})]},e))}),(0,R.jsxs)(`h3`,{className:J.sectionTitle,children:[`Graphs (`,t.graphs.length,`)`]}),(0,R.jsx)(`ul`,{className:J.entryList,"aria-label":`Graphs in the set`,children:t.graphs.map(e=>(0,R.jsxs)(`li`,{className:J.entry,children:[(0,R.jsxs)(`div`,{className:J.entryMain,children:[(0,R.jsx)(`code`,{className:J.entryId,children:e.id}),(0,R.jsxs)(`span`,{className:J.entryCounts,children:[Bl(e.nodeCount,`node`),` · `,Bl(e.connectionCount,`connection`)]})]}),(0,R.jsx)(`button`,{type:`button`,className:J.entryAction,onClick:()=>{O(e.id)},title:`Replace the session's draft with this graph (the UI asks first when a graph is loaded)`,children:`Import as draft`})]},e.id))}),le,l.length>0&&(0,R.jsx)(`ul`,{className:J.fileErrors,role:`alert`,children:l.map(e=>(0,R.jsxs)(`li`,{children:[`⚠️ `,e]},e))})]}),(0,R.jsxs)(`footer`,{className:J.footer,children:[(0,R.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:a,disabled:r,children:`Back`}),(0,R.jsx)(`div`,{className:J.footerActions,children:(0,R.jsx)(`button`,{type:`button`,className:J.primaryButton,onClick:k,disabled:r,title:`Loads this set's graphs and manifest into the editor, replacing its list`,children:`Edit as new set`})})]})]})})}return(0,R.jsx)(`div`,{className:J.root,children:(0,R.jsxs)(`section`,{className:J.card,"aria-label":zl,children:[(0,R.jsxs)(`header`,{className:J.ribbon,children:[(0,R.jsxs)(`div`,{className:J.titleGroup,children:[(0,R.jsx)(`span`,{className:J.title,children:zl}),(0,R.jsx)(`span`,{className:J.subtitle,children:ce})]}),(0,R.jsx)(`button`,{type:`button`,className:J.ribbonClose,onClick:i,"aria-label":`Close graph-set panel`,title:`Close (Esc)`,disabled:r,children:(0,R.jsx)(al,{className:J.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,R.jsxs)(`div`,{className:J.body,children:[(0,R.jsxs)(`p`,{className:J.description,children:[`Graph models packed together as one set, `,(0,R.jsx)(`code`,{children:ce}),`, checked by the deployment gate as they are packed. A set deploys all or none through `,(0,R.jsx)(`code`,{children:`graphs.yaml`}),`; a set of one graph is how one graph is signed.`]}),le,(0,R.jsxs)(`div`,{className:J.addRow,children:[(0,R.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:d,disabled:!u||r,title:u?`Adds the graph in the Graph view, named after its graph id`:`Load a graph first`,children:`Add current graph`}),o.length>0&&(0,R.jsx)(`button`,{type:`button`,className:J.linkButton,onClick:p,disabled:r,children:`Clear list`})]}),l.length>0&&(0,R.jsx)(`ul`,{className:J.fileErrors,role:`alert`,children:l.map(e=>(0,R.jsxs)(`li`,{children:[`⚠️ `,e]},e))}),(0,R.jsxs)(`h3`,{className:J.sectionTitle,children:[`Graphs (`,o.length,`)`]}),o.length===0?(0,R.jsxs)(`p`,{className:J.empty,children:[`No graphs yet. Drop `,(0,R.jsx)(`code`,{children:`<graph-id>.json`}),` files or add the current graph.`]}):(0,R.jsx)(`ul`,{className:J.entryList,"aria-label":`Graphs in the set`,children:o.map(e=>{let t=s.get(e.key);return(0,R.jsxs)(`li`,{className:`${J.entry}${t?` ${J.entryFlagged}`:``}`,children:[(0,R.jsxs)(`div`,{className:J.entryMain,children:[(0,R.jsx)(`code`,{className:J.entryId,children:e.id||`(no graph id)`}),(0,R.jsxs)(`span`,{className:J.entryCounts,children:[Bl(e.nodeCount,`node`),` · `,Bl(e.connectionCount,`connection`),e.label!==`${e.id}.json`&&(0,R.jsxs)(R.Fragment,{children:[` · `,e.label]})]}),t&&(0,R.jsxs)(`span`,{className:J.entryIssue,role:`alert`,children:[`⚠️ `,t]})]}),(0,R.jsx)(`button`,{type:`button`,className:J.entryRemove,onClick:()=>f(e.key),disabled:r,"aria-label":`Remove ${e.id||e.label} from the set`,title:`Remove from the set`,children:`×`})]},e.key)})}),(0,R.jsx)(`h3`,{className:J.sectionTitle,children:`Manifest`}),(0,R.jsx)(`label`,{className:J.fieldLabel,htmlFor:N,children:`Set name`}),(0,R.jsxs)(`div`,{className:J.inputRow,children:[(0,R.jsx)(`input`,{id:N,ref:ne,className:`${J.input}${se?` ${J.inputInvalid}`:``}`,type:`text`,value:m,onChange:e=>{h(e.target.value),te(!0)},onBlur:()=>te(!0),"aria-describedby":P,"aria-invalid":se,autoComplete:`off`,spellCheck:!1,maxLength:80,disabled:r}),(0,R.jsx)(`span`,{className:J.extension,"aria-hidden":`true`,children:El})]}),(0,R.jsx)(`p`,{id:P,className:`${J.hint}${se?` ${J.hintError}`:``}`,role:se?`alert`:void 0,children:se?g:`Required. It names the set and its file, ${ce}.`}),(0,R.jsx)(`div`,{className:J.fieldLabel,children:`Fields`}),(0,R.jsxs)(`div`,{className:J.rows,role:`group`,"aria-label":`Manifest fields`,children:[_.map(e=>{let t=v.get(e.key);return(0,R.jsxs)(`div`,{className:J.rowBlock,children:[(0,R.jsxs)(`div`,{className:J.row,children:[(0,R.jsx)(`input`,{className:`${J.input} ${J.rowName}${t?` ${J.inputInvalid}`:``}`,type:`text`,value:e.name,onChange:t=>y(e.key,{name:t.target.value}),placeholder:`field`,"aria-label":`Manifest field name`,autoComplete:`off`,spellCheck:!1,disabled:r}),(0,R.jsx)(`input`,{className:`${J.input} ${J.rowValue}`,type:`text`,value:e.value,onChange:t=>y(e.key,{value:t.target.value}),placeholder:`value`,"aria-label":`Value of manifest field ${e.name||`(unnamed)`}`,autoComplete:`off`,disabled:r}),(0,R.jsx)(`button`,{type:`button`,className:J.entryRemove,onClick:()=>x(e.key),disabled:r,"aria-label":`Remove manifest field ${e.name||`(unnamed)`}`,title:`Remove field`,children:`×`})]}),t&&(0,R.jsxs)(`span`,{className:J.entryIssue,role:`alert`,children:[`⚠️ `,t]})]},e.key)}),(0,R.jsx)(`button`,{type:`button`,className:J.linkButton,onClick:b,disabled:r,children:`+ Add field`})]}),(0,R.jsxs)(`p`,{className:J.hint,children:[`Optional text fields that travel in the manifest, such as `,(0,R.jsx)(`code`,{children:`version`}),` and `,(0,R.jsx)(`code`,{children:`description`}),`; a blank value is left out. `,(0,R.jsx)(`code`,{children:Ol}),` names the set's entry-point graph.`]}),T&&(0,R.jsxs)(`div`,{className:J.errorBanner,role:`alert`,children:[`❌ `,T]})]}),(0,R.jsxs)(`footer`,{className:J.footer,children:[(0,R.jsx)(`span`,{className:J.footerNote,children:t?`The next dialog picks the folder.`:`The browser saves it to its download folder.`}),(0,R.jsxs)(`div`,{className:J.footerActions,children:[(0,R.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:i,disabled:r,children:`Cancel`}),(0,R.jsx)(`button`,{type:`button`,className:J.primaryButton,onClick:()=>{te(!0),C()},disabled:!S,"aria-busy":w,title:S?`Packs the set on the engine and saves ${ce}`:`Add a graph and name the set first`,children:w?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:J.spinner,"aria-hidden":`true`}),` Packing…`]}):D?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:J.spinner,"aria-hidden":`true`}),` Reading…`]}):`Pack and download`})]})]})]})})}var Z={root:`_root_7s9c5_8`,card:`_card_7s9c5_18`,ribbon:`_ribbon_7s9c5_31`,titleGroup:`_titleGroup_7s9c5_42`,title:`_title_7s9c5_42`,subtitle:`_subtitle_7s9c5_55`,ribbonClose:`_ribbonClose_7s9c5_62`,ribbonCloseIcon:`_ribbonCloseIcon_7s9c5_91`,tabs:`_tabs_7s9c5_97`,tab:`_tab_7s9c5_97`,tabActive:`_tabActive_7s9c5_131`,tabNode:`_tabNode_7s9c5_138`,dirtyDot:`_dirtyDot_7s9c5_144`,body:`_body_7s9c5_153`,description:`_description_7s9c5_162`,empty:`_empty_7s9c5_163`,hint:`_hint_7s9c5_172`,sectionTitle:`_sectionTitle_7s9c5_173`,issueList:`_issueList_7s9c5_174`,sectionHeader:`_sectionHeader_7s9c5_182`,loading:`_loading_7s9c5_225`,rows:`_rows_7s9c5_235`,rowCard:`_rowCard_7s9c5_241`,rowFlagged:`_rowFlagged_7s9c5_251`,rowLine:`_rowLine_7s9c5_256`,input:`_input_7s9c5_263`,select:`_select_7s9c5_264`,inputInvalid:`_inputInvalid_7s9c5_287`,pathInput:`_pathInput_7s9c5_291`,typeSelect:`_typeSelect_7s9c5_296`,itemSelect:`_itemSelect_7s9c5_301`,descriptionInput:`_descriptionInput_7s9c5_306`,exampleInput:`_exampleInput_7s9c5_311`,requiredLabel:`_requiredLabel_7s9c5_315`,chips:`_chips_7s9c5_331`,chip:`_chip_7s9c5_331`,chipDeclared:`_chipDeclared_7s9c5_353`,chipDiscovered:`_chipDiscovered_7s9c5_358`,chipRun:`_chipRun_7s9c5_362`,chipNew:`_chipNew_7s9c5_367`,chipConstraints:`_chipConstraints_7s9c5_371`,rowIssue:`_rowIssue_7s9c5_378`,removeButton:`_removeButton_7s9c5_383`,linkButton:`_linkButton_7s9c5_412`,errorBanner:`_errorBanner_7s9c5_437`,warningBanner:`_warningBanner_7s9c5_452`,footer:`_footer_7s9c5_462`,footerTools:`_footerTools_7s9c5_473`,footerActions:`_footerActions_7s9c5_480`,secondaryButton:`_secondaryButton_7s9c5_487`,primaryButton:`_primaryButton_7s9c5_488`,spinner:`_spinner_7s9c5_538`,spin:`_spin_7s9c5_538`,spinnerDark:`_spinnerDark_7s9c5_549`};async function Hl(e){let t=await e.text();try{let e=JSON.parse(t);if(typeof e.message==`string`)return e.message}catch{}return t||`HTTP ${e.status}`}var Ul=[``,`string`,`number`,`integer`,`boolean`,`object`,`array`],Wl=[``,`string`,`number`,`integer`,`boolean`],Gl=[``,`string`,`number`,`integer`,`boolean`],Kl=new Set([`type`,`properties`,`required`,`items`,`description`,`example`]),ql={input:`root`,output:`end`},Jl=`schema`;function Yl(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function Xl(e,t){if(!Yl(e)||!Array.isArray(e.paths))throw Error(`The contract view has no '${t}' part.`);let n=[];for(let t of e.paths){if(!Yl(t)||typeof t.path!=`string`)continue;let e=t.origin===`declared`||t.origin===`both`?t.origin:`discovered`;n.push({path:t.path,type:typeof t.type==`string`?t.type:void 0,origin:e,required:t.required===!0,usedBy:Array.isArray(t.usedBy)?t.usedBy.filter(e=>typeof e==`string`):[]})}return{declared:e.declared===!0,schema:Yl(e.schema)?e.schema:void 0,paths:n}}function Zl(e){if(!Yl(e)||typeof e.graph!=`string`||!Yl(e.input)||!Yl(e.output))throw Error(`The answer is not a graph contract: expected 'graph', 'input' and 'output'.`);let t=Array.isArray(e.output.status)?e.output.status.filter(e=>typeof e==`number`):[];return{graph:e.graph,purpose:typeof e.purpose==`string`?e.purpose:void 0,input:{body:Xl(e.input.body,`input.body`),header:Xl(e.input.header,`input.header`)},output:{body:Xl(e.output.body,`output.body`),header:Xl(e.output.header,`output.header`),status:t},issues:Array.isArray(e.issues)?e.issues.filter(e=>typeof e==`string`):[]}}var Ql=/^([A-Za-z0-9_-]+)(\[\])?$/,$l=/^[A-Za-z0-9_-]+$/,eu=`dot-separated names of letters, digits, hyphen or underscore, with [] after an array`,tu=`letters, digits, hyphen or underscore`;function nu(e){let t=e.trim();if(t===``)return null;let n=[];for(let e of t.split(`.`)){let t=e.match(Ql);if(!t)return null;n.push({name:t[1],array:t[2]!==void 0})}return n}function ru(e){return $l.test(e.trim())}function iu(e,t){let n=`${t}.`;return(e.startsWith(n)?e.slice(n.length):e).replace(/\[(\*|\d+)\]/g,`[]`)}function au(e){let t=e.lastIndexOf(`.`);return t<0?null:e.slice(0,t)}function ou(e,t){let n=nu(t);if(!e||!n)return;let r=e;for(let e of n){let t=r?.properties,n=Yl(t)?t[e.name]:void 0;if(!Yl(n))return;r=n,e.array&&(r=Yl(r.items)?r.items:{})}return r}function su(e,t){let n=au(t),r=n===null?e:ou(e,n),i=nu(t);if(!r||!i)return!1;let a=i[i.length-1].name;return Array.isArray(r.required)&&r.required.includes(a)}function cu(e,t){return typeof e==`string`&&t.includes(e)?e:``}function lu(e){let t=e?.items,n=Yl(t)?t.type:void 0;return typeof n==`string`&&Gl.includes(n)?n:``}function uu(e){return e==null?``:typeof e==`string`?e:typeof e==`number`||typeof e==`boolean`?String(e):JSON.stringify(e)}function du(e){let t={};if(!e)return t;for(let[n,r]of Object.entries(e))Kl.has(n)||(t[n]=r);return t}function fu(e,t,n,r,i){let a=n===`header`?Wl:Ul,o=[],s=new Set;for(let n of e.paths){let c=iu(n.path,r);if(s.has(c))continue;s.add(c);let l=c.endsWith(`[]`)?c.slice(0,-2):c,u=ou(t,l),d=ou(e.schema,l),f=cu(n.type??d?.type,a);o.push({key:i(),path:c,type:c.endsWith(`[]`)?`array`:f,itemType:c.endsWith(`[]`)?lu(u)||lu(d):``,required:n.required===!0||su(t,c),description:uu(u?.description),example:uu(u?.example),source:n.origin,usedBy:n.usedBy??[],constraints:du(u)})}return o}var pu=80;function mu(e){return typeof e==`string`?`string`:typeof e==`boolean`?`boolean`:typeof e==`number`?Number.isInteger(e)?`integer`:`number`:``}function hu(e){if(e==null||typeof e==`object`)return``;let t=String(e);return t.length>pu?`${t.slice(0,pu-1)}…`:t}function gu(e,t,n,r,i){if(Yl(t))for(let[a,o]of Object.entries(t)){if(!$l.test(a))continue;let t=e?`${e}.${a}`:a;if(i===`header`){n.push(_u(r(),t,`string`,``,hu(o)));continue}if(Array.isArray(o)){let e=o.find(e=>e!=null),a=Yl(e)?``:mu(e);if(n.push(_u(r(),`${t}[]`,`array`,a,``)),Yl(e)){let e={};for(let t of o)if(Yl(t))for(let[n,r]of Object.entries(t))n in e||(e[n]=r);gu(`${t}[]`,e,n,r,i)}}else Yl(o)?(n.push(_u(r(),t,`object`,``,``)),gu(t,o,n,r,i)):n.push(_u(r(),t,mu(o),``,hu(o)))}}function _u(e,t,n,r,i){return{key:e,path:t,type:n,itemType:r,required:!1,description:``,example:i,source:`run`,usedBy:[],constraints:{}}}function vu(e,t,n){let r=[];return gu(``,e,r,n,t),r}function yu(e,t){let n=0,r=new Map(e.map(e=>[e.path,e])),i=e.map(e=>({...e}));for(let e of t){let t=r.get(e.path);if(!t){i.push(e),n++;continue}let a=i.find(e=>e.key===t.key),o=!1,s=!1;a.type===``&&e.type!==``&&(a.type=e.type,o=!0),a.type===`array`&&a.itemType===``&&e.itemType!==``&&(a.itemType=e.itemType,o=!0),a.example===``&&e.example!==``&&(a.example=e.example,s=!0),(o||s)&&n++,o&&a.source===`discovered`&&(a.source=`run`)}return{rows:i,changed:n}}function bu(e,t){let n=new Map,r=new Map;for(let t of e){let e=t.path.trim();e!==``&&r.set(e,(r.get(e)??0)+1)}for(let i of e){let a=i.path.trim();if(a===``){n.set(i.key,t===`header`?`A header name is required.`:`A path is required.`);continue}if(t===`header`){ru(a)?Wl.includes(i.type)?(r.get(a)??0)>1&&n.set(i.key,`Duplicate header "${a}".`):n.set(i.key,`A header is text: string, number, integer or boolean.`):n.set(i.key,`A header name uses ${tu}.`);continue}nu(a)?(r.get(a)??0)>1?n.set(i.key,`Duplicate path "${a}".`):a.endsWith(`[]`)&&i.type!==`array`?n.set(i.key,`A path ending in [] is an array.`):!a.endsWith(`[]`)&&i.type===`array`?n.set(i.key,`An array path ends in [] - items[] for a list of items.`):i.type!==``&&i.type!==`object`&&i.type!==`array`&&e.some(e=>e!==i&&e.path.trim().startsWith(`${a}.`))?n.set(i.key,`"${a}" has nested paths, so its type is object (or leave it blank).`):i.type===`array`&&i.itemType!==``&&e.some(e=>e!==i&&e.path.trim().startsWith(`${a}.`))&&n.set(i.key,`"${a}" has nested paths, so its items are objects - clear the item type.`):n.set(i.key,`A path uses ${eu}.`)}return n}function xu(e){return{row:null,array:e,children:new Map}}function Su(e,t){let n=e.row,r={},i={},a=[];for(let[n,r]of e.children)i[n]=Su(r,t),r.row?.required&&a.push(n);let o=e.children.size>0;if(e.array)if(r.type=`array`,o){let e={type:`object`,properties:i};a.length>0&&(e.required=a),r.items=e}else n&&n.itemType!==``?r.items={type:n.itemType}:n&&Yl(n.constraints.items)&&(r.items=n.constraints.items);else{let e=n?.type??``;o||e===`object`?(r.type=`object`,o&&(r.properties=i),a.length>0&&(r.required=a)):e!==``&&(r.type=e)}if(n){n.description.trim()!==``&&(r.description=n.description.trim()),n.example.trim()!==``&&(r.example=n.example.trim());for(let[e,t]of Object.entries(n.constraints))e===`items`&&`items`in r||e in r||(r[e]=t)}return r}function Cu(e,t){let n=e.filter(e=>e.path.trim()!==``);if(n.length===0)return null;let r=xu(!1);for(let e of n){let n=t===`header`?[{name:e.path.trim(),array:!1}]:nu(e.path.trim());if(!n)continue;let i=r;for(let e of n){let t=i.children.get(e.name);t?e.array&&(t.array=!0):(t=xu(e.array),i.children.set(e.name,t)),i=t}i.row=e}return Su(r,t)}function wu(e,t){let n={};return e&&(n.body=e),t&&(n.header=t),Object.keys(n).length>0?n:null}var Tu=`'''`;function Eu(e,t,n){if(t!=null){if(Array.isArray(t)){for(let r of t){if(r==null||typeof r==`object`)throw Error(`'${e}' holds a list of objects, which the node grammar cannot carry.`);n.push(nr(`${e}[]`,Du(e,r)))}return}if(Yl(t)){for(let[r,i]of Object.entries(t)){if(!$l.test(r))throw Error(`'${e}.${r}' - a property name uses ${tu}.`);Eu(`${e}.${r}`,i,n)}return}n.push(nr(e,Du(e,t)))}}function Du(e,t){let n=typeof t==`string`?t:String(t);if(n.includes(Tu))throw Error(`'${e}' contains ''' which the node grammar cannot carry.`);return n}function Ou(e,t=Jl){let n=[];return Eu(t,e,n),n}function ku(e){let t=e.trim();return t===`schema`||t.startsWith(`schema.`)||t.startsWith(`schema[`)}function Au(e){let t=e?.properties[Jl];return Yl(t)?t:void 0}function ju(e){return Object.entries(e).map(([e,t])=>{if(Array.isArray(t))return`${e} (${t.length})`;if(Yl(t))return e;let n=String(t);return n.length>16?e:`${e} ${n}`}).join(` · `)}function Mu(e,t){let n=fr(e);if(!n.valid||!n.formState)throw Error(n.message??`The node cannot be edited from the Schema panel.`);let r=n.formState.properties.filter(e=>e.key.trim()!==``&&!ku(e.key)),i=t?Ou(t):[];return _r({...n.formState,properties:[...r,...i]},e.alias)}function Nu(e){return`/api/openapi/session/${e}?view=contract`}function Pu(e){return`/api/openapi/session/${e}`}function Fu(e,t){return`/api/inspect/${e}/${t}`}var Iu=1e4,Lu=[`input.body`,`input.header`,`output.body`,`output.header`];function Ru(e,t){return`${e}.${t}`}var zu={"input.body":[],"input.header":[],"output.body":[],"output.header":[]},Bu={input:!1,output:!1};function Vu(e,t){let n=(e.headers.get(`Content-Disposition`)??``).match(/filename="?([^";]+)"?/);return n?n[1]:t}function Hu({addToast:e,sessionId:t,connected:n,graphData:r,graphName:i,bus:a,sendRawText:o}){let s=(0,F.useRef)(1),c=(0,F.useCallback)(()=>s.current++,[]),[l,u]=(0,F.useState)(!1),[d,f]=(0,F.useState)(`input`),[p,m]=(0,F.useState)(null),[h,g]=(0,F.useState)(!1),[_,v]=(0,F.useState)(null),[y,b]=(0,F.useState)(zu),[x,S]=(0,F.useState)(Bu),[C,w]=(0,F.useState)(!1),[T,E]=(0,F.useState)(!1),[D,O]=(0,F.useState)(null),[k,A]=(0,F.useState)(!1),ee=(0,F.useRef)(r);ee.current=r;let j=(0,F.useRef)(y);j.current=y;let te=(0,F.useRef)(x);te.current=x;let ne=(0,F.useRef)(l);ne.current=l;let M=(0,F.useRef)(null),N=(0,F.useRef)(null),P=(0,F.useCallback)(e=>r?.nodes.find(t=>t.alias===ql[e])??null,[r]),re=(0,F.useCallback)(()=>{M.current&&=(clearTimeout(M.current.timer),null),E(!1)},[]),ie=(0,F.useCallback)(async(e=!1)=>{if(t===null){v(`The session id is not known yet.`);return}N.current?.abort();let n=new AbortController;N.current=n,g(!0),v(null);try{let r=await fetch(Nu(t),{headers:{Accept:`application/json`},signal:n.signal});if(!r.ok){v(await Hl(r));return}let i=Zl(await r.json());if(!e&&(te.current.input||te.current.output))return;let a=ee.current,o=Au(a?.nodes.find(e=>e.alias===ql.input)),s=Au(a?.nodes.find(e=>e.alias===ql.output)),l=(e,t)=>e&&typeof e[t]==`object`&&e[t]!==null?e[t]:void 0;m(i),b({"input.body":fu(i.input.body,l(o,`body`),`body`,`input.body`,c),"input.header":fu(i.input.header,l(o,`header`),`header`,`input.header`,c),"output.body":fu(i.output.body,l(s,`body`),`body`,`output.body`,c),"output.header":fu(i.output.header,l(s,`header`),`header`,`output.header`,c)}),S(Bu),O(null)}catch(e){if(e.name===`AbortError`)return;v(`The contract could not be read: ${e.message}`)}finally{N.current===n&&(N.current=null),g(!1)}},[t,c]),ae=(0,F.useRef)(ie);ae.current=ie,(0,F.useEffect)(()=>()=>{N.current?.abort()},[]),(0,F.useEffect)(()=>{m(null),b(zu),S(Bu),v(null),re()},[t,re]);let I=(0,F.useCallback)((e=`input`)=>{f(e),u(!0);let t=te.current;!t.input&&!t.output&&ae.current()},[]),L=(0,F.useCallback)(()=>{N.current?.abort(),N.current=null,g(!1),u(!1)},[]),oe=(0,F.useCallback)(()=>{ae.current(!0)},[]);(0,F.useEffect)(()=>a.on(`graph.mutation`,()=>{if(!ne.current)return;let e=te.current;e.input||e.output||M.current||ae.current()}),[a]);let se=(0,F.useCallback)(e=>{let t=e.startsWith(`input`)?`input`:`output`;S(e=>e[t]?e:{...e,[t]:!0}),O(null)},[]),ce=(0,F.useCallback)((e,t,n)=>{b(r=>({...r,[e]:r[e].map(e=>e.key===t?{...e,...n}:e)})),se(e)},[se]),le=(0,F.useCallback)(e=>{let t=e.endsWith(`header`)?`string`:``;b(n=>({...n,[e]:[...n[e],{key:c(),path:``,type:t,itemType:``,required:!1,description:``,example:``,source:`new`,usedBy:[],constraints:{}}]})),se(e)},[c,se]),ue=(0,F.useCallback)((e,t)=>{b(n=>({...n,[e]:n[e].filter(e=>e.key!==t)})),se(e)},[se]),de=(0,F.useMemo)(()=>({"input.body":bu(y[`input.body`],`body`),"input.header":bu(y[`input.header`],`header`),"output.body":bu(y[`output.body`],`body`),"output.header":bu(y[`output.header`],`header`)}),[y]),fe=(0,F.useCallback)(e=>(p?.issues??[]).filter(t=>t.startsWith(`${e}.`)),[p]),R=(0,F.useCallback)(async()=>{if(t===null){e(`The session id is not known yet.`,`error`);return}w(!0);try{let n=await Promise.all(Lu.map(async e=>{let n=await fetch(Fu(t,e),{headers:{Accept:`application/json`}});if(n.status===404)return null;if(!n.ok)throw Error(await Hl(n));return(await n.json()).outcome??null}));if(n.every(e=>e===null)){e(`No instance data yet - Instantiate, Upload and Run first.`,`info`);return}let r=0,i={input:!1,output:!1},a={...j.current};Lu.forEach((e,t)=>{let o=n[t];if(o===null)return;let s=e.endsWith(`header`)?`header`:`body`,l=yu(a[e],vu(o,s,c));l.changed>0&&(a[e]=l.rows,r+=l.changed,i[e.startsWith(`input`)?`input`:`output`]=!0)}),r===0?e(`The last run adds nothing: every row already has its type and example.`,`info`):(b(a),S(e=>({input:e.input||i.input,output:e.output||i.output})),O(null),e(`${r} row${r===1?``:`s`} filled from the last run - review and Save.`,`success`))}catch(t){e(`Fill from last run failed: ${t.message}`,`error`)}finally{w(!1)}},[t,e,c]),pe=de[Ru(d,`body`)].size+de[Ru(d,`header`)].size,me=n&&x[d]&&pe===0&&!T&&!h&&P(d)!==null,he=(0,F.useCallback)(()=>{if(!me)return;let e=P(d);if(!e)return;let t;try{t=Mu(e,wu(Cu(y[Ru(d,`body`)],`body`),Cu(y[Ru(d,`header`)],`header`)))}catch(e){O(e.message);return}if(O(null),!o(t)){O(`The command was not sent because the WebSocket is not open. Your rows are kept.`);return}E(!0),M.current={side:d,alias:e.alias,timer:setTimeout(()=>{M.current=null,E(!1),O(`The engine did not answer within ${Iu/1e3} s - check the console.`)},Iu)}},[me,P,d,y,o]);(0,F.useEffect)(()=>a.on(`minigraph.nodeAction.textResult`,t=>{let n=M.current;if(!n)return;let r=t.action===`edit-node`&&t.alias===n.alias,i=t.status===`error`&&t.action===null;!r&&!i||(re(),t.status===`accepted`?(S(e=>({...e,[n.side]:!1})),e(`Schema of node ${n.alias} saved`,`success`),ae.current()):O(t.message))}),[a,e,re]),(0,F.useEffect)(()=>()=>{M.current&&clearTimeout(M.current.timer)},[]);let ge=(0,F.useCallback)(async()=>{if(t===null){e(`The session id is not known yet.`,`error`);return}A(!0);try{let n=await fetch(Pu(t),{headers:{Accept:`application/yaml`}});if(!n.ok){e(`The document could not be read - ${await Hl(n)}`,`error`);return}let r=await fo(await n.text(),Vu(n,`${eo(i)}.yaml`));r.saved&&e(`OpenAPI document saved as ${r.fileName}`,`success`)}catch(t){e(`Download failed: ${t.message}`,`error`)}finally{A(!1)}},[t,i,e]);return{isOpen:l,open:I,close:L,side:d,setSide:f,busy:T||C||k,contract:p,isLoading:h,loadError:_,reload:oe,rows:y,issues:de,dirty:x,updateRow:ce,addRow:le,removeRow:ue,nodeOf:P,issuesOf:fe,fillFromRun:R,isFilling:C,save:he,canSave:me,isSaving:T,saveError:D,download:ge,isDownloading:k}}var Uu=`📐 Graph schema`,Wu={input:`Input`,output:`Output`},Gu={input:`request`,output:`response`},Ku={declared:`declared`,discovered:`discovered`,both:`declared`,run:`from last run`,new:`new`},qu={declared:Z.chipDeclared,discovered:Z.chipDiscovered,both:Z.chipDeclared,run:Z.chipRun,new:Z.chipNew};function Ju(e){let t=e.usedBy.length>0?`Referenced by ${e.usedBy.join(`, `)}`:`Not referenced by the model`;switch(e.source){case`declared`:return`Declared on the node. ${t}.`;case`both`:return`Declared on the node and discovered in the model. ${t}.`;case`discovered`:return`Discovered in the model, not declared yet. ${t}.`;case`run`:return`Typed from the instance's last run.`;default:return`Added here; saved with the declaration.`}}function Yu(e){return e===``?`(any)`:e}function Xu({controller:e,graphName:t,supportsFolderPicker:n}){let{side:r,setSide:i,busy:a,close:o,contract:s,isLoading:c,loadError:l,reload:u,rows:d,issues:f,dirty:p,updateRow:m,addRow:h,removeRow:g,nodeOf:_,issuesOf:v,fillFromRun:y,isFilling:b,save:x,canSave:S,isSaving:C,saveError:w,download:T,isDownloading:E}=e,D=(0,F.useRef)(()=>void 0);D.current=()=>{a||o()},(0,F.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),D.current())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[]);let O=_(r),k=ql[r],A=v(r),ee=t?.trim()||s?.graph||`draft`,j=e=>{let t=Ru(r,e),n=d[t],i=f[t],o=e===`header`?Wl:Ul,s=`${r}.${e}`,c=e===`header`?`header`:`path`;return(0,R.jsxs)(R.Fragment,{children:[(0,R.jsxs)(`div`,{className:Z.sectionHeader,children:[(0,R.jsxs)(`h3`,{className:Z.sectionTitle,children:[e===`header`?`Headers`:`Body`,` `,(0,R.jsx)(`code`,{children:s})]}),(0,R.jsxs)(`span`,{className:Z.hint,children:[n.length,` `,c,n.length===1?``:`s`]})]}),n.length===0?(0,R.jsx)(`p`,{className:Z.empty,children:e===`header`?(0,R.jsxs)(R.Fragment,{children:[`No header. Add one when the `,Gu[r],` carries a header the graph `,r===`input`?`reads`:`sets`,`.`]}):(0,R.jsx)(R.Fragment,{children:`No path. Add one, or Fill from last run after a dry run.`})}):(0,R.jsx)(`div`,{className:Z.rows,role:`group`,"aria-label":`${s} rows`,children:n.map(n=>{let s=i.get(n.key),l=n.path.trim()||`(new ${c})`,u=ju(n.constraints);return(0,R.jsxs)(`div`,{className:`${Z.rowCard}${s?` ${Z.rowFlagged}`:``}`,children:[(0,R.jsxs)(`div`,{className:Z.rowLine,children:[(0,R.jsx)(`input`,{className:`${Z.input} ${Z.pathInput}${s?` ${Z.inputInvalid}`:``}`,type:`text`,value:n.path,onChange:e=>m(t,n.key,{path:e.target.value}),placeholder:e===`header`?`X-Header-Name`:`path, items[] for a list`,"aria-label":e===`header`?`Header name`:`Path`,"aria-invalid":!!s,autoComplete:`off`,spellCheck:!1,disabled:a}),(0,R.jsx)(`select`,{className:`${Z.select} ${Z.typeSelect}`,value:n.type,onChange:e=>m(t,n.key,{type:e.target.value}),"aria-label":`Type of ${l}`,disabled:a,children:o.map(e=>(0,R.jsx)(`option`,{value:e,children:Yu(e)},e))}),n.type===`array`&&(0,R.jsx)(`select`,{className:`${Z.select} ${Z.itemSelect}`,value:n.itemType,onChange:e=>m(t,n.key,{itemType:e.target.value}),"aria-label":`Item type of ${l}`,title:`The element type of the list; leave it blank for objects described by nested paths`,disabled:a,children:Gl.map(e=>(0,R.jsx)(`option`,{value:e,children:e===``?`of …`:`of ${e}`},e))}),(0,R.jsxs)(`label`,{className:Z.requiredLabel,title:r===`input`?`A missing value fails validation`:`Always present in the response`,children:[(0,R.jsx)(`input`,{type:`checkbox`,checked:n.required,onChange:e=>m(t,n.key,{required:e.target.checked}),"aria-label":`Required ${l}`,disabled:a}),`req`]}),(0,R.jsx)(`button`,{type:`button`,className:Z.removeButton,onClick:()=>g(t,n.key),disabled:a,"aria-label":`Remove ${l}`,title:`Remove from the declaration`,children:`×`})]}),(0,R.jsxs)(`div`,{className:Z.rowLine,children:[(0,R.jsx)(`input`,{className:`${Z.input} ${Z.descriptionInput}`,type:`text`,value:n.description,onChange:e=>m(t,n.key,{description:e.target.value}),placeholder:`description`,"aria-label":`Description of ${l}`,autoComplete:`off`,disabled:a}),(0,R.jsx)(`input`,{className:`${Z.input} ${Z.exampleInput}`,type:`text`,value:n.example,onChange:e=>m(t,n.key,{example:e.target.value}),placeholder:`example`,"aria-label":`Example of ${l}`,autoComplete:`off`,spellCheck:!1,disabled:a})]}),(0,R.jsxs)(`div`,{className:Z.chips,children:[(0,R.jsx)(`span`,{className:`${Z.chip} ${qu[n.source]}`,title:Ju(n),children:Ku[n.source]}),n.usedBy.length>0&&(0,R.jsxs)(`span`,{className:Z.chip,title:`Referenced by ${n.usedBy.join(`, `)}`,children:[`used by `,n.usedBy.length===1?n.usedBy[0]:`${n.usedBy.length} nodes`]}),u!==``&&(0,R.jsx)(`span`,{className:`${Z.chip} ${Z.chipConstraints}`,title:`Carried unchanged - edit in the node editor:\n${JSON.stringify(n.constraints,null,1)}`,children:u}),s&&(0,R.jsxs)(`span`,{className:Z.rowIssue,role:`alert`,children:[`⚠️ `,s]})]})]},n.key)})}),(0,R.jsxs)(`button`,{type:`button`,className:Z.linkButton,onClick:()=>h(t),disabled:a,children:[`+ Add `,c]})]})};return(0,R.jsx)(`div`,{className:Z.root,children:(0,R.jsxs)(`section`,{className:Z.card,"aria-label":Uu,children:[(0,R.jsxs)(`header`,{className:Z.ribbon,children:[(0,R.jsxs)(`div`,{className:Z.titleGroup,children:[(0,R.jsx)(`span`,{className:Z.title,children:Uu}),(0,R.jsxs)(`span`,{className:Z.subtitle,children:[ee,` · node `,k]})]}),(0,R.jsx)(`button`,{type:`button`,className:Z.ribbonClose,onClick:o,"aria-label":`Close schema panel`,title:`Close (Esc)`,disabled:a,children:(0,R.jsx)(al,{className:Z.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,R.jsx)(`div`,{className:Z.tabs,role:`tablist`,"aria-label":`Contract side`,children:[`input`,`output`].map(e=>(0,R.jsxs)(`button`,{type:`button`,role:`tab`,className:`${Z.tab}${r===e?` ${Z.tabActive}`:``}`,"aria-selected":r===e,onClick:()=>i(e),title:`The ${Gu[e]}: schema on node ${ql[e]}`,children:[Wu[e],` `,(0,R.jsx)(`span`,{className:Z.tabNode,children:ql[e]}),p[e]&&(0,R.jsx)(`span`,{className:Z.dirtyDot,"aria-label":`unsaved`,title:`Unsaved changes`})]},e))}),(0,R.jsxs)(`div`,{className:Z.body,role:`tabpanel`,"aria-label":`${Wu[r]} contract`,children:[(0,R.jsx)(`p`,{className:Z.description,children:r===`input`?(0,R.jsxs)(R.Fragment,{children:[`The request contract, `,(0,R.jsx)(`code`,{children:`schema`}),` on the root node: `,(0,R.jsx)(`code`,{children:`input.body`}),` is validated against it on every run, before anything else - a bad request never reaches the graph. Headers are matched by name, case-insensitively.`]}):(0,R.jsxs)(R.Fragment,{children:[`The response contract, `,(0,R.jsx)(`code`,{children:`schema`}),` on the end node: documentary - it describes `,(0,R.jsx)(`code`,{children:`output.body`}),` and `,(0,R.jsx)(`code`,{children:`output.header`}),` in the OpenAPI document and is never enforced.`]})}),l&&(0,R.jsxs)(`div`,{className:Z.errorBanner,role:`alert`,children:[(0,R.jsxs)(`span`,{children:[`❌ `,l]}),(0,R.jsx)(`button`,{type:`button`,className:Z.linkButton,onClick:u,disabled:c,children:`Retry`})]}),c&&(0,R.jsxs)(`p`,{className:Z.loading,children:[(0,R.jsx)(`span`,{className:`${Z.spinner} ${Z.spinnerDark}`,"aria-hidden":`true`}),` Reading the contract…`]}),!O&&!c&&(0,R.jsxs)(`div`,{className:Z.warningBanner,role:`status`,children:[`This graph has no `,(0,R.jsx)(`code`,{children:k}),` node; the `,Gu[r],` contract is declared on it. Create the node, then Save.`]}),A.length>0&&(0,R.jsx)(`ul`,{className:Z.issueList,"aria-label":`Declaration versus model`,children:A.map(e=>(0,R.jsxs)(`li`,{children:[`⚠️ `,e]},e))}),j(`body`),j(`header`),(0,R.jsxs)(`p`,{className:Z.hint,children:[`Types are strict JSON types: a numeric string is not a number; money is a `,(0,R.jsx)(`code`,{children:`string`}),` with a `,(0,R.jsx)(`code`,{children:`pattern`}),`. A row's other keywords (`,(0,R.jsx)(`code`,{children:`enum`}),`, `,(0,R.jsx)(`code`,{children:`minimum`}),`, `,(0,R.jsx)(`code`,{children:`pattern`}),`, …) are kept as declared and edited in the node editor.`]}),w&&(0,R.jsx)(`div`,{className:Z.errorBanner,role:`alert`,children:(0,R.jsxs)(`span`,{children:[`❌ `,w]})})]}),(0,R.jsxs)(`footer`,{className:Z.footer,children:[(0,R.jsxs)(`div`,{className:Z.footerTools,children:[(0,R.jsx)(`button`,{type:`button`,className:Z.secondaryButton,onClick:()=>{y()},disabled:a||c,"aria-busy":b,title:`Types and examples from the instance's actual input and output, after a dry run`,children:b?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:`${Z.spinner} ${Z.spinnerDark}`,"aria-hidden":`true`}),` Reading…`]}):`Fill from last run`}),(0,R.jsx)(`button`,{type:`button`,className:Z.secondaryButton,onClick:u,disabled:a||c,title:`Read the graph again, discarding unsaved rows`,children:`Reload`}),(0,R.jsx)(`button`,{type:`button`,className:Z.secondaryButton,onClick:()=>{T()},disabled:a||c,"aria-busy":E,title:`The draft's OpenAPI 3.0 document as YAML - ${n?`the next dialog picks the folder`:`saved to the download folder`}`,children:E?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:`${Z.spinner} ${Z.spinnerDark}`,"aria-hidden":`true`}),` Fetching…`]}):`Download YAML`})]}),(0,R.jsxs)(`div`,{className:Z.footerActions,children:[(0,R.jsx)(`button`,{type:`button`,className:Z.secondaryButton,onClick:o,disabled:a,children:`Cancel`}),(0,R.jsx)(`button`,{type:`button`,className:Z.primaryButton,onClick:x,disabled:!S,"aria-busy":C,title:S?`Writes the declaration to node ${k} through update node - every member sees it`:p[r]?`Fix the flagged rows first`:`Nothing changed`,children:C?(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(`span`,{className:Z.spinner,"aria-hidden":`true`}),` Saving…`]}):`Save ${k} schema`})]})]})]})})}var Zu={popover:`_popover_qw337_7`,header:`_header_qw337_19`,endpoint:`_endpoint_qw337_30`,arrow:`_arrow_qw337_41`,chips:`_chips_qw337_46`,chip:`_chip_qw337_46`,customRow:`_customRow_qw337_73`,input:`_input_qw337_78`,submitButton:`_submitButton_qw337_100`,errorText:`_errorText_qw337_122`,statusText:`_statusText_qw337_128`,warningText:`_warningText_qw337_129`},Qu=12;function $u({formState:e,phase:t,lockReason:n,serverMessage:r,validationErrors:i,anchor:a,onFormStateChange:o,onSubmit:s,onClose:c}){let l=(0,F.useRef)(null),u=(0,F.useRef)(null),[d,f]=(0,F.useState)(null),p=t===`sending`,m=n===`disconnected`,h=p||m,g=(0,F.useRef)(null);(0,F.useEffect)(()=>{g.current!==null&&e.relation===g.current&&(g.current=null,h||s())},[h,e.relation,s]),(0,F.useEffect)(()=>{u.current?.focus()},[]),(0,F.useLayoutEffect)(()=>{let e={x:window.innerWidth/2,y:96},t=a??e,n=l.current?.getBoundingClientRect(),r=n?.width??280,i=n?.height??180;f({left:Math.min(Math.max(t.x-r/2,Qu),Math.max(Qu,window.innerWidth-r-Qu)),top:Math.min(Math.max(t.y+14,Qu),Math.max(Qu,window.innerHeight-i-Qu))})},[a]),(0,F.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),p||c())},t=e=>{p||l.current&&!l.current.contains(e.target)&&c()};return document.addEventListener(`keydown`,e),document.addEventListener(`pointerdown`,t),()=>{document.removeEventListener(`keydown`,e),document.removeEventListener(`pointerdown`,t)}},[c,p]);let _=(0,F.useCallback)(t=>{o({...e,relation:t})},[e,o]),v=(0,F.useCallback)(t=>{h||(g.current=t,o({...e,relation:t}))},[h,e,o]),y=(0,F.useCallback)(e=>{e.preventDefault(),!h&&s()},[h,s]),b=i.relation??i.command??i.sourceAlias??i.targetAlias;return(0,R.jsxs)(`div`,{ref:l,className:Zu.popover,style:d?{left:d.left,top:d.top}:{visibility:`hidden`},role:`dialog`,"aria-label":`Create connection from ${e.sourceAlias} to ${e.targetAlias}`,children:[(0,R.jsxs)(`div`,{className:Zu.header,children:[(0,R.jsx)(`span`,{className:Zu.endpoint,children:e.sourceAlias}),(0,R.jsx)(`span`,{className:Zu.arrow,"aria-hidden":`true`,children:`→`}),(0,R.jsx)(`span`,{className:Zu.endpoint,children:e.targetAlias})]}),(0,R.jsx)(`div`,{className:Zu.chips,children:wi.map(e=>(0,R.jsx)(`button`,{type:`button`,className:Zu.chip,style:{"--chip-color":Ti[e]},disabled:h,onClick:()=>v(e),children:e},e))}),(0,R.jsxs)(`form`,{className:Zu.customRow,onSubmit:y,children:[(0,R.jsx)(`input`,{ref:u,className:Zu.input,value:e.relation,placeholder:`custom relation…`,autoComplete:`off`,autoCorrect:`off`,spellCheck:!1,disabled:h,"aria-label":`Relation name`,"aria-invalid":!!i.relation,onChange:e=>_(e.target.value)}),(0,R.jsx)(`button`,{type:`submit`,className:Zu.submitButton,disabled:h||!e.relation.trim(),children:p?`Creating…`:`Connect`})]}),b&&!p&&(0,R.jsx)(`div`,{className:Zu.errorText,role:`alert`,children:b}),r&&(0,R.jsx)(`div`,{className:m?Zu.warningText:Zu.statusText,role:`status`,children:r})]})}var ed=1e4,td=`A graph authoring action is already pending. Wait for it to finish before starting another.`,nd=`Could not send the create-node command because the WebSocket is not open. The form values remain in this dialog.`,rd=`Could not send the edit-node command because the WebSocket is not open. Your changes remain in this dialog.`,id=`Could not send the delete-node command because the WebSocket is not open.`,ad=`Could not send the create-connection command because the WebSocket is not open. The form values remain in this dialog.`,od=`Could not send delete-node commands because the WebSocket is not open.`,sd=`No selected nodes are available to delete.`,cd=`Select 100 or fewer nodes to delete at once.`,ld=`Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.`,ud=`This node is no longer available in the current graph.`,dd=`Connection disconnected. Refresh the page and create the node again after the app reconnects.`,fd=`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,pd=`Connection disconnected. Refresh the page and create the connection again after the app reconnects.`,md=`Connection disconnected while the graph authoring action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.`,hd={status:`closed`,pendingSubmit:null,serverMessage:null};function gd(e){return e.pendingSubmit}function _d(e){return e.action===`delete-nodes`}function vd(e){return e===`create-connection`?ad:e===`edit-node`?rd:e===`delete-node`?id:nd}function yd(e){return _d(e)?ld:`The ${e.action} command was sent, but no backend result was observed yet. The outcome is unknown.`}function Q(e){return e===`create-connection`?pd:e===`edit-node`?fd:dd}function bd(e,t){return!e||!t?!1:e.trim().toLowerCase()===t.trim().toLowerCase()}function xd(e,t){return e?.nodes.find(e=>e.alias.toLowerCase()===t.toLowerCase())??null}function Sd(e,t){return e.status===`error`?!0:t.action===`create-connection`?e.action===`create-connection`?e.alias===null?!0:bd(e.alias,t.alias)?e.status===`accepted`?bd(e.targetAlias,t.targetAlias):e.targetAlias===null||bd(e.targetAlias,t.targetAlias):!1:e.action===null?bd(e.alias,t.alias)||bd(e.alias,t.targetAlias):!1:bd(e.alias,t.alias)?e.action===null||e.action===t.action:!1}function Cd(e,t){let n=gd(e);return!n||_d(n)||!Sd(t,n)?null:t.status===`accepted`?{state:hd,acceptedResult:{status:t.status,action:t.action,alias:t.alias,targetAlias:t.targetAlias,message:t.message}}:e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:t.status===`error`?`Backend returned an error while this submit was pending: ${t.message}`:t.message}}:{state:hd,notification:{message:t.message,type:`error`}}}function wd(e){return{...e,phase:`editing`,pendingSubmit:null,serverMessage:vd(e.action)}}function Td(e){let t=gd(e);if(!t)return null;let n=yd(t);return e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n}}:{state:hd,notification:{message:n,type:`error`}}}function Ed(e){let t=gd(e);if(e.status===`open`){let n=t?md:Q(e.action);return{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n,connectionLost:!0}}}return t?{state:hd,notification:{message:md,type:`error`}}:null}function Dd(e){return`sourceAlias`in e}function Od({bus:e,connected:t,graphData:n,executor:r,timeoutMs:i=ed,onAccepted:a,onUserMessage:o}){let[s,c]=(0,F.useState)(hd),[l,u]=(0,F.useState)({}),d=(0,F.useRef)(s),f=(0,F.useRef)(null),p=(0,F.useRef)(t),m=(0,F.useRef)(n),h=(0,F.useRef)(a),g=(0,F.useRef)(o);(0,F.useEffect)(()=>{d.current=s},[s]),(0,F.useEffect)(()=>{m.current=n},[n]),(0,F.useEffect)(()=>{h.current=a},[a]),(0,F.useEffect)(()=>{g.current=o},[o]);let _=(0,F.useCallback)((e,t=`error`)=>{g.current?.(e,t)},[]),v=(0,F.useCallback)(e=>{d.current=e,c(e)},[]),y=(0,F.useCallback)(()=>{f.current!==null&&(clearTimeout(f.current),f.current=null)},[]),b=(0,F.useCallback)(()=>{y(),f.current=setTimeout(()=>{let e=Td(d.current);e&&(v(e.state),e.notification&&_(e.notification.message,e.notification.type)),f.current=null},i)},[y,_,v,i]),x=(0,F.useCallback)(e=>{if(!t)return;if(gd(d.current)){_(td,`error`);return}let n=rr(e);u({}),v({status:`open`,action:`create-node`,phase:`editing`,formState:n,originalAlias:null,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),S=(0,F.useCallback)((e,n)=>{if(!t)return;if(gd(d.current)){_(td,`error`);return}let r={sourceAlias:e,targetAlias:n,relation:``},{relation:i,...a}=$n(r,{graphData:m.current,connected:t}).errors;Object.keys(a).length>0||(u({}),v({status:`open`,action:`create-connection`,phase:`editing`,formState:r,pendingSubmit:null,serverMessage:null,connectionLost:!1}))},[t,_,v]),C=(0,F.useCallback)(e=>{if(!t){_(fd,`error`);return}if(gd(d.current)){_(td,`error`);return}let n=xd(m.current,e.alias);if(!n){_(ud,`error`);return}let r=fr(n);if(!r.valid||!r.formState){_(r.message??`This node cannot be edited in the UI.`,`error`);return}u({}),v({status:`open`,action:`edit-node`,phase:`editing`,formState:r.formState,originalAlias:n.alias,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),w=(0,F.useCallback)(e=>{if(!t){_(id,`error`);return}if(gd(d.current)){_(td,`error`);return}let n=Xn(e.alias,{graphData:m.current});if(!n.valid){_(Object.values(n.errors)[0]??`Invalid node alias.`,`error`);return}let i;try{i=vr(e.alias,{graphData:m.current})}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}if(!r.execute(i)){_(id,`error`);return}let a={action:`delete-node`,alias:e.alias.trim(),command:i,sentAt:new Date().toISOString()};u({}),v({status:`closed`,pendingSubmit:a,serverMessage:null}),b()},[t,r,_,v,b]),T=(0,F.useCallback)(e=>{if(!t){_(od,`error`);return}if(gd(d.current)){_(td,`error`);return}if(e.length===0){_(sd,`info`);return}if(e.length>100){_(cd,`error`);return}let n=new Set,i=e.filter(e=>{let t=e.alias.trim().toLowerCase();return n.has(t)?!1:(n.add(t),!0)}),a=[],o=[];for(let e of i){let t=Xn(e.alias,{graphData:m.current});if(!t.valid){_(Object.values(t.errors)[0]??sd,`error`);return}try{o.push(e.alias.trim()),a.push(vr(e.alias,{graphData:m.current}))}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}}for(let[e,t]of a.entries())if(!r.execute(t)){_(e===0?od:md,`error`);return}let s={action:`delete-nodes`,aliases:o,commands:a,sentAt:new Date().toISOString(),results:{}};u({}),v({status:`closed`,pendingSubmit:s,serverMessage:null}),_(`${o.length} delete-node commands sent. Waiting for backend response.`,`info`),b()},[t,r,_,v,b]),E=(0,F.useCallback)(e=>{let t=d.current;if(t.status===`open`&&!(t.phase===`sending`||t.connectionLost)){if(u({}),t.action===`create-connection`){if(!Dd(e))return;v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1});return}Dd(e)||v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1})}},[v]),D=(0,F.useCallback)(()=>{let e=d.current;if(e.status!==`open`||e.phase===`sending`||e.connectionLost)return;let n=e.action;if(!t){v({...e,serverMessage:vd(n)});return}let i=e.action===`create-connection`?$n(e.formState,{graphData:m.current,connected:t}):Yn(e.formState,e.action===`edit-node`?{mode:`edit`,originalAlias:e.originalAlias}:{graphData:m.current});if(!i.valid){u(i.errors);return}let a,o,s=null;try{if(e.action===`edit-node`)o=e.originalAlias?.trim()??``,a=_r(e.formState,o);else if(e.action===`create-node`)o=e.formState.alias.trim(),a=gr(e.formState);else if(Dd(e.formState))o=e.formState.sourceAlias.trim(),s=e.formState.targetAlias.trim(),a=br(e.formState);else{u({command:`Invalid connection form state.`});return}}catch(e){u({command:e instanceof Error?e.message:String(e)});return}if(!r.execute(a)){v(wd(e));return}let c={action:n,alias:o,targetAlias:s,command:a,sentAt:new Date().toISOString()};u({}),v({...e,phase:`sending`,pendingSubmit:c,serverMessage:null,connectionLost:!1}),b()},[t,r,v,b]),O=(0,F.useCallback)(()=>{let e=d.current;e.status===`open`&&e.phase!==`sending`&&(y(),u({}),v(hd))},[y,v]);return(0,F.useEffect)(()=>e.on(`minigraph.nodeAction.textResult`,e=>{let t=d.current,n=gd(t);if(!n)return;if(_d(n)){let r=Rr(n,e);if(!r)return;if(e.status===`accepted`&&h.current?.({status:e.status,action:e.action,alias:e.alias,targetAlias:e.targetAlias,message:e.message}),!zr(r)){v({...t,pendingSubmit:r});return}y(),v(hd);let i=Br(r);_(i.message,i.type);return}let r=Cd(t,e);r&&(y(),r.acceptedResult&&u({}),v(r.state),r.acceptedResult&&h.current?.(r.acceptedResult),r.notification&&_(r.notification.message,r.notification.type))}),[e,y,_,v]),(0,F.useEffect)(()=>{if(p.current&&!t){let e=Ed(d.current);e&&(y(),v(e.state),e.notification&&_(e.notification.message,e.notification.type))}p.current=t},[y,t,_,v]),(0,F.useEffect)(()=>()=>{y()},[y]),{state:s,validationErrors:l,openCreateNode:x,openCreateConnection:S,openEditNode:C,deleteNode:w,deleteNodes:T,updateFormState:E,submit:D,close:O}}var kd=/^ws-\d+-\d+$/;function Ad(e){return kd.test(e.trim())}var jd={sessionId:null,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null};function Md(e){return Array.from(new Set(e)).sort()}function Nd({enabled:e,connected:t,bus:n,classificationMap:r,sendRawText:i,addToast:a}){let[o,s]=(0,F.useState)(jd),c=(0,F.useRef)(new Set),l=(0,F.useRef)(0),u=(0,F.useRef)(i),d=(0,F.useRef)(a);(0,F.useEffect)(()=>{u.current=i},[i]),(0,F.useEffect)(()=>{d.current=a},[a]);let f=(0,F.useCallback)(()=>{if(!e||!t)return!1;s(e=>({...e,loading:!0,pendingCommand:`refresh`,error:null,lastInfo:null}));let n=u.current(`session`);if(!n){let e=`Could not load session details because the WebSocket is not open.`;s(t=>({...t,loading:!1,pendingCommand:null,error:e})),d.current(e,`error`)}return n},[t,e]),p=(0,F.useCallback)(e=>{let t=`${e.kind}:${e.msgId}`;if(!c.current.has(t)){if(c.current.add(t),e.kind===`minigraph.session.started`){s({...jd,sessionId:e.sessionId});return}if(e.kind===`minigraph.session.status`){s(t=>({...t,sessionId:e.sessionId,startedSince:e.startedSince,subscribedTo:e.subscribedTo,subscribers:Md(e.subscribers),loading:!1,pendingCommand:null,error:null,lastInfo:null}));return}if(e.kind===`minigraph.session.commandResult`){if(e.status===`accepted`){s(t=>({...t,subscribedTo:e.command===`subscribe`?e.sessionId:e.command===`unsubscribe`?null:t.subscribedTo,pendingCommand:null,error:null,lastInfo:null}));return}s(t=>({...t,pendingCommand:null,error:e.message,lastInfo:null}));return}if(e.kind===`minigraph.session.notification`){e.type===`host-closed`?s(t=>({...t,subscribedTo:t.subscribedTo===e.sessionId?null:t.subscribedTo,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null})):e.type===`subscriber-joined`?s(t=>({...t,subscribers:Md([...t.subscribers,e.sessionId]),error:null,lastInfo:null})):s(t=>({...t,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null}));return}e.kind===`session.reset`&&(s(e=>({...e,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null})),f())}},[f]),m=(0,F.useCallback)(()=>{s(e=>({...e,error:null,lastInfo:null}))},[]),h=(0,F.useCallback)(n=>{let r=n.trim();if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null)return!1;if(!Ad(r))return s(e=>({...e,error:`Enter a valid session ID like ws-123456-1.`,lastInfo:null})),!1;s(e=>({...e,pendingCommand:`subscribe`,error:null,lastInfo:null}));let c=i(`session subscribe ${r}`);if(!c){let e=`Could not subscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return c},[a,t,e,i,o.pendingCommand,o.subscribedTo]),g=(0,F.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo===null)return!1;s(e=>({...e,pendingCommand:`unsubscribe`,error:null,lastInfo:null}));let n=i(`session unsubscribe`);if(!n){let e=`Could not unsubscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo]),_=(0,F.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null||o.subscribers.length===0)return!1;s(e=>({...e,pendingCommand:`reset`,error:null,lastInfo:null}));let n=i(`session reset`);if(!n){let e=`Could not reset because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo,o.subscribers.length]);(0,F.useEffect)(()=>{e&&t||(c.current.clear(),l.current=0,s(jd))},[t,e]),(0,F.useEffect)(()=>{if(!e)return;let t=n.on(`minigraph.session.started`,e=>{p(e)}),r=n.on(`minigraph.session.status`,e=>{p(e)}),i=n.on(`minigraph.session.commandResult`,e=>{p(e)}),a=n.on(`minigraph.session.notification`,e=>{p(e)}),o=n.on(`session.reset`,e=>{p(e)});return()=>{t(),r(),i(),a(),o()}},[n,e,p]),(0,F.useEffect)(()=>{!e||!t||f()},[t,e,f]),(0,F.useEffect)(()=>{if(!e||!r)return;let t=l.current;for(let[e,n]of r)if(!(e<=l.current)){for(let e of n)(e.kind===`minigraph.session.started`||e.kind===`minigraph.session.status`||e.kind===`minigraph.session.commandResult`||e.kind===`minigraph.session.notification`||e.kind===`session.reset`)&&p(e);t=Math.max(t,e)}l.current=t,c.current.clear()},[r,e,p]);let v=o.subscribedTo===null,y=o.subscribers.length>0;return(0,F.useMemo)(()=>({state:o,connected:t,isPrimary:v,hasSubscribers:y,canSubscribe:e&&t&&o.pendingCommand===null&&o.subscribedTo===null,canUnsubscribe:e&&t&&o.subscribedTo!==null&&o.pendingCommand===null,canReset:e&&t&&o.pendingCommand===null&&o.subscribedTo===null&&o.subscribers.length>0,subscribeToSession:h,unsubscribe:g,resetSession:_,clearMessage:m}),[m,t,e,y,v,_,o,h,g])}var Pd=(e,t)=>t.some(t=>e instanceof t),Fd,Id;function Ld(){return Fd||=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction]}function Rd(){return Id||=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey]}var zd=new WeakMap,Bd=new WeakMap,Vd=new WeakMap;function Hd(e){let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`success`,i),e.removeEventListener(`error`,a)},i=()=>{t(Jd(e.result)),r()},a=()=>{n(e.error),r()};e.addEventListener(`success`,i),e.addEventListener(`error`,a)});return Vd.set(t,e),t}function Ud(e){if(zd.has(e))return;let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`complete`,i),e.removeEventListener(`error`,a),e.removeEventListener(`abort`,a)},i=()=>{t(),r()},a=()=>{n(e.error||new DOMException(`AbortError`,`AbortError`)),r()};e.addEventListener(`complete`,i),e.addEventListener(`error`,a),e.addEventListener(`abort`,a)});zd.set(e,t)}var Wd={get(e,t,n){if(e instanceof IDBTransaction){if(t===`done`)return zd.get(e);if(t===`store`)return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return Jd(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){return e instanceof IDBTransaction&&(t===`done`||t===`store`)||t in e}};function Gd(e){Wd=e(Wd)}function Kd(e){return Rd().includes(e)?function(...t){return e.apply(Yd(this),t),Jd(this.request)}:function(...t){return Jd(e.apply(Yd(this),t))}}function qd(e){return typeof e==`function`?Kd(e):(e instanceof IDBTransaction&&Ud(e),Pd(e,Ld())?new Proxy(e,Wd):e)}function Jd(e){if(e instanceof IDBRequest)return Hd(e);if(Bd.has(e))return Bd.get(e);let t=qd(e);return t!==e&&(Bd.set(e,t),Vd.set(t,e)),t}var Yd=e=>Vd.get(e);function Xd(e,t,{blocked:n,upgrade:r,blocking:i,terminated:a}={}){let o=indexedDB.open(e,t),s=Jd(o);return r&&o.addEventListener(`upgradeneeded`,e=>{r(Jd(o.result),e.oldVersion,e.newVersion,Jd(o.transaction),e)}),n&&o.addEventListener(`blocked`,e=>n(e.oldVersion,e.newVersion,e)),s.then(e=>{a&&e.addEventListener(`close`,()=>a()),i&&e.addEventListener(`versionchange`,e=>i(e.oldVersion,e.newVersion,e))}).catch(()=>{}),s}function Zd(e,{blocked:t}={}){let n=indexedDB.deleteDatabase(e);return t&&n.addEventListener(`blocked`,e=>t(e.oldVersion,e)),Jd(n).then(()=>void 0)}var Qd=[`get`,`getKey`,`getAll`,`getAllKeys`,`count`],$d=[`put`,`add`,`delete`,`clear`],ef=new Map;function tf(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t==`string`))return;if(ef.get(t))return ef.get(t);let n=t.replace(/FromIndex$/,``),r=t!==n,i=$d.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(i||Qd.includes(n)))return;let a=async function(e,...t){let a=this.transaction(e,i?`readwrite`:`readonly`),o=a.store;return r&&(o=o.index(t.shift())),(await Promise.all([o[n](...t),i&&a.done]))[0]};return ef.set(t,a),a}Gd(e=>({...e,get:(t,n,r)=>tf(t,n)||e.get(t,n,r),has:(t,n)=>!!tf(t,n)||e.has(t,n)}));var nf=[`continue`,`continuePrimaryKey`,`advance`],rf={},af=new WeakMap,of=new WeakMap,sf={get(e,t){if(!nf.includes(t))return e[t];let n=rf[t];return n||=rf[t]=function(...e){af.set(this,of.get(this)[t](...e))},n}};async function*cf(...e){let t=this;if(t instanceof IDBCursor||(t=await t.openCursor(...e)),!t)return;t=t;let n=new Proxy(t,sf);for(of.set(n,t),Vd.set(n,Yd(t));t;)yield n,t=await(af.get(n)||t.continue()),af.delete(n)}function lf(e,t){return t===Symbol.asyncIterator&&Pd(e,[IDBIndex,IDBObjectStore,IDBCursor])||t===`iterate`&&Pd(e,[IDBIndex,IDBObjectStore])}Gd(e=>({...e,get(t,n,r){return lf(t,n)?cf:e.get(t,n,r)},has(t,n){return lf(t,n)||e.has(t,n)}}));var uf=`minigraph-clipboard`,df=1,ff=`items`,pf=null;function mf(){return Xd(uf,df,{upgrade(e){e.objectStoreNames.contains(ff)&&e.deleteObjectStore(ff);let t=e.createObjectStore(ff,{keyPath:`id`});t.createIndex(`by-alias`,`node.alias`,{unique:!0}),t.createIndex(`by-clippedAt`,`clippedAt`)}})}function hf(){return pf||=mf().catch(async e=>(console.warn(`[clipboard/db] openDB failed, deleting and recreating:`,e),pf=null,await Zd(uf),mf())),pf}async function gf(){return(await(await hf()).getAllFromIndex(ff,`by-clippedAt`)).reverse()}async function _f(e){return(await hf()).getFromIndex(ff,`by-alias`,e)}async function vf(e){await(await hf()).add(ff,e)}async function yf(e,t){let n=(await hf()).transaction(ff,`readwrite`);await n.store.delete(e),await n.store.add(t),await n.done}async function bf(e){await(await hf()).delete(ff,e)}async function xf(){await(await hf()).clear(ff)}var Sf=`minigraph-clipboard-sync`;function Cf(){return new BroadcastChannel(Sf)}function wf(e,t){switch(t.type){case`HYDRATE`:return{items:t.items,isLoading:!1};case`ITEM_ADDED`:return{...e,items:[t.item,...e.items]};case`ITEM_REPLACED`:{let n=e.items.filter(e=>e.id!==t.previousId);return{...e,items:[t.item,...n]}}case`ITEM_REMOVED`:return{...e,items:e.items.filter(e=>e.id!==t.id)};case`ITEMS_CLEARED`:return{...e,items:[]};default:return e}}var Tf=(0,F.createContext)(null);function Ef({children:e}){let[t,n]=(0,F.useReducer)(wf,{items:[],isLoading:!0}),r=(0,F.useRef)(null);(0,F.useEffect)(()=>{gf().then(e=>n({type:`HYDRATE`,items:e}))},[]),(0,F.useEffect)(()=>{let e;try{e=Cf()}catch{return}return r.current=e,e.onmessage=e=>{let t=e.data;switch(t.type){case`item-added`:n({type:`ITEM_ADDED`,item:t.item});break;case`item-replaced`:n({type:`ITEM_REPLACED`,item:t.item,previousId:t.previousId});break;case`item-removed`:n({type:`ITEM_REMOVED`,id:t.id});break;case`items-cleared`:n({type:`ITEMS_CLEARED`});break}},()=>{e.close(),r.current=null}},[]);let i=(0,F.useCallback)(e=>{r.current?.postMessage(e)},[]),a=(0,F.useCallback)(async(e,t,r)=>{try{let a={id:crypto.randomUUID(),clippedAt:new Date().toISOString(),sourceWsPath:r.sourceWsPath,sourceLabel:r.sourceLabel,node:e,connections:t},o=await _f(e.alias);if(o)return{status:`duplicate`,existingItem:o,pendingItem:a};try{await vf(a)}catch(t){if(t instanceof DOMException&&t.name===`ConstraintError`){let t=await _f(e.alias);if(t)return{status:`duplicate`,existingItem:t,pendingItem:a}}throw t}return n({type:`ITEM_ADDED`,item:a}),i({type:`item-added`,item:a}),{status:`added`}}catch(e){return{status:`error`,message:e instanceof Error?e.message:String(e)}}},[i]),o=(0,F.useCallback)(async(e,t)=>{await yf(t,e),n({type:`ITEM_REPLACED`,item:e,previousId:t}),i({type:`item-replaced`,item:e,previousId:t})},[i]),s=(0,F.useCallback)(async e=>{await bf(e),n({type:`ITEM_REMOVED`,id:e}),i({type:`item-removed`,id:e})},[i]),c=(0,F.useCallback)(async()=>{await xf(),n({type:`ITEMS_CLEARED`}),i({type:`items-cleared`})},[i]);return(0,R.jsx)(Tf.Provider,{value:{items:t.items,isLoading:t.isLoading,clipNode:a,confirmReplace:o,removeItem:s,clearAll:c},children:e})}function Df(){let e=(0,F.useContext)(Tf);if(!e)throw Error(`useClipboardContext must be used inside <ClipboardProvider>`);return e}var Of=new Intl.Collator(void 0,{sensitivity:`base`,numeric:!0});function kf(e){return e.node.types[0]?.trim()||`unknown`}function Af(e,t){return Of.compare(e,t)}function jf(e,t){return e-t}function Mf(e){return e===`recent`||e===`connections`?`descending`:`ascending`}function Nf(e,t){return t===`descending`?-e:e}function Pf(e,t){let n=t.trim();if(!n)return{missing:!0,value:``};let r=e.node.properties[n];return r==null?{missing:!0,value:``}:typeof r==`string`?{missing:!1,value:r}:typeof r==`number`||typeof r==`boolean`?{missing:!1,value:String(r)}:{missing:!1,value:JSON.stringify(r)}}function Ff(e,t,n,r){let i=Pf(e,n),a=Pf(t,n);return i.missing&&!a.missing?1:!i.missing&&a.missing?-1:Nf(Af(i.value,a.value),r)}function If(e,t){let n=t.direction??Mf(t.field);return e.map((e,t)=>({item:e,originalIndex:t})).sort((e,r)=>{let i=0;switch(t.field){case`type`:i=Af(kf(e.item),kf(r.item));break;case`alias`:i=Af(e.item.node.alias,r.item.node.alias);break;case`source`:i=Af(e.item.sourceLabel,r.item.sourceLabel);break;case`connections`:i=e.item.connections.length-r.item.connections.length;break;case`property`:i=Ff(e.item,r.item,t.propertyKey??``,n);break;default:i=Date.parse(e.item.clippedAt)-Date.parse(r.item.clippedAt);break}return t.field!==`property`&&(i=Nf(i,n)),i===0?jf(e.originalIndex,r.originalIndex):i}).map(({item:e})=>e)}function Lf(e){let t=Date.now()-new Date(e).getTime();if(t<0)return`just now`;let n=Math.floor(t/1e3);if(n<60)return`just now`;let r=Math.floor(n/60);if(r<60)return`${r} min ago`;let i=Math.floor(r/60);if(i<24)return`${i} hour${i>1?`s`:``} ago`;let a=Math.floor(i/24);return a===1?`yesterday`:a<30?`${a} days ago`:new Date(e).toLocaleDateString()}var Rf={item:`_item_1ne62_1`,previewFrame:`_previewFrame_1ne62_13`,preview:`_preview_1ne62_13`,previewShell:`_previewShell_1ne62_25`,metaBlock:`_metaBlock_1ne62_29`,timestamp:`_timestamp_1ne62_35`,removeChrome:`_removeChrome_1ne62_40`,removeIcon:`_removeIcon_1ne62_71`};function zf({item:e,onRemove:t,onOpenMenu:n,onCloseMenu:r}){let{node:i,clippedAt:a,sourceLabel:o}=e;return(0,R.jsxs)(`div`,{className:Rf.item,children:[(0,R.jsxs)(`div`,{className:Rf.previewFrame,children:[(0,R.jsx)(`button`,{type:`button`,className:Rf.removeChrome,draggable:!1,"aria-label":`Remove node ${i.alias} from clipboard`,onClick:n=>{n.stopPropagation(),r(),t(e.id)},children:(0,R.jsx)(al,{className:Rf.removeIcon,"aria-hidden":`true`,focusable:`false`})}),(0,R.jsx)(`div`,{className:Rf.preview,role:`group`,draggable:!0,onDragStart:t=>{r(),Gs(t.dataTransfer,e.id)},onContextMenu:t=>{t.preventDefault(),n(e.id,t.clientX,t.clientY)},onKeyDown:t=>{if(t.key===`ContextMenu`||t.key===`F10`&&t.shiftKey){t.preventDefault();let r=t.currentTarget.getBoundingClientRect();n(e.id,Math.round(r.left+8),Math.round(r.top+8))}},tabIndex:0,"aria-label":`Drag node ${i.alias} into the graph to paste`,children:(0,R.jsx)(`div`,{className:Rf.previewShell,style:gi(i.types[0]??`unknown`),children:(0,R.jsx)(bi,{alias:i.alias,nodeType:i.types[0]??`unknown`,properties:i.properties})})})]}),(0,R.jsx)(`div`,{className:Rf.metaBlock,children:(0,R.jsxs)(`div`,{className:Rf.timestamp,children:[`Clipped `,Lf(a),` from `,o]})})]})}var Bf={menu:`_menu_164vh_1`,menuItem:`_menuItem_164vh_12`},Vf=16;function Hf(e,t,n){let r=Vf,i=Math.max(Vf,n-t-Vf);return Math.min(Math.max(e,r),i)}function Uf({open:e,x:t,y:n,canPasteToInput:r,onPasteToInput:i,onInspect:a,onClose:o}){let s=(0,F.useRef)(null),c=(0,F.useRef)(null),l=(0,F.useRef)(null),[u,d]=(0,F.useState)({left:t,top:n});return(0,F.useLayoutEffect)(()=>{if(!e||!s.current)return;let r=s.current.getBoundingClientRect();d({left:Hf(t,r.width,window.innerWidth),top:Hf(n,r.height,window.innerHeight)})},[e,t,n]),(0,F.useEffect)(()=>{if(!e)return;r?c.current?.focus():l.current?.focus();let t=e=>{s.current&&!s.current.contains(e.target)&&o()},n=e=>{e.key===`Escape`&&(e.preventDefault(),o())},i=()=>o();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,i,!0),window.addEventListener(`resize`,i),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,i,!0),window.removeEventListener(`resize`,i)}},[e,r,o]),e?(0,R.jsxs)(`div`,{ref:s,className:Bf.menu,style:{left:u.left,top:u.top},role:`menu`,"aria-label":`Clipboard item actions`,children:[(0,R.jsx)(`button`,{ref:c,role:`menuitem`,type:`button`,className:Bf.menuItem,disabled:!r,onClick:()=>{r&&i()},children:`Paste to Input`}),(0,R.jsx)(`button`,{ref:l,role:`menuitem`,type:`button`,className:Bf.menuItem,onClick:a,children:`Inspect`})]}):null}var $={sidebar:`_sidebar_1jo34_2`,header:`_header_1jo34_12`,headerTitle:`_headerTitle_1jo34_25`,clearBtn:`_clearBtn_1jo34_32`,sortBar:`_sortBar_1jo34_48`,sortMenuWrapper:`_sortMenuWrapper_1jo34_63`,sortMenuButton:`_sortMenuButton_1jo34_68`,sortButtonLabel:`_sortButtonLabel_1jo34_93`,sortButtonValue:`_sortButtonValue_1jo34_98`,sortButtonDirection:`_sortButtonDirection_1jo34_106`,sortButtonCaret:`_sortButtonCaret_1jo34_114`,sortButtonCaretOpen:`_sortButtonCaretOpen_1jo34_121`,sortPopover:`_sortPopover_1jo34_125`,sortGroup:`_sortGroup_1jo34_139`,propertySortRow:`_propertySortRow_1jo34_150`,sortGroupTitle:`_sortGroupTitle_1jo34_154`,sortOption:`_sortOption_1jo34_164`,propertyLabel:`_propertyLabel_1jo34_197`,propertyInput:`_propertyInput_1jo34_205`,itemList:`_itemList_1jo34_223`,loading:`_loading_1jo34_233`,emptyState:`_emptyState_1jo34_243`,emptyIcon:`_emptyIcon_1jo34_256`,emptyTitle:`_emptyTitle_1jo34_261`,emptyHint:`_emptyHint_1jo34_265`,inspectPanel:`_inspectPanel_1jo34_271`,inspectHeader:`_inspectHeader_1jo34_279`,inspectClose:`_inspectClose_1jo34_293`,inspectBody:`_inspectBody_1jo34_307`,dialog:`_dialog_1jo34_313`,dialogTitle:`_dialogTitle_1jo34_328`,dialogBody:`_dialogBody_1jo34_335`,dialogActions:`_dialogActions_1jo34_342`,cancelBtn:`_cancelBtn_1jo34_349`,replaceBtn:`_replaceBtn_1jo34_363`};function Wf(){return(0,R.jsxs)(`div`,{className:$.emptyState,children:[(0,R.jsx)(`span`,{className:$.emptyIcon,children:`📋`}),(0,R.jsx)(`span`,{className:$.emptyTitle,children:`No items clipped yet.`}),(0,R.jsx)(`span`,{className:$.emptyHint,children:`Right-click a node in the Graph view to get started.`})]})}var Gf=[{value:`recent`,label:`Recent`},{value:`type`,label:`Type`},{value:`alias`,label:`Alias`},{value:`source`,label:`Source`},{value:`connections`,label:`Connections`},{value:`property`,label:`Property`}],Kf=[{value:`ascending`,label:`Ascending`},{value:`descending`,label:`Descending`}],qf=Gf.reduce((e,t)=>({...e,[t.value]:t.label}),{});function Jf({connected:e,onPasteToInput:t}){let n=(0,F.useId)(),i=(0,F.useId)(),a=(0,F.useRef)(null),s=Df(),[c,l]=(0,F.useState)(null),[u,d]=(0,F.useState)(null),[f,p]=(0,F.useState)(`recent`),[m,h]=(0,F.useState)(Mf(`recent`)),[g,_]=(0,F.useState)(``),[v,y]=(0,F.useState)(!1),b=(e,t,n)=>{d({itemId:e,x:t,y:n})},x=()=>{d(null)},S=e=>{x(),t(e)},C=e=>{x(),l(t=>t?.id===e.id?null:e)},w=e=>{x(),l(t=>t?.id===e?null:t),s.removeItem(e)},T=()=>{x(),l(null),s.clearAll()},E=e=>{p(e),h(Mf(e))};(0,F.useEffect)(()=>{let e=new Set(s.items.map(e=>e.id));u&&!e.has(u.itemId)&&d(null),c&&!e.has(c.id)&&l(null)},[s.items,u,c]),(0,F.useEffect)(()=>{if(!v)return;let e=e=>{a.current?.contains(e.target)||y(!1)},t=e=>{e.key===`Escape`&&y(!1)};return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,t),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,t)}},[v]);let D=(0,F.useMemo)(()=>u?s.items.find(e=>e.id===u.itemId)??null:null,[u,s.items]),O=(0,F.useMemo)(()=>If(s.items,{field:f,direction:m,propertyKey:g}),[s.items,m,f,g]);return(0,R.jsxs)(`div`,{className:$.sidebar,children:[(0,R.jsxs)(`div`,{className:$.header,children:[(0,R.jsx)(`span`,{className:$.headerTitle,children:`Workspace`}),s.items.length>0&&(0,R.jsx)(`button`,{className:$.clearBtn,onClick:T,"aria-label":`Clear all workspace items`,children:`Clear`})]}),s.items.length>0&&(0,R.jsx)(`div`,{className:$.sortBar,children:(0,R.jsxs)(`div`,{className:$.sortMenuWrapper,ref:a,children:[(0,R.jsxs)(`button`,{type:`button`,className:$.sortMenuButton,onClick:()=>y(e=>!e),"aria-expanded":v,"aria-controls":n,children:[(0,R.jsx)(`span`,{className:$.sortButtonLabel,children:`Sort`}),(0,R.jsx)(`span`,{className:$.sortButtonValue,children:qf[f]}),(0,R.jsx)(`span`,{className:$.sortButtonDirection,children:m===`ascending`?`Asc`:`Desc`}),(0,R.jsx)(`span`,{className:`${$.sortButtonCaret}${v?` ${$.sortButtonCaretOpen}`:``}`,"aria-hidden":`true`,children:`▾`})]}),v&&(0,R.jsxs)(`div`,{id:n,className:$.sortPopover,children:[(0,R.jsxs)(`div`,{className:$.sortGroup,role:`group`,"aria-labelledby":`${n}-field-title`,children:[(0,R.jsx)(`div`,{id:`${n}-field-title`,className:$.sortGroupTitle,children:`Sort By`}),Gf.map(e=>(0,R.jsxs)(`label`,{className:$.sortOption,children:[(0,R.jsx)(`input`,{type:`radio`,name:`${n}-field`,value:e.value,checked:f===e.value,onChange:()=>E(e.value)}),(0,R.jsx)(`span`,{children:e.label})]},e.value))]}),f===`property`&&(0,R.jsxs)(`div`,{className:$.propertySortRow,children:[(0,R.jsx)(`label`,{className:$.propertyLabel,htmlFor:i,children:`Property Key`}),(0,R.jsx)(`input`,{id:i,className:$.propertyInput,value:g,onChange:e=>_(e.target.value),placeholder:`skill`,"aria-label":`Property key to sort by`})]}),(0,R.jsxs)(`div`,{className:$.sortGroup,role:`group`,"aria-labelledby":`${n}-direction-title`,children:[(0,R.jsx)(`div`,{id:`${n}-direction-title`,className:$.sortGroupTitle,children:`Sort Direction`}),Kf.map(e=>(0,R.jsxs)(`label`,{className:$.sortOption,children:[(0,R.jsx)(`input`,{type:`radio`,name:`${n}-direction`,value:e.value,checked:m===e.value,onChange:()=>h(e.value)}),(0,R.jsx)(`span`,{children:e.label})]},e.value))]})]})]})}),(0,R.jsx)(`div`,{className:$.itemList,children:s.isLoading?(0,R.jsx)(`div`,{className:$.loading,children:`Loading…`}):s.items.length===0?(0,R.jsx)(Wf,{}):O.map(e=>(0,R.jsx)(zf,{item:e,onRemove:w,onOpenMenu:b,onCloseMenu:x},e.id))}),c&&(0,R.jsxs)(`div`,{className:$.inspectPanel,children:[(0,R.jsxs)(`div`,{className:$.inspectHeader,children:[(0,R.jsxs)(`span`,{children:[`Inspect node `,c.node.alias]}),(0,R.jsx)(`button`,{className:$.inspectClose,onClick:()=>l(null),"aria-label":`Close inspect panel`,children:`✕`})]}),(0,R.jsx)(`div`,{className:$.inspectBody,children:(0,R.jsx)(o,{data:{node:c.node,connections:c.connections},style:r})})]}),u&&D&&(0,R.jsx)(Uf,{open:!0,x:u.x,y:u.y,canPasteToInput:e,onPasteToInput:()=>S(D),onInspect:()=>C(D),onClose:x})]})}function Yf(e){let{wheelTargetRef:t,scrollRef:n,contentWrapperRef:r,currentIndex:i,totalPages:a,onNavigatePrev:o,onNavigateNext:s}=e,c=(0,F.useRef)(0),l=(0,F.useRef)(null),u=(0,F.useRef)(!1),d=(0,F.useRef)(null),f=(0,F.useRef)(o),p=(0,F.useRef)(s),m=(0,F.useRef)(i),h=(0,F.useRef)(a);(0,F.useEffect)(()=>{f.current=o}),(0,F.useEffect)(()=>{p.current=s}),(0,F.useEffect)(()=>{m.current=i}),(0,F.useEffect)(()=>{h.current=a}),(0,F.useEffect)(()=>{d.current!==null&&(clearTimeout(d.current),d.current=null),r.current&&(r.current.style.transition=`none`,r.current.style.transform=`translateY(0)`),c.current=0,l.current=null},[i]),(0,F.useEffect)(()=>{let e=t.current;if(!e)return;function i(){c.current=0,l.current=null,r.current&&(r.current.style.transition=`transform 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94)`,r.current.style.transform=`translateY(0)`)}function a(e){if(e.deltaY===0)return;let t=n.current;if(!t)return;let a=t.scrollTop<=0,o=t.scrollTop+t.clientHeight>=t.scrollHeight-1,s=e.deltaY<0,g=e.deltaY>0,_=a&&s,v=o&&g;if(!_&&!v){i();return}if(u.current)return;let y=m.current,b=h.current;if(_&&y===0||v&&y===b-1)return;let x=_?`prev`:`next`;if(l.current!==null&&l.current!==x&&i(),l.current=x,c.current+=Math.abs(e.deltaY),r.current){let e=x===`prev`?-1:1,t=c.current*(18/120),n=Math.min(t,18)*e;r.current.style.transition=`none`,r.current.style.transform=`translateY(${n}px)`}if(d.current!==null&&clearTimeout(d.current),d.current=setTimeout(i,180),c.current>=120){d.current!==null&&clearTimeout(d.current);let e=l.current;i(),u.current=!0,e===`prev`?f.current():p.current(),setTimeout(()=>{u.current=!1},650)}}return e.addEventListener(`wheel`,a,{passive:!0}),()=>{d.current!==null&&clearTimeout(d.current),e.removeEventListener(`wheel`,a)}},[])}var Xf={helpRoot:`_helpRoot_18tja_2`,categoryNav:`_categoryNav_18tja_11`,categoryTabScroller:`_categoryTabScroller_18tja_21`,categoryTab:`_categoryTab_18tja_21`,categoryTabActive:`_categoryTabActive_18tja_71`,maximizeButton:`_maximizeButton_18tja_78`,closeButton:`_closeButton_18tja_100`,helpBody:`_helpBody_18tja_122`,emptyFallback:`_emptyFallback_18tja_130`,helpContent:`_helpContent_18tja_147`,topicLink:`_topicLink_18tja_226`,helpBodyContent:`_helpBodyContent_18tja_271`,chipStrip:`_chipStrip_18tja_276`,chipStripLabel:`_chipStripLabel_18tja_294`,topicChip:`_topicChip_18tja_310`,topicChipActive:`_topicChipActive_18tja_338`};function Zf(e){return typeof e==`string`?e:typeof e==`number`?String(e):Array.isArray(e)?e.map(Zf).join(``):F.isValidElement(e)?Zf(e.props.children):``}function Qf(e){if(!e.trim().toLowerCase().startsWith(`help `))return null;let t=e.trim().slice(5).replace(/\s*\(.*\)\s*$/,``).trim().toLowerCase();return t.length>0?t:null}function $f({activeTopic:e,onNavigate:t,onClose:n,onToggleMaximize:r,isMaximized:i,contentProfile:a=`minigraph`}){let o=(0,F.useRef)(null),s=(0,F.useRef)(null),c=(0,F.useRef)(null),l=(0,F.useRef)(null);(0,F.useEffect)(()=>{o.current&&(o.current.scrollTop=0)},[e]),(0,F.useEffect)(()=>{let e=l.current;if(!e)return;let t=e.querySelector(`[aria-current="step"]`);t&&t.scrollIntoView({block:`nearest`,inline:`nearest`,behavior:`smooth`})},[e]);let u=(0,F.useMemo)(()=>$t(a),[a]),d=(0,F.useMemo)(()=>an(a),[a]),f=tn(e,a),p=(0,F.useMemo)(()=>nn(f,a),[f,a]),m=p.length,h=(0,F.useMemo)(()=>u.find(e=>e.id===f)?.chipStripLabel??null,[f,u]),g=d.indexOf(e),_=g<0?0:g,v=d.length;Yf({wheelTargetRef:s,scrollRef:o,contentWrapperRef:c,currentIndex:_,totalPages:v,onNavigatePrev:()=>t(d[_-1]??``),onNavigateNext:()=>t(d[_+1]??d[d.length-1])});let y=Yt(e,a);return(0,R.jsxs)(`div`,{className:Xf.helpRoot,role:`region`,"aria-label":`Help browser`,ref:s,children:[(0,R.jsxs)(`nav`,{className:Xf.categoryNav,"aria-label":`Help categories`,children:[(0,R.jsx)(`div`,{className:Xf.categoryTabScroller,children:u.map(e=>(0,R.jsx)(`button`,{className:[Xf.categoryTab,e.id===f?Xf.categoryTabActive:``].join(` `).trim(),"aria-current":e.id===f?`true`:void 0,onClick:()=>{t(nn(e.id,a)[0]??``)},children:e.label},e.id))}),r&&(0,R.jsx)(`button`,{className:Xf.maximizeButton,onClick:r,"aria-label":i?`Restore help panel`:`Maximize help panel`,children:i?`⊞`:`⛶`}),n&&(0,R.jsx)(`button`,{className:Xf.closeButton,onClick:n,"aria-label":`Close help panel`,children:`×`})]}),m>1&&(0,R.jsxs)(`div`,{className:Xf.chipStrip,ref:l,children:[h!==null&&(0,R.jsx)(`span`,{className:Xf.chipStripLabel,children:h}),p.map(n=>{let r=n===e,i=rn(n,f);return(0,R.jsx)(`button`,{className:[Xf.topicChip,r?Xf.topicChipActive:``].join(` `).trim(),"aria-current":r?`step`:void 0,onClick:()=>t(n),children:i},n)})]}),(0,R.jsx)(`div`,{className:Xf.helpBody,ref:o,children:(0,R.jsx)(`div`,{className:Xf.helpBodyContent,ref:c,children:y===null?(0,R.jsxs)(`div`,{className:Xf.emptyFallback,children:[(0,R.jsxs)(`code`,{children:[`help `,e||``]}),`\xA0 not found in the local bundle.`]}):(0,R.jsx)(`div`,{className:Xf.helpContent,children:(0,R.jsx)(O,{remarkPlugins:[A],components:e===``?{li:({children:e,...n})=>{let r=Qf(Zf(e).trim());return r!==null&&Yt(r,a)!==null?(0,R.jsx)(`li`,{...n,children:(0,R.jsx)(`button`,{className:Xf.topicLink,"aria-label":`Open help topic: ${r}`,onClick:()=>t(r),children:e})}):(0,R.jsx)(`li`,{...n,children:e})}}:void 0,children:y})})})})]})}function ep({existingItem:e,pendingItem:t,onReplace:n,onCancel:r}){let i=(0,F.useRef)(null);return(0,F.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal()},[]),(0,R.jsxs)(`dialog`,{ref:i,className:$.dialog,onClose:r,"aria-labelledby":`duplicate-dialog-title`,children:[(0,R.jsx)(`h2`,{id:`duplicate-dialog-title`,className:$.dialogTitle,children:`Duplicate Node`}),(0,R.jsxs)(`p`,{className:$.dialogBody,children:[`A clipboard item with alias `,(0,R.jsxs)(`strong`,{children:[`"`,t.node.alias,`"`]}),` already exists (clipped `,Lf(e.clippedAt),`).`]}),(0,R.jsx)(`p`,{className:$.dialogBody,children:`Replace it with the new snapshot?`}),(0,R.jsxs)(`div`,{className:$.dialogActions,children:[(0,R.jsx)(`button`,{className:$.cancelBtn,onClick:r,children:`Cancel`}),(0,R.jsx)(`button`,{className:$.replaceBtn,onClick:n,children:`Replace`})]})]})}var tp={dialog:`_dialog_18jha_2`,title:`_title_18jha_20`,body:`_body_18jha_27`,label:`_label_18jha_34`,inputRow:`_inputRow_18jha_42`,input:`_input_18jha_42`,inputInvalid:`_inputInvalid_18jha_65`,extension:`_extension_18jha_69`,hint:`_hint_18jha_75`,hintError:`_hintError_18jha_82`,actions:`_actions_18jha_86`,cancelBtn:`_cancelBtn_18jha_93`,primaryBtn:`_primaryBtn_18jha_103`};function np({defaultGraphId:e,supportsFolderPicker:t,onConfirm:n,onCancel:r}){let i=(0,F.useRef)(null),a=(0,F.useRef)(null),[o,s]=(0,F.useState)(e),[c,l]=(0,F.useState)(!1),u=(0,F.useId)(),d=(0,F.useId)(),f=(0,F.useId)(),p=o.trim(),m=$a(p),h=c&&!m;return(0,F.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal(),a.current?.select()},[]),(0,R.jsx)(`dialog`,{ref:i,className:tp.dialog,onClose:r,"aria-labelledby":u,children:(0,R.jsxs)(`form`,{onSubmit:e=>{e.preventDefault(),l(!0),m&&n(p)},noValidate:!0,children:[(0,R.jsx)(`h2`,{id:u,className:tp.title,children:`Download graph`}),(0,R.jsx)(`label`,{className:tp.label,htmlFor:d,children:`Graph ID`}),(0,R.jsxs)(`div`,{className:tp.inputRow,children:[(0,R.jsx)(`input`,{id:d,ref:a,className:`${tp.input}${h?` ${tp.inputInvalid}`:``}`,type:`text`,value:o,onChange:e=>{s(e.target.value),l(!0)},"aria-describedby":f,"aria-invalid":h,autoComplete:`off`,spellCheck:!1,maxLength:80}),(0,R.jsx)(`span`,{className:tp.extension,"aria-hidden":`true`,children:`.json`})]}),(0,R.jsx)(`p`,{id:f,className:`${tp.hint}${h?` ${tp.hintError}`:``}`,role:h?`alert`:void 0,children:h?`A graph id uses ${Qa}.`:`Saved as ${to(p||`graph-id`)}. The root node's name in the file becomes the graph id, as "export graph as" does. `+(t?`The next dialog picks the folder.`:`The browser saves it to its download folder.`)}),(0,R.jsxs)(`div`,{className:tp.actions,children:[(0,R.jsx)(`button`,{type:`button`,className:tp.cancelBtn,onClick:r,children:`Cancel`}),(0,R.jsx)(`button`,{type:`submit`,className:tp.primaryBtn,disabled:!m,children:`Download`})]})]})})}function rp({pending:e,onReplace:t,onCancel:n}){let r=(0,F.useRef)(null),i=(0,F.useId)();(0,F.useEffect)(()=>{let e=r.current;e&&!e.open&&e.showModal()},[]);let a=(e,t)=>`${e} ${t}${e===1?``:`s`}`,o=`${a(e.nodeCount,`node`)}, ${a(e.connectionCount,`connection`)}`+(e.name?`, named "${e.name}"`:``);return(0,R.jsxs)(`dialog`,{ref:r,className:tp.dialog,onClose:n,"aria-labelledby":i,children:[(0,R.jsx)(`h2`,{id:i,className:tp.title,children:`Replace the current graph?`}),(0,R.jsxs)(`p`,{className:tp.body,children:[`Importing `,(0,R.jsxs)(`strong`,{children:[`"`,e.fileName,`"`]}),` (`,o,`) replaces the draft in this session and in every member of a shared session; a graph instance is cleared.`]}),(0,R.jsx)(`p`,{className:tp.body,children:`This cannot be undone. Download or save the current graph first if you want to keep it.`}),(0,R.jsxs)(`div`,{className:tp.actions,children:[(0,R.jsx)(`button`,{type:`button`,className:tp.cancelBtn,onClick:n,children:`Cancel`}),(0,R.jsx)(`button`,{type:`button`,className:tp.primaryBtn,onClick:t,children:`Replace`})]})]})}function ip(e){return`/api/graph/import/${e}`}function ap({sessionId:e,connected:t,hasGraph:n,addToast:r}){let[i,a]=(0,F.useState)(null),[o,s]=(0,F.useState)(!1),c=t&&e!==null,l=(0,F.useCallback)(async t=>{if(e===null){r(`Could not import because the session id is not known yet.`,`error`);return}s(!0);try{let n=await fetch(ip(e),{method:`POST`,headers:{"Content-Type":`application/json`},body:JSON.stringify(t.model)});n.ok||r(`Import of "${t.fileName}" refused - ${await Hl(n)}`,`error`)}catch(e){r(`Import of "${t.fileName}" failed: ${e.message}`,`error`)}finally{s(!1)}},[e,r]),u=(0,F.useCallback)(async i=>{let o=Array.from(i);if(o.length===0)return;if(!t||e===null){r(`Connect first to import a graph.`,`error`);return}if(o.length>1){r(`Drop one graph file at a time.`,`error`);return}let s=o[0],c=pl(s);if(c){r(c,`error`);return}let u;try{u=JSON.parse(await fl(s))}catch(e){r(`"${s.name}" is not valid JSON: ${e.message}`,`error`);return}let d=io(u);if(!d.ok){r(`"${s.name}" is not a graph model: ${d.error}`,`error`);return}let f={...d,fileName:s.name};if(n){a(f);return}await l(f)},[t,e,n,r,l]),d=(0,F.useCallback)(()=>{if(!i)return;let e=i;a(null),l(e)},[i,l]),f=(0,F.useCallback)(()=>a(null),[]);return{canImport:c,importFiles:u,openFilePicker:(0,F.useCallback)(()=>{if(!t||e===null){r(`Connect first to import a graph.`,`error`);return}let n=document.createElement(`input`);n.type=`file`,n.accept=`.json,application/json`,n.style.display=`none`;let i=()=>n.remove();n.addEventListener(`change`,()=>{let e=n.files;i(),e&&u(e)},{once:!0}),n.addEventListener(`cancel`,i,{once:!0}),document.body.appendChild(n),n.click()},[t,e,r,u]),pending:i,confirmPending:d,cancelPending:f,isImporting:o}}function op(e){return Al.map(t=>({key:e(),name:t,value:``}))}function sp({addToast:e,importFiles:t,graphData:n,graphName:r}){let i=(0,F.useRef)(1),a=(0,F.useCallback)(()=>i.current++,[]),[o,s]=(0,F.useState)(!1),[c,l]=(0,F.useState)(`assemble`),[u,d]=(0,F.useState)([]),[f,p]=(0,F.useState)([]),[m,h]=(0,F.useState)(``),[g,_]=(0,F.useState)(()=>op(a)),[v,y]=(0,F.useState)(!1),[b,x]=(0,F.useState)(null),[S,C]=(0,F.useState)(null),[w,T]=(0,F.useState)(!1),E=(0,F.useRef)(null),D=(0,F.useCallback)(()=>{E.current?.abort(),E.current=null},[]);(0,F.useEffect)(()=>D,[D]);let O=(0,F.useMemo)(()=>Fl(u),[u]),k=(0,F.useMemo)(()=>new Set(u.map(e=>e.id)),[u]),A=(0,F.useMemo)(()=>Il(g,k),[g,k]),ee=(0,F.useMemo)(()=>Pl(m),[m]),j=u.length>0&&O.size===0&&A.size===0&&ee===null&&!v&&!w,te=(0,F.useCallback)(()=>s(!0),[]),ne=(0,F.useCallback)(()=>{D(),y(!1),T(!1),l(`assemble`),C(null),s(!1)},[D]),M=(0,F.useCallback)(async e=>{D();let t=new AbortController;E.current=t,T(!0);try{let n=await fetch(Tl,{method:`POST`,headers:{"Content-Type":`application/octet-stream`,Accept:`application/json`},body:e,signal:t.signal});if(!n.ok){p([`"${e.name}" was not read - ${await Hl(n)}`]);return}let r=X(await n.json());C({fileName:e.name,set:r}),l(`inspect`)}catch(t){if(t.name===`AbortError`)return;p([`"${e.name}" was not read: ${t.message}`])}finally{E.current===t&&(E.current=null),T(!1)}},[D]),N=(0,F.useCallback)(async e=>{let t=Array.from(e);if(t.length===0)return;x(null);let n=t.filter(Ml);if(n.length>0){if(t.length>1){p([`Drop one .pack file on its own to inspect it, or .json graph files to pack.`]);return}p([]),await M(n[0]);return}let r=[],i=[];for(let e of t){let t=pl(e);if(t){r.push(t);continue}let n;try{n=JSON.parse(await fl(e))}catch(t){r.push(`"${e.name}" is not valid JSON: ${t.message}`);continue}let o=io(n);if(!o.ok){r.push(`"${e.name}" is not a graph model: ${o.error}`);continue}i.push({key:a(),id:jl(e.name),label:e.name,model:o.model,name:o.name,nodeCount:o.nodeCount,connectionCount:o.connectionCount,source:`file`})}p(r),i.length>0&&d(e=>[...e,...i])},[M,a]),P=n!==null,re=(0,F.useCallback)(()=>{if(!n)return;let t=eo(r),i=io(JSON.parse(ao(n,t)));if(!i.ok){e(`The current graph cannot be packed: ${i.error}`,`error`);return}let o={key:a(),id:t,label:`current graph`,model:i.model,name:i.name,nodeCount:i.nodeCount,connectionCount:i.connectionCount,source:`draft`};x(null),d(e=>{let n=e.findIndex(e=>e.source===`draft`&&e.id===t);if(n<0)return[...e,o];let r=[...e];return r[n]=o,r})},[n,r,a,e]),ie=(0,F.useCallback)(e=>{x(null),d(t=>t.filter(t=>t.key!==e))},[]),ae=(0,F.useCallback)(()=>{x(null),p([]),d([])},[]),I=(0,F.useCallback)((e,t)=>{x(null),_(n=>n.map(n=>n.key===e?{...n,...t}:n))},[]),L=(0,F.useCallback)(()=>{_(e=>[...e,{key:a(),name:``,value:``}])},[a]),oe=(0,F.useCallback)(e=>{x(null),_(t=>t.filter(t=>t.key!==e))},[]),se=(0,F.useCallback)(async()=>{if(!j)return;D();let t=new AbortController;E.current=t,x(null),y(!0);let n=m.trim();try{let r=await fetch(wl,{method:`POST`,headers:{"Content-Type":`application/json`,Accept:`application/octet-stream`},body:JSON.stringify(Ll(n,g,u)),signal:t.signal});if(!r.ok){x(await Hl(r));return}let i=await po(new Uint8Array(await r.arrayBuffer()),Nl(n));i.saved&&e(`Graph set saved as ${i.fileName}`,`success`)}catch(e){if(e.name===`AbortError`)return;x(`Packing failed: ${e.message}`)}finally{E.current===t&&(E.current=null),y(!1)}},[j,D,m,g,u,e]),ce=(0,F.useCallback)(async e=>{let n=S?.set.graphs.find(t=>t.id===e);n&&await t([new File([JSON.stringify(n.model)],`${e}.json`,{type:`application/json`})])},[S,t]),le=(0,F.useCallback)(()=>{if(!S)return;let{fileName:t,set:n}=S;d(n.graphs.map(e=>({key:a(),id:e.id,label:`${e.id}.json from ${t}`,model:e.model,name:e.name,nodeCount:e.nodeCount,connectionCount:e.connectionCount,source:`set`})));let r=Rl(n.manifest,a);_(r.length>0?r:op(a)),h(n.manifest.set??``),p([]),x(null),C(null),l(`assemble`);let i=n.graphs.length;e(`Editing ${t}: ${i} graph${i===1?``:`s`} and ${r.length} manifest field${r.length===1?``:`s`} loaded`,`info`)},[S,a,e]),ue=(0,F.useCallback)(()=>{C(null),l(`assemble`)},[]);return{isOpen:o,open:te,close:ne,mode:c,busy:v||w,entries:u,entryIssues:O,addFiles:N,fileErrors:f,canAddCurrentGraph:P,addCurrentGraph:re,removeEntry:ie,clearEntries:ae,setName:m,setSetName:h,setNameError:ee,rows:g,rowIssues:A,updateRow:I,addRow:L,removeRow:oe,canPack:j,pack:se,isPacking:v,packError:b,inspected:S,isReading:w,importInspectedGraph:ce,editInspectedSet:le,leaveInspect:ue}}var cp=class{constructor(){this.listeners=new Map}on(e,t){let n=e;return this.listeners.has(n)||this.listeners.set(n,new Set),this.listeners.get(n).add(t),()=>{this.listeners.get(n)?.delete(t)}}emit(e){let t=this.listeners.get(e.kind);t&&t.forEach(t=>{try{t(e)}catch(t){console.error(`[ProtocolBus] listener for '${e.kind}' threw:`,t)}})}clear(){this.listeners.clear()}},lp=`(ws-\\d+-\\d+)`,up=RegExp(`^session ${lp} started(?:\\nCompanion endpoint: (\\/api\\/companion\\/${lp}))?$`,`i`),dp=RegExp(`^Session ${lp} started since (.+)$`),fp=RegExp(`^subscribed to ${lp}$`),pp=/^subscribed by \[(.*)]$/,mp=RegExp(`^Subscribed to ${lp}$`),hp=RegExp(`^Session unsubscribed from ${lp}$`),gp=RegExp(`^Session ${lp} not found$`),_p=RegExp(`^${lp} is not a primary session$`),vp=RegExp(`^You have already subscribed to ${lp}(?:\\nPlease do 'session reset' before subscribing to another session)?$`),yp=RegExp(`^${lp} subscribed to your session$`),bp=RegExp(`^${lp} unsubscribed from your session$`),xp=RegExp(`^Session ${lp} has closed$`);function Sp(e){let t=e.trim();return t.length===0||t.startsWith(`> `)?null:t}function Cp(e){return e.split(`,`).map(e=>e.trim()).filter(e=>e.length>0&&Ad(e))}function wp(e){let t=Sp(e);if(!t)return null;let n=t.match(up);return n?{sessionId:n[1],companionEndpoint:n[2]??null}:null}function Tp(e){let t=Sp(e);if(!t)return null;let n=t.split(`
`).map(e=>e.trim()).filter(Boolean),r=n[0];if(!r)return null;let i=r.match(dp);if(!i)return null;let a=null,o=[];for(let e of n.slice(1)){let t=e.match(fp);if(t){a=t[1];continue}let n=e.match(pp);n&&(o=Cp(n[1]))}return{sessionId:i[1],startedSince:i[2],subscribedTo:a,subscribers:o}}function Ep(e){let t=Sp(e);if(!t)return null;let n=t.match(mp);if(n)return{command:`subscribe`,status:`accepted`,sessionId:n[1],message:t};let r=t.match(hp);if(r)return{command:`unsubscribe`,status:`accepted`,sessionId:r[1],message:t};let i=t.match(gp);if(i)return{command:`subscribe`,status:`rejected`,sessionId:i[1],message:t};let a=t.match(_p);if(a)return{command:`subscribe`,status:`rejected`,sessionId:a[1],message:t};let o=t.match(vp);return o?{command:`subscribe`,status:`rejected`,sessionId:o[1],message:t}:t===`You cannot subscribe to yourself`?{command:`subscribe`,status:`rejected`,sessionId:null,message:t}:t===`Nothing to unsubscribe`?{command:`unsubscribe`,status:`rejected`,sessionId:null,message:t}:t===`Invalid session command`?{command:`unknown`,status:`rejected`,sessionId:null,message:t}:null}function Dp(e){let t=Sp(e);if(!t)return null;let n=t.match(yp);if(n)return{type:`subscriber-joined`,sessionId:n[1],message:t};let r=t.match(bp);if(r)return{type:`subscriber-left`,sessionId:r[1],message:t};let i=t.match(xp);return i?{type:`host-closed`,sessionId:i[1],message:t}:null}var Op=new Set([`info`,`error`,`ping`,`welcome`]);function kp(e,t){let n=[],r={msgId:e,raw:t},i=!1,a=!1,o=!1,s=!1,c=!1,l=!1,u=be(t);if(u.isJSON){let e=u.data;if(typeof e.type==`string`){let i=e.type;return n.push({...r,kind:`lifecycle`,type:i,knownType:Op.has(i),message:typeof e.message==`string`?e.message:t,time:e.time??null}),n.length>0?n:[{...r,kind:`unclassified`}]}return n.push({...r,kind:`json.response`,data:u.data}),n.length>0?n:[{...r,kind:`unclassified`}]}let d=De(t);d&&(c=!0,n.push({...r,kind:`payload.large`,apiPath:d.apiPath,byteSize:d.byteSize,filename:d.filename}));let f=Oe(t);f&&(o=!0,n.push({...r,kind:`upload.invitation`,uploadPath:f}));let p=Ee(t);if(p&&(s=!0,n.push({...r,kind:`upload.contentPath`,uploadPath:p})),Te(t)){a=!0;let e=we(t);e&&n.push({...r,kind:`graph.link`,apiPath:e})}if(a){let e=xe(t);e&&n.push({...r,kind:`graph.exported`,graphName:e.graphName,apiPath:e.apiPath})}let m=Ve(t);m&&n.push({...r,kind:`graph.mutation`,mutationType:m});let h=Mn(t);h&&n.push({...r,kind:`graph.instance.created`,mockEntries:h.mockEntries,ttlMs:h.ttlMs}),Nn(t)&&n.push({...r,kind:`graph.instance.cleared`});let g=Pn(t);g&&n.push({...r,kind:`graph.run.terminal`,status:g.status,elapsedMs:g.elapsedMs});let _=Fn(t);_&&n.push({...r,kind:`command.error`,message:_});let v=Be(t);v&&n.push({...r,kind:`minigraph.nodeAction.textResult`,status:v.status,action:v.action,alias:v.alias,targetAlias:v.targetAlias,message:v.message}),v&&(v.action===`create-node`||v.status===`error`)&&n.push({...r,kind:`minigraph.createNode.textResult`,status:v.status,alias:v.alias,message:v.message}),t===`Session restarted`&&(l=!0,n.push({...r,kind:`session.reset`}));let y=wp(t);y&&(l=!0,n.push({...r,kind:`minigraph.session.started`,sessionId:y.sessionId,companionEndpoint:y.companionEndpoint}));let b=Tp(t);b&&(l=!0,n.push({...r,kind:`minigraph.session.status`,sessionId:b.sessionId,startedSince:b.startedSince,subscribedTo:b.subscribedTo,subscribers:b.subscribers}));let x=Ep(t);x&&(l=!0,n.push({...r,kind:`minigraph.session.commandResult`,command:x.command,status:x.status,sessionId:x.sessionId,message:x.message}));let S=Dp(t);S&&(l=!0,n.push({...r,kind:`minigraph.session.notification`,type:S.type,sessionId:S.sessionId,message:S.message})),t.startsWith(`> `)&&(i=!0,n.push({...r,kind:`command.echo`,commandText:t.slice(2)})),ke(t)&&n.push({...r,kind:`command.helpOrDescribe`,commandText:t.slice(2)});let C=Ae(t);C&&n.push({...r,kind:`command.importGraph`,graphName:C});let w=Se(t);return w&&n.push({...r,kind:`graph.export.failed`,reason:w.reason}),!i&&!a&&!o&&!s&&!c&&!l&&Ce(t)&&n.push({...r,kind:`docs.response`,isMarkdown:!0}),n.length===0&&n.push({...r,kind:`unclassified`}),n}function Ap({messages:e,bus:t}){let n=(0,F.useRef)(-1);(0,F.useEffect)(()=>{e.length>0&&(n.current=e[e.length-1].id)},[]);let r=(0,F.useMemo)(()=>{let t=new Map;for(let n of e)t.set(n.id,kp(n.id,n.raw));return t},[e]);return(0,F.useEffect)(()=>{if(e.length===0)return;let i=e.filter(e=>e.id>n.current);if(i.length!==0){n.current=e[e.length-1].id;for(let e of i){let n=r.get(e.id);if(n)for(let e of n)t.emit(e)}}},[e,t,r]),{classificationMap:r}}var jp={console:100/3,"node-edit":30,upload:30,package:30,schema:38},Mp=20;function Np(e){return`${e}%`}function Pp({config:e}){let{title:t,wsPath:n,storageKeyPayload:r,storageKeyHistory:i,storageKeyTab:a,storageKeySavedGraphs:o,supportsUpload:s,supportsClipboard:c,supportsHelp:l,helpContentProfile:u=`minigraph`,supportsAuthoring:d,supportsGraphRun:f,supportsSessionCollaboration:p,tabs:m}=e,h=T(),[g,_]=ce(r,``),v=_e(),[y,b]=(0,F.useState)(()=>v.peekPendingPayload(n)),{takePendingPayload:x}=v;(0,F.useEffect)(()=>{let e=x(n);e!==null&&b(e)},[x,n]);let S=y??g,C=(0,F.useCallback)(e=>{b(null),_(e)},[_]),w=(0,F.useMemo)(()=>S?L(S):{valid:!0,error:null,type:null},[S]),{toasts:E,addToast:D,removeToast:O}=se(),k=(0,F.useRef)(new cp).current,A=We({wsPath:n,storageKeyHistory:i,payload:S,addToast:D,bus:k,handleLocalCommand:(0,F.useCallback)(e=>on(e,l===!0,u)!==null,[l,u])}),{classificationMap:ne}=Ap({messages:A.messages,bus:k}),M=Nd({enabled:p===!0,bus:k,classificationMap:ne,connected:A.connected,sendRawText:A.sendRawText,addToast:D}),N=M.state.sessionId===null?null:`/api/graph/session/${M.state.sessionId}`,{graphData:P,setGraphData:re,rightTab:ae,setRightTab:I,isRefreshing:le,refetchGraph:ue}=Ye(N,D,m[0],m,a),{uploadPanelPath:de,successfulUploadPaths:fe,handleOpenUploadPanel:pe,handleCloseUploadPanel:me,handleUploadSuccess:he,handleUploadError:ge,resetSuccessfulPaths:ve}=ln({addToast:D}),ye=ap({sessionId:M.state.sessionId,connected:A.connected,hasGraph:P!==null,addToast:D}),[be,xe]=(0,F.useState)(!1),Se=(0,F.useCallback)(()=>{if(!P){D(`No graph to download yet.`,`info`);return}xe(!0)},[P,D]),Ce=(0,F.useCallback)(async e=>{if(xe(!1),P)try{let t=await H(ao(P,e),to(e));t.saved&&D(`Graph saved as ${t.fileName}`,`success`)}catch(e){D(`Download failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[P,D]),we=(0,F.useCallback)(()=>re(null),[re]);Xe({bus:k,hasGraph:P!==null,connected:A.connected,refetchGraph:ue,clearGraph:we,addToast:D});let Te=Ir({bus:k,connected:A.connected,sendRawText:A.sendRawText,addToast:D}),Ee=(0,F.useRef)(null),De=(0,F.useRef)(null),Oe=(0,F.useRef)(new Map),ke=(0,F.useCallback)((e,t)=>{if(t===null){D(e,`success`);return}D(e,`success`,{durationMs:6e3,action:{label:`Undo`,onClick:()=>Te.undoEntry(t)}})},[D,Te.undoEntry]),Ae=(0,F.useCallback)(e=>{if(e.action===`edit-node`){let t=Ee.current;Ee.current=null,t&&t.alias===e.alias&&ke(`Updated node ${t.alias}`,Te.push(jr(t)));return}if(e.action===`create-node`&&e.alias){ke(`Created node ${e.alias}`,Te.push(Mr(e.alias)));return}if(e.action===`create-connection`){let t=De.current;De.current=null,ke(`Connected ${e.alias??``} → ${e.targetAlias??``}`,Te.push(t));return}if(e.action===`delete-node`&&e.alias){let t=Oe.current.get(e.alias)??null;Oe.current.delete(e.alias),ke(`Deleted node ${e.alias}`,Te.push(t))}},[Te.push,ke]),je=(0,F.useRef)(!1);(0,F.useEffect)(()=>{je.current&&!A.connected&&re(null),je.current=A.connected},[A.connected,re]);let[Me,Ne]=ce(e.storageKeyHelpTopic??`help-topic-fallback`,``),[Pe,Fe]=ce(`help-panel-open`,!1),[Ie,Le]=(0,F.useState)(()=>!!l&&!Pe),[Re,ze]=(0,F.useState)(!1),Be=(0,F.useRef)(null),Ve=(0,F.useCallback)(()=>{Ie&&(ze(!0),Be.current=setTimeout(()=>Le(!1),400))},[Ie]);(0,F.useEffect)(()=>{if(!Ie||Re)return;let e=setTimeout(Ve,3e3);return()=>clearTimeout(e)},[Ie,Re,Ve]),(0,F.useEffect)(()=>{Pe&&Ie&&Ve()},[Pe,Ie,Ve]),(0,F.useEffect)(()=>()=>{Be.current&&clearTimeout(Be.current)},[]),(0,F.useEffect)(()=>{if(!l)return;let e=e=>{e.ctrlKey&&e.key==="`"&&(e.preventDefault(),Fe(e=>!e))};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[l,Fe]),sn({bus:k,setHelpTopic:Ne,onTabSwitch:l?()=>Fe(!0):()=>{},enabled:l===!0,contentProfile:u}),un({bus:k,connected:A.connected,appendMessage:A.appendMessage,addToast:D});let[He,Ue]=ce(`console-panel-open`,!0),Ke=Df(),[qe,Je]=ce(`clipboard-sidebar-open`,!1),[Ze,Qe]=(0,F.useState)(null),$e=(0,F.useCallback)(e=>{let t;try{t=Sr(e,P)}catch(e){D(`Paste failed: ${e instanceof Error?e.message:String(e)}`,`error`);return}A.setCommand(t.command),D(`${t.verb===`create`?`Create`:`Update`} command for "${e.node.alias}" pasted to input`,`info`)},[P,A.setCommand,D]),et=(0,F.useCallback)(e=>{let t=Ke.items.find(t=>t.id===e);if(!t){D(`Clipboard item is no longer available. It may have been removed in another tab.`,`error`);return}let n;try{n=Sr(t,P)}catch(e){D(`Paste failed: ${e instanceof Error?e.message:String(e)}`,`error`);return}if(!A.sendRawText(n.command)){D(`Could not send clipboard paste command because the WebSocket is not open.`,`error`);return}D(`Clipboard node "${t.node.alias}" sent as ${n.verb}. Waiting for backend response.`,`info`)},[Ke.items,P,A.sendRawText,D]),tt=(0,F.useCallback)(async(t,r)=>{try{let i=await Ke.clipNode(t,r,{sourceWsPath:n,sourceLabel:e.label});switch(i.status){case`added`:D(`Node "${t.alias}" clipped to workspace`,`success`);break;case`duplicate`:Qe({pendingItem:i.pendingItem,existingItem:i.existingItem});break;case`error`:D(`Clip failed: ${i.message}`,`error`);break}}catch(e){D(`Clip failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[Ke,n,e.label,D]),nt=(0,F.useCallback)(async t=>{if(t.length===0){D(`No selected nodes are available to clip.`,`info`);return}if(t.length>100){D(`Select 100 or fewer nodes to clip at once.`,`error`);return}let r={added:0,duplicates:0,failed:0};for(let i of t){if(!i.node?.alias?.trim()){r.failed+=1;continue}try{let t=await Ke.clipNode(i.node,i.connections,{sourceWsPath:n,sourceLabel:e.label});t.status===`added`&&(r.added+=1),t.status===`duplicate`&&(r.duplicates+=1),t.status===`error`&&(r.failed+=1)}catch{r.failed+=1}}let i=wr(r);D(i.message,i.type)},[D,Ke,e.label,n]),rt=dn(o??``),{defaultName:it,savedName:at,resetName:ot}=_n(o?`${o}-untitled-counter`:`untitled-counter`,k,A.connected,A.connectionEpoch),st=(0,F.useMemo)(()=>{let e=P?.nodes.find(e=>e.types.includes(`Root`)),t=typeof e?.properties?.name==`string`?e.properties.name:void 0;return t?.trim()?t:null},[P])??it,ct=sp({addToast:D,importFiles:ye.importFiles,graphData:P,graphName:st}),lt=Hu({addToast:D,sessionId:M.state.sessionId,connected:A.connected,graphData:P,graphName:st,bus:k,sendRawText:A.sendRawText}),ut=(0,F.useMemo)(()=>Tr(A.sendRawText),[A.sendRawText]),dt=Od({bus:k,connected:A.connected,graphData:P,executor:ut,onAccepted:Ae,onUserMessage:D}),ft=dt.state,pt=d&&ft.status===`open`&&(ft.action===`edit-node`||ft.action===`create-node`)?ft:null,mt=d&&ft.status===`open`&&ft.action===`create-connection`?ft:null,[ht,gt]=(0,F.useState)(null),_t=pt!==null||mt!==null;(0,F.useEffect)(()=>{let e=e=>{if(e.key!==`z`&&e.key!==`Z`||!(e.metaKey||e.ctrlKey)||e.shiftKey||e.altKey)return;let t=e.target;t instanceof Element&&t.closest(`input, textarea, select, [contenteditable="true"]`)||_t||Te.hasEntries()&&(e.preventDefault(),Te.undoLast())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[_t,Te.hasEntries,Te.undoLast]);let vt=(0,F.useCallback)((e,t,n)=>{gt(n??null),De.current=P?Nr(P,e,t):null,dt.openCreateConnection(e,t)},[dt.openCreateConnection,P]),yt=(0,F.useCallback)(e=>{if(!P)return;let t=Dr(P,e);t&&(Te.runCommands(t.commands),ke(`Deleted ${Or(t.removed)}`,Te.push(Ar(t.removed))))},[P,Te.push,Te.runCommands,ke]),bt=(0,F.useCallback)(e=>{Ee.current=e,dt.openEditNode(e)},[dt.openEditNode]),xt=(0,F.useCallback)(e=>{Oe.current.set(e.alias,P?Pr(P,e):null),dt.deleteNode(e)},[dt.deleteNode,P]),St=A.consoleRef,Ct=He&&pt===null&&de===null&&!ct.isOpen&&!lt.isOpen;(0,F.useEffect)(()=>{Ct&&St.current&&(St.current.scrollTop=St.current.scrollHeight)},[Ct,St]);let wt=Bn({enabled:f===!0,bus:k,connected:A.connected,connectionEpoch:A.connectionEpoch,graphData:P,graphIdentity:N,isPrimary:M.isPrimary,sendRawText:A.sendRawText,addToast:D}),{handleSaveGraph:Tt,handleLoadGraph:Et}=Hn({bus:k,connected:A.connected,sendRawText:A.sendRawText,saveGraph:o?rt.saveGraph:null,addToast:D}),Dt=(0,F.useCallback)(e=>{ne.get(e.id)?.find(e=>e.kind===`graph.link`)&&(ue(),I(`graph`))},[ne,ue,I]),{handleSendToJsonPath:Ot}=cn({ctx:v,navigate:h,addToast:D,wsPath:n}),kt=Ge(`(max-width: 768px)`),At=pt===null?de===null?ct.isOpen?`package`:lt.isOpen?`schema`:He?`console`:null:`upload`:`node-edit`,jt=(0,F.useRef)(null),Mt=(0,F.useRef)(At);(0,F.useEffect)(()=>{let e=Mt.current;if(Mt.current=At,At===null||At===e)return;let t=requestAnimationFrame(()=>{jt.current?.resize(Np(jp[At]))});return()=>cancelAnimationFrame(t)},[At]);let Nt=!!c&&qe,Pt=Np(100-(At===null?0:jp[At])-(Nt?Mp:0)),Ft=(0,F.useCallback)(()=>C(oe(S)),[S]),It=(0,F.useCallback)(()=>{A.clearMessages(),re(null),ve(),ot()},[A.clearMessages,re,ve,ot]),Lt=(0,F.useCallback)(()=>{let e=M.state.sessionId;if(e===null){D(`Could not open the upload form because the session id is not known yet.`,`error`);return}pe(`/api/mock/${e}`)},[M.state.sessionId,pe,D]),Rt=(0,F.useCallback)(()=>{if(pt!==null){D(`Close the node editor first.`,`info`);return}de!==null&&me(),lt.isOpen&&lt.close(),ct.open()},[pt,de,me,ct.open,lt,D]),zt=(0,F.useCallback)((e=`input`,t=!1)=>{if(pt!==null){if(!t){D(`Close the node editor first.`,`info`);return}if(pt.phase===`sending`)return;dt.close()}if(de!==null&&me(),ct.isOpen){if(ct.busy){D(`Wait for the graph set to finish packing.`,`info`);return}ct.close()}lt.open(e)},[pt,dt,de,me,ct,lt,D]),Bt=(0,F.useCallback)(()=>zt(`input`),[zt]),Vt=pt!==null&&pt.action===`edit-node`?Object.keys(ql).find(e=>ql[e]===pt.formState.alias.trim())??null:null;return(0,R.jsxs)(`div`,{className:ie.wrapper,children:[(0,R.jsx)(Hr,{toasts:E,onRemove:O}),be&&P&&(0,R.jsx)(np,{defaultGraphId:eo(st),supportsFolderPicker:oo(),onConfirm:Ce,onCancel:()=>xe(!1)}),ye.pending&&(0,R.jsx)(rp,{pending:ye.pending,onReplace:ye.confirmPending,onCancel:ye.cancelPending}),mt!==null&&(0,R.jsx)($u,{formState:mt.formState,phase:mt.phase,lockReason:mt.phase===`sending`?`sending`:mt.connectionLost?`disconnected`:null,serverMessage:mt.serverMessage,validationErrors:dt.validationErrors,anchor:ht,onFormStateChange:dt.updateFormState,onSubmit:dt.submit,onClose:dt.close}),(0,R.jsxs)(`header`,{className:ie.header,children:[(0,R.jsx)(`h1`,{className:ie.title,children:t}),(0,R.jsxs)(`div`,{className:ie.headerActions,children:[o&&(0,R.jsx)(ri,{disabled:!P,defaultName:it,savedName:at,onSave:Tt,nameExists:rt.hasGraph,connected:A.connected}),o&&rt.savedGraphs.length>0&&(0,R.jsx)(ai,{savedGraphs:rt.savedGraphs,onLoad:Et,onDelete:rt.deleteGraph,connected:A.connected}),(0,R.jsx)(`button`,{className:ie.panelToggle,onClick:()=>{if(pt!==null){pt.phase!==`sending`&&(dt.close(),Ue(!0));return}if(de!==null){me(),Ue(!0);return}if(ct.isOpen){ct.busy||(ct.close(),Ue(!0));return}if(lt.isOpen){lt.busy||(lt.close(),Ue(!0));return}Ue(e=>!e)},"aria-label":pt===null?de===null?ct.isOpen?`Show console panel and close the graph-set panel`:lt.isOpen?`Show console panel and close the schema panel`:He?`Hide console panel`:`Show console panel`:`Show console panel and close the upload form`:`Show console panel and close the node editor`,"aria-pressed":Ct,title:pt===null?de===null?ct.isOpen?`Closes the graph-set panel`:lt.isOpen?`Closes the schema panel`:void 0:`Closes the upload form`:`Closes the node editor`,children:`Console`}),c&&(0,R.jsxs)(`button`,{className:ie.panelToggle,onClick:()=>Je(e=>!e),"aria-label":qe?`Close workspace sidebar`:`Open workspace sidebar`,"aria-pressed":qe,children:[`Workspace`,Ke.items.length>0?` (${Ke.items.length})`:``]}),(0,R.jsx)(ei,{addToast:D,sessionCollaboration:p?M:null,onPackageGraphs:d?Rt:void 0,onEditSchema:d?Bt:void 0}),l&&(0,R.jsxs)(`div`,{className:ie.helpButtonWrapper,children:[(0,R.jsx)(`button`,{className:`${ie.helpToggle}${Ie&&!Re?` ${ie.helpTogglePulsing}`:``}`,onClick:()=>Fe(e=>!e),"aria-label":Pe?`Close help panel`:`Open help panel`,"aria-pressed":Pe,children:`?`}),Ie&&(0,R.jsxs)(`div`,{className:`${ie.helpHint}${Re?` ${ie.helpHintFading}`:``}`,onClick:Ve,role:`status`,children:[(0,R.jsx)(`kbd`,{className:ie.helpHintKbd,children:"Ctrl + `"}),` to toggle help`]})]})]})]}),Ze&&(0,R.jsx)(ep,{existingItem:Ze.existingItem,pendingItem:Ze.pendingItem,onReplace:async()=>{try{await Ke.confirmReplace(Ze.pendingItem,Ze.existingItem.id),Qe(null),D(`Clipboard item "${Ze.pendingItem.node.alias}" replaced`,`success`)}catch(e){D(`Replace failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},onCancel:()=>{Qe(null),D(`Clip cancelled`,`info`)}}),(0,R.jsxs)(te,{className:ie.panelGroup,orientation:kt?`vertical`:`horizontal`,children:[At!==null&&(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(ee,{panelRef:jt,defaultSize:Np(jp[At]),minSize:`25%`,children:pt===null?de===null?ct.isOpen?(0,R.jsx)(Vl,{controller:ct,supportsFolderPicker:oo()}):lt.isOpen?(0,R.jsx)(Xu,{controller:lt,graphName:st,supportsFolderPicker:oo()}):(0,R.jsx)(il,{messages:A.messages,classificationMap:ne,onCopy:A.copyMessages,onClear:It,consoleRef:A.consoleRef,command:A.command,onCommandChange:A.setCommand,onCommandKeyDown:A.handleKeyDown,onSend:A.sendCommand,sendDisabled:!A.connected||!A.command.trim(),inputDisabled:!A.connected,commandHistory:A.history,onGraphLinkMessage:Dt,onCopyMessage:()=>D(`Copied to clipboard`,`success`),onSendToJsonPath:Ot,onUploadMockData:pe,successfulUploadPaths:fe}):(0,R.jsx)(Cl,{uploadPath:de,onSuccess:he,onClose:me,onError:ge,description:`The JSON becomes the instance's input.body, the optional headers its input.header and the optional model variables its model.{name}. In a shared session every member's instance receives them.`,inputPathHints:wt.inputBodyPaths,inputHeaderHints:wt.inputHeaderNames},de):(0,R.jsx)(ul,{mode:pt.action===`edit-node`?`edit`:`create`,formState:pt.formState,phase:pt.phase,lockReason:pt.phase===`sending`?`sending`:pt.connectionLost?`disconnected`:null,serverMessage:pt.serverMessage,validationErrors:dt.validationErrors,onFormStateChange:dt.updateFormState,onSubmit:dt.submit,onClose:dt.close,onOpenSchemaPanel:Vt===null?void 0:()=>zt(Vt,!0)})}),(0,R.jsx)(j,{className:ie.resizeHandle,"aria-label":`Resize panels`})]}),(0,R.jsx)(ee,{defaultSize:Pt,minSize:`20%`,children:(0,R.jsx)(Wc,{tabs:m,payload:S,onChange:C,validation:w,onFormat:Ft,onUpload:s?A.uploadPayload:void 0,graphData:P,graphName:st,activeTab:ae,onTabChange:I,onGraphRenderError:e=>D(e,`error`),onGraphDataCopySuccess:()=>D(`Graph JSON copied to clipboard!`,`success`),onGraphDataCopyError:()=>D(`Copy failed`,`error`),graphRunControls:f?{phase:wt.phase,canInstantiate:wt.canInstantiate,canUpload:wt.canUpload,canRun:wt.canRun,disabledReason:wt.disabledReason,inputBodyPaths:wt.inputBodyPaths,inputHeaderNames:wt.inputHeaderNames,onInstantiate:wt.instantiateGraph,onUpload:Lt,onRun:wt.runGraph}:void 0,isGraphRefreshing:le,onClipNode:c?tt:void 0,onClipNodes:c?nt:void 0,onClipboardDrop:c?et:void 0,onImportFiles:d?ye.importFiles:void 0,onImportGraph:d?ye.openFilePicker:void 0,onDownloadGraph:m.includes(`graph`)?Se:void 0,onGraphImageSaved:e=>D(`Image saved as ${e}`,`success`),onGraphImageError:e=>D(`Image export failed: ${e}`,`error`),isConnected:A.connected,supportsAuthoring:d,onCreateNode:d?dt.openCreateNode:void 0,onCreateConnection:d?vt:void 0,onEditNode:d?bt:void 0,onDeleteNode:d?xt:void 0,onDeleteNodes:d?dt.deleteNodes:void 0,onDeleteConnections:d?yt:void 0,panelLayoutKey:`${At??`closed`}|${qe}|${Pe}`,helpPanel:l&&Pe?((e,t)=>(0,R.jsx)($f,{activeTopic:Me,contentProfile:u,onNavigate:Ne,onClose:()=>Fe(!1),onToggleMaximize:e,isMaximized:t})):void 0})}),c&&qe&&(0,R.jsxs)(R.Fragment,{children:[(0,R.jsx)(j,{className:ie.resizeHandle,"aria-label":`Resize clipboard`}),(0,R.jsx)(ee,{defaultSize:Np(Mp),minSize:`10%`,maxSize:`40%`,children:(0,R.jsx)(Jf,{connected:A.connected,onPasteToInput:$e})})]})]})]})}function Fp(){let e=ue[0].path;return(0,R.jsx)(ge,{children:(0,R.jsx)(Ef,{children:(0,R.jsx)(D,{children:(0,R.jsxs)(S,{children:[ue.map(e=>(0,R.jsx)(C,{path:e.path,element:(0,R.jsx)(Pp,{config:e},e.path)},e.path)),(0,R.jsx)(C,{path:`*`,element:(0,R.jsx)(E,{to:e,replace:!0})})]})})})})}(0,re.createRoot)(document.getElementById(`root`)).render((0,R.jsx)(F.StrictMode,{children:(0,R.jsx)(Fp,{})}));
//# sourceMappingURL=index-Dhz_Pqc_.js.map
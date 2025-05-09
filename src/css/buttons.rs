pub const CSS: &str = ".growable {
   transform: scale(1);
}
.growable:hover {
   transform: scale(1.16);
}
.info-btn {
   transition-duration: 0.18s;
   filter: brightness(100%);
	color: white;
	background: transparent;
   min-width: 31px;
   min-height: 31px;
   transform: scale(1);
   margin-top: 5px;
   margin-bottom: 5px;
   font-family: 'Segoe UI', Arial, 'Helvetica Neue', Helvetica, 'Noto Sans', sans-serif;
}
.info-btn:active {
   transition-duration: 0.05s;
   transform: scale(1);
   filter: brightness(50%);
}
.next-btn {
   transition-duration: 0.165s;
   border-radius: 0px 36px 36px 0px;
   background: transparent;
   opacity: 0%;
}
.next-btn:hover {
   background-color: rgba(0, 0, 0, 0.08);
}
.next-btn:active {
   transform: scale(0.92);
}
.next-btn.focus {
   opacity: 100%;
}
.previous-btn {
   transition-duration: 0.165s;
   border-radius: 36px 0px 0px 36px;
   background: transparent;
   opacity: 0%;
}
.previous-btn:hover {
   background-color: rgba(0, 0, 0, 0.08);
}
.previous-btn:active {
   transform: scale(0.92);
}
.previous-btn.focus {
   opacity: 100%;
}
.launch-btn {
   font-size: 25px;
   text-shadow: 0px 0px 10px black;
   transition-duration: 0.25s;
   border-radius: 100px;
   min-height: 60px;
   margin: 8px;
   transform: scale(1);
   filter: brightness(100%);
}
.launch-btn.enabled {
   background: rgba(0, 255, 210, 0.2);
   border: 1px solid rgba(0, 255, 210, 0.53);
   box-shadow: 0px 0px 14px rgba(0, 255, 210, 0.38);
}
.launch-btn.enabled:hover {
   background: rgba(0, 255, 210, 0.3);
}
.launch-btn.disabled {
   background: rgba(255, 255, 255, 0.2);
   border: 1px solid rgba(255, 255, 255, 0.4);
   box-shadow: 0px 0px 14px rgba(255, 255, 255, 0.38);
}
.launch-btn.disabled:hover {
   background: rgba(255, 255, 255, 0.3);
}
.launch-btn:active {
   transition-duration: 0.045s;
   transform: scale(0.96);
   filter: brightness(50%);
}
.mods-btn {
   transition-duration: 0.25s;
   border: 1px solid rgba(0, 255, 210, 0.53);
   background: rgba(0, 0, 0, 0.2);
   border-radius: 12px;
   transform: scale(1);
   margin-left: 48px;
   margin-right: 48px;
   filter: brightness(100%);
}
.mods-btn:hover {
   background: rgba(0, 255, 210, 0.18);
}
.mods-btn:active {
   transition-duration: 0.045s;
   transform: scale(0.96);
   filter: brightness(50%);
}
";
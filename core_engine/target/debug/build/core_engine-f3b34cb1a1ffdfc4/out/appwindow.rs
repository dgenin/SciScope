mod slint_generatedAppWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_17_0 = slint :: VersionCheck_1_17_0 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFluentPalette_78 {
         r#accent_background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_78 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (_self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4284534271f64) as u32) as _)) as _ }
                     else {
                         _self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4278214584f64) as u32) as _) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let _root = _self . globals . get () . unwrap () . upgrade () . unwrap () . root_item_tree_weak . upgrade () . unwrap () ;
                         sp :: context_for_root (& _root) . map_or (sp :: ColorScheme :: Unknown , | c | c . color_scheme (Some (& _root))) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_78_color_scheme = ({
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#color_scheme () }
                         . apply_pin (_self) . get ()) . clone () ;
                         if ! ((((r#tmp_FluentPalette_78_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Unknown) . clone ()))) {
                             (((((r#tmp_FluentPalette_78_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())))) as _ }
                         else {
                             ((({
                                 let _root = _self . globals . get () . unwrap () . upgrade () . unwrap () . root_item_tree_weak . upgrade () . unwrap () ;
                                 sp :: context_for_root (& _root) . map_or (sp :: ColorScheme :: Unknown , | c | c . color_scheme (Some (& _root))) }
                            ) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())) }
                         }
                    ) as _ }
                ) ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_accentify (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Color ,) -> sp :: Color {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let r#local_accent_color = (sp :: accent_color (& _self . globals . get () . unwrap () . upgrade () . unwrap () . root_item_tree_weak . upgrade () . unwrap ())) . clone () ;
                 if ! ((((((r#local_accent_color) . clone () . to_argb_u8 ()) . r#alpha) . clone ()) as f64) > (((0f64) . clone ()) as f64)) {
                     (args . 0 . clone ()) as _ }
                 else {
                     {
                         let r#local_default_lch = ((args . 0 . clone ()) . clone () . to_oklch ()) . clone () ;
                         let r#local_accent_lch = ((r#local_accent_color) . clone () . to_oklch ()) . clone () ;
                         {
                             let l : f32 = (((r#local_default_lch) . r#lightness) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             let c : f32 = (((r#local_accent_lch) . r#chroma) . clone () as f32) . max (0.) as f32 ;
                             let alpha : f32 = ((1f64) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             sp :: Color :: from_oklch (l , c , ((r#local_accent_lch) . r#hue) . clone () as f32 , alpha) }
                         }
                     }
                 }
            ) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_1 {
         r#root_1 : sp :: r#BasicBorderRectangle , r#rectangle_2 : sp :: r#BasicBorderRectangle , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerCheckBox_root_3 {
         r#root_3 : sp :: r#Empty , r#background_5 : sp :: r#BasicBorderRectangle , r#border_6 : sp :: r#BasicBorderRectangle , r#icon_visibility_7 : sp :: r#Clip , r#icon_8 : sp :: r#ImageItem , r#touch_area_11 : sp :: r#TouchArea , r#focus_scope_12 : sp :: r#FocusScope , r#root_3_accessible_checked : sp :: Property < bool > , r#root_3_background_5_height : sp :: Property < sp :: LogicalLength > , r#root_3_background_5_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_has_focus : sp :: Property < bool > , r#root_3_height : sp :: Property < sp :: LogicalLength > , r#root_3_icon_8_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_3_layout_4_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_3_layout_4_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_3_layout_4_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_layout_4_min_height : sp :: Property < sp :: LogicalLength > , r#root_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_min_height : sp :: Property < sp :: LogicalLength > , r#root_3_state : sp :: Property < i32 > , r#root_3_text : sp :: Property < sp :: SharedString > , r#root_3_text_color : sp :: Property < sp :: Color > , r#root_3_width : sp :: Property < sp :: LogicalLength > , r#root_3_x : sp :: Property < sp :: LogicalLength > , r#root_3_y : sp :: Property < sp :: LogicalLength > , r#root_3_accessible_action_default : sp :: Callback < () , () > , r#root_3_toggled : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_text_9 > , repeater1 : sp :: Conditional < InnerComponent_focusborder_13 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerCheckBox_root_3 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerCheckBox_root_3 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) . clone ())) != (((sp :: SharedString :: from ("")) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_has_focus ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                                 ({
                                     (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . set ((! (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get ()) as _) ;
                                     {
                                         (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) . call (& ()) ;
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + (((sp :: Item :: layout_info ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_has_focus ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_icon_8_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ((((({
                     let r#image_implicit_size = ((sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) . clone () . size ()) . clone () ;
                     ((((r#image_implicit_size) . r#height) . clone ()) as f64) / ((((r#image_implicit_size) . r#width) . clone ()) as f64) }
                ) . clone ()) as f64) * (((12f64) . clone ()) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_icon_8_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (18f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (18f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (0f64) . clone () as _ ;
                                 the_struct . r#end = (0f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (12f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_icon_8_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (18f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (18f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 12f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (18f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (18f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_0 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info_0) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((18f64 as sp :: Coord) . max (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_focus_scope_12_enabled = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone () ;
                         if ! r#tmp_focus_scope_12_enabled {
                             (1f64) as _ }
                         else {
                             if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get () {
                                 (2f64) as _ }
                             else {
                                 if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                                     (3f64) as _ }
                                 else {
                                     if ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get ()) . clone ())) && (((r#tmp_focus_scope_12_enabled) . clone ())) {
                                         (4f64) as _ }
                                     else {
                                         0f64 }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                        ) . color ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                        ) . color () }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (200f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                                     (({
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.8f64) . clone () as f32)) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((318767103f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((385875968f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                     (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                                         (({
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.9f64) . clone () as f32)) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((251658240f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                         ({
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((436207616f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((83886080f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (slint :: Brush :: SolidColor (if {
                                 * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2583691263f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                         (0f64) as _ }
                     else {
                         1f64 }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#clip ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (slint :: Brush :: SolidColor (if {
                                 * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                                 ({
                                     (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . set ((! (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get ()) as _) ;
                                     {
                                         (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) . call (& ()) ;
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from (" ")) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\n")) . clone ())))) . clone ()))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     {
                                         (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) ;
                                         }
                                     ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 () + sp :: r#Clip :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . visit (order , visitor) }
                 1u32 => {
                     InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             {
                 _changed |= InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((18f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((18f64) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 6u32 => (((18f64) . clone ()) . clone () as sp :: Coord , ((18f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 7u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 8u32 => ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((12f64) . clone ()) . clone () as sp :: Coord , ((3f64) . clone ()) . clone () as sp :: Coord , ((((((((((18f64) . clone ()) as f64) - ((((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Checkbox , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_background_5_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((sp :: Item :: layout_info ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , args . 0 . clone () as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_layout_4_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                 items_vec . push ({
                     let mut the_struct = sp :: LayoutItemInfo :: default () ;
                     the_struct . r#constraint = ({
                         let r#layout_info = (_self . r#fn_background_5_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone () as _ ;
                     the_struct }
                ) ;
                 InnerCheckBox_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater0 . len () {
                     if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                 sp :: r#box_layout_info_ortho (r#cells as _ , & {
                     let mut the_struct = sp :: Padding :: default () ;
                     the_struct . r#begin = (0f64) . clone () as _ ;
                     the_struct . r#end = (0f64) . clone () as _ ;
                     the_struct }
                 as _) }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let r#layout_info_0 = ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone () ;
                 {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                     the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                     the_struct . r#min = ((18f64 as sp :: Coord) . max ((InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_min_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) . clone () as _ ;
                     the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                     the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                     the_struct . r#stretch = ((r#layout_info_0) . r#stretch) . clone () as _ ;
                     the_struct }
                 }
            ) . clone ())) + (((_self . r#fn_layout_4_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_9 {
         r#text_9 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_9 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_9 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Left) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_9 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_9 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_9) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_9 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_9 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_9 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_9 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 2u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_9 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_9 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_13 {
         r#focusborder_13 : InnerFocusBorder_root_1 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_13 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_13 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerCheckBox_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_13 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) , sp :: VOffset :: new (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_13) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_13 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_13 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_13 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_13 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerSliderBase_root_15 {
         r#root_15 : sp :: r#Empty , r#touch_area_16 : sp :: r#TouchArea , r#focus_scope_17 : sp :: r#FocusScope , r#root_15_handle_has_hover : sp :: Property < bool > , r#root_15_handle_height : sp :: Property < sp :: LogicalLength > , r#root_15_handle_pressed : sp :: Property < bool > , r#root_15_handle_width : sp :: Property < sp :: LogicalLength > , r#root_15_handle_x : sp :: Property < sp :: LogicalLength > , r#root_15_handle_y : sp :: Property < sp :: LogicalLength > , r#root_15_height : sp :: Property < sp :: LogicalLength > , r#root_15_maximum : sp :: Property < f32 > , r#root_15_minimum : sp :: Property < f32 > , r#root_15_orientation : sp :: Property < sp :: r#Orientation > , r#root_15_ref_size : sp :: Property < sp :: LogicalLength > , r#root_15_step : sp :: Property < f32 > , r#root_15_touch_area_16_pressed_value : sp :: Property < f32 > , r#root_15_value : sp :: Property < f32 > , r#root_15_width : sp :: Property < sp :: LogicalLength > , r#root_15_changed : sp :: Callback < (f32 ,) , () > , r#root_15_released : sp :: Callback < (f32 ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerSliderBase_root_15 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerSliderBase_root_15 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_has_hover ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_15_handle_x = ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_x ()) . apply_pin (_self) . get () . get ()) . clone () ;
                         let r#tmp_root_15_handle_y = ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_y ()) . apply_pin (_self) . get () . get ()) . clone () ;
                         let r#tmp_touch_area_16_mouse_x = ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone () ;
                         let r#tmp_touch_area_16_mouse_y = ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone () ;
                         (((((((((((((((r#tmp_touch_area_16_mouse_x) . clone ()) as f64) >= (((r#tmp_root_15_handle_x) . clone ()) as f64))) . clone ())) && (((((((r#tmp_touch_area_16_mouse_x) . clone ()) as f64) <= (((((((r#tmp_root_15_handle_x) . clone ()) as f64) + ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64))) . clone ())))) . clone ())) && (((((((r#tmp_touch_area_16_mouse_y) . clone ()) as f64) >= (((r#tmp_root_15_handle_y) . clone ()) as f64))) . clone ())))) . clone ())) && (((((((r#tmp_touch_area_16_mouse_y) . clone ()) as f64) <= (((((((r#tmp_root_15_handle_y) . clone ()) as f64) + ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64))) . clone ())) }
                    ) as _ }
                ) ;
                 }
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . set ({
                 (100f64) as f32 }
            ) ;
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_ref_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))) {
                         ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_width ()) . apply_pin (_self) . get () . get ()) as _ }
                     else {
                         (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_height ()) . apply_pin (_self) . get () . get () }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_step ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (! (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) {
                                 ({
                                     {
                                         _self . r#fn_set_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_touch_area_16_pressed_value ()) . apply_pin (_self) . get ()) . clone ()) as f64) + (((if ! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))) {
                                             (_self . r#fn_size_to_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as _ , (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_ref_size ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as _)) as _ }
                                         else {
                                             _self . r#fn_size_to_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as _ , (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_ref_size ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as _) }
                                        ) . clone ()) as f64)) as _) }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((args . 0 . clone ()) . r#button) . clone ())) != (((sp :: r#PointerEventButton :: r#Left) . clone ()))) {
                                 ({
                                     {
                                         if ! (((((args . 0 . clone ()) . r#kind) . clone ())) == (((sp :: r#PointerEventKind :: r#Up) . clone ()))) {
                                             ({
                                                 {
                                                     if ! (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_has_hover ()) . apply_pin (_self) . get () {
                                                         ({
                                                             _self . r#fn_set_value (((((if ! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))) {
                                                                 (_self . r#fn_size_to_value ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x ()) . apply_pin (_self) . get () . get () as _ , (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get () as _)) as _ }
                                                             else {
                                                                 _self . r#fn_size_to_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as _ , (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get () as _) }
                                                            ) . clone ()) as f64) + ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) . clone ()) as f64)) as _) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_touch_area_16_pressed_value ()) . apply_pin (_self) . set ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get () as _) ;
                                                     sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true , sp :: FocusReason :: Programmatic) ;
                                                     (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_pressed ()) . apply_pin (_self) . set (true as _) }
                                                 }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 if (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_pressed ()) . apply_pin (_self) . get () {
                                                     ({
                                                         {
                                                             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_released ()) . apply_pin (_self) . call (& (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                                                             }
                                                         }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         }
                                                     }
                                                 ;
                                                 (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_pressed ()) . apply_pin (_self) . set (false as _) }
                                             }
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((! (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ())) . clone ())) || ((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_step ()) . apply_pin (_self) . get ()) . clone ()) as f64) <= (((0f64) . clone ()) as f64))) . clone ()))) {
                                 ({
                                     {
                                         let r#returned_expression0 = ({
                                             let r#return_check_merge0 = (if ((((((((! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f703}")) . clone ())))) . clone ())))) . clone ())) || ((((((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f700}")) . clone ())))) . clone ())))) . clone ())) {
                                                 ((((false) . clone ()) . clone () , (({
                                                     _self . r#fn_increment () ;
                                                     sp :: r#EventResult :: r#Accept }
                                                ) . clone ()) . clone () ,)) as _ }
                                             else {
                                                 if ((((((((! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f702}")) . clone ())))) . clone ())))) . clone ())) || ((((((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f701}")) . clone ())))) . clone ())))) . clone ())) {
                                                     ((((false) . clone ()) . clone () , (({
                                                         _self . r#fn_decrement () ;
                                                         sp :: r#EventResult :: r#Accept }
                                                    ) . clone ()) . clone () ,)) as _ }
                                                 else {
                                                     if ((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f729}")) . clone ())) {
                                                         ((((false) . clone ()) . clone () , (({
                                                             _self . r#fn_set_value ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get () as _) ;
                                                             sp :: r#EventResult :: r#Accept }
                                                        ) . clone ()) . clone () ,)) as _ }
                                                     else {
                                                         if ! (((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f72b}")) . clone ()))) {
                                                             ((((true) . clone ()) . clone () , ((sp :: r#EventResult :: r#Reject) . clone ()) . clone () ,)) as _ }
                                                         else {
                                                             (((false) . clone ()) . clone () , (({
                                                                 _self . r#fn_set_value ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get () as _) ;
                                                                 sp :: r#EventResult :: r#Accept }
                                                            ) . clone ()) . clone () ,) }
                                                         }
                                                     }
                                                 }
                                            ) . clone () ;
                                             if (r#return_check_merge0) . 0 {
                                                 (((({
                                                     sp :: r#EventResult :: r#Reject }
                                                ) . clone ()) . clone () , ((true) . clone ()) . clone () , ((sp :: r#EventResult :: r#Reject) . clone ()) . clone () ,)) as _ }
                                             else {
                                                 (((sp :: r#EventResult :: r#Reject) . clone ()) . clone () , ((false) . clone ()) . clone () , (((r#return_check_merge0) . 1) . clone ()) . clone () ,) }
                                             }
                                        ) . clone () ;
                                         if (r#returned_expression0) . 1 {
                                             ((r#returned_expression0) . 0) as _ }
                                         else {
                                             (r#returned_expression0) . 2 }
                                         }
                                     }
                                ) as _ }
                             else {
                                 {
                                     sp :: r#EventResult :: r#Reject }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_released ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (! (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) {
                                 ({
                                     {
                                         if ((((((((((((((((((((((((! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f703}")) . clone ())))) . clone ())))) . clone ())) || ((((((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f701}")) . clone ())))) . clone ())))) . clone ())))) . clone ())) || ((((((((! (((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ()))))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f702}")) . clone ())))) . clone ())))) . clone ())))) . clone ())) || ((((((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())))) . clone ())) && ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f700}")) . clone ())))) . clone ())))) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f729}")) . clone ())))) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\u{f72b}")) . clone ())))) . clone ())) {
                                             ({
                                                 {
                                                     (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_released ()) . apply_pin (_self) . call (& (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                                                     }
                                                 }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 }
                                             }
                                         ;
                                         sp :: r#EventResult :: r#Accept }
                                     }
                                ) as _ }
                             else {
                                 {
                                     sp :: r#EventResult :: r#Reject }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) , sp :: Orientation :: Vertical => ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((((((1f64) . clone ()) as f64) * ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_decrement (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (_self . r#fn_set_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_step ()) . apply_pin (_self) . get ()) . clone ()) as f64)) as _)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_increment (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (_self . r#fn_set_value ((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone ()) as f64) + ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_step ()) . apply_pin (_self) . get ()) . clone ()) as f64)) as _)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_value (self : :: core :: pin :: Pin < & Self > , arg_0 : f32 ,) -> () {
             let _self = self ;
             let args = (arg_0 ,) ;
             (if ! sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((args . 0 . clone ()) . clone () as f64)) {
                 ({
                     (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . set (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get () as f32) . max (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get () as f32) . min (args . 0 . clone () as f32) as f32) as _) ;
                     {
                         (InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_changed ()) . apply_pin (_self) . call (& (((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone () as _ ,)) ;
                         }
                     }
                ) ;
                 }
             else {
                 {
                     }
                 }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_size_to_value (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord , arg_1 : sp :: Coord ,) -> f32 {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             (((((((((args . 0 . clone ()) . clone ()) as f64) * ((((((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) . clone ()) as f64) - ((((InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64) / (((args . 1 . clone ()) . clone ()) as f64))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerSlider_root_18 {
         r#root_18 : sp :: r#Empty , r#rail_19 : sp :: r#BasicBorderRectangle , r#track_20 : sp :: r#BasicBorderRectangle , r#thumb_21 : sp :: r#BasicBorderRectangle , r#thumb_border_22 : sp :: r#BasicBorderRectangle , r#thumb_inner_23 : sp :: r#BasicBorderRectangle , r#base_24 : InnerSliderBase_root_15 , r#root_18_accessible_value_step : sp :: Property < f32 > , r#root_18_height : sp :: Property < sp :: LogicalLength > , r#root_18_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_18_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_18_rail_19_height : sp :: Property < sp :: LogicalLength > , r#root_18_rail_19_width : sp :: Property < sp :: LogicalLength > , r#root_18_rail_19_x : sp :: Property < sp :: LogicalLength > , r#root_18_rail_19_y : sp :: Property < sp :: LogicalLength > , r#root_18_state : sp :: Property < i32 > , r#root_18_thumb_21_x : sp :: Property < sp :: LogicalLength > , r#root_18_thumb_21_y : sp :: Property < sp :: LogicalLength > , r#root_18_thumb_inner_23_width : sp :: Property < sp :: LogicalLength > , r#root_18_track_20_height : sp :: Property < sp :: LogicalLength > , r#root_18_track_20_width : sp :: Property < sp :: LogicalLength > , r#root_18_track_20_x : sp :: Property < sp :: LogicalLength > , r#root_18_track_20_y : sp :: Property < sp :: LogicalLength > , r#root_18_width : sp :: Property < sp :: LogicalLength > , r#root_18_x : sp :: Property < sp :: LogicalLength > , r#root_18_y : sp :: Property < sp :: LogicalLength > , r#root_18_accessible_action_decrement : sp :: Callback < () , () > , r#root_18_accessible_action_increment : sp :: Callback < () , () > , r#root_18_accessible_action_set_value : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerSlider_root_18 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerSlider_root_18 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerSliderBase_root_15 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 4u32 - 1 , tree_index_of_first_child + 7u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_decrement ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . r#fn_decrement () }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_increment ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . r#fn_increment () }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_set_value ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if sp :: string_to_float ((args . 0 . clone ()) . clone () . as_str ()) . is_some () {
                                 ({
                                     InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . r#fn_set_value (sp :: string_to_float ((args . 0 . clone ()) . clone () . as_str ()) . unwrap_or_default () as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_value_step ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_step ()) . apply_pin (_self) . get () as f32) . min ((((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64) / (((100f64) . clone ()) as f64)) as f32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_1 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_1) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_1) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                 (20f64) as _ }
                             else {
                                 0f64 }
                            ) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_1) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_1) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                 (0f64) as _ }
                             else {
                                 1f64 }
                            ) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#stretch) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_2 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_2) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_2) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                 (0f64) as _ }
                             else {
                                 20f64 }
                            ) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_2) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_2) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                 (1f64) as _ }
                             else {
                                 0f64 }
                            ) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ()))) . r#stretch) . clone () as _ ;
                         the_struct }
                    ) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) as _ }
                     else {
                         4f64 }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         (4f64) as _ }
                     else {
                         ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) as _ }
                     else {
                         10f64 }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         (10f64) as _ }
                     else {
                         ((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_pressed ()) . apply_pin (_self) . get ()) . clone ())) || ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) . clone ())) {
                             (2f64) as _ }
                         else {
                             if (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 0f64 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) as _ }
                     else {
                         {
                             let r#tmp_base_24_minimum = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) . clone () ;
                             let r#tmp_root_18_width = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone () ;
                             (0f64 as sp :: Coord) . max ((((((((((((((r#tmp_root_18_width) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) * ((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone ()) as f64) - (((r#tmp_base_24_minimum) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64) / ((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) . clone ()) as f64) - (((r#tmp_base_24_minimum) . clone ()) as f64))) . clone ()) as f64)) as sp :: Coord) . min (((((r#tmp_root_18_width) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord) as sp :: Coord) }
                         }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ({
                             let r#tmp_base_24_maximum = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) . clone () ;
                             let r#tmp_root_18_height = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone () ;
                             (0f64 as sp :: Coord) . max ((((((((((((((r#tmp_root_18_height) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) * (((((((r#tmp_base_24_maximum) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64))) . clone ()) as f64) / (((((((r#tmp_base_24_maximum) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) . clone ()) as f64))) . clone ()) as f64)) as sp :: Coord) . min (((((r#tmp_root_18_height) . clone ()) as f64) - (((20f64) . clone ()) as f64)) as sp :: Coord) as sp :: Coord) }
                        ) as _ }
                     else {
                         ((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((20f64) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_18_state = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_18_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                             (10f64) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_18_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                 (14f64) as _ }
                             else {
                                 12f64 }
                             }
                         }
                     as sp :: Coord)) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) - (((20f64) . clone ()) as f64))) as _ }
                     else {
                         (InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_height ()) . apply_pin (_self) . get () . get () }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_width ()) . apply_pin (_self) . get () . get ()) as _ }
                     else {
                         (InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_x ()) . apply_pin (_self) . get () . get () }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) as _ }
                     else {
                         10f64 }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                         ((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_y ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) + (((10f64) . clone ()) as f64))) as _ }
                     else {
                         ((((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((1f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                        )) as _ }
                     else {
                         {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () }
                    ) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4282729797f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) . get ()) . clone () as f64) , & ((2f64) . clone () as f64)) {
                         (slint :: Brush :: SolidColor (if {
                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((318767103f64) as u32) , position : 1f64 as _ }
                        ]))) as _ }
                     else {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                        ])) }
                    ) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_inner_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_18_state = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_18_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (slint :: Brush :: SolidColor (if {
                                 * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                             . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_18_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (({
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.8f64) . clone () as f32)) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_18_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                     (({
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.9f64) . clone () as f32)) as _ }
                                 else {
                                     {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_inner_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) / (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_x ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_y ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_inner_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_inner_23 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_height ()) . apply_pin (_self) . set_constant () ;
             (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_handle_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerSliderBase_root_15 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                             (20f64) as _ }
                         else {
                             0f64 }
                        ) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                             (0f64) as _ }
                         else {
                             1f64 }
                        ) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                             (0f64) as _ }
                         else {
                             20f64 }
                        ) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (if ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                             (1f64) as _ }
                         else {
                             0f64 }
                        ) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_rail_19_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_track_20_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 3u32 => (((20f64) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_21_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 4u32 => (((((((1f64) . clone ()) as f64) * ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => (((21f64) . clone ()) . clone () as sp :: Coord , ((21f64) . clone ()) . clone () as sp :: Coord , ((- 0.5f64) . clone ()) . clone () as sp :: Coord , ((- 0.5f64) . clone ()) . clone () as sp :: Coord ,) , 6u32 => ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((((((((((20f64) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((((((20f64) . clone ()) as f64) - ((((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_thumb_inner_23_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , 7u32 ..= 8u32 => return InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . item_geometry (index - 7u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Slider , 4u32 => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessible_role (0) , 7u32 ..= 8u32 => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessible_role (index - 7u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Orientation) => sp :: Some (match (InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get () {
                     sp :: r#Orientation :: r#Horizontal => sp :: SharedString :: from ("horizontal") , sp :: r#Orientation :: r#Vertical => sp :: SharedString :: from ("vertical") , _ => sp :: SharedString :: default () }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (sp :: shared_string_from_number (((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) as f64)) , (0u32 , sp :: AccessibleStringProperty :: r#ValueMaximum) => sp :: Some (sp :: shared_string_from_number (((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) as f64)) , (0u32 , sp :: AccessibleStringProperty :: r#ValueMinimum) => sp :: Some (sp :: shared_string_from_number (((InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) as f64)) , (0u32 , sp :: AccessibleStringProperty :: r#ValueStep) => sp :: Some (sp :: shared_string_from_number (((InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_value_step ()) . apply_pin (_self) . get ()) as f64)) , (4u32 , _) => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessible_string_property (0 , what) , (7u32 ..= 8u32 , _) => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessible_string_property (index - 7u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Decrement) => {
                     {
                         (InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_decrement ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (0u32 , sp :: AccessibilityAction :: r#Increment) => {
                     {
                         (InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_increment ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (0u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     {
                         (InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_set_value ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) ;
                         }
                     }
                 (4u32 , _) => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessibility_action (0 , action) , (7u32 ..= 8u32 , _) => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . accessibility_action (index - 7u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Decrement | sp :: SupportedAccessibilityAction :: r#Increment | sp :: SupportedAccessibilityAction :: r#SetValue , 4u32 => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . supported_accessibility_actions (0) , 7u32 ..= 8u32 => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . supported_accessibility_actions (index - 7u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 7u32 ..= 8u32 => InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () . apply_pin (_self) . item_element_infos (index - 7u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_25 {
         r#root_25 : sp :: r#Empty , r#i_background_26 : sp :: r#BasicBorderRectangle , r#i_border_27 : sp :: r#BasicBorderRectangle , r#i_touch_area_33 : sp :: r#TouchArea , r#i_focus_scope_34 : sp :: r#FocusScope , r#root_25_checked : sp :: Property < bool > , r#root_25_has_focus : sp :: Property < bool > , r#root_25_height : sp :: Property < sp :: LogicalLength > , r#root_25_i_background_26_width : sp :: Property < sp :: LogicalLength > , r#root_25_i_layout_28_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_25_i_layout_28_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_25_i_layout_28_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_25_i_layout_28_min_height : sp :: Property < sp :: LogicalLength > , r#root_25_i_layout_28_padding_bottom : sp :: Property < sp :: LogicalLength > , r#root_25_i_layout_28_padding_top : sp :: Property < sp :: LogicalLength > , r#root_25_icon : sp :: Property < sp :: Image > , r#root_25_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_25_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_25_min_height : sp :: Property < sp :: LogicalLength > , r#root_25_pressed : sp :: Property < bool > , r#root_25_state : sp :: Property < i32 > , r#root_25_text : sp :: Property < sp :: SharedString > , r#root_25_text_color : sp :: Property < slint :: Brush > , r#root_25_vertical_stretch : sp :: Property < f32 > , r#root_25_width : sp :: Property < sp :: LogicalLength > , r#root_25_x : sp :: Property < sp :: LogicalLength > , r#root_25_y : sp :: Property < sp :: LogicalLength > , r#root_25_accessible_action_default : sp :: Callback < () , () > , r#root_25_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_29 > , repeater1 : sp :: Conditional < InnerComponent_text_31 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_35 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_25 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_25 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((((((((sp :: Image :: default ()) . clone () . size ()) . r#width) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())) && (((((((((sp :: Image :: default ()) . clone () . size ()) . r#height) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . get ()) . clone ())) != (((sp :: SharedString :: from ("")) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_has_focus ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_accessible_action_default ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_has_focus ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_background_26_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         4usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [2usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [2usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (12f64) . clone () as _ ;
                                 the_struct . r#end = (12f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_background_26_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (4f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (12f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (5f64) . clone () as _ ;
                             the_struct . r#end = (5f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_bottom ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_top ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_3 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_3) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_3) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_3) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_3) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_4 = ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_4) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_4) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_4) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_4) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_min_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((32f64 as sp :: Coord) . max (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_pressed ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_pressed ()) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if (InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . set ({
                 (0f64) as f32 }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                     (({
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.8f64) . clone () as f32)) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                     (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                         (({
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) . clone () . with_alpha ((0.9f64) . clone () as f32)) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                         ({
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get ()) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state ()) . apply_pin (_self) . get ()) . clone () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_25_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     (if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                    ) as _ }
                                 else {
                                     if {
                                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . set ((! (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             {
                                 (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_clicked ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from (" ")) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\n")) . clone ())))) . clone ()))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     {
                                         (InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) ;
                                         }
                                     ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             {
                 _changed |= InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerButton_root_25 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_i_background_26_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_i_layout_28_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_i_layout_28_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                 InnerButton_root_25 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater0 . len () {
                     if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 InnerButton_root_25 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater1 . len () {
                     if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                 sp :: r#box_layout_info_ortho (r#cells as _ , & {
                     let mut the_struct = sp :: Padding :: default () ;
                     the_struct . r#begin = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_top ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct . r#end = ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_bottom ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct }
                 as _) }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let r#layout_info_4 = ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                     the_struct . r#min = (0f64) . clone () as _ ;
                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                     the_struct . r#preferred = (0f64) . clone () as _ ;
                     the_struct . r#stretch = (1f64) . clone () as _ ;
                     the_struct }
                ) . clone () ;
                 {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = ((r#layout_info_4) . r#max) . clone () as _ ;
                     the_struct . r#max_percent = ((r#layout_info_4) . r#max_percent) . clone () as _ ;
                     the_struct . r#min = ((32f64 as sp :: Coord) . max ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_min_height ()) . apply_pin (_self) . get () . get () as sp :: Coord)) . clone () as _ ;
                     the_struct . r#min_percent = ((r#layout_info_4) . r#min_percent) . clone () as _ ;
                     the_struct . r#preferred = ((r#layout_info_4) . r#preferred) . clone () as _ ;
                     the_struct . r#stretch = (0f64) . clone () as _ ;
                     the_struct }
                 }
            ) . clone ())) + (((_self . r#fn_i_background_26_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_29 {
         r#image_29 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_29 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_29 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon ()) . apply_pin (x . as_pin_ref ())) . map (| x | sp :: Property :: link_two_way ((InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , x)) ;
                 }
             ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (20f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_29 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_29 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_29) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_29 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_29 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_29 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_29 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_29 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_29 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_31 {
         r#text_31 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_31 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_31 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#enabled = (true) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_31 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_31 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_31) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_31 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_31 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_31 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_31 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_31 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_31 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_35 {
         r#focusborder_35 : InnerFocusBorder_root_1 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_35 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_35 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_35 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_35 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_35 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_35 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_35 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_25 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_35 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) , sp :: VOffset :: new (InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_35) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_35 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_35 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_35 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_35 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_35 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_35 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_37 : sp :: r#WindowItem , r#empty_39 : sp :: r#Empty , r#empty_40 : sp :: r#Empty , r#rectangle_44 : sp :: r#Empty , r#empty_46 : sp :: r#Empty , r#empty_49 : sp :: r#Empty , r#text_50 : sp :: r#SimpleText , r#empty_52 : sp :: r#Empty , r#rectangle_56 : sp :: r#Empty , r#empty_58 : sp :: r#Empty , r#empty_62 : sp :: r#Empty , r#rectangle_66 : sp :: r#Empty , r#empty_68 : sp :: r#Empty , r#rectangle_71 : sp :: r#BasicBorderRectangle , r#image_72 : sp :: r#ImageItem , r#checkbox_47 : InnerCheckBox_root_3 , r#checkbox_48 : InnerCheckBox_root_3 , r#slider_51 : InnerSlider_root_18 , r#checkbox_59 : InnerCheckBox_root_3 , r#checkbox_60 : InnerCheckBox_root_3 , r#checkbox_61 : InnerCheckBox_root_3 , r#button_69 : InnerButton_root_25 , r#button_70 : InnerButton_root_25 , r#root_37_awb_enabled : sp :: Property < bool > , r#root_37_empty_38_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_38_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_38_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_38_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_39_alignment : sp :: Property < sp :: r#LayoutAlignment > , r#root_37_empty_39_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_39_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_39_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_39_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_39_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_41_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_41_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_41_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_41_padding_bottom : sp :: Property < sp :: LogicalLength > , r#root_37_empty_41_padding_top : sp :: Property < sp :: LogicalLength > , r#root_37_empty_41_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_45_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_45_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_45_layout_organized_data : sp :: Property < sp :: SharedVector < u16 , > > , r#root_37_empty_45_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_45_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_45_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_46_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_46_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_46_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_46_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_46_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_49_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_49_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_49_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_53_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_53_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_53_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_53_padding_bottom : sp :: Property < sp :: LogicalLength > , r#root_37_empty_53_padding_top : sp :: Property < sp :: LogicalLength > , r#root_37_empty_53_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_57_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_57_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_57_layout_organized_data : sp :: Property < sp :: SharedVector < u16 , > > , r#root_37_empty_57_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_57_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_57_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_58_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_58_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_58_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_58_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_58_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_63_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_63_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_63_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_63_padding_bottom : sp :: Property < sp :: LogicalLength > , r#root_37_empty_63_padding_top : sp :: Property < sp :: LogicalLength > , r#root_37_empty_63_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_empty_67_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_67_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_67_layout_organized_data : sp :: Property < sp :: SharedVector < u16 , > > , r#root_37_empty_67_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_67_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_67_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_68_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_empty_68_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_68_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_empty_68_padding : sp :: Property < sp :: LogicalLength > , r#root_37_empty_68_spacing : sp :: Property < sp :: LogicalLength > , r#root_37_exposure_val : sp :: Property < f32 > , r#root_37_flip_h_enabled : sp :: Property < bool > , r#root_37_flip_v_enabled : sp :: Property < bool > , r#root_37_grid_enabled : sp :: Property < bool > , r#root_37_hdr_enabled : sp :: Property < bool > , r#root_37_image_72_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_37_image_72_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_37_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_rectangle_44_vertical_stretch : sp :: Property < f32 > , r#root_37_rectangle_56_vertical_stretch : sp :: Property < f32 > , r#root_37_rectangle_66_vertical_stretch : sp :: Property < f32 > , r#root_37_video_frame : sp :: Property < sp :: Image > , r#root_37_capture_image : sp :: Callback < () , () > , r#root_37_export_csv : sp :: Callback < () , () > , callback_tracker_root_37_capture_image : sp :: Property < () > , callback_tracker_root_37_export_csv : sp :: Property < () > , repeater0 : sp :: Conditional < InnerComponent_text_42 > , repeater1 : sp :: Conditional < InnerComponent_text_54 > , repeater2 : sp :: Conditional < InnerComponent_text_64 > , repeater3 : sp :: Conditional < InnerComponent_rectangle_73 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAppWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                 }
            ) ;
             _self . repeater3 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             InnerCheckBox_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 12u32 - 1) ;
             InnerCheckBox_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 10u32 - 1 , tree_index_of_first_child + 20u32 - 1) ;
             InnerSlider_root_18 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 29u32 - 1 , tree_index_of_first_child + 30u32 - 1) ;
             InnerCheckBox_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 41u32 - 1 , tree_index_of_first_child + 44u32 - 1) ;
             InnerCheckBox_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 42u32 - 1 , tree_index_of_first_child + 52u32 - 1) ;
             InnerCheckBox_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 43u32 - 1 , tree_index_of_first_child + 60u32 - 1) ;
             InnerButton_root_25 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 71u32 - 1 , tree_index_of_first_child + 73u32 - 1) ;
             InnerButton_root_25 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 72u32 - 1 , tree_index_of_first_child + 80u32 - 1) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_awb_enabled ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4280163870f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (250f64) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (250f64) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (10f64) . clone () as _ ;
                             the_struct . r#end = (10f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (1280f64) . clone () as _ , r#spacing : (10f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (250f64) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (250f64) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (10f64) . clone () as _ ;
                         the_struct . r#end = (10f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (10f64) . clone () as _ ;
                         the_struct . r#end = (10f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#LayoutAlignment :: r#Start) as sp :: r#LayoutAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Start) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (700f64) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (16f64) . clone () as _ ;
                                 the_struct . r#end = (8f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (16f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_bottom ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_top ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (234f64) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Horizontal as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Vertical as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#organize_grid_layout (sp :: Slice :: from_slice (& [(sp :: GridLayoutInputData {
                         r#col : (65536f64) . clone () as _ , r#colspan : (1f64) . clone () as _ , r#new_row : (false) . clone () as _ , r#row : (65536f64) . clone () as _ , r#rowspan : (1f64) . clone () as _ , }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Horizontal as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 28u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                         (20f64) as _ }
                                     else {
                                         0f64 }
                                    ) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                         (0f64) as _ }
                                     else {
                                         1f64 }
                                    ) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 28u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                     (20f64) as _ }
                                 else {
                                     0f64 }
                                ) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                     (0f64) as _ }
                                 else {
                                     1f64 }
                                ) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 28u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                     (0f64) as _ }
                                 else {
                                     20f64 }
                                ) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (if ((((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get ()) . clone ())) == (((sp :: r#Orientation :: r#Vertical) . clone ())) {
                                     (1f64) as _ }
                                 else {
                                     0f64 }
                                ) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater1 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (16f64) . clone () as _ ;
                                 the_struct . r#end = (8f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater1 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater1 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (16f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_bottom ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_top ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (234f64) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Horizontal as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Vertical as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#organize_grid_layout (sp :: Slice :: from_slice (& [(sp :: GridLayoutInputData {
                         r#col : (65536f64) . clone () as _ , r#colspan : (1f64) . clone () as _ , r#new_row : (false) . clone () as _ , r#row : (65536f64) . clone () as _ , r#rowspan : (1f64) . clone () as _ , }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Horizontal as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                        ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((18f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_layout_4_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater2 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater2 . len () as u32 ;
                         for i in 0 .. _self . repeater2 . len () {
                             if let Some (sub_comp) = _self . repeater2 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (16f64) . clone () as _ ;
                                 the_struct . r#end = (8f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater2 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater2 . len () {
                             if let Some (sub_comp) = _self . repeater2 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                                 the_struct . r#min = (0f64) . clone () as _ ;
                                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                                 the_struct . r#preferred = (0f64) . clone () as _ ;
                                 the_struct . r#stretch = (1f64) . clone () as _ ;
                                 the_struct }
                            ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater2 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                         for i in 0 .. _self . repeater2 . len () {
                             if let Some (sub_comp) = _self . repeater2 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             else {
                                 items_vec . push (:: core :: default :: Default :: default ()) ;
                                 }
                             }
                         items_vec . push ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                                     the_struct . r#max_percent = (100f64) . clone () as _ ;
                                     the_struct . r#min = (0f64) . clone () as _ ;
                                     the_struct . r#min_percent = (0f64) . clone () as _ ;
                                     the_struct . r#preferred = (0f64) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                ) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (1f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (16f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_bottom ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_top ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : (234f64) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Horizontal as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#organized_data : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) . get ()) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (0f64) . clone () as _ ;
                             the_struct . r#end = (0f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone () as _ , r#spacing : (0f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: r#Orientation :: r#Vertical as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#organize_grid_layout (sp :: Slice :: from_slice (& [(sp :: GridLayoutInputData {
                         r#col : (65536f64) . clone () as _ , r#colspan : (1f64) . clone () as _ , r#new_row : (false) . clone () as _ , r#row : (65536f64) . clone () as _ , r#rowspan : (1f64) . clone () as _ , }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Horizontal as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (0f64) . clone () as _ ;
                         the_struct . r#end = (0f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_padding ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_spacing ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_h_enabled ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_v_enabled ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_hdr_enabled ()) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (720f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#image_72 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 87u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#image_72 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 87u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_44_vertical_stretch ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_56_vertical_stretch ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_66_vertical_stretch ()) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Real-Time Microscopy - Scientific Suite")) as sp :: SharedString }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1280f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_awb_enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Auto White Balance (AWB)")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_awb_enabled ()) . apply_pin (_self) . set ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_hdr_enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Live HDR (Local Contrast)")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_hdr_enabled ()) . apply_pin (_self) . set ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4294967295f64) as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: SharedString :: from ("Exposure: ")) . clone ())) + (((sp :: shared_string_from_number (((((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . get ()) . clone ()) as f64) * (((10f64) . clone ()) as f64))) . clone () as f64) . round ()) . clone ()) as f64) / (((10f64) . clone ()) as f64))) as f64)) . clone ()) . as_str ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_changed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . set (args . 0 . clone () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . set ({
                 (3f64) as f32 }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . set ({
                 (0.1f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_x ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_y ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Show Grid Overlay")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . set ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_h_enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Flip Horizontal")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_h_enabled ()) . apply_pin (_self) . set ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_v_enabled ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Flip Vertical")) as sp :: SharedString }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_toggled ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_v_enabled ()) . apply_pin (_self) . set ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerAppWindow :: FIELD_OFFSETS . callback_tracker_root_37_capture_image ()) . apply_pin (_self) . get () ;
                                 (InnerAppWindow :: FIELD_OFFSETS . r#root_37_capture_image ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Capture HD Frame")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 (InnerAppWindow :: FIELD_OFFSETS . callback_tracker_root_37_export_csv ()) . apply_pin (_self) . get () ;
                                 (InnerAppWindow :: FIELD_OFFSETS . r#root_37_export_csv ()) . apply_pin (_self) . call (& ()) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Export Measurements (CSV)")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4278190080f64) as u32))) as slint :: Brush }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((4281545523f64) as u32))) as slint :: Brush }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * (((700f64) . clone ()) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_video_frame ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64) . clone ()) as f64) * ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_padding ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_spacing ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_44_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_56_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_66_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_y ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_background_5_height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_bottom ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_padding_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_x ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#image_72 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerCheckBox_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (x)) ,) ;
             InnerCheckBox_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (x)) ,) ;
             InnerSlider_root_18 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (x)) ,) ;
             InnerCheckBox_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (x)) ,) ;
             InnerCheckBox_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (x)) ,) ;
             InnerCheckBox_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (x)) ,) ;
             InnerButton_root_25 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (x)) ,) ;
             InnerButton_root_25 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . visit (order , visitor) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . visit (order , visitor) }
                 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . visit (order , visitor) }
                 3u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater3 () . apply_pin (_self) . visit (order , visitor) }
                 4u32 ..= 5u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 4u32 , order , visitor) }
                 6u32 ..= 7u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 6u32 , order , visitor) }
                 8u32 ..= 9u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 8u32 , order , visitor) }
                 10u32 ..= 11u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 10u32 , order , visitor) }
                 12u32 ..= 13u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 12u32 , order , visitor) }
                 14u32 ..= 16u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 14u32 , order , visitor) }
                 17u32 ..= 19u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 17u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             {
                 _changed |= InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_42 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_54 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_64 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             {
                 _changed |= InnerAppWindow :: FIELD_OFFSETS . repeater3 () . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_73 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                 }
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . ensure_instantiated () ;
             _changed |= InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . ensure_instantiated () ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (1280f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (1280f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (720f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (720f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 3u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater3 () . apply_pin (_self) . track_instance_changes () ;
                     sp :: IndexRange :: from (_self . repeater3 . range ()) }
                 4u32 ..= 5u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . subtree_range (dyn_index - 4u32) }
                 6u32 ..= 7u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . subtree_range (dyn_index - 6u32) }
                 8u32 ..= 9u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . subtree_range (dyn_index - 8u32) }
                 10u32 ..= 11u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . subtree_range (dyn_index - 10u32) }
                 12u32 ..= 13u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . subtree_range (dyn_index - 12u32) }
                 14u32 ..= 16u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . subtree_range (dyn_index - 14u32) }
                 17u32 ..= 19u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . subtree_range (dyn_index - 17u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 3u32 => {
                     if let Some (instance) = _self . repeater3 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 4u32 ..= 5u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . subtree_component (dyn_index - 4u32 , subtree_index , result) }
                 6u32 ..= 7u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . subtree_component (dyn_index - 6u32 , subtree_index , result) }
                 8u32 ..= 9u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . subtree_component (dyn_index - 8u32 , subtree_index , result) }
                 10u32 ..= 11u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . subtree_component (dyn_index - 10u32 , subtree_index , result) }
                 12u32 ..= 13u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . subtree_component (dyn_index - 12u32 , subtree_index , result) }
                 14u32 ..= 16u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . subtree_component (dyn_index - 14u32 , subtree_index , result) }
                 17u32 ..= 19u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . subtree_component (dyn_index - 17u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((720f64) . clone ()) . clone () as sp :: Coord , ((1280f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((700f64) . clone ()) . clone () as sp :: Coord , ((250f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((10f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((700f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , ((10f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 4u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 7u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 8u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_v ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 9u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 10u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 11u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 28u32 => ((((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 29u32 => ((((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord ,) , 39u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 40u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_v ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 41u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 42u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 43u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [5usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_layout_cache ()) . apply_pin (_self) . get () [4usize]) . clone ()) . clone () as sp :: Coord ,) , 69u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 70u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_v ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_v ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 71u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 72u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_cache_h ()) . apply_pin (_self) . get () [1usize]) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 87u32 => (((((((1f64) . clone ()) as f64) * (((700f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * ((((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 12u32 ..= 19u32 => return InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . item_geometry (index - 12u32 + 1) , 20u32 ..= 27u32 => return InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . item_geometry (index - 20u32 + 1) , 30u32 ..= 37u32 => return InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . item_geometry (index - 30u32 + 1) , 44u32 ..= 51u32 => return InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . item_geometry (index - 44u32 + 1) , 52u32 ..= 59u32 => return InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . item_geometry (index - 52u32 + 1) , 60u32 ..= 67u32 => return InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . item_geometry (index - 60u32 + 1) , 73u32 ..= 79u32 => return InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . item_geometry (index - 73u32 + 1) , 80u32 ..= 86u32 => return InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . item_geometry (index - 80u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#Groupbox , 4u32 => sp :: r#AccessibleRole :: r#Groupbox , 5u32 => sp :: r#AccessibleRole :: r#Groupbox , 9u32 => sp :: r#AccessibleRole :: r#Checkbox , 10u32 => sp :: r#AccessibleRole :: r#Checkbox , 28u32 => sp :: r#AccessibleRole :: r#Text , 29u32 => sp :: r#AccessibleRole :: r#Slider , 41u32 => sp :: r#AccessibleRole :: r#Checkbox , 42u32 => sp :: r#AccessibleRole :: r#Checkbox , 43u32 => sp :: r#AccessibleRole :: r#Checkbox , 71u32 => sp :: r#AccessibleRole :: r#Button , 72u32 => sp :: r#AccessibleRole :: r#Button , 87u32 => sp :: r#AccessibleRole :: r#Image , 9u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessible_role (0) , 12u32 ..= 19u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessible_role (index - 12u32 + 1) , 10u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessible_role (0) , 20u32 ..= 27u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessible_role (index - 20u32 + 1) , 29u32 => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessible_role (0) , 30u32 ..= 37u32 => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessible_role (index - 30u32 + 1) , 41u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessible_role (0) , 44u32 ..= 51u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessible_role (index - 44u32 + 1) , 42u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessible_role (0) , 52u32 ..= 59u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessible_role (index - 52u32 + 1) , 43u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessible_role (0) , 60u32 ..= 67u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessible_role (index - 60u32 + 1) , 71u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessible_role (0) , 73u32 ..= 79u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessible_role (index - 73u32 + 1) , 72u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessible_role (0) , 80u32 ..= 86u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessible_role (index - 80u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Live Optimizations")) , (4u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Alignment & Orientation")) , (5u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (5u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Scientific Tools")) , (9u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (10u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (28u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#text_50 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (29u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (29u32 , sp :: AccessibleStringProperty :: r#Orientation) => sp :: Some (match (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_orientation ()) . apply_pin (_self) . get () {
                     sp :: r#Orientation :: r#Horizontal => sp :: SharedString :: from ("horizontal") , sp :: r#Orientation :: r#Vertical => sp :: SharedString :: from ("vertical") , _ => sp :: SharedString :: default () }
                ) , (29u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (sp :: shared_string_from_number (((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_value ()) . apply_pin (_self) . get ()) as f64)) , (29u32 , sp :: AccessibleStringProperty :: r#ValueMaximum) => sp :: Some (sp :: shared_string_from_number (((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_maximum ()) . apply_pin (_self) . get ()) as f64)) , (29u32 , sp :: AccessibleStringProperty :: r#ValueMinimum) => sp :: Some (sp :: shared_string_from_number (((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15_minimum ()) . apply_pin (_self) . get ()) as f64)) , (29u32 , sp :: AccessibleStringProperty :: r#ValueStep) => sp :: Some (sp :: shared_string_from_number (((InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_value_step ()) . apply_pin (_self) . get ()) as f64)) , (41u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (41u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (41u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (41u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (42u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (42u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (42u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (42u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (43u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if true {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (43u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (43u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (43u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (71u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (71u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (71u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (71u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . get ()) , (72u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (72u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (72u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (72u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text ()) . apply_pin (_self) . get ()) , (9u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessible_string_property (0 , what) , (12u32 ..= 19u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessible_string_property (index - 12u32 + 1 , what) , (10u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessible_string_property (0 , what) , (20u32 ..= 27u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessible_string_property (index - 20u32 + 1 , what) , (29u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessible_string_property (0 , what) , (30u32 ..= 37u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessible_string_property (index - 30u32 + 1 , what) , (41u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessible_string_property (0 , what) , (44u32 ..= 51u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessible_string_property (index - 44u32 + 1 , what) , (42u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessible_string_property (0 , what) , (52u32 ..= 59u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessible_string_property (index - 52u32 + 1 , what) , (43u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessible_string_property (0 , what) , (60u32 ..= 67u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessible_string_property (index - 60u32 + 1 , what) , (71u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessible_string_property (0 , what) , (73u32 ..= 79u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessible_string_property (index - 73u32 + 1 , what) , (72u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessible_string_property (0 , what) , (80u32 ..= 86u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessible_string_property (index - 80u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (9u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (10u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (29u32 , sp :: AccessibilityAction :: r#Decrement) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_decrement ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (29u32 , sp :: AccessibilityAction :: r#Increment) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_increment ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (29u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18_accessible_action_set_value ()) . apply_pin (_self) . call (& ((args . 0 . clone ()) . clone () as _ ,)) ;
                         }
                     }
                 (41u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (42u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (43u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (71u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (72u32 , sp :: AccessibilityAction :: r#Default) => {
                     {
                         (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_accessible_action_default ()) . apply_pin (_self) . call (& ()) ;
                         }
                     }
                 (9u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessibility_action (0 , action) , (12u32 ..= 19u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . accessibility_action (index - 12u32 + 1 , action) , (10u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessibility_action (0 , action) , (20u32 ..= 27u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . accessibility_action (index - 20u32 + 1 , action) , (29u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessibility_action (0 , action) , (30u32 ..= 37u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . accessibility_action (index - 30u32 + 1 , action) , (41u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessibility_action (0 , action) , (44u32 ..= 51u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . accessibility_action (index - 44u32 + 1 , action) , (42u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessibility_action (0 , action) , (52u32 ..= 59u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . accessibility_action (index - 52u32 + 1 , action) , (43u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessibility_action (0 , action) , (60u32 ..= 67u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . accessibility_action (index - 60u32 + 1 , action) , (71u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessibility_action (0 , action) , (73u32 ..= 79u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . accessibility_action (index - 73u32 + 1 , action) , (72u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessibility_action (0 , action) , (80u32 ..= 86u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . accessibility_action (index - 80u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 => sp :: SupportedAccessibilityAction :: r#Default , 10u32 => sp :: SupportedAccessibilityAction :: r#Default , 29u32 => sp :: SupportedAccessibilityAction :: r#Decrement | sp :: SupportedAccessibilityAction :: r#Increment | sp :: SupportedAccessibilityAction :: r#SetValue , 41u32 => sp :: SupportedAccessibilityAction :: r#Default , 42u32 => sp :: SupportedAccessibilityAction :: r#Default , 43u32 => sp :: SupportedAccessibilityAction :: r#Default , 71u32 => sp :: SupportedAccessibilityAction :: r#Default , 72u32 => sp :: SupportedAccessibilityAction :: r#Default , 9u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . supported_accessibility_actions (0) , 12u32 ..= 19u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . supported_accessibility_actions (index - 12u32 + 1) , 10u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . supported_accessibility_actions (0) , 20u32 ..= 27u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . supported_accessibility_actions (index - 20u32 + 1) , 29u32 => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . supported_accessibility_actions (0) , 30u32 ..= 37u32 => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . supported_accessibility_actions (index - 30u32 + 1) , 41u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . supported_accessibility_actions (0) , 44u32 ..= 51u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . supported_accessibility_actions (index - 44u32 + 1) , 42u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . supported_accessibility_actions (0) , 52u32 ..= 59u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . supported_accessibility_actions (index - 52u32 + 1) , 43u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . supported_accessibility_actions (0) , 60u32 ..= 67u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . supported_accessibility_actions (index - 60u32 + 1) , 71u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . supported_accessibility_actions (0) , 73u32 ..= 79u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . supported_accessibility_actions (index - 73u32 + 1) , 72u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . supported_accessibility_actions (0) , 80u32 ..= 86u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . supported_accessibility_actions (index - 80u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 12u32 ..= 19u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . item_element_infos (index - 12u32 + 1) , 20u32 ..= 27u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . item_element_infos (index - 20u32 + 1) , 30u32 ..= 37u32 => InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () . apply_pin (_self) . item_element_infos (index - 30u32 + 1) , 44u32 ..= 51u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . item_element_infos (index - 44u32 + 1) , 52u32 ..= 59u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . item_element_infos (index - 52u32 + 1) , 60u32 ..= 67u32 => InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . item_element_infos (index - 60u32 + 1) , 73u32 ..= 79u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . item_element_infos (index - 73u32 + 1) , 80u32 ..= 86u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . item_element_infos (index - 80u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_empty_38_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_39_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_rectangle_71_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_39_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_40_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_52_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_62_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_39_alignment ()) . apply_pin (_self) . get () as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_40_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_41_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_41_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                 InnerAppWindow :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater0 . len () {
                     if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 items_vec . push ({
                     let mut the_struct = sp :: LayoutItemInfo :: default () ;
                     the_struct . r#constraint = ({
                         let r#layout_info = (_self . r#fn_rectangle_44_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_44_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone () as _ ;
                     the_struct }
                ) ;
                 let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                 sp :: r#box_layout_info (r#cells as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                     let mut the_struct = sp :: Padding :: default () ;
                     the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_top ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_padding_bottom ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct }
                 as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_45_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_46_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_45_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_46_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_49_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_46_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_52_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_53_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_53_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater1 . len ()) ;
                 InnerAppWindow :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater1 . len () {
                     if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 items_vec . push ({
                     let mut the_struct = sp :: LayoutItemInfo :: default () ;
                     the_struct . r#constraint = ({
                         let r#layout_info = (_self . r#fn_rectangle_56_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_56_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone () as _ ;
                     the_struct }
                ) ;
                 let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                 sp :: r#box_layout_info (r#cells as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                     let mut the_struct = sp :: Padding :: default () ;
                     the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_top ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_padding_bottom ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct }
                 as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_57_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_58_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_57_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_58_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_58_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_62_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_63_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_63_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater2 . len ()) ;
                 InnerAppWindow :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . track_instance_changes () ;
                 for i in 0 .. _self . repeater2 . len () {
                     if let Some (sub_comp) = _self . repeater2 . instance_at (i) {
                         items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                         }
                     else {
                         items_vec . push (:: core :: default :: Default :: default ()) ;
                         }
                     }
                 items_vec . push ({
                     let mut the_struct = sp :: LayoutItemInfo :: default () ;
                     the_struct . r#constraint = ({
                         let r#layout_info = (_self . r#fn_rectangle_66_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_rectangle_66_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone () as _ ;
                     the_struct }
                ) ;
                 let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                 sp :: r#box_layout_info (r#cells as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                     let mut the_struct = sp :: Padding :: default () ;
                     the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_top ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_padding_bottom ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                     the_struct }
                 as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_67_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#grid_layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_layout_organized_data ()) . apply_pin (_self) . get () as _ , sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = (_self . r#fn_empty_68_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , sp :: Slice :: from_slice (& []) as _ , sp :: Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_67_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#Orientation :: r#Vertical as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_empty_68_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone () , ({
                 let mut the_struct = sp :: LayoutItemInfo :: default () ;
                 the_struct . r#constraint = ({
                     let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () . apply_pin (_self) . r#fn_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_min_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_vertical_stretch ()) . apply_pin (_self) . get ()) . clone () as _ ;
                         the_struct }
                     }
                ) . clone () as _ ;
                 the_struct }
            ) . clone ()]) as _ , (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_spacing ()) . apply_pin (_self) . get () . get () as _ , & {
                 let mut the_struct = sp :: Padding :: default () ;
                 the_struct . r#begin = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#end = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_68_padding ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct }
             as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             (((((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_37 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + (((_self . r#fn_empty_38_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_44_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_45_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_56_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_57_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_66_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + (((_self . r#fn_empty_67_layoutinfo_v_with_constraint (args . 0 . clone () as _)) . clone ())))) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_rectangle_71_layoutinfo_v_with_constraint (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Coord ,) -> sp :: LayoutInfo {
             let _self = self ;
             let args = (arg_0 ,) ;
             ((((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = (0f64) . clone () as _ ;
                 the_struct . r#stretch = (1f64) . clone () as _ ;
                 the_struct }
            ) . clone ())) + ((({
                 let mut the_struct = sp :: LayoutInfo :: default () ;
                 the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                 the_struct . r#max_percent = (100f64) . clone () as _ ;
                 the_struct . r#min = (0f64) . clone () as _ ;
                 the_struct . r#min_percent = (0f64) . clone () as _ ;
                 the_struct . r#preferred = ((InnerAppWindow :: FIELD_OFFSETS . r#root_37_image_72_preferred_height ()) . apply_pin (_self) . get () . get ()) . clone () as _ ;
                 the_struct . r#stretch = (0f64) . clone () as _ ;
                 the_struct }
            ) . clone ())))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_42 {
         r#text_42 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_42 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_42 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Live Optimizations")) as sp :: SharedString }
            ) ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (234f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_41_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Live Optimizations")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_42 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_42 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_42 :: FIELD_OFFSETS . r#text_42 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_42) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_42 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_42 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_42 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_42 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_42 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_42 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_54 {
         r#text_54 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_54 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_54 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Alignment & Orientation")) as sp :: SharedString }
            ) ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (234f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_53_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Alignment & Orientation")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_54 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_54 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_54 :: FIELD_OFFSETS . r#text_54 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_54) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_54 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_54 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_54 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_54 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 38u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_54 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_54 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_64 {
         r#text_64 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_64 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_64 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_78 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_78 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((600f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Scientific Tools")) as sp :: SharedString }
            ) ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (234f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = (sp :: Item :: layout_info ((InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((r#layout_info) . r#min) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [1usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((234f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_63_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Scientific Tools")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_64 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_64 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_64 :: FIELD_OFFSETS . r#text_64 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_64) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_64 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_64 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_64 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_64 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 68u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_64 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_64 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_73 {
         r#rectangle_73 : sp :: r#BasicBorderRectangle , r#rectangle_74 : sp :: r#Rectangle , r#rectangle_75 : sp :: r#Rectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_73 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_73 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((1426128640f64) as u32))) as slint :: Brush }
            ) ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_74 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((1426128640f64) as u32))) as slint :: Brush }
            ) ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_75 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((1426128640f64) as u32))) as slint :: Brush }
            ) ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_74 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_75 () + sp :: r#Rectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             # ! [allow (unused)] let _self = self ;
             let mut _changed = false ;
             _changed }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((1f64) . clone ()) as f64) * (((700f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get () [3usize]) . unwrap_or_default ()) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => (((((((1f64) . clone ()) as f64) * (((((((1f64) . clone ()) as f64) * (((700f64) . clone ()) as f64))) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((1f64) . clone ()) . clone () as sp :: Coord , ((((((((((1f64) . clone ()) as f64) * (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get () [3usize]) . unwrap_or_default ()) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => (((1f64) . clone ()) . clone () as sp :: Coord , ((((((1f64) . clone ()) as f64) * (((((((1f64) . clone ()) as f64) * (((_self . parent . upgrade () . as_ref () . map (| x | (InnerAppWindow :: FIELD_OFFSETS . r#root_37_empty_38_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get () [3usize]) . unwrap_or_default ()) . clone ()) as f64))) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((((((((((1f64) . clone ()) as f64) * (((700f64) . clone ()) as f64))) . clone ()) as f64) / (((2f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_rectangle_73 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             3usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_73 , sp :: ItemVTable , sp :: AllowPin > ;
             3usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_73 ()) , sp :: VOffset :: new (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_74 ()) , sp :: VOffset :: new (InnerComponent_rectangle_73 :: FIELD_OFFSETS . r#rectangle_75 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_73) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_73 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_73 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_73 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_73 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 88u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_73 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_73 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     impl InnerAppWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             89usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 87u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 38u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 68u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 3u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 9u32 , parent_index : 7u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 5u32 , children_index : 12u32 , parent_index : 8u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 5u32 , children_index : 20u32 , parent_index : 8u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 28u32 , parent_index : 8u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 17u32 , parent_index : 9u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 9u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 9u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 12u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 19u32 , parent_index : 12u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 18u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 25u32 , parent_index : 10u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 10u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 10u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 26u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 30u32 , parent_index : 11u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 30u32 , parent_index : 11u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 29u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 29u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 34u32 , parent_index : 29u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 36u32 , parent_index : 29u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 32u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 32u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 33u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 33u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 40u32 , parent_index : 4u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 41u32 , parent_index : 39u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 5u32 , children_index : 44u32 , parent_index : 40u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 5u32 , children_index : 52u32 , parent_index : 40u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 5u32 , children_index : 60u32 , parent_index : 40u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 49u32 , parent_index : 41u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 41u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 41u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 9u32 , parent_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 51u32 , parent_index : 44u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 51u32 , parent_index : 44u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 50u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 57u32 , parent_index : 42u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 10u32 , parent_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 60u32 , parent_index : 42u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 60u32 , parent_index : 42u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 11u32 , parent_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 59u32 , parent_index : 52u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 59u32 , parent_index : 52u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 60u32 , parent_index : 58u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 65u32 , parent_index : 43u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 12u32 , parent_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 68u32 , parent_index : 43u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 68u32 , parent_index : 43u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 13u32 , parent_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 67u32 , parent_index : 60u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 67u32 , parent_index : 60u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 68u32 , parent_index : 66u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 70u32 , parent_index : 5u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 71u32 , parent_index : 69u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 73u32 , parent_index : 70u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 80u32 , parent_index : 70u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 77u32 , parent_index : 71u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 80u32 , parent_index : 71u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 80u32 , parent_index : 71u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 16u32 , parent_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 80u32 , parent_index : 73u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 14u32 , parent_index : 73u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 15u32 , parent_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 84u32 , parent_index : 72u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 87u32 , parent_index : 72u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 87u32 , parent_index : 72u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 19u32 , parent_index : 72u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 87u32 , parent_index : 80u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 17u32 , parent_index : 80u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 18u32 , parent_index : 80u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 89u32 , parent_index : 2u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             69usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#root_37 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_39 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_71 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_40 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_52 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_62 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_44 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_46 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_49 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_47 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_48 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#text_50 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#root_18 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#rail_19 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#track_20 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_21 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#root_15 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_border_22 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#thumb_inner_23 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#touch_area_16 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#slider_51 () + InnerSlider_root_18 :: FIELD_OFFSETS . r#base_24 () + InnerSliderBase_root_15 :: FIELD_OFFSETS . r#focus_scope_17 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_56 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_58 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_59 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_60 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#background_5 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#border_6 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_visibility_7 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#checkbox_61 () + InnerCheckBox_root_3 :: FIELD_OFFSETS . r#icon_8 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#rectangle_66 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#empty_68 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_69 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_70 () + InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#image_72 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn ensure_instantiated (self : :: core :: pin :: Pin < & Self >) -> bool {
             self . ensure_instantiated () }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_context (ctx : sp :: SlintContext) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_context (ctx) ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_existing_window (window : & slint :: Window) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_existing (window) ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             let window = inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             sp :: WindowInner :: from_pub (window . window ()) . ensure_tree_instantiated () ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_awb_enabled (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_awb_enabled ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_awb_enabled (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_awb_enabled ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_capture_image (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_capture_image ()) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_capture_image (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] (InnerAppWindow :: FIELD_OFFSETS . r#root_37_capture_image ()) . apply_pin (_self) . set_handler (move | args | f ()) ;
             (InnerAppWindow :: FIELD_OFFSETS . callback_tracker_root_37_capture_image ()) . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn invoke_export_csv (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_export_csv ()) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_export_csv (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] (InnerAppWindow :: FIELD_OFFSETS . r#root_37_export_csv ()) . apply_pin (_self) . set_handler (move | args | f ()) ;
             (InnerAppWindow :: FIELD_OFFSETS . callback_tracker_root_37_export_csv ()) . apply_pin (_self) . mark_dirty () ;
             }
         # [allow (dead_code)] pub fn get_exposure_val (& self) -> f32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_exposure_val (& self , value : f32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_exposure_val ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_flip_h_enabled (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_h_enabled ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_flip_h_enabled (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_h_enabled ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_flip_v_enabled (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_v_enabled ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_flip_v_enabled (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_flip_v_enabled ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_grid_enabled (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_grid_enabled (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_grid_enabled ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_hdr_enabled (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_hdr_enabled ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_hdr_enabled (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_hdr_enabled ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_video_frame (& self) -> sp :: Image {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_video_frame ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_video_frame (& self , value : sp :: Image) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_37_video_frame ()) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: StrongHandle for r#AppWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow > ;
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             sp :: WindowInner :: from_pub (self . window ()) . context () . run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         }
     struct SharedGlobals {
         global_FluentPalette_78 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_78 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_FluentPalette_78 : InnerFluentPalette_78 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_FluentPalette_78 . clone () . init (& _self) ;
             _self }
         # [allow (dead_code)] fn clone_with_window_adapter (& self , window_adapter : sp :: WindowAdapterRc) -> sp :: Rc < Self > {
             sp :: Rc :: new (Self {
                 global_FluentPalette_78 : self . global_FluentPalette_78 . clone () , window_adapter : window_adapter . into () , root_item_tree_weak : :: core :: default :: Default :: default () , }
            ) }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         # [cfg (false)] fn create_window_from_context (& self , ctx : sp :: SlintContext) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = ctx . platform () . create_window_adapter () ? ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_context (ctx) ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         # [cfg (false)] fn create_window_from_existing (& self , window : & slint :: Window) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = sp :: WindowInner :: from_pub (window) . window_adapter () ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"11\" height=\"8\" viewBox=\"0 0 11 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.00195312 3.99805C0.00195312 3.86133 0.0507812 3.74414 0.148438 3.64648C0.246094 3.54883 0.363281 3.5 0.5 3.5C0.636719 3.5 0.753906 3.54883 0.851562 3.64648L3.5 6.29492L9.14844 0.646484C9.24609 0.548828 9.36328 0.5 9.5 0.5C9.57031 0.5 9.63477 0.513672 9.69336 0.541016C9.75586 0.564453 9.80859 0.599609 9.85156 0.646484C9.89844 0.689453 9.93555 0.742187 9.96289 0.804688C9.99023 0.863281 10.0039 0.927734 10.0039 0.998047C10.0039 1.13477 9.95312 1.25391 9.85156 1.35547L3.85156 7.35547C3.75391 7.45312 3.63672 7.50195 3.5 7.50195C3.36328 7.50195 3.24609 7.45312 3.14844 7.35547L0.148438 4.35547C0.0507812 4.25781 0.00195312 4.13867 0.00195312 3.99805Z\" fill=\"white\"/>\n</svg>\n" ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;

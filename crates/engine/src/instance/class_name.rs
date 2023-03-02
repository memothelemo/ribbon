use macros::class_name_tree;

#[rustfmt::skip]
class_name_tree!(BaseInstance {
  Accoutrement {
    Accessory,
    Hat
  },
  AdPortal,
  AdService,
  AdvancedDragger,
  AnalyticsService,
  Animation,
  AnimationClip {
    CurveAnimation,
    KeyframeSequence
  },
  AnimationClipProvider,
  AnimationController,
  AnimationFromVideoCreatorService,
  AnimationFromVideoCreatorStudioService,
  AnimationRigData,
  AnimationStreamTrack,
  AnimationTrack,
  Animator,
  AppUpdateService,
  AssetCounterService,
  AssetDeliveryProxy,
  AssetImportService,
  AssetImportSession,
  AssetManagerService,
  AssetPatchSettings,
  AssetService,
  Atmosphere,
  Attachment {
      Bone
  },
  AvatarEditorService,
  AvatarImportService,
  Backpack,
  BadgeService,
  BasePlayerGui {
    CoreGui,
    PlayerGui,
    StarterGui
  },
  BaseWrap {
    WrapLayer,
    WrapTarget
  },
  Beam,
  BindableEvent,
  BindableFunction,
  BodyMover {
    BodyAngularVelocity,
    BodyForce,
    BodyGyro,
    BodyPosition,
    BodyThrust,
    BodyVelocity,
    RocketPropulsion
  },
  Breakpoint,
  BrowserService,
  BulkImportService,
  CacheableContentProvider {
    HSRDataContentProvider,
    MeshContentProvider,
    SolidModelContentProvider
  },
  CalloutService,
  Camera,
  ChangeHistoryService,
  CharacterAppearance {
    BodyColors,
    CharacterMesh,
    Clothing {
      Pants,
      Shirt
    },
    ShirtGraphic,
    Skin
  },
  Chat,
  ClickDetector,
  Clouds,
  ClusterPacketCache,
  CollectionService,
  CommandInstance,
  CommandService,
  Configuration,
  ConfigureServerService,
  Constraint {
    AlignOrientation,
    AlignPosition,
    AngularVelocity,
    AnimationConstraint,
    BallSocketConstraint,
    HingeConstraint,
    LineForce,
    LinearVelocity,
    PlaneConstraint {
      Plane
    },
    RigidConstraint,
    RodConstraint,
    RopeConstraint,
    SlidingBallConstraint {
      CylindricalConstraint,
      PrismaticConstraint
    },
    SpringConstraint,
    Torque,
    TorsionSpringConstraint,
    UniversalConstraint,
    VectorForce,
  },
  ContentProvider,
  ContextActionService,
  Controller {
    HumanoidController,
    SkateboardController,
    VehicleController
  },
  ControllerBase {
    AirController,
    ClimbController,
    GroundController,
    SwimController
  },
  ControllerManager,
  ControllerService,
  CookiesService,
  CorePackages,
  CoreScriptDebuggingManagerHelper,
  CoreScriptSyncService,
  CrossDMScriptChangeListener,
  CustomEvent,
  CustomEventReceiver,
  DataModelMesh {
    BevelMesh {
      BlockMesh,
      CylinderMesh
    },
    FileMesh {
      SpecialMesh
    }
  },
  DataModelPatchService,
  DataModelSession,
  DataStoreIncrementOptions,
  DataStoreInfo,
  DataStoreKey,
  DataStoreKeyInfo,
  DataStoreObjectVersionInfo,
  DataStoreOptions,
  DataStoreService,
  DataStoreSetOptions,
  Debris,
  DebugSettings,
  DebuggablePluginWatcher,
  DebuggerBreakpoint,
  DebuggerConnection {
    LocalDebuggerConnection
  },
  DebuggerConnectionManager,
  DebuggerLuaResponse,
  DebuggerManager,
  DebuggerUIService,
  DebuggerVariable,
  DebuggerWatch,
  DeviceIdService,
  Dialog,
  DialogChoice,
  DraftsService,
  Dragger,
  DraggerService,
  EulerRotationCurve,
  EventIngestService,
  ExperienceAuthService,
  ExperienceInviteOptions,
  Explosion,
  FaceAnimatorService,
  FaceControls,
  FaceInstance {
    Decal {
      Texture
    }
  },
  FacialAnimationRecordingService,
  FacialAnimationStreamingService,
  FacialAnimationStreamingServiceV2,
  Feature {
    Hole,
    MotorFeature
  },
  File,
  Fire,
  FlagStandService,
  FloatCurve,
  FlyweightService {
    CSGDictionaryService,
    NonReplicatedCSGDictionaryService
  },
  Folder,
  ForceField,
  FriendService,
  FunctionalTest,
  GamePassService,
  GameSettings,
  GamepadService,
  Geometry,
  GetTextBoundsParams,
  GlobalDataStore {
    DataStore,
    OrderedDataStore
  },
  GoogleAnalyticsConfiguration,
  GroupService,
  GuiBase {
    GuiBase2d {
      GuiObject {
        CanvasGroup,
        Frame,
        GuiButton {
          ImageButton,
          TextButton
        },
        GuiLabel {
          ImageLabel,
          TextLabel
        },
        ScrollingFrame,
        TextBox,
        VideoFrame,
        ViewportFrame
      },
      LayerCollector {
        BillboardGui,
        PluginGui {
          DockWidgetPluginGui,
          QWidgetPluginGui
        },
        ScreenGui {
          GuiMain
        },
        SurfaceGuiBase {
          AdGui,
          SurfaceGui
        },
        TextureGuiExperimental
      }
    },
    GuiBase3d {
      FloorWire,
      InstanceAdornment {
        SelectionBox
      },
      PVAdornment {
        HandleAdornment {
          BoxHandleAdornment,
          ConeHandleAdornment,
          CylinderHandleAdornment,
          ImageHandleAdornment,
          LineHandleAdornment,
          SphereHandleAdornment,
          WireframeHandleAdornment
        },
        ParabolaAdornment,
        SelectionSphere
      },
      PartAdornment {
        HandlesBase {
          ArcHandles,
          Handles
        },
        SurfaceSelection
      },
      SelectionLasso {
        SelectionPartLasso,
        SelectionPointLasso
      }
    }
  },
  GuiService,
  GuidRegistryService,
  HapticService,
  HeightmapImporterService,
  HiddenSurfaceRemovalAsset,
  Highlight,
  Hopper,
  HttpRbxApiService,
  HttpRequest,
  HttpService,
  Humanoid,
  HumanoidDescription,
  IKControl,
  ILegacyStudioBridge {
    LegacyStudioBridge
  },
  IXPService,
  ImageDataExperimental,
  ImporterBaseSettings {
    ImporterAnimationSettings,
    ImporterFacsSettings,
    ImporterGroupSettings,
    ImporterJointSettings,
    ImporterMaterialSettings,
    ImporterMeshSettings,
    ImporterRootSettings
  },
  IncrementalPatchBuilder,
  InputObject,
  InsertService,
  JointInstance {
    DynamicRotate {
      RotateP,
      RotateV,
    },
    Glue,
    ManualSurfaceJointInstance {
      ManualGlue,
      ManualWeld
    },
    Motor {
      Motor6D
    },
    Rotate,
    Snap,
    VelocityMotor,
    Weld
  },
  JointsService,
  KeyboardService,
  Keyframe,
  KeyframeMarker,
  KeyframeSequenceProvider,
  LSPFileSyncService,
  LanguageService,
  Light {
    PointLight,
    SpotLight,
    SurfaceLight
  },
  Lighting,
  LiveScriptingService,
  LocalStorageService {
    AppStorageService,
    UserStorageService
  },
  LocalizationService,
  LocalizationTable {
    CloudLocalizationTable
  },
  LodDataEntity,
  LodDataService,
  LogService,
  LoginService,
  LuaSettings,
  LuaSourceContainer {
    BaseScript {
      CoreScript,
      Script {
        LocalScript
      }
    },
    ModuleScript
  },
  LuaWebService,
  LuauScriptAnalyzerService,
  MarkerCurve,
  MarketplaceService,
  MaterialService,
  MaterialVariant,
  MemStorageConnection,
  MemStorageService,
  MemoryStoreQueue,
  MemoryStoreService,
  MemoryStoreSortedMap,
  Message { Hint },
  MessageBusConnection,
  MessageBusService,
  MessagingService,
  MetaBreakpoint,
  MetaBreakpointContext,
  MetaBreakpointManager,
  Mouse {
    PlayerMouse,
    PluginMouse
  },
  MouseService,
  MultipleDocumentInterfaceInstance,
  NetworkMarker,
  NetworkPeer {
    NetworkClient,
    NetworkServer
  },
  NetworkReplicator {
    ClientReplicator,
    ServerReplicator
  },
  NetworkSettings,
  NoCollisionConstraint,
  NotificationService,
  PVInstance {
    BasePart {
      CornerWedgePart,
      FormFactorPart {
        Part {
          FlagStand,
          Platform,
          Seat,
          SkateboardPlatform,
          SpawnLocation
        },
        WedgePart
      },
      Terrain,
      TriangleMeshPart {
        MeshPart,
        PartOperation {
          IntersectOperation,
          NegateOperation,
          UnionOperation
        }
      },
      TrussPart,
      VehicleSeat
    },
    Model {
      Actor,
      BackpackItem {
        HopperBin,
        Tool {
          Flag
        }
      },
      Status,
      WorldRoot {
        Workspace,
        WorldModel
      }
    }
  }
});

impl std::fmt::Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::ClassName;

    #[test]
    fn get_base_classes() {
        assert_eq!(
            ClassName::BasePart.base_classes(),
            &[ClassName::PVInstance, ClassName::BaseInstance,]
        );
    }
}

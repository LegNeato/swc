use crate::{extract_class_body_span, Babelify, Context};
use serde_json::value::Value;
use swc_babel_ast::{
    ClassBody, ClassBodyEl, ClassExpression, ClassMethod as BabelClassMethod, ClassMethodKind,
    ClassPrivateMethod, ClassPrivateProperty, ClassProperty, Decorator as BabelDecorator,
};
use swc_ecma_ast::{
    Class, ClassMember, ClassMethod, ClassProp, Constructor, Decorator, MethodKind, PrivateMethod,
    PrivateProp,
};

impl Babelify for Class {
    type Output = ClassExpression;

    fn babelify(self, ctx: &Context) -> Self::Output {
        ClassExpression {
            base: ctx.base(self.span),
            decorators: Some(
                self.decorators
                    .iter()
                    .map(|dec| dec.clone().babelify(ctx))
                    .collect(),
            ),
            body: extract_class_body(&self, &ctx),
            super_class: self
                .super_class
                .map(|expr| Box::new(expr.babelify(ctx).into())),
            type_parameters: self.type_params.map(|param| param.babelify(ctx).into()),
            super_type_parameters: self
                .super_type_params
                .map(|param| param.babelify(ctx).into()),
            implements: Some(
                self.implements
                    .iter()
                    .map(|imp| imp.clone().babelify(ctx).into())
                    .collect(),
            ),
            id: Default::default(),
            mixins: Default::default(),
        }
    }
}

impl Babelify for ClassMember {
    type Output = ClassBodyEl;

    fn babelify(self, ctx: &Context) -> Self::Output {
        match self {
            ClassMember::Constructor(c) => ClassBodyEl::Method(c.babelify(ctx)),
            ClassMember::Method(m) => ClassBodyEl::Method(m.babelify(ctx)),
            ClassMember::PrivateMethod(m) => ClassBodyEl::PrivateMethod(m.babelify(ctx)),
            ClassMember::ClassProp(p) => ClassBodyEl::Prop(p.babelify(ctx)),
            ClassMember::PrivateProp(p) => ClassBodyEl::PrivateProp(p.babelify(ctx)),
            ClassMember::TsIndexSignature(s) => ClassBodyEl::TSIndex(s.babelify(ctx)),
            ClassMember::Empty(_) => panic!(
                "illegal conversion: Cannot convert {:?} to ClassBodyEl",
                &self
            ),
        }
    }
}

fn extract_class_body(class: &Class, ctx: &Context) -> ClassBody {
    ClassBody {
        base: ctx.base(extract_class_body_span(&class, &ctx)),
        body: class
            .body
            .iter()
            .map(|mem| mem.clone().babelify(ctx))
            .collect(),
    }
}

impl Babelify for ClassProp {
    type Output = ClassProperty;

    fn babelify(self, ctx: &Context) -> Self::Output {
        ClassProperty {
            base: ctx.base(self.span),
            key: self.key.babelify(ctx).into(),
            value: self.value.map(|val| Box::new(val.babelify(ctx).into())),
            type_annotation: self.type_ann.map(|ann| Box::new(ann.babelify(ctx).into())),
            is_static: Some(self.is_static),
            decorators: Some(
                self.decorators
                    .iter()
                    .map(|dec| dec.clone().babelify(ctx))
                    .collect(),
            ),
            computed: Some(self.computed),
            accessibility: self.accessibility.map(|access| access.babelify(ctx)),
            is_abstract: Some(self.is_abstract),
            optional: Some(self.is_optional),
            readonly: Some(self.readonly),
            declare: Some(self.declare),
            definite: Some(self.definite),
        }
    }
}

impl Babelify for PrivateProp {
    type Output = ClassPrivateProperty;

    fn babelify(self, ctx: &Context) -> Self::Output {
        ClassPrivateProperty {
            base: ctx.base(self.span),
            key: self.key.babelify(ctx),
            value: self.value.map(|expr| Box::new(expr.babelify(ctx).into())),
            type_annotation: self.type_ann.map(|ann| Box::new(ann.babelify(ctx).into())),
            static_any: Value::Bool(self.is_static),
            decorators: Some(
                self.decorators
                    .iter()
                    .map(|dec| dec.clone().babelify(ctx))
                    .collect(),
            ),
        }
    }
}

impl Babelify for ClassMethod {
    type Output = BabelClassMethod;

    fn babelify(self, ctx: &Context) -> Self::Output {
        BabelClassMethod {
            base: ctx.base(self.span),
            key: self.key.babelify(ctx).into(),
            kind: Some(self.kind.babelify(ctx)),
            is_static: Some(self.is_static),
            access: self.accessibility.map(|access| access.babelify(ctx)),
            accessibility: self.accessibility.map(|access| access.babelify(ctx)),
            is_abstract: Some(self.is_abstract),
            optional: Some(self.is_optional),
            params: self
                .function
                .params
                .iter()
                .map(|param| param.clone().babelify(ctx))
                .collect(),
            body: self.function.body.unwrap().babelify(ctx),
            generator: Some(self.function.is_generator),
            is_async: Some(self.function.is_async),
            decorators: Some(
                self.function
                    .decorators
                    .iter()
                    .map(|dec| dec.clone().babelify(ctx))
                    .collect(),
            ),
            type_parameters: self.function.type_params.map(|t| t.babelify(ctx).into()),
            return_type: self
                .function
                .return_type
                .map(|t| Box::new(t.babelify(ctx).into())),
            computed: Default::default(),
        }
    }
}

impl Babelify for PrivateMethod {
    type Output = ClassPrivateMethod;

    fn babelify(self, ctx: &Context) -> Self::Output {
        ClassPrivateMethod {
            base: ctx.base(self.span),
            key: self.key.babelify(ctx),
            kind: Some(self.kind.babelify(ctx)),
            is_static: Some(self.is_static),
            access: self.accessibility.map(|access| access.babelify(ctx)),
            accessibility: self.accessibility.map(|access| access.babelify(ctx)),
            is_abstract: Some(self.is_abstract),
            optional: Some(self.is_optional),
            params: self
                .function
                .params
                .iter()
                .map(|param| param.clone().babelify(ctx))
                .collect(),
            body: self.function.body.unwrap().babelify(ctx),
            generator: Some(self.function.is_generator),
            is_async: Some(self.function.is_async),
            decorators: Some(
                self.function
                    .decorators
                    .iter()
                    .map(|dec| dec.clone().babelify(ctx))
                    .collect(),
            ),
            type_parameters: self.function.type_params.map(|t| t.babelify(ctx).into()),
            return_type: self
                .function
                .return_type
                .map(|t| Box::new(t.babelify(ctx).into())),
            computed: Default::default(),
        }
    }
}

impl Babelify for Constructor {
    type Output = BabelClassMethod;

    fn babelify(self, ctx: &Context) -> Self::Output {
        BabelClassMethod {
            base: ctx.base(self.span),
            kind: Some(ClassMethodKind::Constructor),
            key: self.key.babelify(ctx).into(),
            params: self
                .params
                .iter()
                .map(|param| param.clone().babelify(ctx))
                .collect(),
            body: self.body.unwrap().babelify(ctx),
            access: self.accessibility.map(|access| access.babelify(ctx)),
            accessibility: self.accessibility.map(|access| access.babelify(ctx)),
            optional: Some(self.is_optional),
            computed: Default::default(),
            is_static: Default::default(),
            generator: Default::default(),
            is_async: Default::default(),
            is_abstract: Default::default(),
            decorators: Default::default(),
            return_type: Default::default(),
            type_parameters: Default::default(),
        }
    }
}

impl Babelify for Decorator {
    type Output = BabelDecorator;

    fn babelify(self, ctx: &Context) -> Self::Output {
        BabelDecorator {
            base: ctx.base(self.span),
            expression: Box::new(self.expr.babelify(ctx).into()),
        }
    }
}

impl Babelify for MethodKind {
    type Output = ClassMethodKind;

    fn babelify(self, _ctx: &Context) -> Self::Output {
        match self {
            MethodKind::Method => ClassMethodKind::Method,
            MethodKind::Getter => ClassMethodKind::Get,
            MethodKind::Setter => ClassMethodKind::Set,
        }
    }
}
